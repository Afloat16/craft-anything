use craft_anything_engine::{CallOptions, Caller, Engine, ToolOutput};
use craft_anything_formats::pcap;
use craft_anything_testkit::pcap as fx;
use serde_json::{Value, json};
use std::path::Path;

fn call(engine: &Engine, tool: &str, path: &Path, mut params: Value) -> Value {
    params["path"] = json!(path);
    match engine.call(tool, params, CallOptions::new(Caller::Cli)).unwrap() {
        ToolOutput::Evidence(evidence) => {
            assert!(evidence.verify());
            serde_json::to_value(evidence).unwrap()
        }
        other => panic!("expected packet evidence, got {other:?}"),
    }
}

fn capture(bytes: &[u8], check: impl FnOnce(&Engine, &Path)) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("capture.pcap");
    std::fs::write(&path, bytes).unwrap();
    let engine = Engine::new(craft_anything_engine::testing::session());
    check(&engine, &path);
}

fn has_limitation(evidence: &Value, code: &str) -> bool {
    evidence["limitations"].as_array().unwrap().iter().any(|limitation| limitation["code"] == code)
}

fn assert_incomplete_scan(evidence: &Value) {
    assert_eq!(evidence["coverage"]["status"], "partial");
    assert!(evidence["coverage"]["total"].is_null());
    assert!(has_limitation(evidence, "read_warnings") || has_limitation(evidence, "tshark_output_capped"));
}

fn assert_readable_prefix(bytes: &[u8], packets: u64) {
    capture(bytes, |engine, path| {
        let overview = call(engine, "packets_capture_show", path, json!({}));
        assert_eq!(overview["result"]["packets"], packets);
        assert_eq!(overview["result"]["flows"][0]["a"], "8.8.8.8:53");
        assert_incomplete_scan(&overview);
        let fields = call(engine, "packets_fields_show", path, json!({}));
        assert_eq!(fields["result"]["total_packets"], packets);
        assert_eq!(fields["result"]["packets"][0]["frame"], 1);
        assert_eq!(fields["result"]["packets"][0]["dns"]["questions"][0]["name"], "example.com");
        assert_incomplete_scan(&fields);
        let inventory = call(engine, "packets_inventory_show", path, json!({"engine": "built_in"}));
        assert_eq!(inventory["result"]["packets"], packets);
        assert_eq!(inventory["result"]["dns_queries"][0]["value"], "example.com (A)");
        assert_incomplete_scan(&inventory);
    });
}

#[test]
fn damaged_classic_capture_retains_its_readable_prefix() {
    let frames = fx::sample_frames();
    let mut bytes = fx::pcap(&frames[..2]);
    bytes.pop();
    assert_readable_prefix(&bytes, 1);
}

#[test]
fn damaged_pcapng_capture_retains_its_readable_prefix() {
    let frames = fx::sample_frames();
    let mut bytes = fx::pcapng(&frames[..2]);
    bytes.pop();
    assert_readable_prefix(&bytes, 1);
}

#[test]
fn rejected_pcapng_record_keeps_later_readable_packets_but_not_complete_coverage() {
    let frames = fx::sample_frames();
    let header = fx::pcapng(&[]).len();
    let mut bytes = fx::pcapng(&frames[..1]);
    let mut invalid_interface = vec![0; 20];
    invalid_interface[..4].copy_from_slice(&7u32.to_le_bytes());
    bytes.extend(fx::block(6, &invalid_interface));
    bytes.extend(&fx::pcapng(&frames[1..2])[header..]);
    assert_readable_prefix(&bytes, 2);
}

#[test]
fn an_invalid_timestamp_keeps_the_packet_and_marks_incomplete_evidence() {
    let frames = fx::sample_frames();
    let mut bytes = fx::pcap(&frames[..2]);
    bytes[28..32].copy_from_slice(&1_000_000u32.to_le_bytes());
    assert_readable_prefix(&bytes, 2);
    capture(&bytes, |engine, path| {
        let fields = call(engine, "packets_fields_show", path, json!({}));
        assert!(fields["result"]["packets"][0]["time"].is_null());
        assert!(fields["result"]["packets"][1]["time"].is_string());
    });
}

#[test]
fn complete_capture_pages_and_filters_keep_their_exact_totals() {
    for bytes in [fx::pcap(&fx::sample_frames()), fx::pcapng(&fx::sample_frames())] {
        capture(&bytes, |engine, path| {
            let first = call(engine, "packets_capture_show", path, json!({"limit": 2}));
            assert_eq!(first["coverage"], json!({"status": "partial", "examined": 2, "total": 3, "next_offset": 2}));
            let last = call(engine, "packets_capture_show", path, json!({"limit": 2, "offset": 2}));
            assert_eq!(last["coverage"], json!({"status": "complete", "examined": 1, "total": 3}));
            let first = call(engine, "packets_fields_show", path, json!({"protocol": "dns", "limit": 1}));
            assert_eq!(first["coverage"], json!({"status": "partial", "examined": 1, "total": 2, "next_offset": 1}));
            let last = call(engine, "packets_fields_show", path, json!({"protocol": "dns", "limit": 1, "offset": 1}));
            assert_eq!(last["coverage"], json!({"status": "complete", "examined": 1, "total": 2}));
            assert_eq!(last["result"]["packets"][0]["dns"]["answers"][0]["value"], "93.184.216.34");
            let inventory = call(engine, "packets_inventory_show", path, json!({"engine": "built_in"}));
            assert_eq!(inventory["coverage"], json!({"status": "complete", "examined": 7, "total": 7}));
        });
    }
}

fn append_record(bytes: &mut Vec<u8>, frame: &[u8]) {
    bytes.extend(1_700_000_000u32.to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend((frame.len() as u32).to_le_bytes());
    bytes.extend((frame.len() as u32).to_le_bytes());
    bytes.extend(frame);
}

#[test]
fn a_real_inventory_cap_stays_partial_after_the_last_flow_page() {
    let count = pcap::MAX_INVENTORY / 2 + 1;
    let mut bytes = fx::pcap(&[]);
    for i in 0..count as u32 {
        let src = (0xc000_0000 + i * 2).to_be_bytes();
        let dst = (0xc000_0001 + i * 2).to_be_bytes();
        append_record(&mut bytes, &fx::eth(0x0800, &fx::ipv4_pkt(17, src, dst, &fx::udp(40000, 40001, &[]))));
    }
    capture(&bytes, |engine, path| {
        let overview = call(engine, "packets_capture_show", path, json!({"offset": count - 1, "limit": 1}));
        assert_eq!(overview["result"]["endpoint_count"], pcap::MAX_INVENTORY);
        assert_eq!(overview["result"]["capped_inventories"], json!(["endpoints"]));
        assert_eq!(overview["coverage"], json!({"status": "partial", "examined": 1, "total": count}));
        let inventory = call(engine, "packets_inventory_show", path, json!({"engine": "built_in", "limit": pcap::MAX_INVENTORY}));
        assert_eq!(inventory["result"]["counts"]["endpoints"], pcap::MAX_INVENTORY);
        assert_eq!(inventory["coverage"], json!({"status": "partial", "examined": count, "total": count}));
        assert_eq!(inventory["result"]["capped_inventories"], json!(["endpoints"]));
        assert_eq!(inventory["result"]["rows_limited"], false);
        assert_eq!(inventory["result"]["truncated"], false);
        assert!(has_limitation(&inventory, "caps_hit"));
        let fields = call(engine, "packets_fields_show", path, json!({"offset": count - 1, "limit": 1}));
        assert_eq!(fields["coverage"], json!({"status": "complete", "examined": 1, "total": count}));
    });
}

#[test]
fn a_real_flow_cap_preserves_packets_and_the_remaining_page() {
    let mut bytes = fx::pcap(&[]);
    for i in 0..=pcap::MAX_FLOWS as u32 {
        let transport = fx::udp(i as u16, (i >> 16) as u16 + 1000, &[]);
        append_record(&mut bytes, &fx::eth(0x0800, &fx::ipv4_pkt(17, [192, 0, 2, 1], [198, 51, 100, 2], &transport)));
    }
    capture(&bytes, |engine, path| {
        let overview = call(engine, "packets_capture_show", path, json!({"offset": pcap::MAX_FLOWS - 2, "limit": 1}));
        assert_eq!(overview["result"]["flow_count"], pcap::MAX_FLOWS);
        assert_eq!(overview["result"]["flows_untracked_frames"], 1);
        assert_eq!(overview["coverage"], json!({"status": "partial", "examined": 1, "next_offset": pcap::MAX_FLOWS - 1}));
        let fields = call(engine, "packets_fields_show", path, json!({"offset": pcap::MAX_FLOWS, "limit": 1}));
        assert_eq!(fields["result"]["total_packets"], pcap::MAX_FLOWS + 1);
        assert!(fields["result"]["packets"][0]["flow"].is_null());
        assert_eq!(fields["result"]["flows_untracked_frames"], 1);
        assert_eq!(fields["coverage"], json!({"status": "partial", "examined": 1, "total": pcap::MAX_FLOWS + 1}));
        let filtered = call(engine, "packets_fields_show", path, json!({"flow": format!("f{}", pcap::MAX_FLOWS + 1)}));
        assert_eq!(filtered["result"]["matched_packets"], 0);
        assert_eq!(filtered["coverage"], json!({"status": "partial", "examined": 0}));
        assert!(has_limitation(&filtered, "caps_hit"));
    });
}

fn empty_records(count: u64) -> Vec<u8> {
    let mut bytes = fx::pcap(&[]);
    bytes.resize(bytes.len() + count as usize * 16, 0);
    bytes
}

#[test]
fn the_real_packet_scan_limit_is_visible_in_all_packet_tools() {
    capture(&empty_records(pcap::MAX_PACKETS + 1), |engine, path| {
        for (tool, params) in [
            ("packets_capture_show", json!({})),
            ("packets_fields_show", json!({"offset": pcap::MAX_PACKETS - 1, "limit": 1})),
            ("packets_inventory_show", json!({"engine": "built_in"})),
        ] {
            let evidence = call(engine, tool, path, params);
            assert_incomplete_scan(&evidence);
            assert_eq!(evidence["result"]["truncated"], true);
            if tool == "packets_inventory_show" {
                assert!(has_limitation(&evidence, "packet_scan_capped"));
            }
            let count_field = if tool == "packets_fields_show" { "total_packets" } else { "packets" };
            assert_eq!(evidence["result"][count_field], pcap::MAX_PACKETS);
            assert!(evidence["coverage"]["next_offset"].is_null());
        }
    });
}

#[test]
fn exactly_the_packet_limit_remains_complete() {
    capture(&empty_records(pcap::MAX_PACKETS), |engine, path| {
        let fields = call(engine, "packets_fields_show", path, json!({"offset": pcap::MAX_PACKETS - 1, "limit": 1}));
        assert_eq!(fields["coverage"], json!({"status": "complete", "examined": 1, "total": pcap::MAX_PACKETS}));
        assert_eq!(fields["result"]["warnings"], json!([]));
    });
}

#[test]
fn limiting_displayed_inventory_rows_keeps_the_known_packet_total() {
    capture(&fx::pcap(&fx::sample_frames()), |engine, path| {
        let inventory = call(engine, "packets_inventory_show", path, json!({"engine": "built_in", "limit": 1}));
        assert_eq!(inventory["coverage"], json!({"status": "partial", "examined": 7, "total": 7}));
        assert_eq!(inventory["result"]["rows_limited"], true);
        assert_eq!(inventory["result"]["capped_inventories"], json!([]));
        assert_eq!(inventory["result"]["truncated"], false);
    });
}

#[cfg(unix)]
fn tshark_output(write: impl FnOnce(&mut std::fs::File), check: impl FnOnce(Value)) {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("fields.tsv");
    {
        let mut file = std::fs::File::create(&output).unwrap();
        write(&mut file);
    }
    let executable = directory.path().join("tshark-fixture");
    let quoted = output.to_string_lossy().replace('\'', "'\"'\"'");
    std::fs::write(&executable, format!("#!/bin/sh\nexec cat '{quoted}'\n")).unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
    let path = directory.path().join("input.pcap");
    std::fs::write(&path, fx::pcap(&[])).unwrap();
    let mut session = craft_anything_engine::testing::session();
    session.settings.engines.tools.tshark = Some(executable);
    let engine = Engine::new(session);
    check(call(&engine, "packets_inventory_show", &path, json!({"engine": "tshark", "limit": pcap::MAX_INVENTORY})));
}

#[cfg(unix)]
#[test]
fn tshark_inventory_caps_keep_counting_existing_values() {
    use std::io::Write as _;
    tshark_output(
        |file| {
            for i in 0..=pcap::MAX_INVENTORY {
                writeln!(file, "eth:ip:udp\t198.18.{}.{}\t\t\t\t\t\t\t\t\t\t\t\t", i / 256, i % 256).unwrap();
            }
            writeln!(file, "eth:ip:udp\t198.18.0.0\t\t\t\t\t\t\t\t\t\t\t\t").unwrap();
        },
        |evidence| {
            assert_eq!(evidence["coverage"], json!({"status": "partial", "examined": pcap::MAX_INVENTORY + 2, "total": pcap::MAX_INVENTORY + 2}));
            assert_eq!(evidence["result"]["capped_inventories"], json!(["endpoints"]));
            assert_eq!(evidence["result"]["counts"]["endpoints"], pcap::MAX_INVENTORY);
            assert_eq!(evidence["result"]["endpoints"][0], json!({"value": "198.18.0.0", "count": 2}));
            assert_eq!(evidence["result"]["rows_limited"], false);
            assert!(has_limitation(&evidence, "caps_hit"));
        },
    );
}

#[cfg(unix)]
fn long_tshark_row(length: usize) -> String {
    let mut row = "eth:ip:tcp:http\t192.0.2.1\t198.51.100.2\t\t\t\t\t\t\t\t\tGET\texample.test\t/".to_string();
    row.extend(std::iter::repeat_n('x', length - row.len() - 1));
    row.push('\n');
    assert_eq!(row.len(), length);
    row
}

#[cfg(unix)]
#[test]
fn capped_tshark_output_never_counts_an_unfinished_record() {
    use std::io::Write as _;
    const OUTPUT_CAP: usize = 64 << 20;
    let row = long_tshark_row(4093);
    let complete = OUTPUT_CAP / row.len();
    assert_ne!(OUTPUT_CAP % row.len(), 0);
    tshark_output(
        |file| {
            for _ in 0..=complete {
                file.write_all(row.as_bytes()).unwrap();
            }
        },
        |evidence| {
            assert_eq!(evidence["result"]["packets"], complete);
            assert_eq!(evidence["result"]["http_requests"][0]["count"], complete);
            assert_eq!(evidence["result"]["counts"]["http_requests"], 1);
            assert_incomplete_scan(&evidence);
            assert_eq!(evidence["result"]["truncated"], true);
            assert_eq!(evidence["result"]["capped_inventories"], json!([]));
            assert_eq!(evidence["result"]["rows_limited"], false);
        },
    );
}

#[cfg(unix)]
#[test]
fn exactly_the_tshark_output_limit_remains_complete() {
    use std::io::Write as _;
    let row = long_tshark_row(4096);
    let complete = (64 << 20) / row.len();
    tshark_output(
        |file| {
            for _ in 0..complete {
                file.write_all(row.as_bytes()).unwrap();
            }
        },
        |evidence| {
            assert_eq!(evidence["result"]["packets"], complete);
            assert_eq!(evidence["coverage"], json!({"status": "complete", "examined": complete, "total": complete}));
            assert!(!has_limitation(&evidence, "tshark_output_capped"));
        },
    );
}
