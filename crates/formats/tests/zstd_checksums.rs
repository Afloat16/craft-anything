use craft_anything_formats::decompress::{DecompressError, zstd};
use craft_anything_formats::zip;

// Produced by libzstd 1.5.5 via Python 3.14's native compression.zstd API,
// with checksum_flag explicitly enabled or disabled.
const EMPTY: &[u8] = &[40, 181, 47, 253, 36, 0, 1, 0, 0, 153, 233, 216, 81];
const EMPTY_UNCHECKED: &[u8] = &[40, 181, 47, 253, 32, 0, 1, 0, 0];
const ASCII: &[u8] = &[40, 181, 47, 253, 36, 17, 137, 0, 0, 67, 114, 97, 102, 116, 32, 99, 104, 101, 99, 107, 115, 117, 109, 115, 46, 10, 52, 192, 155, 125];
const ASCII_UNCHECKED: &[u8] = &[40, 181, 47, 253, 32, 17, 137, 0, 0, 67, 114, 97, 102, 116, 32, 99, 104, 101, 99, 107, 115, 117, 109, 115, 46, 10];
const UNICODE: &[u8] = &[40, 181, 47, 253, 36, 11, 89, 0, 0, 230, 160, 161, 233, 170, 140, 240, 159, 167, 170, 10, 233, 32, 146, 5];
const UNICODE_UNCHECKED: &[u8] = &[40, 181, 47, 253, 32, 11, 89, 0, 0, 230, 160, 161, 233, 170, 140, 240, 159, 167, 170, 10];
const REPEATED: &[u8] = &[
    40, 181, 47, 253, 100, 176, 3, 5, 1, 0, 192, 110, 97, 116, 105, 118, 101, 32, 99, 104, 101, 99, 107, 115, 117, 109, 109, 101, 100, 32, 100, 97, 116, 97,
    10, 1, 0, 84, 178, 255, 28, 3, 61, 4, 21, 123,
];
const REPEATED_UNCHECKED: &[u8] = &[
    40, 181, 47, 253, 96, 176, 3, 5, 1, 0, 192, 110, 97, 116, 105, 118, 101, 32, 99, 104, 101, 99, 107, 115, 117, 109, 109, 101, 100, 32, 100, 97, 116, 97, 10,
    1, 0, 84, 178, 255, 28, 3,
];

fn samples() -> Vec<(&'static [u8], &'static [u8], Vec<u8>)> {
    vec![
        (EMPTY, EMPTY_UNCHECKED, Vec::new()),
        (ASCII, ASCII_UNCHECKED, b"Craft checksums.\n".to_vec()),
        (UNICODE, UNICODE_UNCHECKED, "校验🧪\n".as_bytes().to_vec()),
        (REPEATED, REPEATED_UNCHECKED, b"native checksummed data\n".repeat(50)),
    ]
}

#[test]
fn checksummed_frames_keep_their_exact_contents() {
    for (frame, _, plain) in samples() {
        assert_eq!(zstd(frame, plain.len() as u64).unwrap(), plain);
    }
}

#[test]
fn checksum_free_frames_remain_supported() {
    for (_, frame, plain) in samples() {
        assert_eq!(zstd(frame, plain.len() as u64).unwrap(), plain);
    }
}

#[test]
fn every_checksum_byte_is_verified_including_empty_and_compressed_frames() {
    for (frame, _, _) in samples() {
        for byte in 0..4 {
            let mut damaged = frame.to_vec();
            damaged[frame.len() - 4 + byte] ^= 0x80;
            assert_eq!(zstd(&damaged, 1 << 20), Err(DecompressError::Corrupt));
        }
    }
}

#[test]
fn a_changed_raw_block_is_not_returned_as_recovered_data() {
    let mut damaged = ASCII.to_vec();
    assert_eq!(&damaged[9..26], b"Craft checksums.\n");
    damaged[9] ^= 1;
    assert_eq!(zstd(&damaged, 100), Err(DecompressError::Corrupt));
}

#[test]
fn concatenated_streams_verify_each_frame_across_skippable_metadata() {
    let skip = b"\x50\x2a\x4d\x18\x03\0\0\0xyz";
    let valid = [ASCII, skip.as_slice(), UNICODE].concat();
    assert_eq!(zstd(&valid, 28).unwrap(), [b"Craft checksums.\n".as_slice(), "校验🧪\n".as_bytes()].concat());
    let mut damaged = UNICODE.to_vec();
    *damaged.last_mut().unwrap() ^= 1;
    for input in [[damaged.as_slice(), skip, ASCII].concat(), [ASCII, skip, damaged.as_slice()].concat()] {
        assert_eq!(zstd(&input, 100), Err(DecompressError::Corrupt));
    }
    let mut empty = EMPTY.to_vec();
    *empty.last_mut().unwrap() ^= 1;
    assert_eq!(zstd(&[ASCII, empty.as_slice()].concat(), 100), Err(DecompressError::Corrupt));
}

#[test]
fn output_budgets_and_incomplete_trailers_keep_their_errors() {
    for (frame, _, plain) in samples() {
        for missing in 1..=4 {
            assert!(zstd(&frame[..frame.len() - missing], 1 << 20).is_err());
        }
        if !plain.is_empty() {
            assert_eq!(zstd(frame, plain.len() as u64 - 1), Err(DecompressError::TooLarge));
        }
    }
    let all = [ASCII, UNICODE].concat();
    assert_eq!(zstd(&all, 27), Err(DecompressError::TooLarge));
}

fn zip_member(frame: &[u8], plain: &[u8]) -> Vec<u8> {
    let name = b"fixture.bin";
    let crc = zip::crc32(plain);
    let compressed = u32::try_from(frame.len()).unwrap();
    let size = u32::try_from(plain.len()).unwrap();
    let mut out = Vec::new();
    out.extend_from_slice(b"PK\x03\x04");
    for value in [63u16, 0, 93, 0, 0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    for value in [crc, compressed, size] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    for value in [name.len() as u16, 0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.extend_from_slice(name);
    out.extend_from_slice(frame);
    let directory = u32::try_from(out.len()).unwrap();
    out.extend_from_slice(b"PK\x01\x02");
    for value in [63u16, 63, 0, 93, 0, 0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    for value in [crc, compressed, size] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    for value in [name.len() as u16, 0, 0, 0, 0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    for value in [0u32, 0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.extend_from_slice(name);
    let directory_size = u32::try_from(out.len()).unwrap() - directory;
    out.extend_from_slice(b"PK\x05\x06");
    for value in [0u16, 0, 1, 1] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.extend_from_slice(&directory_size.to_le_bytes());
    out.extend_from_slice(&directory.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

#[test]
fn a_matching_zip_crc_does_not_hide_a_bad_zstandard_checksum() {
    let plain = b"Craft checksums.\n";
    let good = zip_member(ASCII, plain);
    let archive = zip::parse(&good, 10).unwrap();
    assert_eq!(zip::read_entry(&good, &archive.entries[0], 100).unwrap(), plain);
    let mut damaged_frame = ASCII.to_vec();
    *damaged_frame.last_mut().unwrap() ^= 1;
    let bad = zip_member(&damaged_frame, plain);
    let archive = zip::parse(&bad, 10).unwrap();
    assert_eq!(archive.entries[0].crc32, zip::crc32(plain));
    assert!(zip::read_entry(&bad, &archive.entries[0], 100).is_err());
}
