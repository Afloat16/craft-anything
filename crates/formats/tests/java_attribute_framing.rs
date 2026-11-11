use craft_anything_formats::java_class::{self, ClassFile};

struct Fixture {
    bytes: Vec<u8>,
    field_table: usize,
    field_end: usize,
    field_unknown_length: usize,
    method_table: usize,
    method_end: usize,
    method_unknown_length: usize,
    code_length: usize,
    code_start: usize,
    code_table: usize,
    code_end: usize,
    code_unknown_length: usize,
    class_table: usize,
}

fn word(out: &mut Vec<u8>, value: u16) {
    out.extend(value.to_be_bytes());
}

fn attribute(out: &mut Vec<u8>, name: u16, body: &[u8]) -> usize {
    word(out, name);
    let length = out.len();
    out.extend((body.len() as u32).to_be_bytes());
    out.extend(body);
    length
}

// Original class with complete known attributes followed by opaque attributes.
// The opaque bytes deliberately resemble members; only declared table framing
// can identify a later field, method or class attribute.
fn fixture() -> Fixture {
    let mut bytes = vec![0xca, 0xfe, 0xba, 0xbe, 0, 0, 0, 52];
    word(&mut bytes, 18);
    for text in ["FramedAttributes", "java/lang/Object", "value", "I", "run", "()V", "Code", "Opaque", "SourceFile", "FramedAttributes.java", "ConstantValue", "LineNumberTable", "later"] {
        bytes.push(1);
        word(&mut bytes, text.len() as u16);
        bytes.extend(text.as_bytes());
    }
    bytes.push(7);
    word(&mut bytes, 1); // 14: this class
    bytes.push(7);
    word(&mut bytes, 2); // 15: superclass
    bytes.push(3);
    bytes.extend(7i32.to_be_bytes()); // 16: field value
    bytes.push(1);
    word(&mut bytes, 10);
    bytes.extend(b"Deprecated"); // 17

    for value in [0x0021, 14, 15, 0, 1, 0x0019, 3, 4] {
        word(&mut bytes, value);
    }
    let field_table = bytes.len();
    word(&mut bytes, 2);
    attribute(&mut bytes, 11, &16u16.to_be_bytes());
    let mut fake_method_table = Vec::new();
    for value in [1, 0x0009, 13, 6, 0, 0] {
        word(&mut fake_method_table, value);
    }
    let field_unknown_length = attribute(&mut bytes, 8, &fake_method_table);
    let field_end = bytes.len();

    for value in [2, 0x0009, 5, 6] {
        word(&mut bytes, value);
    }
    let method_table = bytes.len();
    word(&mut bytes, 2);
    let mut code = vec![0, 0, 0, 0, 0, 0, 0, 1, 0xb1, 0, 0];
    let relative_table = code.len();
    word(&mut code, 2);
    attribute(&mut code, 12, &[0, 1, 0, 0, 0, 11]);
    let relative_unknown_length = attribute(&mut code, 8, &[0, 1, 0, 0]);
    let code_length = attribute(&mut bytes, 7, &code);
    let code_start = code_length + 4;
    let code_table = code_start + relative_table;
    let code_unknown_length = code_start + relative_unknown_length;
    let code_end = bytes.len();
    let mut fake_member = Vec::new();
    for value in [0x0009, 13, 6, 0, 0] {
        word(&mut fake_member, value);
    }
    let method_unknown_length = attribute(&mut bytes, 8, &fake_member);
    let method_end = bytes.len();

    for value in [0x0009, 13, 6, 1] {
        word(&mut bytes, value);
    }
    attribute(&mut bytes, 7, &[0, 0, 0, 0, 0, 0, 0, 1, 0xb1, 0, 0, 0, 0]);

    let class_table = bytes.len();
    word(&mut bytes, 2);
    attribute(&mut bytes, 9, &10u16.to_be_bytes());
    attribute(&mut bytes, 8, &[0xff, 0, 0x80, 1]);
    Fixture {
        bytes,
        field_table,
        field_end,
        field_unknown_length,
        method_table,
        method_end,
        method_unknown_length,
        code_length,
        code_start,
        code_table,
        code_end,
        code_unknown_length,
        class_table,
    }
}

fn parsed(bytes: &[u8]) -> ClassFile {
    let class = java_class::parse(bytes).unwrap();
    assert_eq!(class.this_class, "FramedAttributes");
    assert_eq!(class.super_class.as_deref(), Some("java/lang/Object"));
    class
}

fn incomplete(class: &ClassFile, scope: &str) {
    assert!(class.warnings.iter().any(|warning| warning.starts_with(scope) && warning.contains("truncated")), "{:?}", class.warnings);
}

fn overwrite_length(bytes: &mut [u8], at: usize, length: u32) {
    bytes[at..at + 4].copy_from_slice(&length.to_be_bytes());
}

#[test]
fn complete_known_and_opaque_attributes_preserve_all_scopes() {
    let class = parsed(&fixture().bytes);
    assert!(class.warnings.is_empty(), "{:?}", class.warnings);
    assert_eq!(class.fields.len(), 1);
    assert_eq!(class.fields[0].constant_value.as_deref(), Some("7"));
    assert_eq!(class.methods.iter().map(|method| method.name.as_str()).collect::<Vec<_>>(), ["run", "later"]);
    let code = class.methods[0].code.as_ref().unwrap();
    assert_eq!(code.bytecode, [0xb1]);
    assert_eq!(code.line_numbers, [(0, 11)]);
    assert_eq!(class.methods[1].code.as_ref().unwrap().bytecode, [0xb1]);
    assert_eq!(class.source_file.as_deref(), Some("FramedAttributes.java"));
}

#[test]
fn every_truncated_class_attribute_prefix_retains_members_and_known_attributes() {
    let fixture = fixture();
    for end in fixture.class_table..fixture.bytes.len() {
        let class = parsed(&fixture.bytes[..end]);
        incomplete(&class, "class");
        assert_eq!(class.fields.len(), 1);
        assert_eq!(class.methods.len(), 2);
        assert_eq!(class.methods[0].code.as_ref().unwrap().bytecode, [0xb1]);
        if end >= fixture.class_table + 10 {
            assert_eq!(class.source_file.as_deref(), Some("FramedAttributes.java"));
        }
    }
}

#[test]
fn every_truncated_field_attribute_prefix_preserves_the_field_without_inventing_tables() {
    let fixture = fixture();
    for end in fixture.field_table..fixture.field_end {
        let class = parsed(&fixture.bytes[..end]);
        incomplete(&class, "field");
        assert_eq!(class.fields.len(), 1);
        assert_eq!(class.fields[0].name, "value");
        if end >= fixture.field_table + 10 {
            assert_eq!(class.fields[0].constant_value.as_deref(), Some("7"));
        }
        assert!(class.methods.is_empty());
        assert!(class.source_file.is_none());
    }
}

#[test]
fn every_truncated_method_attribute_prefix_retains_its_readable_code() {
    let fixture = fixture();
    for end in fixture.method_table..fixture.method_end {
        let class = parsed(&fixture.bytes[..end]);
        incomplete(&class, "method");
        assert_eq!(class.fields[0].constant_value.as_deref(), Some("7"));
        assert_eq!(class.methods.len(), 1);
        assert_eq!(class.methods[0].name, "run");
        assert_eq!(class.methods[0].code.is_some(), end >= fixture.code_end);
        assert!(class.source_file.is_none());
    }
}

#[test]
fn an_oversized_field_attribute_cannot_turn_its_payload_into_methods() {
    let mut fixture = fixture();
    overwrite_length(&mut fixture.bytes, fixture.field_unknown_length, u32::MAX);
    let class = parsed(&fixture.bytes);
    incomplete(&class, "field");
    assert_eq!(class.fields[0].constant_value.as_deref(), Some("7"));
    assert!(class.methods.is_empty(), "{:?}", class.methods);
    assert!(class.source_file.is_none());
}

#[test]
fn an_oversized_method_attribute_cannot_turn_its_payload_into_another_member() {
    let mut fixture = fixture();
    overwrite_length(&mut fixture.bytes, fixture.method_unknown_length, u32::MAX);
    let class = parsed(&fixture.bytes);
    incomplete(&class, "method");
    assert_eq!(class.methods.len(), 1);
    assert_eq!(class.methods[0].name, "run");
    assert_eq!(class.methods[0].code.as_ref().unwrap().bytecode, [0xb1]);
    assert!(class.source_file.is_none());
}

#[test]
fn every_truncated_nested_table_keeps_bytecode_and_the_following_method() {
    let fixture = fixture();
    for end in fixture.code_table..fixture.code_end {
        let mut bytes = fixture.bytes.clone();
        bytes.drain(end..fixture.code_end);
        overwrite_length(&mut bytes, fixture.code_length, (end - fixture.code_start) as u32);
        let class = parsed(&bytes);
        incomplete(&class, "Code");
        assert_eq!(class.methods.len(), 2);
        let code = class.methods[0].code.as_ref().unwrap();
        assert_eq!(code.bytecode, [0xb1]);
        if end >= fixture.code_table + 14 {
            assert_eq!(code.line_numbers, [(0, 11)]);
        }
        assert_eq!(class.methods[1].name, "later");
        assert_eq!(class.methods[1].code.as_ref().unwrap().bytecode, [0xb1]);
        assert_eq!(class.source_file.as_deref(), Some("FramedAttributes.java"));
    }
}

#[test]
fn an_oversized_nested_attribute_stays_inside_its_code_attribute() {
    let mut fixture = fixture();
    overwrite_length(&mut fixture.bytes, fixture.code_unknown_length, u32::MAX);
    let class = parsed(&fixture.bytes);
    incomplete(&class, "Code");
    assert_eq!(class.methods.len(), 2);
    assert_eq!(class.methods[0].code.as_ref().unwrap().line_numbers, [(0, 11)]);
    assert_eq!(class.methods[1].code.as_ref().unwrap().bytecode, [0xb1]);
    assert_eq!(class.source_file.as_deref(), Some("FramedAttributes.java"));
}

#[test]
fn an_unreadable_outer_code_body_preserves_member_identity_and_marks_the_gap() {
    let mut fixture = fixture();
    overwrite_length(&mut fixture.bytes, fixture.code_length, u32::MAX);
    let class = parsed(&fixture.bytes);
    incomplete(&class, "method");
    assert_eq!(class.methods.len(), 1);
    assert_eq!(class.methods[0].name, "run");
    assert!(class.methods[0].code.is_none());
    assert_eq!(class.fields[0].constant_value.as_deref(), Some("7"));
}
