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
