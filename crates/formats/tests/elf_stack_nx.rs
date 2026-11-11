use craft_anything_formats::parse_binary;
use craft_anything_testkit::elf64_core;
use craft_anything_testkit::flutter::SymbolTable;
use craft_anything_testkit::sectionless::sectionless_elf;

fn put(data: &mut [u8], at: usize, width: usize, value: u64, big: bool) {
    let bytes = if big { value.to_be_bytes() } else { value.to_le_bytes() };
    let bytes = if big { &bytes[8 - width..] } else { &bytes[..width] };
    data[at..at + width].copy_from_slice(bytes);
}

fn stack_sample(is64: bool, big: bool, flags: Option<u64>) -> Vec<u8> {
    let mut data = sectionless_elf(is64, big, &["entry"], SymbolTable::DynamicHash);
    if let Some(flags) = flags {
        // The builder leaves header padding before its data at offset 0x100.
        let at = if is64 { 64 + 2 * 56 } else { 52 + 2 * 32 };
        put(&mut data, at, 4, 0x6474_e551, big);
        put(&mut data, at + if is64 { 4 } else { 24 }, 4, flags, big);
        put(&mut data, at + if is64 { 48 } else { 28 }, if is64 { 8 } else { 4 }, 16, big);
        put(&mut data, if is64 { 56 } else { 44 }, 2, 3, big);
    }
    data
}

#[test]
fn missing_stack_declaration_keeps_nx_unknown() {
    for is64 in [false, true] {
        for big in [false, true] {
            for kind in [2, 3] {
                let mut data = stack_sample(is64, big, None);
                put(&mut data, 16, 2, kind, big);
                let binary = parse_binary(&data).unwrap();
                assert_eq!(binary.mitigations.nx, None, "class64={is64}, big={big}, type={kind}");
                assert!(binary.exports.iter().any(|export| export.name == "entry"));
                assert!(binary.imports.iter().any(|import| import.name == "puts"));
            }
        }
    }
}

#[test]
fn declared_non_executable_stack_remains_known() {
    for is64 in [false, true] {
        for big in [false, true] {
            let binary = parse_binary(&stack_sample(is64, big, Some(6))).unwrap();
            assert_eq!(binary.mitigations.nx, Some(true), "class64={is64}, big={big}");
        }
    }
}

#[test]
fn declared_executable_stack_remains_known() {
    for is64 in [false, true] {
        for big in [false, true] {
            let binary = parse_binary(&stack_sample(is64, big, Some(7))).unwrap();
            assert_eq!(binary.mitigations.nx, Some(false), "class64={is64}, big={big}");
        }
    }
}

#[test]
fn relocatable_without_program_headers_has_unknown_nx() {
    for is64 in [false, true] {
        for big in [false, true] {
            let mut data = stack_sample(is64, big, None);
            put(&mut data, 16, 2, 1, big);
            put(&mut data, if is64 { 32 } else { 28 }, if is64 { 8 } else { 4 }, 0, big);
            put(&mut data, if is64 { 56 } else { 44 }, 2, 0, big);
            assert_eq!(parse_binary(&data).unwrap().mitigations.nx, None, "class64={is64}, big={big}");
        }
    }
}

#[test]
fn core_without_stack_declaration_has_unknown_nx() {
    assert_eq!(parse_binary(&elf64_core(0x401000, 0xdead)).unwrap().mitigations.nx, None);
}
