use craft_anything_disasm::{Image, Limits, Mode, NoControl, Op, Program, decode, pseudo};
use craft_anything_formats::Arch;

fn width(word: u32, expected: u8) {
    for fmt in [false, true] {
        let instruction = decode(Mode::A64, &word.to_le_bytes(), 0x1000, fmt).unwrap();
        let memory = instruction
            .ops
            .iter()
            .find_map(|op| match op {
                Op::Mem(m) => Some(m),
                _ => None,
            })
            .unwrap();
        assert_eq!(memory.size, expected, "{:08x} fmt={fmt}: {}", word, instruction.text());
    }
}

// Original assembly fixtures were assembled and independently disassembled before use.
// Pair widths describe one element, and exclusive-store status registers are not data.
#[test]
fn load_x() {
    width(0xf9400020, 8); // ldr x0, [x1]
}

#[test]
fn load_w() {
    width(0xb9400020, 4); // ldr w0, [x1]
}

#[test]
fn load_byte() {
    width(0x39400020, 1); // ldrb w0, [x1]
}

#[test]
fn load_half() {
    width(0x79400020, 2); // ldrh w0, [x1]
}

#[test]
fn load_signed_byte() {
    width(0x39800020, 1); // ldrsb x0, [x1]
}

#[test]
fn load_signed_half() {
    width(0x79c00020, 2); // ldrsh w0, [x1]
}

#[test]
fn load_signed_word() {
    width(0xb9800020, 4); // ldrsw x0, [x1]
}

#[test]
fn store_x() {
    width(0xf9000020, 8); // str x0, [x1]
}

#[test]
fn store_w() {
    width(0xb9000020, 4); // str w0, [x1]
}

#[test]
fn store_byte() {
    width(0x39000020, 1); // strb w0, [x1]
}

#[test]
fn store_half() {
    width(0x79000020, 2); // strh w0, [x1]
}

#[test]
fn unscaled_x() {
    width(0xf85f8020, 8); // ldur x0, [x1, #-8]
}

#[test]
fn unscaled_word_store() {
    width(0xb81fc020, 4); // stur w0, [x1, #-4]
}

#[test]
fn unscaled_signed_byte() {
    width(0x389ff020, 1); // ldursb x0, [x1, #-1]
}

#[test]
fn pair_x() {
    width(0xa9400820, 8); // ldp x0, x2, [x1]
}

#[test]
fn pair_w_store() {
    width(0x29000820, 4); // stp w0, w2, [x1]
}

#[test]
fn signed_pair() {
    width(0x69400820, 4); // ldpsw x0, x2, [x1]
}

#[test]
fn exclusive_byte() {
    width(0x085ffc20, 1); // ldaxrb w0, [x1]
}

#[test]
fn exclusive_x_store() {
    width(0xc8027c20, 8); // stxr w2, x0, [x1]
}

#[test]
fn exclusive_w_store() {
    width(0x8802fc20, 4); // stlxr w2, w0, [x1]
}

#[test]
fn unprivileged_half() {
    width(0x78400820, 2); // ldtrh w0, [x1]
}

#[test]
fn preindex_x() {
    width(0xf8408c20, 8); // ldr x0, [x1, #8]!
}

#[test]
fn postindex_w() {
    width(0xb8404420, 4); // ldr w0, [x1], #4
}

#[test]
fn signed_index_w() {
    width(0xb862d820, 4); // ldr w0, [x1, w2, sxtw #2]
}

#[test]
fn scaled_index_x() {
    width(0xf8227820, 8); // str x0, [x1, x2, lsl #3]
}

#[test]
fn unknown_simd() {
    width(0x3dc00020, 0); // ldr q0, [x1]
}

#[test]
fn prefetch() {
    width(0xf9800020, 0); // prfm pldl1keep, [x1]
}

fn lift(words: &[u32]) -> String {
    let bytes = words.iter().copied().chain([0xd65f_03c0]).flat_map(u32::to_le_bytes).collect::<Vec<_>>();
    let image = Image::raw(bytes.into(), Arch::Aarch64, 0x1000);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    pseudo::lift(&image, &program, program.functions.get(&0x1000).unwrap(), &|_| None).text
}

#[test]
fn loads_and_stores_use_their_access_width_in_pseudo_c() {
    for (word, expected) in [(0xb940_0020, "w0 = *(uint32_t*)(x1);"), (0x3900_0020, "*(uint8_t*)(x1) = w0;"), (0xf940_0020, "x0 = *(uint64_t*)(x1);")] {
        let text = lift(&[word]);
        assert!(text.contains(expected), "{text}");
    }
}

#[test]
fn a_resolved_byte_store_keeps_its_width() {
    // adr x1, 0x1020; strb w0, [x1]
    let text = lift(&[0x1000_0101, 0x3900_0020]);
    assert!(text.contains("*(uint8_t*)0x1020 = w0;"), "{text}");
}

#[test]
fn literal_loads_retain_a_sized_memory_operand_and_reference() {
    // ldr w0, 0x1020
    let instruction = decode(Mode::A64, &0x1800_0100u32.to_le_bytes(), 0x1000, true).unwrap();
    let memory = instruction
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Mem(m) => Some(m),
            _ => None,
        })
        .unwrap();
    assert_eq!((memory.size, memory.addr), (4, Some(0x1020)));
    assert_eq!(instruction.refs, vec![0x1020]);
    let text = lift(&[0x1800_0100]);
    assert!(text.contains("w0 = *(uint32_t*)0x1020;"), "{text}");
}

#[test]
fn address_calculations_and_branches_remain_address_operands() {
    for word in [0x1000_0101u32, 0x1400_0008] {
        let instruction = decode(Mode::A64, &word.to_le_bytes(), 0x1000, true).unwrap();
        assert!(instruction.ops.iter().any(|op| matches!(op, Op::Addr(0x1020))));
        assert!(!instruction.ops.iter().any(|op| matches!(op, Op::Mem(_))));
    }
}

#[test]
fn x86_memory_widths_are_unchanged() {
    let instruction = decode(Mode::X64, &[0x48, 0x8b, 0x01], 0x1000, true).unwrap();
    assert!(instruction.ops.iter().any(|op| matches!(op, Op::Mem(m) if m.size == 8)));
}
