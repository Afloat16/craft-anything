//! A tiny static file server on loopback, for previewing and checking a built website.
//!
//! GET/HEAD only, one short-lived connection at a time per request thread, paths confined to the
//! root (no `..`, symlinks resolved and re-checked), unknown extension-less paths fall back to
//! `index.html` (single-page apps). It stops when the [`StaticServer`] is dropped.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub struct StaticServer {
    pub port: u16,
    pub root: PathBuf,
    stop: Arc<AtomicBool>,
}

impl StaticServer {
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.port)
    }
}

impl Drop for StaticServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// Serve `root` on `127.0.0.1` at a free port.
pub fn serve_dir(root: &Path) -> std::io::Result<StaticServer> {
    let root = std::fs::canonicalize(root)?;
    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let port = listener.local_addr()?.port();
    let stop = Arc::new(AtomicBool::new(false));
    let (r, s) = (root.clone(), stop.clone());
    std::thread::Builder::new().name("static-server".into()).spawn(move || {
        while !s.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let r = r.clone();
                    let _ = std::thread::Builder::new().name("static-conn".into()).spawn(move || {
                        let _ = handle(stream, &r);
                    });
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(15)),
                Err(_) => std::thread::sleep(Duration::from_millis(50)),
            }
        }
    })?;
    Ok(StaticServer { port, root, stop })
}

fn content_type(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("html" | "htm") => "text/html; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json" | "map") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("wasm") => "application/wasm",
        Some("txt") => "text/plain; charset=utf-8",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        _ => "application/octet-stream",
    }
}

fn percent_decode(s: &str) -> Option<String> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b.get(i) {
            Some(b'%') => {
                let hex = s.get(i + 1..i + 3)?;
                out.push(u8::from_str_radix(hex, 16).ok()?);
                i += 3;
            }
            Some(c) => {
                out.push(*c);
                i += 1;
            }
            None => break,
        }
    }
    String::from_utf8(out).ok()
}

/// Map a request path to a file under `root`, or `None` when it is outside or missing.
pub fn resolve(root: &Path, url_path: &str) -> Option<PathBuf> {
    let path = url_path.split(['?', '#']).next().unwrap_or("/");
    let decoded = percent_decode(path)?;
    let mut rel = PathBuf::new();
    for c in Path::new(decoded.trim_start_matches('/')).components() {
        match c {
            Component::Normal(n) => rel.push(n),
            Component::CurDir => {}
            _ => return None,
        }
    }
    let mut candidate = root.join(&rel);
    if candidate.is_dir() {
        candidate = candidate.join("index.html");
    }
    // A static export may write a page as `about.html` (Next.js without trailing slashes).
    if !candidate.is_file() && rel.extension().is_none() {
        let page = root.join(&rel).with_extension("html");
        candidate = if !rel.as_os_str().is_empty() && page.is_file() { page } else { root.join("index.html") };
    }
    let real = std::fs::canonicalize(&candidate).ok()?;
    (real.starts_with(root) && real.is_file()).then_some(real)
}

fn request_head(mut stream: &TcpStream) -> Result<String, u16> {
    const MAX_HEAD: usize = 16 * 1024;
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    stream.set_read_timeout(Some(Duration::from_millis(100))).map_err(|_| 400u16)?;
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        if std::time::Instant::now() >= deadline {
            return Err(408);
        }
        let before = bytes.len();
        match stream.read(&mut chunk) {
            Ok(0) => return Err(400),
            Ok(n) => bytes.extend_from_slice(chunk.get(..n).unwrap_or_default()),
            Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut | std::io::ErrorKind::Interrupted) => continue,
            Err(_) => return Err(400),
        }
        let from = before.saturating_sub(3);
        if let Some(end) = bytes.get(from..).and_then(|b| b.windows(4).position(|w| w == b"\r\n\r\n")).map(|i| from + i + 4) {
            if end > MAX_HEAD {
                return Err(431);
            }
            return bytes.get(..end).and_then(|b| std::str::from_utf8(b).ok()).map(String::from).ok_or(400);
        }
        if bytes.len() >= MAX_HEAD {
            return Err(431);
        }
    }
}

fn valid_header(line: &str) -> bool {
    line.split_once(':').is_some_and(|(name, value)| {
        !name.is_empty()
            && name.bytes().all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
            && !value.bytes().any(|b| b.is_ascii_control() && b != b'\t')
    })
}

fn valid_target(target: &str) -> bool {
    let mut bytes = target.bytes();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            if !bytes.next().is_some_and(|b| b.is_ascii_hexdigit()) || !bytes.next().is_some_and(|b| b.is_ascii_hexdigit()) {
                return false;
            }
        } else if !b.is_ascii_alphanumeric() && !b"-._~!$&'()*+,;=:@/?".contains(&b) {
            return false;
        }
    }
    true
}

fn valid_host(value: &str) -> bool {
    let value = value.trim_matches([' ', '\t']);
    let name_byte = |b: u8| b.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=".contains(&b);
    if let Some(literal) = value.strip_prefix('[') {
        let Some((host, suffix)) = literal.split_once(']') else { return false };
        let future = host.strip_prefix(['v', 'V']).and_then(|s| s.split_once('.')).is_some_and(|(version, address)| {
            !version.is_empty() && version.bytes().all(|b| b.is_ascii_hexdigit()) && !address.is_empty() && address.bytes().all(|b| name_byte(b) || b == b':')
        });
        return (host.parse::<std::net::Ipv6Addr>().is_ok() || future)
            && (suffix.is_empty() || suffix.strip_prefix(':').is_some_and(|port| port.bytes().all(|b| b.is_ascii_digit())));
    }
    let (host, port) = value.split_once(':').unwrap_or((value, ""));
    if host.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let mut bytes = host.bytes();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            if !bytes.next().is_some_and(|b| b.is_ascii_hexdigit()) || !bytes.next().is_some_and(|b| b.is_ascii_hexdigit()) {
                return false;
            }
        } else if !name_byte(b) {
            return false;
        }
    }
    true
}

fn handle(mut stream: TcpStream, root: &Path) -> std::io::Result<()> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(20)))?;
    let head = match request_head(&stream) {
        Ok(head) => head,
        Err(status) => {
            let reason = match status {
                408 => "Request Timeout",
                431 => "Request Header Fields Too Large",
                _ => "Bad Request",
            };
            return stream.write_all(format!("HTTP/1.1 {status} {reason}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").as_bytes());
        }
    };
    let mut lines = head.split("\r\n");
    let mut parts = lines.next().unwrap_or_default().split(' ');
    let (method, target, version) = (parts.next().unwrap_or_default(), parts.next().unwrap_or_default(), parts.next().unwrap_or_default());
    let mut host_seen = false;
    let headers_valid = lines.take_while(|line| !line.is_empty()).all(|line| {
        if !valid_header(line) {
            return false;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("host")
        {
            return !std::mem::replace(&mut host_seen, true) && valid_host(value);
        }
        true
    });
    if parts.next().is_some()
        || !matches!(version, "HTTP/1.0" | "HTTP/1.1")
        || method.is_empty()
        || !method.bytes().all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
        || !target.starts_with('/')
        || !valid_target(target)
        || !headers_valid
        || (version == "HTTP/1.1" && !host_seen)
    {
        return stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    }
    if method != "GET" && method != "HEAD" {
        return stream.write_all(b"HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    }
    match resolve(root, target).and_then(|p| std::fs::read(&p).ok().map(|b| (p, b))) {
        Some((p, body)) => {
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
                content_type(&p),
                body.len()
            );
            stream.write_all(head.as_bytes())?;
            if method == "GET" {
                stream.write_all(&body)?;
            }
            Ok(())
        }
        None => stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\nConnection: close\r\n\r\nNot found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_values_follow_uri_authority_syntax() {
        for host in ["localhost", "localhost:", "127.0.0.1:80", "[::1]", "[::1]:80", "[v1.local:host]:80", "[VA.host]", "local%68ost", "\t localhost:80 \t"] {
            assert!(valid_host(host), "{host}");
        }
        for host in [
            "",
            ":80",
            "a b",
            "user@localhost",
            "localhost/",
            "localhost#x",
            "localhost:port",
            "::1",
            "[::1",
            "[::1]extra",
            "[v.host]",
            "[v1.]",
            "host%",
            "host%2",
            "host%GG",
        ] {
            assert!(!valid_host(host), "{host}");
        }
    }

    fn get(port: u16, path: &str) -> String {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        s.write_all(format!("GET {path} HTTP/1.1\r\nHost: x\r\n\r\n").as_bytes()).unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    }

    #[test]
    fn serves_files_spa_fallback_and_refuses_escapes() {
        let dir = tempfile::tempdir().unwrap();
        let site = dir.path().join("dist");
        std::fs::create_dir_all(site.join("assets")).unwrap();
        std::fs::write(site.join("index.html"), "<h1>home</h1>").unwrap();
        std::fs::write(site.join("assets/app.js"), "console.log(1)").unwrap();
        std::fs::write(dir.path().join("secret.txt"), "nope").unwrap();
        let srv = serve_dir(&site).unwrap();
        assert!(get(srv.port, "/").contains("<h1>home</h1>"));
        let js = get(srv.port, "/assets/app.js?v=1");
        assert!(js.contains("text/javascript") && js.contains("console.log(1)"));
        assert!(get(srv.port, "/pricing").contains("<h1>home</h1>"), "client-side routes fall back to index.html");
        std::fs::write(site.join("about.html"), "<h1>about</h1>").unwrap();
        assert!(get(srv.port, "/about").contains("<h1>about</h1>") && get(srv.port, "/about/").contains("<h1>about</h1>"), "an exported page");
        assert!(get(srv.port, "/missing.png").starts_with("HTTP/1.1 404"));
        for bad in ["/../secret.txt", "/%2e%2e/secret.txt", "/assets/../../secret.txt"] {
            assert!(!get(srv.port, bad).contains("nope"), "{bad}");
        }
        let url = srv.url();
        drop(srv);
        assert!(url.starts_with("http://127.0.0.1:"));
    }
}
