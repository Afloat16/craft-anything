use craft_anything_engine::{CallOptions, Caller, Engine, ToolOutput};
use serde_json::{Value, json};

fn evidence(bytes: &[u8]) -> Value {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("arm.core");
    std::fs::write(&path, bytes).unwrap();
    let engine = Engine::new(craft_anything_engine::testing::session());
    match engine.call("native_crash_show", json!({"path": path, "with_debugger_context": false}), CallOptions::new(Caller::Cli)).unwrap() {
        ToolOutput::Evidence(value) => {
            assert!(value.verify());
            serde_json::to_value(value).unwrap()
        }
        other => panic!("expected crash evidence, got {other:?}"),
    }
}

fn registers() -> [u32; 18] {
    let mut values = std::array::from_fn(|index| 0x8000_1000 + index as u32);
    values[15] = 0x10234;
    values[16] = 0x30;
    values
}

#[test]
fn arm_core_evidence_locates_the_recorded_program_counter_in_both_byte_orders() {
    for big in [false, true] {
        let value = evidence(&craft_anything_testkit::elf_core::arm32(&registers(), big));
        assert_eq!(value["result"]["program_counter"], "0x10234");
        assert_eq!(value["result"]["crash_location"]["file"], "/usr/bin/arm-fixture");
        assert_eq!(value["result"]["crash_location"]["file_offset"], "0x234");
        assert!(value["summary"].as_str().unwrap().contains(" at 0x10234 in /usr/bin/arm-fixture"));
    }
}

#[test]
fn arm_thread_registers_reach_the_verified_public_evidence_without_truncating_values() {
    for big in [false, true] {
        let value = evidence(&craft_anything_testkit::elf_core::arm32(&registers(), big));
        let thread = &value["result"]["threads"][0];
        assert_eq!(thread["pid"], 4242);
        assert_eq!(thread["signal"], 4);
        let registers = thread["registers"].as_array().unwrap();
        assert_eq!(registers.len(), 18);
        let register = |name: &str| &registers.iter().find(|reg| reg["name"] == name).unwrap()["value"];
        assert_eq!(register("r0"), "0x80001000");
        assert_eq!(register("sp"), "0x8000100d");
        assert_eq!(register("lr"), "0x8000100e");
        assert_eq!(register("pc"), "0x10234");
        assert_eq!(register("cpsr"), "0x30");
        assert_eq!(register("orig_r0"), "0x80001011");
    }
}

#[test]
fn existing_x86_64_crash_locations_remain_unchanged() {
    let value = evidence(&craft_anything_testkit::elf64_core(0x400234, 0x400234));
    assert_eq!(value["result"]["program_counter"], "0x400234");
    assert_eq!(value["result"]["crash_location"]["file"], "/usr/bin/crashy");
    assert_eq!(value["result"]["crash_location"]["file_offset"], "0x234");
    assert_eq!(value["result"]["threads"][0]["registers"].as_array().unwrap().len(), 27);
}
