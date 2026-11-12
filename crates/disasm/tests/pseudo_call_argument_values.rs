use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::Arch;

fn lift(bytes: Vec<u8>, arch: Arch) -> String {
    let image = Image::raw(bytes.into(), arch, 0x1000);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    let function = program.functions.get(&0x1000).unwrap();
    assert!(!function.truncated);
    pseudo::lift(&image, &program, function, &|_| None).text
}

fn x86(prefix: &[u8]) -> String {
    let mut bytes = prefix.to_vec();
    bytes.extend_from_slice(&[0xff, 0xd1, 0xc3]); // call rcx; ret
    lift(bytes, Arch::X86_64)
}

fn a64(prefix: &[u32], call: u32) -> String {
    let bytes = prefix.iter().copied().chain([call, 0xd65f_03c0]).flat_map(u32::to_le_bytes).collect();
    lift(bytes, Arch::Aarch64)
}

fn kept(text: &str, assignment: &str, call: &str) {
    let assignment_pos = text.find(assignment).unwrap_or_else(|| panic!("assignment/effect was lost: {assignment}\n{text}"));
    let call_pos = text.find(call).unwrap_or_else(|| panic!("argument no longer uses its saved value: {call}\n{text}"));
    assert!(assignment_pos < call_pos, "effect must remain before its call:\n{text}");
}

#[test]
fn an_overwritten_source_register_does_not_change_the_argument() {
    let text = x86(&[0x48, 0x89, 0xc7, 0x48, 0xc7, 0xc0, 7, 0, 0, 0]); // mov rdi,rax; mov rax,7
    kept(&text, "rdi = rax;", "(*rcx)(rdi);");
}

#[test]
fn a_partial_source_write_does_not_change_the_argument() {
    let text = x86(&[0x48, 0x89, 0xc7, 0xb0, 7]); // mov rdi,rax; mov al,7
    kept(&text, "rdi = rax;", "(*rcx)(rdi);");
}

#[test]
fn a_memory_store_does_not_reload_an_earlier_argument() {
    let text = x86(&[0x48, 0x8b, 0x38, 0x48, 0x89, 0x18]); // mov rdi,[rax]; mov [rax],rbx
    kept(&text, "rdi = *(uint64_t*)(rax);", "(*rcx)(rdi);");
}

#[test]
fn a_memory_base_write_does_not_change_the_argument_address() {
    let text = x86(&[0x48, 0x8b, 0x38, 0x48, 0xc7, 0xc0, 7, 0, 0, 0]); // mov rdi,[rax]; mov rax,7
    kept(&text, "rdi = *(uint64_t*)(rax);", "(*rcx)(rdi);");
}

#[test]
fn an_unknown_memory_exchange_does_not_reload_an_argument() {
    let text = x86(&[0x48, 0x8b, 0x38, 0x48, 0x87, 0x18]); // mov rdi,[rax]; xchg [rax],rbx
    kept(&text, "rdi = *(uint64_t*)(rax);", "(*rcx)(rdi);");
}

#[test]
fn an_implicit_stack_write_does_not_change_a_saved_stack_pointer() {
    let text = x86(&[0x48, 0x89, 0xe7, 0x53]); // mov rdi,rsp; push rbx
    kept(&text, "rdi = rsp;", "(*rcx)(rdi);");
}

#[test]
fn an_indirect_callee_assignment_is_not_deleted() {
    let text = lift(vec![0x48, 0x89, 0xc7, 0xff, 0xd7, 0xc3], Arch::X86_64); // mov rdi,rax; call rdi
    kept(&text, "rdi = rax;", "(*rdi)(rdi);");
}

#[test]
fn an_indirect_callee_memory_base_is_not_deleted() {
    let text = lift(vec![0x48, 0x89, 0xc7, 0xff, 0x17, 0xc3], Arch::X86_64); // mov rdi,rax; call [rdi]
    assert!(text.contains("rdi = rax;"), "callee address assignment must survive:\n{text}");
}

#[test]
fn a_pop_is_not_moved_after_a_stack_read() {
    let bytes = vec![
        0xe9, 0, 0, 0, 0,    // entry jumps into a non-entry, non-exit block
        0x5f, // pop rdi
        0x48, 0x8b, 0x1c, 0x24, // mov rbx,[rsp]
        0xff, 0xd1, // call rcx
        0xe9, 0, 0, 0, 0, // next block
        0xc3,
    ];
    let text = lift(bytes, Arch::X86_64);
    kept(&text, "rdi = pop();", "(*rcx)(rdi);");
    assert!(text.find("rdi = pop();").unwrap() < text.find("rbx =").unwrap(), "the stack read must stay after pop:\n{text}");
}

#[test]
fn an_implicit_multiply_result_is_not_an_assignment_to_its_source() {
    let text = x86(&[0x48, 0xc7, 0xc0, 2, 0, 0, 0, 0x48, 0xf7, 0xe7]); // mov rax,2; mul rdi
    assert!(text.contains("rdx:rax = rax * rdi;"), "multiply must not be removed as an argument assignment:\n{text}");
    assert!(!text.contains("(*rcx)(rax * rdi)"));
}

#[test]
fn an_a64_store_is_not_an_argument_assignment() {
    let text = a64(&[0xf900_0020], 0xd63f_0200); // str x0,[x1]; blr x16
    assert!(text.contains("*(void*)(x1) = x0;"), "store must not be removed:\n{text}");
}

#[test]
fn an_a64_overwritten_source_keeps_the_saved_argument() {
    let text = a64(&[0xaa09_03e0, 0xd280_00e9], 0xd63f_0200); // mov x0,x9; mov x9,7
    kept(&text, "x0 = x9;", "(*x16)(x0);");
}

#[test]
fn an_a64_word_write_invalidates_the_full_source_alias() {
    let text = a64(&[0xaa09_03e0, 0x5280_00e9], 0xd63f_0200); // mov x0,x9; mov w9,7
    kept(&text, "x0 = x9;", "(*x16)(x0);");
}

#[test]
fn an_a64_store_does_not_reload_a_saved_argument() {
    let text = a64(&[0xf940_0120, 0xf900_012a], 0xd63f_0200); // ldr x0,[x9]; str x10,[x9]
    kept(&text, "x0 = *(void*)(x9);", "(*x16)(x0);");
}

#[test]
fn an_a64_indirect_callee_assignment_is_retained() {
    let text = a64(&[0xaa09_03e0], 0xd63f_0000); // mov x0,x9; blr x0
    kept(&text, "x0 = x9;", "(*x0)(x0);");
}

#[test]
fn an_unrelated_register_write_keeps_safe_inlining() {
    let text = x86(&[0x48, 0x89, 0xc7, 0x48, 0xc7, 0xc3, 7, 0, 0, 0]); // mov rdi,rax; mov rbx,7
    assert!(text.contains("(*rcx)(rax);"), "{text}");
    assert!(!text.contains("rdi = rax;"), "{text}");
}

#[test]
fn an_unchanged_memory_value_can_still_be_inlined() {
    let text = x86(&[0x48, 0x8b, 0x38, 0x90]);
    assert!(text.contains("(*rcx)(*(uint64_t*)(rax));"), "{text}");
}

#[test]
fn an_address_expression_does_not_reread_stored_memory() {
    let text = x86(&[0x48, 0x8d, 0x38, 0x48, 0x89, 0x18]); // lea rdi,[rax]; mov [rax],rbx
    assert!(text.contains("(*rcx)(rax);"), "{text}");
}

#[test]
fn a_constant_argument_survives_unrelated_mutations() {
    let text = x86(&[0x48, 0xc7, 0xc7, 7, 0, 0, 0, 0x48, 0xc7, 0xc0, 9, 0, 0, 0]);
    assert!(text.contains("(*rcx)(7);"), "{text}");
}

#[test]
fn replacing_an_argument_replaces_its_dependencies() {
    let text = x86(&[0x48, 0x89, 0xc7, 0x48, 0xc7, 0xc7, 7, 0, 0, 0, 0x48, 0xc7, 0xc0, 9, 0, 0, 0]);
    assert!(text.contains("(*rcx)(7);"), "{text}");
}
