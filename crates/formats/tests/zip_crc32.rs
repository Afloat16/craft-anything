//! CRC compatibility checks use fixed vectors, not checksums made by the ZIP writer under test.

use craft_anything_formats::zip::{self, ZipError};

#[test]
fn ieee_crc32_keeps_known_vectors() {
    for (input, expected) in [(&b""[..], 0), (&b"123456789"[..], 0xcbf4_3926), (&b"The quick brown fox jumps over the lazy dog"[..], 0x414f_a339)] {
        assert_eq!(zip::crc32(input), expected);
    }
    let all_bytes: Vec<u8> = (0..=255).collect();
    assert_eq!(zip::crc32(&all_bytes), 0x2905_8c73);
}

#[test]
fn ieee_crc32_keeps_checksums_across_lengths_and_alignments() {
    // Fixed checksums are cross-checked independently with Python's zlib.crc32.
    let cases = [
        (0, 0x0000_0000),
        (1, 0xd202_ef8d),
        (2, 0x4ed9_ebcb),
        (3, 0xc0fb_df36),
        (4, 0x0c79_8a97),
        (7, 0x22bd_235d),
        (8, 0xeaf8_906a),
        (15, 0xbb05_b300),
        (16, 0x4d06_0e9b),
        (31, 0x75ad_0036),
        (32, 0x9c73_c1d9),
        (63, 0x4015_2cbd),
        (64, 0xda2f_c78c),
        (127, 0x85ca_4935),
        (128, 0xac36_5979),
        (255, 0xf075_4021),
        (256, 0x4696_a149),
        (257, 0xe62a_a0d0),
        (511, 0xfd0a_d2ee),
        (512, 0x762e_2132),
        (1_023, 0x176c_a56d),
        (1_024, 0x74a9_d44f),
        (4_095, 0xf07b_0fb9),
        (4_096, 0x6f4f_669e),
        (4_097, 0x2bd4_7f84),
        (65_535, 0x911a_cdd9),
        (65_536, 0xd893_5b67),
        (65_537, 0x3a62_a8e5),
        (1_048_576, 0x0ef9_4ed8),
    ];
    let payload: Vec<u8> = (0..1_048_576).map(|i| ((i * 73) ^ (i >> 3) ^ (i >> 11)) as u8).collect();
    for (len, expected) in cases {
        for offset in 0..16 {
            let mut padded = vec![0xa5; offset];
            padded.extend_from_slice(&payload[..len]);
            padded.extend_from_slice(&[0x5a; 16]);
            assert_eq!(zip::crc32(&padded[offset..offset + len]), expected, "length {len}, offset {offset}");
        }
    }
}

// Both headers receive a caller-supplied, independent checksum.
fn stored_zip(payload: &[u8], crc: u32) -> Vec<u8> {
    let name = b"entry.bin";
    let size = payload.len() as u32;
    let mut file = b"PK\x03\x04".to_vec();
    file.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    file.extend_from_slice(&crc.to_le_bytes());
    file.extend_from_slice(&size.to_le_bytes());
    file.extend_from_slice(&size.to_le_bytes());
    file.extend_from_slice(&(name.len() as u16).to_le_bytes());
    file.extend_from_slice(&0u16.to_le_bytes());
    file.extend_from_slice(name);
    file.extend_from_slice(payload);

    let directory = file.len() as u32;
    file.extend_from_slice(b"PK\x01\x02");
    file.extend_from_slice(&[20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    file.extend_from_slice(&crc.to_le_bytes());
    file.extend_from_slice(&size.to_le_bytes());
    file.extend_from_slice(&size.to_le_bytes());
    file.extend_from_slice(&(name.len() as u16).to_le_bytes());
    file.extend_from_slice(&[0; 16]);
    file.extend_from_slice(name);

    let directory_size = file.len() as u32 - directory;
    file.extend_from_slice(b"PK\x05\x06\0\0\0\0\x01\0\x01\0");
    file.extend_from_slice(&directory_size.to_le_bytes());
    file.extend_from_slice(&directory.to_le_bytes());
    file.extend_from_slice(&0u16.to_le_bytes());
    file
}

#[test]
fn stored_entries_keep_valid_bytes_and_output_limits() {
    for (payload, crc) in [(&b""[..], 0), (&b"123456789"[..], 0xcbf4_3926)] {
        let file = stored_zip(payload, crc);
        let archive = zip::parse(&file, 1).unwrap();
        let entry = &archive.entries[0];
        assert_eq!(entry.crc32, crc);
        assert_eq!(zip::read_entry(&file, entry, payload.len() as u64).unwrap(), payload);
        if !payload.is_empty() {
            let limit = payload.len() as u64 - 1;
            assert_eq!(zip::read_entry(&file, entry, limit), Err(ZipError::TooLarge(limit)));
        }
    }
}

#[test]
fn damaged_stored_entries_keep_exact_checksum_errors() {
    let file = stored_zip(b"123456789", 0xcbf4_3926);
    let archive = zip::parse(&file, 1).unwrap();
    let entry = &archive.entries[0];
    let mut damaged = file.clone();
    damaged[30 + b"entry.bin".len()] = b'0';
    assert_eq!(zip::read_entry(&damaged, entry, 9), Err(ZipError::CrcMismatch { expected: 0xcbf4_3926, found: 0xdc8f_2d65 }));
    assert_eq!(zip::read_entry_unverified(&damaged, entry, 9).unwrap(), b"023456789");

    let mut wrong_crc = entry.clone();
    wrong_crc.crc32 = 0;
    assert_eq!(zip::read_entry(&file, &wrong_crc, 9), Err(ZipError::CrcMismatch { expected: 0, found: 0xcbf4_3926 }));
}
