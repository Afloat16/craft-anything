use base64::Engine as _;
use craft_anything_engines::cdp::{CdpClient, CdpEvent, Deadline, WsConn, parse_endpoint, ws_target};
use craft_anything_evidence::{ErrorCode, ToolError};
use craft_anything_platform::cancel::CancelToken;
use serde_json::{Value, json};
use sha1::{Digest, Sha1};
use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn event() -> Value {
    json!({"method": "Runtime.consoleAPICalled", "params": {"text": "你好🌍"}})
}

fn event_bytes() -> Vec<u8> {
    event().to_string().into_bytes()
}

fn frame(opcode: u8, fin: bool, data: &[u8]) -> Vec<u8> {
    assert!(data.len() < 126);
    let mut out = vec![opcode | if fin { 0x80 } else { 0 }, data.len() as u8];
    out.extend_from_slice(data);
    out
}

fn read_client_frame(stream: &mut TcpStream) -> Option<(u8, Vec<u8>)> {
    let mut head = [0; 2];
    match stream.read_exact(&mut head) {
        Ok(()) => {}
        Err(error) if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof | std::io::ErrorKind::ConnectionReset) => return None,
        Err(error) => panic!("client frame: {error}"),
    }
    assert_ne!(head[1] & 0x80, 0);
    let mut length = usize::from(head[1] & 0x7f);
    if length == 126 {
        let mut bytes = [0; 2];
        stream.read_exact(&mut bytes).unwrap();
        length = usize::from(u16::from_be_bytes(bytes));
    }
    assert!(length < 1024);
    let mut mask = [0; 4];
    stream.read_exact(&mut mask).unwrap();
    let mut data = vec![0; length];
    stream.read_exact(&mut data).unwrap();
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= mask[i & 3];
    }
    Some((head[0] & 0x0f, data))
}

struct ResultAfterWindow {
    command: Result<Value, ToolError>,
    events: Vec<CdpEvent>,
    pongs: usize,
}

/// The peer sends the message prefix with the upgrade. It only sends the tail after the next
/// command, so the observation window must end between those two parts. For cancellation, the
/// peer cancels only after receiving a pong, proving the prefix has already been consumed.
fn resume(initial: Vec<u8>, tail: Vec<u8>, cap: usize, cancel: bool, windows: usize) -> ResultAfterWindow {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let url = format!("ws://{}/devtools/page/fixture", listener.local_addr().unwrap());
    let token = CancelToken::default();
    let server_token = token.clone();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        stream.set_write_timeout(Some(Duration::from_secs(10))).unwrap();
        let mut reader = std::io::BufReader::new(&mut stream);
        let mut key = None;
        loop {
            let mut line = String::new();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            if line == "\r\n" {
                break;
            }
            if let Some((name, value)) = line.split_once(':')
                && name.eq_ignore_ascii_case("sec-websocket-key")
            {
                key = Some(value.trim().to_string());
            }
        }
        let mut hash = Sha1::new();
        hash.update(key.unwrap().as_bytes());
        hash.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let accept = base64::engine::general_purpose::STANDARD.encode(hash.finalize());
        let mut response = format!(
            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
        )
        .into_bytes();
        response.extend_from_slice(&initial);
        stream.write_all(&response).unwrap();
        let mut pongs = 0;
        while let Some((opcode, data)) = read_client_frame(&mut stream) {
            match opcode {
                10 => {
                    assert_eq!(data, b"checkpoint");
                    pongs += 1;
                    if cancel {
                        server_token.cancel();
                    }
                }
                1 => {
                    let command: Value = serde_json::from_slice(&data).unwrap();
                    assert_eq!(command["method"], "Runtime.evaluate");
                    let reply = json!({"id": command["id"], "result": {"value": 2}}).to_string();
                    let response = [tail.as_slice(), frame(1, true, reply.as_bytes()).as_slice()].concat();
                    let _ = stream.write_all(&response);
                }
                8 => break,
                _ => panic!("unexpected client opcode {opcode}"),
            }
        }
        pongs
    });
    let endpoint = parse_endpoint(&origin).unwrap();
    let target = ws_target(&url, &endpoint).unwrap();
    let conn = WsConn::connect(&target, cap, &Deadline::new(Duration::from_secs(5), CancelToken::default())).unwrap();
    let mut client = CdpClient::new(conn);
    for _ in 0..windows {
        let duration = if cancel { Duration::from_secs(5) } else { Duration::from_millis(100) };
        let observed = client.pump_until(&Deadline::new(duration, token.clone()));
        if cancel {
            assert_eq!(observed.unwrap_err().code, ErrorCode::Cancelled);
        } else {
            observed.unwrap();
        }
    }
    let command = client.call("Runtime.evaluate", json!({"expression": "1+1"}), None, &Deadline::new(Duration::from_secs(5), CancelToken::default()));
    let events = client.take_events();
    drop(client);
    let pongs = server.join().unwrap();
    ResultAfterWindow { command, events, pongs }
}

fn ping() -> Vec<u8> {
    frame(9, true, b"checkpoint")
}

fn check_event(result: ResultAfterWindow) {
    assert_eq!(result.command.unwrap(), json!({"value": 2}));
    assert_eq!(result.events.len(), 1);
    assert_eq!(result.events[0].method, event()["method"].as_str().unwrap());
    assert_eq!(result.events[0].params, event()["params"]);
}

#[test]
fn observation_deadlines_preserve_text_binary_utf8_and_empty_first_fragments() {
    let bytes = event_bytes();
    let emoji = bytes.windows(4).position(|part| part == "🌍".as_bytes()).unwrap() + 1;
    for (opcode, cut) in [(1, 31), (2, 31), (1, emoji), (1, 0)] {
        check_event(resume(frame(opcode, false, &bytes[..cut]), frame(0, true, &bytes[cut..]), 4096, false, 1));
    }
}

#[test]
fn repeated_observation_windows_keep_all_previous_fragments() {
    let bytes = event_bytes();
    let initial = [frame(1, false, &bytes[..12]), frame(0, false, &bytes[12..31])].concat();
    check_event(resume(initial, frame(0, true, &bytes[31..]), 4096, false, 2));
}

#[test]
fn cancellation_after_an_observed_prefix_can_resume_with_a_fresh_token() {
    let bytes = event_bytes();
    let initial = [frame(1, false, &bytes[..31]), ping()].concat();
    let result = resume(initial, frame(0, true, &bytes[31..]), 4096, true, 1);
    assert_eq!(result.pongs, 1);
    check_event(result);
}

#[test]
fn pings_between_fragments_work_on_both_sides_of_the_deadline() {
    let bytes = event_bytes();
    let initial = [frame(1, false, &bytes[..31]), ping()].concat();
    let tail = [ping(), frame(0, true, &bytes[31..])].concat();
    let result = resume(initial, tail, 4096, false, 1);
    assert_eq!(result.pongs, 2);
    check_event(result);
}

#[test]
fn aggregate_message_limits_still_apply_after_resuming() {
    let bytes = event_bytes();
    let initial = frame(1, false, &bytes[..31]);
    let tail = frame(0, true, &bytes[31..]);
    let too_large = resume(initial.clone(), tail.clone(), bytes.len() - 1, false, 1);
    assert_eq!(too_large.command.unwrap_err().code, ErrorCode::OverLimit);
    assert!(too_large.events.is_empty());
    check_event(resume(initial, tail, bytes.len(), false, 1));
}

#[test]
fn a_new_data_frame_cannot_replace_an_unfinished_message_after_a_deadline() {
    let bytes = event_bytes();
    let result = resume(frame(1, false, &bytes[..31]), frame(1, true, &bytes), 4096, false, 1);
    assert_eq!(result.command.unwrap_err().code, ErrorCode::GarbledOutput);
    assert!(result.events.is_empty());
}

#[test]
fn a_clean_close_discards_the_partial_message() {
    let bytes = event_bytes();
    let result = resume(frame(1, false, &bytes[..31]), frame(8, true, &[]), 4096, false, 1);
    assert_eq!(result.command.unwrap_err().code, ErrorCode::EngineMissing);
    assert!(result.events.is_empty());
}

#[test]
fn complete_events_and_empty_observation_windows_remain_usable() {
    check_event(resume(frame(1, true, &event_bytes()), Vec::new(), 4096, false, 1));
    let result = resume(Vec::new(), Vec::new(), 4096, false, 1);
    assert_eq!(result.command.unwrap(), json!({"value": 2}));
    assert!(result.events.is_empty());
}

#[test]
fn a_frame_split_across_socket_reads_keeps_its_existing_buffer() {
    let frame = frame(1, true, &event_bytes());
    check_event(resume(frame[..20].to_vec(), frame[20..].to_vec(), 4096, false, 1));
}
