use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::Arch;
use std::sync::Arc;

const RET: u32 = 0xd65f_03c0;

fn lift(base: u64, words: &[u32]) -> String {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    let image = Image::raw(Arc::from(bytes), Arch::Aarch64, base);
    let program = Program::analyze(&image, &Limits::default(), &NoControl).unwrap();
    pseudo::lift(&image, &program, program.functions.get(&base).unwrap(), &|_| None).text
}

#[test]
fn a_hidden_page_replaces_the_old_word_immediate() {
    // MOVZ W9, #0x2493; ADRP X9, #0x1000; MOVK W9, #0x9249, LSL #16.
    let text = lift(0x1000, &[0x5284_9269, 0x9000_0009, 0x72b2_4929, RET]);
    assert!(text.contains("w9 = 0x92491000;"), "{text}");
    assert!(!text.contains("w9 = 0x92492493;") && !text.contains("(w9 &"), "{text}");
}

#[test]
fn a_hidden_page_replaces_the_old_x_immediate() {
    let text = lift(0x1000, &[0xd284_9269, 0x9000_0009, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92491000;"), "{text}");
    assert!(!text.contains("x9 = 0x92492493;") && !text.contains("(x9 &"), "{text}");
}

#[test]
fn a_page_without_a_previous_immediate_is_still_the_current_value() {
    let text = lift(0x1000, &[0x9000_0009, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92491000;"), "{text}");
}

#[test]
fn word_insert_zero_extends_a_page_above_four_gib() {
    let text = lift(0x1_0000_1000, &[0x9000_0009, 0x72b2_4929, RET]);
    assert!(text.contains("w9 = 0x92491000;"), "{text}");
}

#[test]
fn x_insert_preserves_a_page_above_four_gib() {
    let text = lift(0x1_0000_1000, &[0x9000_0009, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x192491000;"), "{text}");
}

#[test]
fn a_second_page_load_supplies_the_latest_page() {
    // ADRP X9, #0x1000; ADRP X9, #0x3000.
    let text = lift(0x1000, &[0x9000_0009, 0xd000_0009, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92493000;"), "{text}");
}

#[test]
fn a_backward_page_reference_keeps_its_decoded_address() {
    // ADRP X9, #0; MOVK X9, #0x9249, LSL #16.
    let text = lift(0x1000, &[0xf0ff_ffe9, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92490000;"), "{text}");
}

#[test]
fn unrelated_instructions_leave_the_page_value_available() {
    // MOV W10, #7 and NOP neither read nor write X9.
    let text = lift(0x1000, &[0x9000_0009, 0x5280_00ea, 0xd503_201f, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92491000;") && text.contains("w10 = 7;"), "{text}");
}

#[test]
fn later_inserts_can_extend_the_correct_page_based_constant() {
    // The final MOVK writes 0x1234 into bits 48..63.
    let text = lift(0x1_0000_1000, &[0x9000_0009, 0xf2b2_4929, 0xf2e2_4689, RET]);
    assert!(text.contains("x9 = 0x1234000192491000;"), "{text}");
}

#[test]
fn a_later_immediate_replaces_the_page() {
    let text = lift(0x1000, &[0x9000_0009, 0xd284_9269, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92492493;"), "{text}");
}

#[test]
fn a_page_load_to_another_register_keeps_the_original_constant() {
    let text = lift(0x1000, &[0xd284_9269, 0x9000_000a, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = 0x92492493;"), "{text}");
}

#[test]
fn a_call_does_not_leave_a_page_in_a_temporary_register_known() {
    let text = lift(0x1000, &[0x9000_0009, 0xd63f_0200, 0xf2b2_4929, RET]);
    assert!(text.contains("x9 = (x9 & 0xffffffff0000ffff) | 0x92490000;"), "{text}");
    assert!(!text.contains("x9 = 0x92491000;"), "{text}");
}
