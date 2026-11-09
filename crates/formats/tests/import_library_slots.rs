use craft_anything_formats::parse_binary;
use craft_anything_testkit::{Buf, PE_BASE, PE_IAT_RVA, pe64, program_facts::elf_versioned_library};

fn same_named_imports(second_library: &[u8]) -> Vec<u8> {
    let mut data = pe64();
    data[0x114..0x118].copy_from_slice(&60u32.to_le_bytes());
    for (offset, value) in [(0x714, 0x3020u32), (0x720, 0x3060), (0x724, 0x3040)] {
        data[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    data[0x820..0x828].copy_from_slice(&0x2180u64.to_le_bytes());
    data[0x840..0x848].copy_from_slice(&0x2180u64.to_le_bytes());
    data[0x860..0x860 + second_library.len()].copy_from_slice(second_library);
    data
}

#[test]
fn same_named_imports_keep_their_individually_recorded_slots() {
    for library in [b"other.dll\0".as_slice(), b"msvcrt.dll\0".as_slice()] {
        let binary = parse_binary(&same_named_imports(library)).unwrap();
        let imports: Vec<_> = binary.imports.iter().filter(|import| import.name == "puts").collect();
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].library.as_deref(), Some("msvcrt.dll"));
        assert_eq!(imports[0].slot, Some(PE_BASE + PE_IAT_RVA));
        assert_eq!(imports[1].library.as_deref(), Some(std::str::from_utf8(&library[..library.len() - 1]).unwrap()));
        assert_eq!(imports[1].slot, Some(PE_BASE + 0x3040));
    }
}

#[test]
fn a_unique_import_keeps_its_existing_slot() {
    let binary = parse_binary(&pe64()).unwrap();
    let import = binary.imports.iter().find(|import| import.name == "puts").unwrap();
    assert_eq!(import.slot, Some(PE_BASE + PE_IAT_RVA));
}

fn versioned_elf_slots(relocations: &[(u64, u64)]) -> (Vec<u8>, u64) {
    let bytes = elf_versioned_library();
    let shoff = u64::from_le_bytes(bytes[40..48].try_into().unwrap()) as usize;
    let sections = u16::from_le_bytes(bytes[60..62].try_into().unwrap());
    let header = shoff + usize::from(sections) * 64;
    let rela = header + 64;
    let got = rela + relocations.len() * 24;
    let mut data = Buf(bytes);
    data.0.resize(got + 16, 0);
    data.u16(60, sections + 1);
    // An additional SHT_RELA section linked to the existing .dynsym (index 2).
    data.u32(header + 4, 4);
    data.u64(header + 8, 2);
    data.u64(header + 16, rela as u64);
    data.u64(header + 24, rela as u64);
    data.u64(header + 32, (relocations.len() * 24) as u64);
    data.u32(header + 40, 2);
    data.u64(header + 48, 8);
    data.u64(header + 56, 24);
    for (i, (kind, slot)) in relocations.iter().enumerate() {
        data.u64(rela + i * 24, got as u64 + slot * 8);
        data.u64(rela + i * 24 + 8, (4 << 32) | kind); // puts is dynamic symbol 4
    }
    let size = data.0.len() as u64;
    data.u64(0x40 + 32, size);
    data.u64(0x40 + 40, size);
    (data.into_vec(), got as u64)
}

#[test]
fn versioned_elf_imports_keep_unique_slots_without_library_metadata() {
    for relocations in [&[(6, 0)][..], &[(7, 0)][..], &[(6, 0), (7, 0)][..]] {
        let (data, got) = versioned_elf_slots(relocations);
        let binary = parse_binary(&data).unwrap();
        let import = binary.imports.iter().find(|import| import.name == "puts").unwrap();
        assert_eq!(import.library.as_deref(), Some("libc.so.6"));
        assert!(binary.import_slots.iter().any(|slot| slot.name == "puts" && slot.library.is_none() && slot.addr == got));
        assert_eq!(import.slot, Some(got));
    }
}

#[test]
fn versioned_elf_imports_do_not_guess_between_unlabelled_slot_addresses() {
    let (data, got) = versioned_elf_slots(&[(6, 0), (7, 1)]);
    let binary = parse_binary(&data).unwrap();
    let slots: Vec<_> = binary.import_slots.iter().filter(|slot| slot.name == "puts").map(|slot| slot.addr).collect();
    assert_eq!(slots, [got, got + 8]);
    let import = binary.imports.iter().find(|import| import.name == "puts").unwrap();
    assert_eq!(import.library.as_deref(), Some("libc.so.6"));
    assert_eq!(import.slot, None);
}
