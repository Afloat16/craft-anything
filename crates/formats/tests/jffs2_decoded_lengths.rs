use craft_anything_formats::fsread::{Limits, jffs2};

fn image(payload: &[u8], compression: u8, declared: u32, file_size: u32) -> Vec<u8> {
    let mut bytes = craft_anything_testkit::firmware::jffs2_image();
    let node = 96;
    bytes.truncate(node + 68);
    for (field, value) in [(4, 68 + payload.len() as u32), (28, file_size), (48, payload.len() as u32), (52, declared)] {
        bytes[node + field..node + field + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[node + 56] = compression;
    bytes.extend_from_slice(payload);
    bytes.resize(bytes.len().div_ceil(4) * 4, 0);
    bytes
}

fn assert_unrecovered(bytes: &[u8]) {
    let result = jffs2::read(bytes, &Limits::default());
    let entry = result.entries.iter().find(|entry| entry.path == "config/settings").unwrap();
    assert!(entry.data.is_none(), "invalid extent became {:?}", entry.data);
    assert!(!result.complete);
    assert_eq!(result.bytes, 0);
    assert!(result.warnings.iter().any(|warning| warning.contains("data node")));
}

#[test]
fn uncompressed_nodes_must_match_their_declared_length() {
    for declared in [3, 5] {
        assert_unrecovered(&image(b"abcd", 0, declared, 8));
    }
}

#[test]
fn compressed_nodes_must_match_their_declared_length() {
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(b"abcd", 6);
    for declared in [3, 5] {
        assert_unrecovered(&image(&compressed, 6, declared, 8));
    }
}

#[test]
fn a_failed_inflate_does_not_become_a_zero_filled_file() {
    assert_unrecovered(&image(b"not a zlib stream", 6, 4, 4));
}

#[test]
fn exact_nodes_can_be_clipped_by_the_final_file_size() {
    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(b"abcd", 6);
    for (payload, compression) in [(b"abcd".as_slice(), 0), (compressed.as_slice(), 6)] {
        for (size, expected) in [(2, b"ab".as_slice()), (4, b"abcd".as_slice())] {
            let result = jffs2::read(&image(payload, compression, 4, size), &Limits::default());
            assert!(result.complete, "{:?}", result.warnings);
            assert_eq!(result.entries.iter().find(|entry| entry.path == "config/settings").unwrap().data.as_deref(), Some(expected));
            assert_eq!(result.bytes, u64::from(size));
        }
    }
}

#[test]
fn zero_nodes_keep_their_explicit_zero_extent() {
    let result = jffs2::read(&image(&[], 1, 4, 4), &Limits::default());
    assert!(result.complete);
    assert_eq!(result.entries.iter().find(|entry| entry.path == "config/settings").unwrap().data.as_deref(), Some([0; 4].as_slice()));
}
