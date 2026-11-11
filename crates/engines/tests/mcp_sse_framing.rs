use craft_anything_engines::mcp_client::{McpClient, RemoteTool, Server, ServerUrl};
use craft_anything_evidence::ToolError;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufRead, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::Duration;

fn reply() -> String {
    json!({"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "echo_你好", "inputSchema": {"type": "object"}}]}}).to_string()
}

fn notice() -> String {
    json!({"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progress": 1}}).to_string()
}

fn event(value: &str, ending: &str) -> String {
    format!("data: {value}{ending}{ending}")
}

fn request(body: Vec<u8>, hold_open: bool) -> Result<Vec<RemoteTool>, ToolError> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/mcp", listener.local_addr().unwrap());
    let (release, wait) = mpsc::channel();
    let server = std::thread::spawn(move || {
        for expected in ["initialize", "notifications/initialized", "tools/list"] {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            stream.set_write_timeout(Some(Duration::from_secs(10))).unwrap();
            let mut reader = std::io::BufReader::new(&mut stream);
            let mut length = 0;
            loop {
                let mut line = String::new();
                assert!(reader.read_line(&mut line).unwrap() > 0);
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
            }
            let mut bytes = vec![0; length];
            reader.read_exact(&mut bytes).unwrap();
            let value: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(value["method"], expected);
            if expected == "notifications/initialized" {
                stream.write_all(b"HTTP/1.1 202 Accepted\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
            } else if expected == "initialize" {
                let result = json!({"jsonrpc": "2.0", "id": value["id"], "result": {"protocolVersion": "2025-06-18", "capabilities": {}}}).to_string();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{result}",
                    result.len()
                )
                .unwrap();
            } else {
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n").unwrap();
                // Rejected oversized input may close the connection before every byte is sent.
                let _ = stream.write_all(&body);
                let _ = stream.flush();
                if hold_open {
                    wait.recv_timeout(Duration::from_secs(10)).unwrap();
                }
            }
        }
    });
    let result = McpClient::connect("fixture", &Server::Url(ServerUrl { url, headers: BTreeMap::new() }), Duration::from_secs(5))
        .and_then(|mut client| client.list_tools(Duration::from_secs(2)));
    let _ = release.send(());
    server.join().unwrap();
    result
}

fn succeeds(body: String) {
    let tools = request(body.into_bytes(), false).unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "echo_你好");
    assert_eq!(tools[0].input_schema, json!({"type": "object"}));
}

#[test]
fn carriage_return_delimits_notifications_and_the_answer() {
    succeeds(event(&notice(), "\r") + &event(&reply(), "\r"));
}

#[test]
fn carriage_return_comments_do_not_hide_the_answer() {
    succeeds(format!(": keepalive\r{}", event(&reply(), "\r")));
}

#[test]
fn a_single_initial_bom_is_removed_for_every_line_ending() {
    for ending in ["\n", "\r\n", "\r"] {
        succeeds(format!("\u{feff}{}", event(&reply(), ending)));
    }
}

#[test]
fn mixed_line_endings_join_multiline_json_data() {
    succeeds("data: {\"jsonrpc\":\rdata: \"2.0\",\ndata: \"id\":2,\r\ndata: \"result\":{\"tools\":[{\"name\":\"echo_你好\",\"inputSchema\":{\"type\":\"object\"}}]}}\r\r".into());
}

#[test]
fn a_complete_carriage_return_event_returns_before_the_stream_closes() {
    let body = event(&notice(), "\r") + &event(&reply(), "\r");
    // The server waits for request() to finish before closing; no EOF can dispatch the answer.
    let tools = request(body.into_bytes(), true).unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "echo_你好");
}

#[test]
fn lf_and_crlf_keep_notifications_unicode_and_multiple_data_fields() {
    for ending in ["\n", "\r\n"] {
        let multiline = [
            "data: {\"jsonrpc\":",
            "data: \"2.0\",",
            "data: \"id\":2,",
            "data: \"result\":{\"tools\":[{\"name\":\"echo_你好\",\"inputSchema\":{\"type\":\"object\"}}]}}",
            "",
            "",
        ]
        .join(ending);
        succeeds(event(&notice(), ending) + &multiline);
    }
}

#[test]
fn only_one_bom_at_the_beginning_is_ignored() {
    for prefix in ["\n\u{feff}", "\u{feff}\u{feff}"] {
        assert!(request(format!("{prefix}{}", event(&reply(), "\n")).into_bytes(), false).is_err());
    }
}

#[test]
fn unrelated_ids_and_fields_do_not_replace_the_requested_answer() {
    let wrong = reply().replace("\"id\":2", "\"id\":99");
    succeeds(format!(": comment\nretry: 20\nid: event-key\nevent: message\n{}{}", event(&wrong, "\n"), event(&reply(), "\n")));
    assert!(request(event(&wrong, "\n").into_bytes(), false).is_err());
}

#[test]
fn invalid_utf8_and_oversized_bodies_still_fail() {
    assert!(request(vec![b'd', b'a', b't', b'a', b':', b' ', 0xff, b'\n', b'\n'], false).is_err());
    let oversized = format!(": {}\n{}", "x".repeat(16 * 1024 * 1024), event(&reply(), "\n"));
    assert!(request(oversized.into_bytes(), false).is_err());
}

#[test]
fn the_existing_unterminated_final_json_compatibility_is_preserved() {
    succeeds(format!("data: {}", reply()));
}
