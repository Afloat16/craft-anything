use craft_anything_formats::zip::{self, ZipArchive, ZipError};
use craft_anything_testkit::electron::zip_stored;

fn warning_contains(archive: &ZipArchive, text: &str) -> bool {
    serde_json::to_value(archive).unwrap()["warnings"]
        .as_array()
        .is_some_and(|warnings| warnings.iter().any(|warning| warning.as_str().is_some_and(|warning| warning.contains(text))))
}

fn zip64_with_extension(extension: &[u8]) -> (Vec<u8>, usize) {
    let mut data = zip_stored(&[("a.txt", b"retained payload")]);
    let end = data.len() - 22;
    let size = u32::from_le_bytes(data[end + 12..end + 16].try_into().unwrap());
    let offset = u32::from_le_bytes(data[end + 16..end + 20].try_into().unwrap());
    let mut record = b"PK\x06\x06".to_vec();
    record.extend((44 + extension.len() as u64).to_le_bytes());
    record.extend([45, 0, 45, 0]);
    record.extend([0; 8]);
    record.extend(1u64.to_le_bytes());
    record.extend(1u64.to_le_bytes());
    record.extend(u64::from(size).to_le_bytes());
    record.extend(u64::from(offset).to_le_bytes());
    record.extend(extension);
    record.extend(b"PK\x06\x07");
    record.extend(0u32.to_le_bytes());
    record.extend((end as u64).to_le_bytes());
    record.extend(1u32.to_le_bytes());
    data[end + 8..end + 12].fill(0xff);
    data.splice(end..end, record);
    (data, end)
}

#[test]
fn readable_fixed_fields_survive_inconsistent_declared_extents() {
    let (original, record) = zip64_with_extension(&[1, 2, 3, 4]);
    for declared in [44u64, 47, 49, 4096] {
        let mut data = original.clone();
        data[record + 4..record + 12].copy_from_slice(&declared.to_le_bytes());
        let archive = zip::parse(&data, 10).unwrap();
        assert!(archive.zip64);
        assert_eq!(archive.declared_entries, 1);
        assert_eq!(archive.entries.len(), 1);
        assert_eq!(archive.prefix_bytes, 0);
        assert!(warning_contains(&archive, "extent"), "declared payload {declared}");
        assert_eq!(zip::read_entry(&data, &archive.entries[0], 100).unwrap(), b"retained payload");
    }
}

#[test]
fn correctly_sized_extensions_need_no_warning() {
    for extension in [&[][..], &[1, 2, 3, 4][..]] {
        let (data, _) = zip64_with_extension(extension);
        let archive = zip::parse(&data, 10).unwrap();
        assert!(archive.zip64);
        assert!(serde_json::to_value(&archive).unwrap().get("warnings").is_none());
        assert_eq!(zip::read_entry(&data, &archive.entries[0], 100).unwrap(), b"retained payload");
    }
}

#[test]
fn too_small_and_overflowing_record_extents_remain_untrusted() {
    let (original, record) = zip64_with_extension(&[]);
    for declared in [0u64, 43, u64::MAX - 12, u64::MAX] {
        let mut data = original.clone();
        data[record + 4..record + 12].copy_from_slice(&declared.to_le_bytes());
        assert!(matches!(zip::parse(&data, 10), Err(ZipError::Corrupt(_))), "declared payload {declared}");
    }
}

#[test]
fn fixed_fields_cannot_borrow_bytes_from_the_locator() {
    let (mut data, record) = zip64_with_extension(&[]);
    data.drain(record + 48..record + 56);
    assert!(zip::parse(&data, 10).is_err());
}

#[test]
fn an_unusable_locator_keeps_the_classic_directory_and_prefix_adjustment() {
    // 65,535 is a valid classic count as well as a ZIP64 sentinel. A final entry comment
    // can look like a locator without actually pointing to a ZIP64 end record.
    let files = vec![("a.txt", &b"retained payload"[..]); 65_535];
    let mut original = zip_stored(&files);
    let end = original.len() - 22;
    let size = u32::from_le_bytes(original[end + 12..end + 16].try_into().unwrap());
    let last_entry = end - 46 - "a.txt".len();
    original[last_entry + 32..last_entry + 34].copy_from_slice(&20u16.to_le_bytes());
    original[end + 12..end + 16].copy_from_slice(&(size + 20).to_le_bytes());
    let mut locator = b"PK\x06\x07".to_vec();
    locator.extend(0u32.to_le_bytes());
    locator.extend(1u64.to_le_bytes());
    locator.extend(1u32.to_le_bytes());
    original.splice(end..end, locator);
    for prefix in [&[][..], &b"MZ synthetic self-extracting prefix"[..]] {
        for offset in [1u64, u64::MAX] {
            let mut data = prefix.to_vec();
            data.extend(&original);
            let locator = data.len() - 22 - 20;
            data[locator + 8..locator + 16].copy_from_slice(&offset.to_le_bytes());
            let archive = zip::parse(&data, 2).unwrap();
            assert!(!archive.zip64);
            assert_eq!(archive.declared_entries, 65_535);
            assert_eq!(archive.entries.len(), 2);
            assert!(archive.truncated);
            assert_eq!(archive.prefix_bytes, prefix.len() as u64);
            assert!(warning_contains(&archive, "classic end record"));
            assert_eq!(zip::read_entry(&data, &archive.entries[0], 100).unwrap(), b"retained payload");
        }
    }
}
