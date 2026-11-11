use craft_anything_formats::zip::{self, ZipEntry, ZipError};
use craft_anything_formats::zip_file;
use std::io::Cursor;

const NAME: &str = "assets/message.txt";
const PLAIN: &[u8] = b"older ZIP payload\0with binary bytes\xff and text";

/// A single-segment Zstandard frame with one raw block; no external compressor is needed.
fn frame(plain: &[u8]) -> Vec<u8> {
    assert!(plain.len() < 256);
    let mut bytes = vec![0x28, 0xb5, 0x2f, 0xfd, 0x20, plain.len() as u8];
    let block = ((plain.len() as u32) << 3) | 1;
    bytes.extend_from_slice(&block.to_le_bytes()[..3]);
    bytes.extend_from_slice(plain);
    bytes
}

fn archive(method: u16, packed: &[u8], plain: &[u8]) -> Vec<u8> {
    let mut bytes = craft_anything_testkit::electron::zip_stored(&[(NAME, packed)]);
    let central = 30 + NAME.len() + packed.len();
    for (version, codec, crc, size) in [(4, 8, 14, 22), (central + 6, central + 10, central + 16, central + 24)] {
        bytes[version..version + 2].copy_from_slice(&63u16.to_le_bytes());
        bytes[codec..codec + 2].copy_from_slice(&method.to_le_bytes());
        bytes[crc..crc + 4].copy_from_slice(&zip::crc32(plain).to_le_bytes());
        bytes[size..size + 4].copy_from_slice(&(plain.len() as u32).to_le_bytes());
    }
    bytes
}

fn entry(bytes: &[u8]) -> ZipEntry {
    zip::parse(bytes, 1).unwrap().entries.remove(0)
}

fn read_both(bytes: &[u8], expected: &[u8]) {
    let parsed = entry(bytes);
    assert_eq!(zip::read_entry(bytes, &parsed, expected.len() as u64).unwrap(), expected);
    assert_eq!(zip_file::read_named_entry(&mut Cursor::new(bytes), NAME, expected.len() as u64).unwrap().as_deref(), Some(expected));
}

#[test]
fn legacy_method_is_named_without_rewriting_its_number() {
    let bytes = archive(20, &frame(PLAIN), PLAIN);
    let parsed = entry(&bytes);
    assert_eq!(parsed.method, 20);
    assert_eq!(parsed.method_name(), "zstd");
}

#[test]
fn legacy_members_read_through_memory_and_seek_paths() {
    let bytes = archive(20, &frame(PLAIN), PLAIN);
    read_both(&bytes, PLAIN);
}

#[test]
fn legacy_members_keep_self_extracting_prefix_offsets() {
    let archive = archive(20, &frame(PLAIN), PLAIN);
    let mut bytes = b"synthetic launcher prefix\n".to_vec();
    bytes.extend_from_slice(&archive);
    assert_eq!(zip::parse(&bytes, 1).unwrap().prefix_bytes, 26);
    read_both(&bytes, PLAIN);
}

#[test]
fn legacy_concatenated_frames_keep_the_whole_member() {
    let mut packed = frame(b"first ");
    packed.extend_from_slice(&frame(b"second"));
    let bytes = archive(20, &packed, b"first second");
    read_both(&bytes, b"first second");
}

#[test]
fn legacy_members_keep_crc_and_decoded_size_checks() {
    let bytes = archive(20, &frame(PLAIN), PLAIN);
    let original = entry(&bytes);
    let mut changed = original.clone();
    changed.crc32 ^= 1;
    assert!(matches!(zip::read_entry(&bytes, &changed, 100), Err(ZipError::CrcMismatch { .. })));
    assert_eq!(zip::read_entry_unverified(&bytes, &changed, 100).unwrap(), PLAIN);
    changed = original;
    changed.uncompressed_size += 1;
    assert!(matches!(zip::read_entry(&bytes, &changed, 100), Err(ZipError::Corrupt(_))));
}

#[test]
fn legacy_damaged_and_truncated_frames_are_not_successful_members() {
    let original = frame(PLAIN);
    let mut corrupt = original.clone();
    corrupt[0] ^= 0xff;
    for packed in [&corrupt[..], &original[..original.len() - 1]] {
        let bytes = archive(20, packed, PLAIN);
        assert!(matches!(zip::read_entry(&bytes, &entry(&bytes), 100), Err(ZipError::Corrupt(_))));
        assert!(matches!(zip_file::read_named_entry(&mut Cursor::new(bytes), NAME, 100), Err(ZipError::Corrupt(_))));
    }
}

#[test]
fn legacy_decoder_honors_the_cap_when_the_directory_understates_size() {
    let bytes = archive(20, &frame(PLAIN), PLAIN);
    let mut parsed = entry(&bytes);
    parsed.uncompressed_size = 0;
    // The existing Zstandard error mapping reports decoder-limit failures as corrupt streams.
    assert!(matches!(zip::read_entry(&bytes, &parsed, PLAIN.len() as u64 - 1), Err(ZipError::Corrupt(_))));
}

#[test]
fn declared_limits_and_encryption_are_checked_before_decoding() {
    for method in [20, 93] {
        let bytes = archive(method, &frame(PLAIN), PLAIN);
        let mut parsed = entry(&bytes);
        assert!(matches!(zip::read_entry(&bytes, &parsed, PLAIN.len() as u64 - 1), Err(ZipError::TooLarge(_))));
        parsed.encrypted = true;
        assert!(matches!(zip::read_entry(&bytes, &parsed, 100), Err(ZipError::Unsupported(_))));
    }
}

#[test]
fn current_zstandard_members_keep_their_existing_identity_and_contents() {
    let bytes = archive(93, &frame(PLAIN), PLAIN);
    let parsed = entry(&bytes);
    assert_eq!(parsed.method, 93);
    assert_eq!(parsed.method_name(), "zstd");
    read_both(&bytes, PLAIN);
}

#[test]
fn unrelated_compression_methods_remain_unsupported() {
    let bytes = archive(21, &frame(PLAIN), PLAIN);
    let parsed = entry(&bytes);
    assert_eq!(parsed.method, 21);
    assert_eq!(parsed.method_name(), "other");
    assert!(matches!(zip::read_entry(&bytes, &parsed, 100), Err(ZipError::Unsupported(_))));
}
