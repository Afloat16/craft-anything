use craft_anything_formats::har::{self, HarError};
use serde_json::json;

fn capture() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "ignored": {"nested": [true, false, null, -5, 1.25]},
        "log": {
            "version": "1.2",
            "creator": {"name": "Synthetic recorder", "version": "7"},
            "pages": [{"id": "first"}, null, {"id": "third"}],
            "entries": [
                {
                    "startedDateTime": "2026-01-01T01:02:03.000Z", "time": 12.5,
                    "_resourceType": "fetch", "serverIPAddress": "192.0.2.1",
                    "request": {
                        "method": "POST", "url": "https://example.test/items/1",
                        "httpVersion": "HTTP/2",
                        "headers": [{"name": "Content-Type", "value": "application/json"}, {"name": "empty"}],
                        "cookies": [{"name": "session", "value": "synthetic"}],
                        "postData": {"mimeType": "application/json", "text": "{\"label\":\"中文\"}",
                            "params": [{"name": "label", "value": "中文"}, {"name": "file"}]},
                        "bodySize": 32
                    },
                    "response": {
                        "status": 201, "statusText": "Created", "httpVersion": "HTTP/2",
                        "headers": [{"name": "X-Trace", "value": "fixture"}],
                        "cookies": [{"name": "result"}], "redirectURL": "",
                        "content": {"size": 2, "mimeType": "application/json", "text": "e30=", "encoding": "base64"},
                        "bodySize": -1
                    },
                    "timings": {"wait": 10, "receive": 2.5, "ssl": -1, "unknown": "ignored"}
                },
                null,
                {"request": {"method": "GET", "url": "https://example.test/third"},
                 "response": {"status": 200, "redirectURL": "https://example.test/next",
                     "content": {"text": "x".repeat(64 * 1024)}}}
            ]
        }
    }))
    .unwrap()
}

#[test]
fn limited_inventory_keeps_complete_count_and_exact_retained_records() {
    let bytes = capture();
    let full = har::parse(&bytes, usize::MAX).unwrap();
    assert_eq!(full.version.as_deref(), Some("1.2"));
    assert_eq!(full.creator.as_deref(), Some("Synthetic recorder 7"));
    assert_eq!(full.pages, 3);
    assert_eq!(full.declared_entries, 3);
    assert!(!full.truncated);
    assert_eq!(full.entries[0].request.method, "POST");
    assert_eq!(full.entries[0].request.headers[1], ("empty".into(), String::new()));
    assert_eq!(full.entries[0].request.cookie_names, ["session"]);
    assert_eq!(full.entries[0].request.post.as_ref().unwrap().params[1], ("file".into(), None));
    assert_eq!(full.entries[0].response.encoding.as_deref(), Some("base64"));
    assert_eq!(full.entries[0].response.body_size, Some(-1));
    assert_eq!(full.entries[0].response.redirect_url, None);
    assert_eq!(full.entries[0].timings, [("receive".into(), 2.5), ("ssl".into(), -1.0), ("wait".into(), 10.0)]);
    assert_eq!(full.entries[1].ordinal, 1);
    assert!(full.entries[1].request.url.is_empty());
    assert_eq!(full.entries[2].response.text.as_ref().unwrap().len(), 64 * 1024);
    for maximum in [0, 1, 2, 3, 4, usize::MAX] {
        let mut expected = full.clone();
        expected.entries.truncate(maximum);
        expected.truncated = maximum < full.entries.len();
        assert_eq!(har::parse(&bytes, maximum).unwrap(), expected, "limit {maximum}");
    }
}

#[test]
fn empty_and_exact_limits_do_not_claim_truncation() {
    for maximum in [0, 1, usize::MAX] {
        let result = har::parse(br#"{"log":{"entries":[]}}"#, maximum).unwrap();
        assert_eq!(result.declared_entries, 0);
        assert!(result.entries.is_empty());
        assert!(!result.truncated);
    }
    assert!(!har::parse(br#"{"log":{"entries":[{}]}}"#, 1).unwrap().truncated);
    assert!(har::parse(br#"{"log":{"entries":[{},{}]}}"#, 1).unwrap().truncated);
}

#[test]
fn duplicate_root_and_log_keys_keep_the_last_value() {
    let bytes = br#"{
        "log":{"entries":[{"request":{"method":"discarded"}}]},
        "log":{"version":"old","version":"1.2","creator":null,"pages":[1,2],"pages":[3],
               "entries":false,"entries":[{"request":{"method":"GET","method":"POST"}}]}
    }"#;
    let result = har::parse(bytes, 1).unwrap();
    assert_eq!(result.version.as_deref(), Some("1.2"));
    assert_eq!(result.creator.as_deref(), Some(""));
    assert_eq!(result.pages, 1);
    assert_eq!(result.declared_entries, 1);
    assert_eq!(result.entries[0].request.method, "POST");
    assert!(!result.truncated);
    assert!(matches!(har::parse(br#"{"log":{"entries":[]},"log":null}"#, 1), Err(HarError::NotHar("no \"log.entries\" array"))));
    assert!(matches!(har::parse(br#"{"log":{"entries":[],"entries":{}}}"#, 1), Err(HarError::NotHar("no \"log.entries\" array"))));
}

#[test]
fn escaped_keys_and_bom_keep_metadata_and_entries() {
    let bytes = br#"{"lo\u0067":{"vers\u0069on":"1.2","cre\u0061tor":{"name":"fixture"},"pag\u0065s":[{}],"entri\u0065s":[{"request":{"url":"https://example.test/\u4e2d"}}]}}"#;
    let plain = har::parse(bytes, 1).unwrap();
    assert_eq!(plain.entries[0].request.url, "https://example.test/中");
    assert_eq!(plain.version.as_deref(), Some("1.2"));
    assert_eq!(plain.creator.as_deref(), Some("fixture"));
    assert_eq!(plain.pages, 1);
    let mut prefixed = vec![0xef, 0xbb, 0xbf];
    prefixed.extend_from_slice(bytes);
    assert_eq!(har::parse(&prefixed, 1).unwrap(), plain);
}

#[test]
fn invalid_json_after_the_retained_prefix_is_still_rejected() {
    for input in [
        r#"{"log":{"entries":[{},{"tail":[1,]}]}}"#,
        r#"{"log":{"entries":[{}]}} trailing"#,
        r#"{"log":{"entries":[{}]},"ignored":{"unfinished":"#,
        r#"{"log":{"entries":[{},{"number":1e400}]}}"#,
        r#"{"log":{"entries":[]},"ignored":"\uD800"}"#,
        r#"{"log":{"entries":[]},"\uD800":0}"#,
    ] {
        for maximum in [0, 1] {
            assert!(matches!(har::parse(input.as_bytes(), maximum), Err(HarError::BadJson(_))), "{input}");
        }
    }
    let deep = format!("{{\"log\":{{\"entries\":[]}},\"ignored\":{}0{}}}", "[".repeat(130), "]".repeat(130));
    assert!(matches!(har::parse(deep.as_bytes(), 0), Err(HarError::BadJson(_))));
}

#[test]
fn invalid_overwritten_values_are_still_validated() {
    for input in [
        r#"{"log":{"entries":[{"number":1e400}]},"log":{"entries":[]}}"#,
        r#"{"log":{"entries":[{"bad":"\uD800"}],"entries":[]}}"#,
        r#"{"log":{"entries":[]},"ignored":1e400,"ignored":0}"#,
    ] {
        assert!(matches!(har::parse(input.as_bytes(), 0), Err(HarError::BadJson(_))));
    }
}

#[test]
fn non_har_shapes_keep_their_original_error_category() {
    for input in [r#"null"#, r#"[]"#, r#"42"#, r#"{}"#] {
        assert!(matches!(har::parse(input.as_bytes(), 0), Err(HarError::NotHar("no \"log\" object"))));
    }
    for input in [r#"{"log":null}"#, r#"{"log":[]}"#, r#"{"log":{"entries":null}}"#, r#"{"log":{"entries":{}}}"#] {
        assert!(matches!(har::parse(input.as_bytes(), 0), Err(HarError::NotHar("no \"log.entries\" array"))));
    }
    let result = har::parse(br#"{"log":{"version":[],"creator":7,"pages":{},"entries":[1,null,"x"]}}"#, 3).unwrap();
    assert_eq!(result.version, None);
    assert_eq!(result.creator.as_deref(), Some(""));
    assert_eq!(result.pages, 0);
    assert_eq!(result.entries.len(), 3);
    assert_eq!(result.declared_entries, 3);
}

#[test]
fn property_order_and_discarded_large_bodies_do_not_change_the_retained_record() {
    let large = "中文\\\"payload".repeat(10_000);
    let bytes = serde_json::to_vec(&json!({
        "after": {"payload": large},
        "log": {
            "entries": [
                {"request": {"method": "GET", "url": "https://example.test/kept"}},
                {"request": {"postData": {"text": large}}, "response": {"content": {"text": large}}}
            ],
            "creator": {"version": "9", "name": "recorder"}, "version": "1.2", "pages": [large]
        }
    }))
    .unwrap();
    let result = har::parse(&bytes, 1).unwrap();
    assert_eq!(result.creator.as_deref(), Some("recorder 9"));
    assert_eq!(result.version.as_deref(), Some("1.2"));
    assert_eq!(result.pages, 1);
    assert_eq!(result.declared_entries, 2);
    assert!(result.truncated);
    assert_eq!(result.entries[0].request.url, "https://example.test/kept");
    assert!(result.entries[0].response.text.is_none());
}

#[test]
fn serde_internal_looking_keys_remain_ordinary_capture_data() {
    for marker in [json!("{\"request\":{\"method\":\"POST\"}}"), json!("not json"), json!(7), json!(null)] {
        let bytes = serde_json::to_vec(&json!({
            "log": {
                "entries": [{"$serde_json::private::RawValue": marker}],
                "creator": {"$serde_json::private::RawValue": marker},
                "version": {"$serde_json::private::RawValue": marker}
            }
        }))
        .unwrap();
        let result = har::parse(&bytes, 1).unwrap();
        assert_eq!(result.declared_entries, 1);
        assert!(result.entries[0].request.method.is_empty());
        assert!(result.entries[0].response.text.is_none());
        assert_eq!(result.creator.as_deref(), Some(""));
        assert_eq!(result.version, None);
    }
}
