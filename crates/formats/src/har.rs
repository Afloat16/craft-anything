//! HTTP Archive (HAR 1.2) captures, read into typed records. No redaction happens here; callers
//! decide what to keep (see the analysis crate's web capture inspector).

use serde::Deserializer;
use serde::de::{self, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HarError {
    #[error("not valid JSON: {0}")]
    BadJson(String),
    #[error("not a HAR capture: {0}")]
    NotHar(&'static str),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Har {
    pub version: Option<String>,
    pub creator: Option<String>,
    pub pages: usize,
    pub entries: Vec<HarEntry>,
    pub declared_entries: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarEntry {
    /// 0-based position in `log.entries`.
    pub ordinal: usize,
    pub started: Option<String>,
    pub time_ms: Option<f64>,
    pub resource_type: Option<String>,
    pub server_ip: Option<String>,
    pub request: HarRequest,
    pub response: HarResponse,
    /// blocked, dns, connect, send, wait, receive, ssl (negative values mean "not applicable").
    pub timings: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarRequest {
    pub method: String,
    pub url: String,
    pub http_version: Option<String>,
    pub headers: Vec<(String, String)>,
    pub cookie_names: Vec<String>,
    pub post: Option<HarBody>,
    pub body_size: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarBody {
    pub mime: Option<String>,
    pub text: Option<String>,
    /// Form parameter names (values are deliberately not surfaced by this type's users).
    pub params: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarResponse {
    pub status: i64,
    pub status_text: Option<String>,
    pub http_version: Option<String>,
    pub headers: Vec<(String, String)>,
    pub cookie_names: Vec<String>,
    pub redirect_url: Option<String>,
    pub content_size: Option<i64>,
    pub mime: Option<String>,
    pub text: Option<String>,
    /// `base64` when `text` is base64-encoded.
    pub encoding: Option<String>,
    pub body_size: Option<i64>,
}

fn s(v: &Value, k: &str) -> Option<String> {
    v.get(k).and_then(Value::as_str).map(String::from)
}

fn pairs(v: Option<&Value>) -> Vec<(String, String)> {
    v.and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|h| Some((h.get("name")?.as_str()?.to_string(), h.get("value").and_then(Value::as_str).unwrap_or_default().to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn names(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array).map(|a| a.iter().filter_map(|c| c.get("name")?.as_str().map(String::from)).collect()).unwrap_or_default()
}

// Validate through normal value deserialization before borrowing raw fragments.
// This retains number-range, Unicode and nesting checks even for discarded values,
// without retaining an allocation for every object, array or string in the capture.
struct CheckedJson;

impl<'de> serde::Deserialize<'de> for CheckedJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Self)
    }
}

impl<'de> Visitor<'de> for CheckedJson {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self, A::Error> {
        while sequence.next_element::<Self>()?.is_some() {}
        Ok(self)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self, A::Error> {
        while map.next_entry::<String, Self>()?.is_some() {}
        Ok(self)
    }
}

fn framing_error() -> serde_json::Error {
    de::Error::custom("invalid JSON object framing")
}

fn whitespace(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&byte| !matches!(byte, b' ' | b'\n' | b'\r' | b'\t')).unwrap_or(bytes.len());
    bytes.get(start..).unwrap_or(&[])
}

// The stream's public byte offset locates the original value without enabling
// a feature that changes how generic JSON objects are deserialized elsewhere.
fn part<T: de::DeserializeOwned>(bytes: &[u8]) -> serde_json::Result<(T, usize)> {
    let mut stream = serde_json::Deserializer::from_slice(bytes).into_iter::<T>();
    let value = stream.next().ok_or_else(framing_error)??;
    Ok((value, stream.byte_offset()))
}

fn fields<'a, const N: usize>(bytes: &'a [u8], names: [&'static str; N]) -> serde_json::Result<[Option<&'a [u8]>; N]> {
    let mut values = [None; N];
    let Some(mut remaining) = whitespace(bytes).strip_prefix(b"{") else { return Ok(values) };
    loop {
        remaining = whitespace(remaining);
        if remaining.starts_with(b"}") {
            return Ok(values);
        }
        let (key, length) = part::<String>(remaining)?;
        remaining = whitespace(remaining.get(length..).ok_or_else(framing_error)?);
        remaining = whitespace(remaining.strip_prefix(b":").ok_or_else(framing_error)?);
        let (_, length) = part::<IgnoredAny>(remaining)?;
        let value = remaining.get(..length).filter(|value| !value.is_empty()).ok_or_else(framing_error)?;
        if let Some((_, slot)) = names.iter().zip(values.iter_mut()).find(|(name, _)| **name == key) {
            // As with the previous JSON value reader, the last duplicate key wins.
            *slot = Some(value);
        }
        remaining = whitespace(remaining.get(length..).ok_or_else(framing_error)?);
        if let Some(after_comma) = remaining.strip_prefix(b",") {
            remaining = after_comma;
        } else if remaining.starts_with(b"}") {
            return Ok(values);
        } else {
            return Err(framing_error());
        }
    }
}

struct ArrayCount;

impl<'de> Visitor<'de> for ArrayCount {
    type Value = usize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON array")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<usize, A::Error> {
        let mut count = 0usize;
        while sequence.next_element::<IgnoredAny>()?.is_some() {
            count = count.checked_add(1).ok_or_else(|| de::Error::custom("too many HAR pages"))?;
        }
        Ok(count)
    }
}

struct Entries {
    maximum: usize,
}

impl<'de> Visitor<'de> for Entries {
    type Value = Har;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a HAR entry array")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Har, A::Error> {
        let mut har = Har::default();
        loop {
            if har.entries.len() < self.maximum {
                let Some(value) = sequence.next_element::<Value>()? else { break };
                har.entries.push(entry(&value, har.declared_entries));
            } else if sequence.next_element::<IgnoredAny>()?.is_none() {
                break;
            }
            har.declared_entries = har.declared_entries.checked_add(1).ok_or_else(|| de::Error::custom("too many HAR entries"))?;
        }
        har.truncated = har.declared_entries > har.entries.len();
        Ok(har)
    }
}

fn metadata(raw: Option<&[u8]>) -> serde_json::Result<Value> {
    raw.map(serde_json::from_slice).unwrap_or(Ok(Value::Null))
}

fn bad_json(error: serde_json::Error) -> HarError {
    HarError::BadJson(error.to_string())
}

/// Parse a HAR document, allocating typed data only for the first max_entries entries.
/// Discarded entries are still counted, and the complete JSON document is validated.
pub fn parse(json: &[u8], max_entries: usize) -> Result<Har, HarError> {
    let json = json.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(json);
    serde_json::from_slice::<CheckedJson>(json).map_err(bad_json)?;
    let [log] = fields(json, ["log"]).map_err(bad_json)?;
    let log = log.ok_or(HarError::NotHar("no \"log\" object"))?;
    let [version, creator, pages, entries] = fields(log, ["version", "creator", "pages", "entries"]).map_err(bad_json)?;
    let entries = entries.filter(|raw| whitespace(raw).starts_with(b"[")).ok_or(HarError::NotHar("no \"log.entries\" array"))?;
    let mut deserializer = serde_json::Deserializer::from_slice(entries);
    let mut har = deserializer.deserialize_seq(Entries { maximum: max_entries }).map_err(bad_json)?;
    har.version = metadata(version).map_err(bad_json)?.as_str().map(String::from);
    har.creator = creator
        .map(|raw| {
            let creator = metadata(Some(raw))?;
            Ok(format!("{} {}", s(&creator, "name").unwrap_or_default(), s(&creator, "version").unwrap_or_default()).trim().to_string())
        })
        .transpose()
        .map_err(bad_json)?;
    if let Some(pages) = pages.filter(|raw| whitespace(raw).starts_with(b"[")) {
        har.pages = serde_json::Deserializer::from_slice(pages).deserialize_seq(ArrayCount).map_err(bad_json)?;
    }
    Ok(har)
}

fn entry(e: &Value, i: usize) -> HarEntry {
    let req = e.get("request").unwrap_or(&Value::Null);
    let res = e.get("response").unwrap_or(&Value::Null);
    let content = res.get("content").unwrap_or(&Value::Null);
    let post = req.get("postData").filter(|p| p.is_object()).map(|p| HarBody {
        mime: s(p, "mimeType"),
        text: s(p, "text"),
        params: p
            .get("params")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|x| Some((x.get("name")?.as_str()?.to_string(), s(x, "value")))).collect())
            .unwrap_or_default(),
    });
    let timings =
        e.get("timings").and_then(Value::as_object).map(|m| m.iter().filter_map(|(k, v)| Some((k.clone(), v.as_f64()?))).collect()).unwrap_or_default();
    HarEntry {
        ordinal: i,
        started: s(e, "startedDateTime"),
        time_ms: e.get("time").and_then(Value::as_f64),
        resource_type: s(e, "_resourceType"),
        server_ip: s(e, "serverIPAddress"),
        request: HarRequest {
            method: s(req, "method").unwrap_or_default(),
            url: s(req, "url").unwrap_or_default(),
            http_version: s(req, "httpVersion"),
            headers: pairs(req.get("headers")),
            cookie_names: names(req.get("cookies")),
            post,
            body_size: req.get("bodySize").and_then(Value::as_i64),
        },
        response: HarResponse {
            status: res.get("status").and_then(Value::as_i64).unwrap_or(0),
            status_text: s(res, "statusText"),
            http_version: s(res, "httpVersion"),
            headers: pairs(res.get("headers")),
            cookie_names: names(res.get("cookies")),
            redirect_url: s(res, "redirectURL").filter(|u| !u.is_empty()),
            content_size: content.get("size").and_then(Value::as_i64),
            mime: s(content, "mimeType"),
            text: s(content, "text"),
            encoding: s(content, "encoding"),
            body_size: res.get("bodySize").and_then(Value::as_i64),
        },
        timings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_har_with_a_utf8_bom() {
        let capture = br#"{"log":{"entries":[]}}"#;
        let mut encoded = vec![0xef, 0xbb, 0xbf];
        encoded.extend_from_slice(capture);
        assert_eq!(parse(&encoded, 1).unwrap(), parse(capture, 1).unwrap());
        assert!(parse(&[0xef, 0xbb], 1).is_err());
    }

    #[test]
    fn parses_minimal_har() {
        let j = br#"{"log":{"version":"1.2","creator":{"name":"x","version":"1"},"entries":[
          {"startedDateTime":"2026-01-01T00:00:00Z","time":12.5,
           "request":{"method":"POST","url":"https://api.example.test/v1/items/42","headers":[{"name":"Cookie","value":"s=1"}],"cookies":[{"name":"s","value":"1"}],
             "postData":{"mimeType":"application/json","text":"{\"a\":1}"}},
           "response":{"status":201,"headers":[],"content":{"size":2,"mimeType":"application/json","text":"{}"}},
           "timings":{"wait":10,"receive":2.5}}]}}"#;
        let h = parse(j, 10).unwrap();
        assert_eq!(h.entries.len(), 1);
        let e = &h.entries[0];
        assert_eq!(e.request.method, "POST");
        assert_eq!(e.response.status, 201);
        assert_eq!(e.request.cookie_names, vec!["s"]);
        assert_eq!(e.timings.len(), 2);
    }

    #[test]
    fn rejects_non_har() {
        assert!(parse(b"{}", 1).is_err());
        assert!(parse(b"nope", 1).is_err());
        assert!(parse(br#"{"log":{"entries":[1,null,"x"]}}"#, 10).is_ok());
    }
}
