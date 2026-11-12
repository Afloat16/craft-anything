use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::Arch;

fn lift(bytes: &[u8], arch: Arch) -> String {
    let mut bytes = bytes.to_vec();
    bytes.push(0xc3);
    let image = Image::raw(bytes.into(), arch, 0x1000);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    let result = pseudo::lift(&image, &program, program.functions.get(&0x1000).unwrap(), &|_| None);
    result.text
}

// Original byte fixtures were assembled and independently disassembled before use.
#[test]
fn sign_byte_to_quad() {
    let text = lift(&[0x48, 0xf, 0xbe, 0xc0], Arch::X86_64); // movsx rax, al
    assert!(text.contains("rax = (int64_t)(int8_t)al;"), "{text}");
}

#[test]
fn sign_word_to_quad() {
    let text = lift(&[0x48, 0xf, 0xbf, 0xc3], Arch::X86_64); // movsx rax, bx
    assert!(text.contains("rax = (int64_t)(int16_t)bx;"), "{text}");
}

#[test]
fn sign_dword_to_quad() {
    let text = lift(&[0x48, 0x63, 0xc1], Arch::X86_64); // movsxd rax, ecx
    assert!(text.contains("rax = (int64_t)(int32_t)ecx;"), "{text}");
}

#[test]
fn zero_byte_to_quad() {
    let text = lift(&[0x48, 0xf, 0xb6, 0xc2], Arch::X86_64); // movzx rax, dl
    assert!(text.contains("rax = (uint64_t)(uint8_t)dl;"), "{text}");
}

#[test]
fn zero_word_to_quad() {
    let text = lift(&[0x48, 0xf, 0xb7, 0xc6], Arch::X86_64); // movzx rax, si
    assert!(text.contains("rax = (uint64_t)(uint16_t)si;"), "{text}");
}

#[test]
fn extended_register_byte() {
    let text = lift(&[0x4d, 0xf, 0xbe, 0xd1], Arch::X86_64); // movsx r10, r9b
    assert!(text.contains("r10 = (int64_t)(int8_t)r9l;"), "{text}");
}

#[test]
fn high_byte_register() {
    let text = lift(&[0xf, 0xbe, 0xdc], Arch::X86_64); // movsx ebx, ah
    assert!(text.contains("ebx = (int32_t)(int8_t)ah;"), "{text}");
}

#[test]
fn frame_register_result_is_not_frame_setup() {
    let text = lift(&[0x48, 0xf, 0xbe, 0xe8], Arch::X86_64); // movsx rbp, al
    assert!(text.contains("rbp = (int64_t)(int8_t)al;"), "{text}");
}

#[test]
fn sign_memory_byte() {
    let text = lift(&[0x48, 0xf, 0xbe, 0x1], Arch::X86_64); // movsx rax, byte ptr [rcx]
    assert!(text.contains("rax = (int64_t)(int8_t)*(uint8_t*)(rcx);"), "{text}");
}

#[test]
fn zero_memory_word() {
    let text = lift(&[0xf, 0xb7, 0x3], Arch::X86_64); // movzx eax, word ptr [rbx]
    assert!(text.contains("eax = (uint32_t)(uint16_t)*(uint16_t*)(rbx);"), "{text}");
}

#[test]
fn sign_byte_in_32bit_mode() {
    let text = lift(&[0xf, 0xbe, 0xc1], Arch::X86); // movsx eax, cl
    assert!(text.contains("eax = (int32_t)(int8_t)cl;"), "{text}");
}

#[test]
fn sign_word_in_32bit_mode() {
    let text = lift(&[0xf, 0xbf, 0xd1], Arch::X86); // movsx edx, cx
    assert!(text.contains("edx = (int32_t)(int16_t)cx;"), "{text}");
}

#[test]
fn zero_memory_in_32bit_mode() {
    let text = lift(&[0xf, 0xb6, 0x3], Arch::X86); // movzx eax, byte ptr [ebx]
    assert!(text.contains("eax = (uint32_t)(uint8_t)*(uint8_t*)(ebx);"), "{text}");
}

#[test]
fn sign_byte_in_16bit_mode() {
    let text = lift(&[0xf, 0xbe, 0xc1], Arch::X86_16); // movsx ax, cl
    assert!(text.contains("ax = (int16_t)(int8_t)cl;"), "{text}");
}

#[test]
fn zero_high_byte_in_16bit_mode() {
    let text = lift(&[0xf, 0xb6, 0xd5], Arch::X86_16); // movzx dx, ch
    assert!(text.contains("dx = (uint16_t)(uint8_t)ch;"), "{text}");
}

#[test]
fn plain_move_is_unchanged() {
    let text = lift(&[0x48, 0x89, 0xc8], Arch::X86_64); // mov rax, rcx
    assert!(text.contains("rax = rcx;"), "{text}");
}

#[test]
fn plain_memory_move_is_unchanged() {
    let text = lift(&[0x8b, 0x3], Arch::X86_64); // mov eax, dword ptr [rbx]
    assert!(text.contains("eax = *(uint32_t*)(rbx);"), "{text}");
}

#[test]
fn a_folded_signed_argument_keeps_its_conversion() {
    let text = lift(&[0x48, 0x0f, 0xbe, 0xf8, 0xff, 0xd1], Arch::X86_64); // movsx rdi,al; call rcx
    assert!(text.contains("(*rcx)((int64_t)(int8_t)al);"), "{text}");
}

#[test]
fn a_folded_unsigned_argument_keeps_its_conversion() {
    let text = lift(&[0x0f, 0xb6, 0xf8, 0xff, 0xd1], Arch::X86_64); // movzx edi,al; call rcx
    assert!(text.contains("(*rcx)((uint32_t)(uint8_t)al);"), "{text}");
}
