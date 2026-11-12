use craft_anything_disasm::{Function, Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::{Arch, Binary, Symbol, SymbolKind};

fn lift(bytes: Vec<u8>, max_insns: usize) -> (pseudo::Pseudo, Function) {
    let image = Image::raw(bytes.into(), Arch::X86_64, 0x1000);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, max_insns_per_function: max_insns, ..Limits::default() }, &NoControl).unwrap();
    let function = program.functions.get(&0x1000).unwrap();
    (pseudo::lift(&image, &program, function, &|_| None), function.clone())
}

fn reverse_chain(blocks: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for block in 0..blocks {
        bytes.push(0xb8); // each visited block writes a different eax value
        bytes.extend_from_slice(&u32::from(block).to_le_bytes());
        if block == 1 {
            bytes.extend_from_slice(&[0xc3, 0x90, 0x90, 0x90, 0x90]);
        } else {
            let target = if block == 0 { blocks - 1 } else { block - 1 };
            let displacement = (i32::from(target) - i32::from(block)) * 10 - 10;
            bytes.push(0xe9);
            bytes.extend_from_slice(&displacement.to_le_bytes());
        }
    }
    bytes
}

fn forward_chain(blocks: usize, tail_call: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 1..blocks {
        bytes.extend_from_slice(&[0xe9, 0, 0, 0, 0]);
    }
    if tail_call {
        bytes.push(0xe9);
        let displacement = 0x90_0000i32 - 0x1000 - i32::try_from(bytes.len()).unwrap() - 4;
        bytes.extend_from_slice(&displacement.to_le_bytes());
    } else {
        bytes.push(0xc3);
    }
    bytes
}

fn loops(stages: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 0..stages {
        // test ecx,ecx; jz exit; dec ecx; jmp header; exit: inc eax
        bytes.extend_from_slice(&[0x85, 0xc9, 0x0f, 0x84, 7, 0, 0, 0, 0xff, 0xc9, 0xe9, 0xf1, 0xff, 0xff, 0xff, 0xff, 0xc0]);
    }
    bytes.push(0xc3);
    bytes
}

fn assert_linear(result: &pseudo::Pseudo) {
    assert!(!result.structured, "unfinished control-flow proof must not claim structured output");
    assert!(result.limitations.iter().any(|(kind, _)| kind == "not_structured"));
    assert!(!result.text.contains("while (true)"), "the acyclic/unknown graph must not acquire loops");
}

fn assert_reverse_edges(result: &pseudo::Pseudo, blocks: u16) {
    assert_eq!(result.gotos, usize::from(blocks - 1));
    for block in (0..blocks).filter(|&b| b != 1) {
        let target = if block == 0 { blocks - 1 } else { block - 1 };
        assert!(result.text.contains(&format!("goto label_{:x};", 0x1000 + u64::from(target) * 10)), "missing jump from block {block}");
    }
}

#[test]
fn unfinished_dominators_do_not_invent_loops() {
    let (result, function) = lift(reverse_chain(1000), 60_000);
    assert_eq!(function.blocks.len(), 1000);
    assert!(!function.truncated);
    assert_linear(&result);
    assert!(result.limitations.iter().any(|(_, message)| message.contains("converge")));
    assert_reverse_edges(&result, 1000);
}

#[test]
fn unfinished_post_dominators_are_not_used_for_structure() {
    let (result, function) = lift(forward_chain(256, false), 60_000);
    assert_eq!(function.blocks.len(), 256);
    assert_linear(&result);
    assert!(result.limitations.iter().any(|(_, message)| message.contains("converge")));
    assert_eq!(result.gotos, 255);
}

#[test]
fn block_limit_fallback_preserves_reverse_jumps() {
    let (result, function) = lift(reverse_chain(1501), 60_000);
    assert_eq!(function.blocks.len(), 1501);
    assert_linear(&result);
    assert_reverse_edges(&result, 1501);
}

#[test]
fn block_limit_fallback_preserves_both_conditional_edges_and_back_edges() {
    let (result, function) = lift(loops(501), 60_000);
    assert!(function.blocks.len() > 1500);
    assert_linear(&result);
    for stage in 0..501u64 {
        let header = 0x1000 + stage * 17;
        assert!(result.text.contains(&format!("goto label_{header:x};")), "missing loop back edge");
        assert!(result.text.contains(&format!("goto label_{:x};", header + 8)), "missing conditional fallthrough");
        assert!(result.text.contains(&format!("if (ecx == 0) goto label_{:x};", header + 15)), "missing conditional taken edge");
    }
}

fn diamond() -> Vec<u8> {
    vec![0x83, 0xf8, 0, 0x74, 6, 0xb8, 1, 0, 0, 0, 0xc3, 0xb8, 2, 0, 0, 0, 0xc3]
}

#[test]
fn instruction_limit_does_not_drop_the_unvisited_branch() {
    let (result, function) = lift(diamond(), 4);
    assert!(function.truncated);
    assert_eq!(function.blocks.len(), 2);
    assert_linear(&result);
    assert!(result.text.contains("if (eax == 0) goto label_100b;"));
    assert!(result.text.contains("goto label_1005;"));
    assert!(result.limitations.iter().any(|(kind, _)| kind == "unresolved_blocks"));
}

#[test]
fn instruction_limit_keeps_both_unvisited_conditional_targets() {
    let (result, function) = lift(diamond(), 2);
    assert!(function.truncated);
    assert_eq!(function.blocks.len(), 1);
    assert_linear(&result);
    assert!(result.text.contains("if (eax == 0) goto label_100b;"));
    assert!(result.text.contains("goto label_1005;"));
}

#[test]
fn missing_fallthrough_is_not_an_unconditional_loop() {
    let (result, function) = lift(vec![0x83, 0xf8, 0, 0x74, 0xfb], 60_000); // cmp eax,0; jz entry; missing fallthrough
    assert!(!function.truncated);
    assert_linear(&result);
    assert!(result.text.contains("if (eax == 0) goto label_1000;"));
    assert!(result.text.contains("goto label_1005;"));
    assert!(result.limitations.iter().any(|(kind, _)| kind == "unresolved_blocks"));
}

#[test]
fn a_nonterminating_instruction_at_eof_keeps_its_unknown_continuation() {
    let (result, function) = lift(vec![0xb8, 1, 0, 0, 0], 60_000);
    assert!(!function.truncated);
    assert_linear(&result);
    assert!(result.text.contains("goto label_1005;"));
    assert!(result.limitations.iter().any(|(kind, _)| kind == "unresolved_blocks"));
}

#[test]
fn an_ordinary_direct_call_at_eof_keeps_its_unknown_continuation() {
    let (result, _) = lift(vec![0xe8, 0xfb, 0x0f, 0, 0], 60_000); // call 0x2000
    assert_linear(&result);
    assert!(result.text.contains("goto label_1005;"));
}

#[test]
fn an_indirect_call_at_eof_keeps_its_unknown_continuation() {
    let (result, _) = lift(vec![0xff, 0xd1], 60_000); // call rcx
    assert_linear(&result);
    assert!(result.text.contains("goto label_1002;"));
}

#[test]
fn a_known_noreturn_call_does_not_get_a_fabricated_continuation() {
    let mut bytes = vec![0xe8, 0x0b, 0, 0, 0]; // call abort at 0x1010
    bytes.resize(16, 0x90);
    bytes.extend_from_slice(&[0x0f, 0x0b]); // ud2
    let mut binary = Binary::raw(Arch::X86_64, 0x1000, bytes.len() as u64, 0);
    binary.symbols.push(Symbol {
        name: "abort".into(),
        demangled: None,
        addr: 0x1010,
        size: 2,
        kind: SymbolKind::Function,
        global: true,
        weak: false,
        defined: true,
        dynamic: false,
    });
    let image = Image::from_binary(bytes.into(), binary);
    let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl).unwrap();
    let function = program.functions.get(&0x1000).unwrap();
    let result = pseudo::lift(&image, &program, function, &|_| None);
    assert!(result.structured);
    assert!(result.text.contains("abort();"));
    assert!(!result.text.contains("goto "));
}

#[test]
fn per_block_decode_limit_retains_an_explicit_continuation() {
    let mut bytes = vec![0x90; 20_001];
    bytes.extend_from_slice(&[0xb8, 7, 0, 0, 0, 0xc3]);
    let (result, function) = lift(bytes, 60_000);
    assert!(!function.truncated);
    assert_eq!(function.insn_count, 20_003);
    assert_linear(&result);
    assert!(result.text.contains("goto label_5e20;"));
    assert!(result.limitations.iter().any(|(kind, _)| kind == "instructions_not_lifted"));
}

#[test]
fn per_block_limit_does_not_jump_past_unlifted_instructions() {
    let mut bytes = vec![0x90; 20_001];
    bytes.extend_from_slice(&diamond());
    let (result, function) = lift(bytes, 60_000);
    assert!(!function.truncated);
    assert_linear(&result);
    let first_block = result.text.split("label_5e26:").next().unwrap();
    assert!(first_block.contains("goto label_5e20;"), "continuation must begin at the first unlifted instruction: {first_block}");
}

#[test]
fn a_small_complete_reverse_chain_stays_structured() {
    let (result, function) = lift(reverse_chain(16), 60_000);
    assert_eq!(function.blocks.len(), 16);
    assert!(result.structured);
    assert!(!result.text.contains("while (true)"));
    assert!(!result.limitations.iter().any(|(kind, _)| kind == "not_structured"));
}

#[test]
fn a_small_complete_diamond_stays_structured() {
    let (result, _) = lift(diamond(), 60_000);
    assert!(result.structured);
    assert!(result.text.contains("if (eax == 0)"));
    assert!(result.text.contains("eax = 1;") && result.text.contains("eax = 2;"));
}

#[test]
fn a_small_complete_loop_stays_structured() {
    let (result, _) = lift(loops(1), 60_000);
    assert!(result.structured);
    assert!(result.text.contains("while (true)"));
}

#[test]
fn linear_fallback_keeps_tail_calls_as_returns() {
    let (result, function) = lift(forward_chain(1501, true), 60_000);
    assert_eq!(function.blocks.len(), 1501);
    assert!(result.text.contains("return sub_900000(); /* tail call */"));
    assert!(!result.text.contains("goto label_900000;"));
}
