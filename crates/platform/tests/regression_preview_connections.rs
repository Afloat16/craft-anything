use craft_anything_platform::serve::serve_dir;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[test]
fn regression_preview_limits_connections_and_releases_listener() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "ok").unwrap();
    let srv = serve_dir(dir.path()).unwrap();
    let held: Vec<_> = (0..64).map(|_| TcpStream::connect(("127.0.0.1", srv.port)).unwrap()).collect();
    std::thread::sleep(Duration::from_millis(250));
    let mut s = TcpStream::connect(("127.0.0.1", srv.port)).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
    s.write_all(format!("GET / HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n", srv.port).as_bytes()).unwrap();
    let mut out = String::new();
    let _ = s.read_to_string(&mut out);
    assert!(out.starts_with("HTTP/1.1 503"), "{out}");
    let port = srv.port;
    drop(srv);
    assert!(TcpStream::connect(("127.0.0.1", port)).is_err(), "the listener is gone when drop returns");
    drop(held);
}
