//! Reproducible whole-lifter timings on encoded x86-64 control-flow fixtures.
//! Run with `cargo run --release -p craft-anything-disasm --example pseudo_graph_benchmark -- 5`.
//! The argument is lifts per timing sample. Compare every `pseudo` value as well as timings.

use craft_anything_disasm::{Image, Limits, NoControl, Program, pseudo};
use craft_anything_formats::Arch;
use serde_json::json;
use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

fn chain(blocks: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 1..blocks.max(1) {
        bytes.extend_from_slice(&[0xe9, 0, 0, 0, 0]); // jump to the next block
    }
    bytes.push(0xc3);
    bytes
}

fn reverse_chain(blocks: u16) -> Vec<u8> {
    let blocks = blocks.max(2);
    let mut bytes = Vec::new();
    for block in 0..blocks {
        if block == 1 {
            bytes.extend_from_slice(&[0xc3, 0x90, 0x90, 0x90, 0x90]);
        } else {
            let target = if block == 0 { blocks - 1 } else { block - 1 };
            let displacement = (i32::from(target) - i32::from(block)) * 5 - 5;
            bytes.push(0xe9);
            bytes.extend_from_slice(&displacement.to_le_bytes());
        }
    }
    bytes
}

fn diamonds(stages: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 0..stages {
        // test eax,eax; jz right; inc ecx; jmp join; right: inc edx; join:
        bytes.extend_from_slice(&[0x85, 0xc0, 0x0f, 0x84, 7, 0, 0, 0, 0xff, 0xc1, 0xe9, 2, 0, 0, 0, 0xff, 0xc2]);
    }
    bytes.push(0xc3);
    bytes
}

fn loops(stages: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 0..stages {
        // header: test ecx,ecx; jz exit; dec ecx; jmp header; exit: inc eax
        bytes.extend_from_slice(&[0x85, 0xc9, 0x0f, 0x84, 7, 0, 0, 0, 0xff, 0xc9, 0xe9, 0xf1, 0xff, 0xff, 0xff, 0xff, 0xc0]);
    }
    bytes.push(0xc3);
    bytes
}

fn many_exits(stages: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for _ in 0..stages {
        // test ecx,ecx; jz return; inc eax; jmp next; return: ret; next:
        bytes.extend_from_slice(&[0x85, 0xc9, 0x0f, 0x84, 7, 0, 0, 0, 0xff, 0xc0, 0xe9, 1, 0, 0, 0, 0xc3]);
    }
    bytes.push(0xc3);
    bytes
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iterations = std::env::args().nth(1).map(|arg| arg.parse::<usize>()).transpose()?.unwrap_or(5).max(1);
    let cases = [
        ("chain_16", chain(16)),
        ("chain_256", chain(256)),
        ("chain_1000", chain(1000)),
        ("chain_1500", chain(1500)),
        ("linear_fallback_1501", chain(1501)),
        ("reverse_chain_1000", reverse_chain(1000)),
        ("diamonds_64", diamonds(64)),
        ("diamonds_400", diamonds(400)),
        ("loops_64", loops(64)),
        ("loops_400", loops(400)),
        ("many_exits_400", many_exits(400)),
    ];
    let mut results = Vec::new();
    for (name, bytes) in cases {
        let code_bytes = bytes.len();
        let image = Image::raw(Arc::from(bytes), Arch::X86_64, 0x1000);
        let program = Program::analyze(&image, &Limits { scan_prologues: false, ..Limits::default() }, &NoControl)?;
        let function = program.functions.get(&0x1000).ok_or("fixture entry was not analyzed")?;
        let result = pseudo::lift(&image, &program, function, &|_| None);
        black_box(&result);
        let mut samples_ns = Vec::new();
        for _ in 0..9 {
            let start = Instant::now();
            for _ in 0..iterations {
                black_box(pseudo::lift(black_box(&image), black_box(&program), black_box(function), &|_| None));
            }
            samples_ns.push(start.elapsed().as_nanos());
        }
        let mut ordered = samples_ns.clone();
        ordered.sort_unstable();
        let median_ns = ordered.get(ordered.len() / 2).copied().unwrap_or(0) / iterations as u128;
        results.push(json!({
            "case": name,
            "code_bytes": code_bytes,
            "blocks": function.blocks.len(),
            "instructions": function.insn_count,
            "iterations_per_sample": iterations,
            "samples_ns": samples_ns,
            "median_ns": median_ns,
            "pseudo": result,
        }));
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
