use craft_anything_formats::wasm::{MAX_ITEMS, WasmReport, inspect};
use craft_anything_testkit::formats as fx;

const COMPONENT: &[u8] = b"\0asm\x0d\0\x01\0";

fn leb(mut value: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        bytes.push(byte | if value == 0 { 0 } else { 0x80 });
        if value == 0 {
            return bytes;
        }
    }
}

fn section(bytes: &mut Vec<u8>, id: u8, payload: &[u8]) -> (u64, u64) {
    bytes.push(id);
    bytes.extend(leb(payload.len()));
    let offset = bytes.len() as u64;
    bytes.extend(payload);
    (offset, payload.len() as u64)
}

fn custom(name: &str, payload: &[u8]) -> Vec<u8> {
    let mut result = leb(name.len());
    result.extend(name.as_bytes());
    result.extend(payload);
    result
}

fn valid(bytes: &[u8]) -> WasmReport {
    let report = inspect(bytes).unwrap();
    assert!(report.validation.valid, "{:?}", report.validation);
    assert!(report.parse_error.is_none(), "{:?}", report.parse_error);
    assert_eq!(report.encoding, "component");
    assert_eq!(report.version, 13);
    report
}

#[test]
fn top_level_component_layout_preserves_ranges_and_order() {
    let mut nested = COMPONENT.to_vec();
    section(&mut nested, 0, &custom("nested-only", &[7]));
    section(&mut nested, 1, &fx::wasm_module());
    let mut bytes = COMPONENT.to_vec();
    let ranges = [
        section(&mut bytes, 0, &custom("outer", &[1, 2])),
        section(&mut bytes, 1, &fx::wasm_module()),
        section(&mut bytes, 4, &nested),
        section(&mut bytes, 0, &custom("tail", &[3])),
    ];
    let report = valid(&bytes);
    assert_eq!(report.sections.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["custom:outer", "core-module", "component", "custom:tail"]);
    assert_eq!(report.sections.iter().map(|s| s.id).collect::<Vec<_>>(), [0, 1, 4, 0]);
    assert_eq!(report.sections.iter().map(|s| (s.offset, s.size)).collect::<Vec<_>>(), ranges);
    assert_eq!(report.custom_sections.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["outer", "tail"]);
    assert!(!report.truncated);
}

#[test]
fn component_section_ids_use_component_names() {
    let mut bytes = COMPONENT.to_vec();
    for id in [2, 3, 5, 6, 7, 8, 10, 11] {
        section(&mut bytes, id, &[0]);
    }
    let report = valid(&bytes);
    assert_eq!(
        report.sections.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(),
        ["core-instance", "core-type", "instance", "alias", "type", "canonical", "import", "export"]
    );
    assert_eq!(report.types, 0);
    assert_eq!(report.start_function, None);
}

#[test]
fn nested_module_metadata_does_not_become_component_metadata() {
    let mut inner = COMPONENT.to_vec();
    section(&mut inner, 1, &fx::wasm_module());
    let mut middle = COMPONENT.to_vec();
    section(&mut middle, 4, &inner);
    let mut bytes = COMPONENT.to_vec();
    section(&mut bytes, 4, &middle);
    section(&mut bytes, 0, &custom("name", &[0, 6, 5, b'o', b'u', b't', b'e', b'r']));
    let report = valid(&bytes);
    assert_eq!(report.sections.len(), 2);
    assert!(report.functions.is_empty() && report.imports.is_empty() && report.exports.is_empty());
    assert!(report.memories.is_empty() && report.tables.is_empty() && report.globals.is_empty() && report.tags.is_empty());
    assert_eq!(report.imported_functions, 0);
    assert_eq!(report.module_name, None);
    assert_eq!(report.data_segments, 0);
    assert_eq!(report.element_segments, 0);
    assert!(report.producers.is_empty() && report.target_features.is_empty());
    assert_eq!(report.custom_sections.len(), 1);
    assert_eq!(report.custom_sections[0].name, "name");
}

#[test]
fn damaged_component_tail_keeps_only_available_layout() {
    let mut bytes = COMPONENT.to_vec();
    section(&mut bytes, 0, &custom("prefix", &[1]));
    bytes.extend([1, 100]);
    bytes.extend(b"\0asm");
    let report = inspect(&bytes).unwrap();
    assert!(!report.validation.valid);
    assert!(report.parse_error.is_some());
    assert_eq!(report.encoding, "component");
    assert_eq!(report.sections.len(), 1);
    assert_eq!(report.sections[0].name, "custom:prefix");
    assert!(report.sections.iter().all(|s| s.offset + s.size <= bytes.len() as u64));
}

#[test]
fn invalid_nested_module_keeps_top_level_identity_and_reports_damage() {
    let mut bytes = COMPONENT.to_vec();
    section(&mut bytes, 1, b"not wasm");
    let report = inspect(&bytes).unwrap();
    assert_eq!(report.encoding, "component");
    assert_eq!(report.version, 13);
    assert!(!report.validation.valid);
    assert!(report.parse_error.is_some());
    assert_eq!(report.sections.len(), 1);
    assert_eq!(report.sections[0].name, "core-module");
    assert!(report.functions.is_empty());
}

#[test]
fn component_layout_and_custom_lists_obey_existing_limits() {
    let mut bytes = COMPONENT.to_vec();
    for _ in 0..=MAX_ITEMS {
        section(&mut bytes, 0, &custom("x", &[]));
    }
    let report = valid(&bytes);
    assert!(report.truncated);
    assert_eq!(report.sections.len(), MAX_ITEMS);
    assert_eq!(report.custom_sections.len(), MAX_ITEMS);
    assert!(report.sections.iter().all(|s| s.name == "custom:x"));
}

#[test]
fn empty_component_stays_a_valid_empty_layout() {
    let report = valid(COMPONENT);
    assert!(report.sections.is_empty() && report.custom_sections.is_empty());
    assert!(!report.truncated);
    assert_eq!(report.warnings.len(), 1);
}

#[test]
fn ordinary_core_module_retains_its_inventory_and_names() {
    let report = inspect(&fx::wasm_module()).unwrap();
    assert!(report.validation.valid && report.parse_error.is_none());
    assert_eq!(report.encoding, "module");
    assert_eq!(report.version, 1);
    assert_eq!(report.module_name.as_deref(), Some("demo"));
    assert_eq!(report.imports.len(), 1);
    assert_eq!(report.functions.len(), 2);
    assert_eq!(report.functions[0].name.as_deref(), Some("add"));
    assert_eq!(report.exports.len(), 3);
    assert!(report.sections.iter().any(|s| s.name == "code"));
    assert!(!report.truncated);
}
