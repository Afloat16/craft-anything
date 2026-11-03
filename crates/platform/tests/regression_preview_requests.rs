use craft_anything_platform::serve::serve_dir;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
fn get(port: u16, path: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    s.write_all(format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n\r\n").as_bytes()).unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    out
}

#[test]
fn regression_incomplete_and_oversize_requests_do_not_serve_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "private-preview-content").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    let cases = [
        ("GET /\r\n\r\n".to_string(), 400),
        ("GET / HTTP/2\r\nHost: localhost\r\n\r\n".to_string(), 400),
        ("GET / HTTP/1.1\r\nHost: localhost\r\n".to_string(), 400),
        ("GET / HTTP/1.1\r\nBad Header: x\r\n\r\n".to_string(), 400),
        (format!("GET / HTTP/1.1\r\nHost: localhost\r\nX-Long: {}\r\n\r\n", "a".repeat(17 * 1024)), 431),
    ];
    for (raw, status) in cases {
        let mut s = TcpStream::connect(("127.0.0.1", srv.port)).unwrap();
        s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        s.write_all(raw.as_bytes()).unwrap();
        s.shutdown(std::net::Shutdown::Write).unwrap();
        let mut out = String::new();
        let _ = s.read_to_string(&mut out);
        assert!(out.starts_with(&format!("HTTP/1.1 {status}")), "{out:?}");
        assert!(!out.contains("private-preview-content"));
    }
    assert!(get(srv.port, "/").contains("private-preview-content"));
}

fn request(port: u16, raw: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    stream.write_all(raw.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn regression_request_targets_reject_fragments() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "private-preview-content").unwrap();
    std::fs::write(dir.path().join("index#part.html"), "encoded-hash-file").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    for target in ["/index.html#fragment", "/index.html?query=value#fragment"] {
        let response = get(srv.port, target);
        assert!(response.starts_with("HTTP/1.1 400"), "{target}: {response:?}");
        assert!(!response.contains("private-preview-content"));
    }
    assert!(get(srv.port, "/index%23part.html").contains("encoded-hash-file"));
    assert!(get(srv.port, "/index.html?query=%23value").contains("private-preview-content"));
}

#[test]
fn regression_http11_requires_one_valid_host() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "private-preview-content").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    for headers in [
        "",
        "Host: localhost\r\nHost: localhost\r\n",
        "Host: localhost\r\nhOsT: localhost\r\n",
        "Host:\r\n",
        "Host: \t \r\n",
        "Host: local host\r\n",
        "Host: user@localhost\r\n",
        "Host: localhost/path\r\n",
        "Host: localhost:port\r\n",
        "Host: ::1\r\n",
        "Host: [::1\r\n",
        "Host: [not-an-ip]\r\n",
        "Host: localhost%\r\n",
    ] {
        let response = request(srv.port, &format!("GET / HTTP/1.1\r\n{headers}\r\n"));
        assert!(response.starts_with("HTTP/1.1 400"), "{headers:?}: {response:?}");
        assert!(!response.contains("private-preview-content"));
    }
    for host in ["localhost".to_string(), format!("127.0.0.1:{}", srv.port), format!("[::1]:{}", srv.port)] {
        let response = request(srv.port, &format!("GET / HTTP/1.1\r\nhOsT:\t {host}\t\r\n\r\n"));
        assert!(response.starts_with("HTTP/1.1 200") && response.contains("private-preview-content"), "{host}: {response:?}");
    }
    let response = request(srv.port, "GET / HTTP/1.0\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 200") && response.contains("private-preview-content"));
}

#[test]
fn regression_incomplete_head_hits_deadline_without_client_eof() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "private-preview-content").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    // Keep the write side open: only the whole-head deadline can finish this request.
    let started = std::time::Instant::now();
    let response = request(srv.port, "GET / HTTP/1.1\r\nHost: localhost\r\n");
    assert!(response.starts_with("HTTP/1.1 408"), "{response:?}");
    assert!(!response.contains("private-preview-content"));
    assert!(started.elapsed() >= Duration::from_secs(5), "do not confuse a read slice with the whole-head deadline");
}

#[test]
fn regression_request_targets_validate_query_percent_escapes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "private-preview-content").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    for method in ["GET", "HEAD"] {
        for target in ["/index.html?value=%", "/index.html?value=%A", "/index.html?value=%GG", "/index.html?%GG=value", "/index.html?value=%2G"] {
            let response = request(srv.port, &format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\n\r\n"));
            assert!(response.starts_with("HTTP/1.1 400"), "{method} {target}: {response:?}");
            assert!(!response.contains("private-preview-content"));
        }
        for target in ["/index.html?value=%25GG", "/index.html?value=%252", "/index.html?value=%FF", "/%69ndex.html?value=%c3%a9&keep=%2F"] {
            let response = request(srv.port, &format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\n\r\n"));
            assert!(response.starts_with("HTTP/1.1 200"), "{method} {target}: {response:?}");
            assert_eq!(response.contains("private-preview-content"), method == "GET", "{method} {target}: {response:?}");
        }
    }
}

#[test]
fn regression_request_targets_reject_non_uri_characters() {
    let dir = tempfile::tempdir().unwrap();
    for name in ["index.html", "braces{ok}.html", "café.html"] {
        std::fs::write(dir.path().join(name), "private-preview-content").unwrap();
    }
    let srv = serve_dir(dir.path()).unwrap();
    for method in ["GET", "HEAD"] {
        for invalid in ['{', '}', '[', ']', '\\', '^', '`', '|', '<', '>', '"', 'é', '\u{7f}'] {
            for target in [format!("/index.html?value={invalid}"), format!("/index.html{invalid}")] {
                let response = request(srv.port, &format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\n\r\n"));
                assert!(response.starts_with("HTTP/1.1 400"), "{method} {target:?}: {response:?}");
                assert!(!response.contains("private-preview-content"));
            }
        }
        for target in [
            "/index.html?syntax=AZaz09-._~!$&'()*+,;=:@/?",
            "/index.html?",
            "/index.html?escaped=%7B%7D%5B%5D%5C%5E%60%7C%3C%3E%22",
            "/braces%7Bok%7D.html?utf8=%C3%A9",
            "/caf%C3%A9.html?single=%257B",
        ] {
            let response = request(srv.port, &format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\n\r\n"));
            assert!(response.starts_with("HTTP/1.1 200"), "{method} {target}: {response:?}");
            assert_eq!(response.contains("private-preview-content"), method == "GET", "{method} {target}: {response:?}");
        }
    }
}
