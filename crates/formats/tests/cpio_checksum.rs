use craft_anything_formats::firmware::{ScanOptions, scan};
use craft_anything_formats::tarcpio::{cpio, member_data};
use craft_anything_testkit::firmware as fx;

fn crc_archive() -> Vec<u8> {
    let mut archive = fx::cpio_newc(&[("file", &[255, 128, 65], 0o100644), ("link", b"file", 0o120777)]);
    let mut offset = 0;
    for expected in [448u32, 416, 0] {
        let size = usize::from_str_radix(std::str::from_utf8(&archive[offset + 54..offset + 62]).unwrap(), 16).unwrap();
        let name = usize::from_str_radix(std::str::from_utf8(&archive[offset + 94..offset + 102]).unwrap(), 16).unwrap();
        archive[offset..offset + 6].copy_from_slice(b"070702");
        archive[offset + 102..offset + 110].copy_from_slice(format!("{expected:08x}").as_bytes());
        let data = (offset + 110 + name + 3) & !3;
        offset = (data + size + 3) & !3;
    }
    archive
}

#[test]
fn crc_members_report_changed_data_without_losing_the_inventory() {
    let mut archive = crc_archive();
    let clean = cpio(&archive, 10).unwrap();
    let first = clean.members[0].data_offset as usize;
    archive[first] = 0;
    let listing = cpio(&archive, 10).unwrap();
    assert!(listing.complete);
    assert_eq!(listing.members.len(), 2);
    assert!(listing.warnings.iter().any(|w| w.contains("file") && w.contains("checksum")), "{:?}", listing.warnings);
    assert_eq!(member_data(&archive, &listing.members[0]), Some([0, 128, 65].as_slice()));
}

#[test]
fn crc_checks_cover_symlink_data_and_valid_unsigned_bytes() {
    let mut archive = crc_archive();
    let clean = cpio(&archive, 10).unwrap();
    assert!(clean.complete);
    assert!(clean.warnings.is_empty(), "{:?}", clean.warnings);
    let link = clean.members[1].data_offset as usize;
    archive[link] = b'F';
    assert!(cpio(&archive, 10).unwrap().warnings.iter().any(|w| w.contains("link") && w.contains("checksum")));
}

#[test]
fn newc_has_no_data_checksum_and_truncated_data_is_not_validated() {
    let mut newc = fx::cpio_newc(&[("file", b"abc", 0o100644)]);
    newc[102..110].copy_from_slice(b"00000126");
    assert!(cpio(&newc, 10).unwrap().warnings.is_empty());
    let archive = crc_archive();
    let data = cpio(&archive, 10).unwrap().members[0].data_offset as usize;
    let partial = cpio(&archive[..data + 2], 10).unwrap();
    assert!(!partial.complete);
    assert!(!partial.warnings.iter().any(|w| w.contains("checksum mismatch")));
}

#[test]
fn firmware_inventory_exposes_checksum_damage() {
    let mut archive = crc_archive();
    let clean = scan(&archive, &ScanOptions::default(), &|| false);
    assert_eq!(clean.regions.iter().find(|r| r.offset == 0 && r.kind == "cpio-crc").unwrap().confidence, "high");
    let data = cpio(&archive, 10).unwrap().members[0].data_offset as usize;
    archive[data] = 0;
    let damaged = scan(&archive, &ScanOptions::default(), &|| false);
    let region = damaged.regions.iter().find(|r| r.offset == 0 && r.kind == "cpio-crc").unwrap();
    assert_eq!(region.confidence, "high");
    assert!(region.details["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("checksum mismatch")));
}

#[test]
fn checksum_fields_require_eight_hexadecimal_digits() {
    let mut archive = crc_archive();
    archive[102..110].copy_from_slice(b"000001C0");
    assert!(cpio(&archive, 10).unwrap().warnings.is_empty(), "uppercase hexadecimal is valid");
    archive[102..110].copy_from_slice(b"+00001c0");
    let listing = cpio(&archive, 10).unwrap();
    assert!(listing.complete);
    assert_eq!(listing.members.len(), 2);
    assert!(listing.warnings.iter().any(|w| w.contains("unreadable data checksum")), "{:?}", listing.warnings);
    assert_eq!(member_data(&archive, &listing.members[0]), Some([255, 128, 65].as_slice()));
}

/// Every candidate has one complete payload ending at the same EOF, but no trailer.
fn overlapping_crc_candidates(count: usize) -> Vec<u8> {
    let mut header = fx::cpio_newc(&[("file", b"", 0o100644)]);
    let data_offset = cpio(&header, 1).unwrap().members[0].data_offset as usize;
    header.truncate(data_offset);
    header[..6].copy_from_slice(b"070702");
    let mut image = vec![b'A'; count * 128 + 8192];
    for index in 0..count {
        let offset = index * 128;
        let size = image.len() - offset - data_offset;
        header[54..62].copy_from_slice(format!("{size:08x}").as_bytes());
        image[offset..offset + header.len()].copy_from_slice(&header);
    }
    image
}

#[test]
fn firmware_crc_candidates_share_one_checksum_byte_budget() {
    let image = overlapping_crc_candidates(16);
    let result = scan(&image, &ScanOptions::default(), &|| false);
    let regions = result.regions.iter().filter(|r| r.kind == "cpio-crc").collect::<Vec<_>>();
    assert_eq!(regions.len(), 16, "budget limits validation, not the member inventory");
    assert!(regions.iter().all(|r| r.confidence == "medium" && r.details["members"] == 1));
    assert!(result.warnings.iter().any(|w| w.contains("checksum byte budget")), "{:?}", result.warnings);
    assert!(
        regions
            .iter()
            .skip(1)
            .all(|r| { r.details["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("checksum validation skipped")) })
    );
    assert!(!result.cancelled && !result.truncated);
}

#[test]
fn firmware_cancellation_is_polled_inside_a_large_crc_payload() {
    use std::cell::Cell;

    let payload = vec![b'A'; 256 * 1024];
    let mut archive = fx::cpio_newc(&[("file", &payload, 0o100644)]);
    let data_offset = cpio(&archive, 10).unwrap().members[0].data_offset as usize;
    let trailer = (data_offset + payload.len() + 3) & !3;
    archive[..6].copy_from_slice(b"070702");
    archive[trailer..trailer + 6].copy_from_slice(b"070702");
    archive[102..110].copy_from_slice(format!("{:08x}", 65 * payload.len()).as_bytes());
    let polls = Cell::new(0);
    let result = scan(&archive, &ScanOptions::default(), &|| {
        polls.set(polls.get() + 1);
        polls.get() >= 3
    });
    assert!(result.cancelled, "the complete archive is smaller than the outer scan's polling interval");
    let region = result.regions.iter().find(|r| r.kind == "cpio-crc").unwrap();
    assert_eq!(region.details["members"], 1);
    assert_eq!(region.details["complete"], false);
    assert!(region.details["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("checksum validation cancelled")));
}

#[test]
fn the_checksum_budget_preserves_disjoint_complete_archives() {
    let mut image = crc_archive();
    let second = image.len() as u64;
    image.extend(crc_archive());
    let result = scan(&image, &ScanOptions::default(), &|| false);
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    assert_eq!(result.regions.iter().map(|r| r.offset).collect::<Vec<_>>(), [0, second]);
    for region in result.regions {
        assert_eq!(region.kind, "cpio-crc");
        assert_eq!(region.confidence, "high");
        assert_eq!(region.details["warnings"], serde_json::json!([]));
    }
}
