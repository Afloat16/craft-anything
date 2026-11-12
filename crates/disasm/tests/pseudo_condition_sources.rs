use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::Arch;

fn lift(bytes: Vec<u8>, arch: Arch) -> pseudo::Pseudo {
    let image = Image::raw(bytes.into(), arch, 0x1000);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    let function = program.functions.get(&0x1000).unwrap();
    assert!(!function.truncated);
    pseudo::lift(&image, &program, function, &|_| None)
}

fn x86(prefix: &[u8]) -> pseudo::Pseudo {
    let mut bytes = prefix.to_vec();
    bytes.extend_from_slice(&[
        0x74, 0x06, // jz: the two arms return different values
        0xb8, 1, 0, 0, 0, 0xc3, // mov eax, 1; ret
        0xb8, 2, 0, 0, 0, 0xc3, // mov eax, 2; ret
    ]);
    lift(bytes, Arch::X86_64)
}

fn a64(prefix: &[u32], branch: u32) -> pseudo::Pseudo {
    let words: Vec<u32> = prefix.iter().copied().chain([branch, 0x5280_0020, 0xd65f_03c0, 0x5280_0040, 0xd65f_03c0]).collect();
    lift(words.into_iter().flat_map(u32::to_le_bytes).collect(), Arch::Aarch64)
}

fn unknown(result: pseudo::Pseudo) {
    assert!(result.text.contains("/* Eq */ flags"), "stale inputs must not become a concrete condition:\n{}", result.text);
    assert!(result.text.contains("if ("), "the conditional branch must remain visible:\n{}", result.text);
}

fn concrete(result: pseudo::Pseudo, expression: &str) {
    assert!(result.text.contains(expression), "expected {expression:?}:\n{}", result.text);
    assert!(!result.text.contains("/* Eq */ flags"), "valid comparison was lost:\n{}", result.text);
}

#[test]
fn call_invalidates_comparison() {
    unknown(x86(&[0x39, 0xd8, 0xff, 0xd1])); // cmp eax, ebx; call rcx
}

#[test]
fn register_write_invalidates_comparison() {
    unknown(x86(&[0x39, 0xd8, 0xb8, 7, 0, 0, 0])); // cmp eax, ebx; mov eax, 7
}

#[test]
fn partial_register_write_invalidates_comparison() {
    unknown(x86(&[0x39, 0xd8, 0xb0, 4])); // cmp eax, ebx; mov al, 4
}

#[test]
fn zero_idiom_result_is_still_a_condition_input() {
    unknown(x86(&[0x31, 0xc0, 0xb8, 1, 0, 0, 0])); // xor eax, eax; mov eax, 1
}

#[test]
fn arithmetic_result_write_invalidates_condition() {
    unknown(x86(&[0x01, 0xd8, 0xb8, 7, 0, 0, 0])); // add eax, ebx; mov eax, 7
}

#[test]
fn implicit_push_stack_write_invalidates_stack_comparison() {
    unknown(x86(&[0x48, 0x39, 0xc4, 0x53])); // cmp rsp, rax; push rbx
}

#[test]
fn implicit_pop_stack_write_invalidates_stack_comparison() {
    unknown(x86(&[0x48, 0x39, 0xc4, 0x5b])); // cmp rsp, rax; pop rbx
}

#[test]
fn memory_store_invalidates_memory_comparison() {
    unknown(x86(&[0x39, 0x03, 0x89, 0x13])); // cmp [rbx], eax; mov [rbx], edx
}

#[test]
fn memory_base_write_invalidates_comparison() {
    unknown(x86(&[0x39, 0x03, 0x48, 0xc7, 0xc3, 7, 0, 0, 0])); // cmp [rbx], eax; mov rbx, 7
}

#[test]
fn memory_index_write_invalidates_comparison() {
    unknown(x86(&[0x39, 0x04, 0x8b, 0xb9, 7, 0, 0, 0])); // cmp [rbx+rcx*4], eax; mov ecx, 7
}

#[test]
fn implicit_memory_store_invalidates_comparison() {
    unknown(x86(&[0x39, 0x03, 0xaa])); // cmp [rbx], eax; stosb
}

#[test]
fn exchange_memory_write_invalidates_comparison() {
    unknown(x86(&[0x39, 0x03, 0x87, 0x13])); // cmp [rbx], eax; xchg [rbx], edx
}

#[test]
fn negation_does_not_leave_a_stale_compare() {
    unknown(x86(&[0x39, 0xd8, 0xf7, 0xd9])); // cmp eax, ebx; neg ecx
}

#[test]
fn restored_flags_do_not_become_a_fabricated_comparison() {
    unknown(x86(&[0x39, 0xd8, 0x9e])); // cmp eax, ebx; sahf
}

#[test]
fn popped_flags_do_not_become_a_fabricated_comparison() {
    unknown(x86(&[0x39, 0xd8, 0x9d])); // cmp eax, ebx; popfq
}

#[test]
fn unrelated_register_write_keeps_comparison() {
    concrete(x86(&[0x39, 0xd8, 0xb9, 7, 0, 0, 0]), "eax == ebx"); // mov ecx, 7
}

#[test]
fn arithmetic_condition_uses_result_not_old_rhs() {
    concrete(x86(&[0x01, 0xd8, 0xbb, 7, 0, 0, 0]), "eax == 0"); // add eax, ebx; mov ebx, 7
}

#[test]
fn readonly_load_keeps_memory_comparison() {
    concrete(x86(&[0x39, 0x03, 0x8b, 0x16]), "*(uint32_t*)(rbx) == eax"); // cmp [rbx], eax; mov edx, [rsi]
}

#[test]
fn nop_keeps_comparison() {
    concrete(x86(&[0x39, 0xd8, 0x90]), "eax == ebx");
}

#[test]
fn new_comparison_after_call_is_used() {
    concrete(x86(&[0x39, 0xd8, 0xff, 0xd1, 0x39, 0xf2]), "edx == esi"); // call rcx; cmp edx, esi
}

#[test]
fn a64_call_invalidates_comparison() {
    unknown(a64(&[0xeb0a_013f, 0xd63f_0200], 0x5400_0060)); // cmp x9, x10; blr x16; b.eq +12
}

#[test]
fn hidden_a64_page_load_invalidates_comparison() {
    unknown(a64(&[0xeb0a_013f, 0x9000_0009], 0x5400_0060)); // cmp x9, x10; adrp x9; b.eq +12
}

#[test]
fn a64_shifted_input_write_invalidates_comparison() {
    unknown(a64(&[0xeb0a_053f, 0x5280_002a], 0x5400_0060)); // cmp x9, x10, lsl #1; mov w10, 1
}

#[test]
fn a64_register_branch_does_not_depend_on_flags() {
    concrete(a64(&[0xeb0a_013f, 0xd63f_0200], 0xb400_0069), "x9 == 0"); // cbz x9, +12
}

#[test]
fn a64_bit_branch_does_not_depend_on_flags() {
    concrete(a64(&[0xeb0a_013f, 0xd63f_0200], 0x3618_0069), ">> 3) & 1) == 0"); // tbz w9, #3, +12
}
