use craft_anything_engine::{CallOptions, Caller, Engine, ToolOutput};
use craft_anything_formats::{bytes::Reader, elf};
use serde_json::{Value, json};

fn signal_note(data: &[u8]) -> (usize, usize) {
    for ph in elf::program_headers(data).unwrap().into_iter().filter(|ph| ph.p_type == 4) {
        let reader = Reader::new(data);
        let mut at = ph.offset as usize;
        let end = at + ph.file_size as usize;
        while at + 12 <= end {
            let names = reader.u32_le(at).unwrap() as usize;
            let desc = at + 12 + ((names + 3) & !3);
            let size = reader.u32_le(at + 4).unwrap() as usize;
            if reader.u32_le(at + 8) == Some(0x5349_4749) {
                return (at, desc);
            }
            at = desc + ((size + 3) & !3);
        }
    }
    panic!("the original core fixture has no signal note");
}

fn core(code: i32) -> Vec<u8> {
    let mut data = craft_anything_testkit::elf64_core(0x400234, 0x400234);
    assert!(elf::parse_core(&data).unwrap().locate(0x400234).is_some(), "the address must be inside an NT_FILE mapping");
    let (_, desc) = signal_note(&data);
    data[desc + 8..desc + 12].copy_from_slice(&code.to_le_bytes());
    data
}

fn evidence(data: &[u8]) -> Value {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("signal.core");
    std::fs::write(&path, data).unwrap();
    let engine = Engine::new(craft_anything_engine::testing::session());
    match engine.call("native_crash_show", json!({"path": path, "with_debugger_context": false}), CallOptions::new(Caller::Cli)).unwrap() {
        ToolOutput::Evidence(value) => {
            assert!(value.verify());
            serde_json::to_value(value).unwrap()
        }
        other => panic!("expected crash evidence, got {other:?}"),
    }
}

#[test]
fn explicit_signal_sources_do_not_produce_a_false_fault_file_or_hardware_diagnosis() {
    for (code, origin) in [(0, "Sent by a process"), (-6, "Sent to a thread"), (-1, "Queued by a process"), (-2, "timer"), (128, "Sent by the kernel")] {
        // The other union variant's bytes deliberately fall inside a known
        // mapping, so the original tool also fabricated a fault-file location.
        let value = evidence(&core(code));
        assert_eq!(value["result"]["signal"]["code"], code);
        assert!(value["result"]["signal"]["fault_addr"].is_null());
        assert!(value["result"]["fault_location"].is_null());
        assert!(value["result"]["signal"]["meaning"].as_str().unwrap().contains(origin));
        assert!(value["summary"].as_str().unwrap().contains(origin));
        assert_eq!(value["result"]["program_counter"], "0x400234");
        assert!(!value["result"]["crash_location"].is_null());
    }
}

#[test]
fn hardware_faults_keep_the_existing_address_location_and_specific_explanation() {
    for (code, explanation) in [(1, "isn't mapped"), (2, "wrong permissions")] {
        let value = evidence(&core(code));
        assert_eq!(value["result"]["signal"]["fault_addr"], "0x400234");
        assert!(!value["result"]["fault_location"].is_null());
        assert!(value["summary"].as_str().unwrap().contains(explanation));
        assert_eq!(value["result"]["program_counter"], "0x400234");
    }
}

#[test]
fn prstatus_without_a_signal_origin_note_does_not_diagnose_a_memory_access() {
    let mut data = core(1);
    let (header, _) = signal_note(&data);
    data[header + 8..header + 12].copy_from_slice(&0x5349_4748u32.to_le_bytes());
    let value = evidence(&data);
    assert_eq!(value["result"]["signal"]["name"], "SIGSEGV");
    assert!(value["result"]["signal"]["fault_addr"].is_null());
    assert!(value["result"]["fault_location"].is_null());
    assert!(value["summary"].as_str().unwrap().contains("origin was not recorded"));
}
