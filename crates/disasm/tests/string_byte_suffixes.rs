use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::{Arch, strings::MAX_STRING_CHARS};
use std::sync::Arc;

fn resolve(payload: &[u8], byte_offset: usize) -> pseudo::Pseudo {
    let base = 0x1000u64;
    let data_offset = 0x100usize;
    let displacement = i32::try_from(data_offset + byte_offset - 7).unwrap();
    // LEA RAX, [RIP + displacement]; RET. The string starts at 0x1100.
    let mut bytes = vec![0x48, 0x8d, 0x05];
    bytes.extend_from_slice(&displacement.to_le_bytes());
    bytes.push(0xc3);
    bytes.resize(data_offset, 0);
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(&[0, 0]);
    let image = Image::raw(Arc::from(bytes), Arch::X86_64, base);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    pseudo::lift(&image, &program, program.functions.get(&base).unwrap(), &|_| None)
}

fn wide(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

fn assert_literal(payload: &[u8], byte_offset: usize, expected: &str) {
    let result = resolve(payload, byte_offset);
    assert_eq!(result.strings, [expected], "{}", result.text);
}

fn assert_address(payload: &[u8], byte_offset: usize) {
    let result = resolve(payload, byte_offset);
    assert!(result.strings.is_empty(), "{:?}\n{}", result.strings, result.text);
    assert!(result.text.contains(&format!("{:x}", 0x1100 + byte_offset)), "{}", result.text);
}

#[test]
fn utf16_one_code_unit_is_two_bytes() {
    assert_literal(&wide("HelloWide"), 2, "elloWide");
}

#[test]
fn utf16_two_code_units_are_four_bytes() {
    assert_literal(&wide("HelloWide"), 4, "lloWide");
}

#[test]
fn utf16_last_character_is_not_an_empty_suffix() {
    assert_literal(&wide("HelloWide"), 16, "e");
}

#[test]
fn an_address_inside_a_utf16_code_unit_is_not_a_literal() {
    assert_address(&wide("HelloWide"), 1);
}

#[test]
fn an_address_inside_the_first_utf8_character_is_not_a_literal() {
    assert_address("你好world".as_bytes(), 1);
}

#[test]
fn an_address_inside_a_later_utf8_character_is_not_a_literal() {
    assert_address("你好world".as_bytes(), 4);
}

#[test]
fn the_first_unretained_ascii_byte_is_not_an_empty_literal() {
    assert_address(&vec![b'q'; MAX_STRING_CHARS + 8], MAX_STRING_CHARS);
}

#[test]
fn a_later_unretained_ascii_byte_is_not_an_empty_literal() {
    assert_address(&vec![b'q'; MAX_STRING_CHARS + 8], MAX_STRING_CHARS + 3);
}

#[test]
fn the_first_unretained_utf16_unit_is_not_an_empty_literal() {
    assert_address(&wide(&"q".repeat(MAX_STRING_CHARS + 8)), MAX_STRING_CHARS * 2);
}

#[test]
fn the_last_retained_utf16_character_remains_available() {
    assert_literal(&wide(&"q".repeat(MAX_STRING_CHARS + 8)), (MAX_STRING_CHARS - 1) * 2, "q");
}

#[test]
fn the_last_retained_utf8_character_uses_its_byte_address() {
    assert_literal("界".repeat(MAX_STRING_CHARS + 8).as_bytes(), (MAX_STRING_CHARS - 1) * 3, "界");
}

#[test]
fn a_complete_utf16_literal_stays_available() {
    assert_literal(&wide("HelloWide"), 0, "HelloWide");
}

#[test]
fn a_normal_ascii_suffix_stays_available() {
    assert_literal(b"hello world", 6, "world");
}

#[test]
fn a_valid_utf8_reference_stays_available() {
    assert_literal("你好world".as_bytes(), 6, "world");
}

#[test]
fn a_complete_ascii_literal_stays_available() {
    assert_literal(b"hello world", 0, "hello world");
}
