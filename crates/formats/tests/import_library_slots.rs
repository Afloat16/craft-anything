use craft_anything_formats::parse_binary;
use craft_anything_testkit::{PE_BASE, PE_IAT_RVA, pe64};

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
