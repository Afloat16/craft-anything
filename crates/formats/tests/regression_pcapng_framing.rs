use craft_anything_formats::pcap::summarize;
use craft_anything_testkit::pcap::{block, pcapng, sample_frames};

#[test]
fn mismatched_block_footers_stop_the_capture_with_a_warning() {
    let original = pcapng(&sample_frames());
    for footer in [24, 56] {
        let mut data = original.clone();
        data[footer..footer + 4].copy_from_slice(&12u32.to_le_bytes());
        let result = summarize(&data, &|| false).unwrap();
        assert!(!result.warnings.is_empty(), "footer at {footer}");
        assert_eq!(result.packets, 0);
    }
}

#[test]
fn fixed_block_headers_cannot_borrow_the_next_blocks_bytes() {
    let header = pcapng(&[]);
    for (kind, short_body) in [(1, vec![]), (2, vec![0; 16]), (6, vec![0; 16]), (3, vec![])] {
        let mut data = header[..28].to_vec();
        if kind != 1 {
            data.extend(&header[28..]);
        }
        data.extend(block(kind, &short_body));
        data.extend(&pcapng(&sample_frames())[28..]);
        let result = summarize(&data, &|| false).unwrap();
        assert!(!result.warnings.is_empty(), "kind {kind}");
        assert_eq!(result.packets, 0, "kind {kind}");
        assert_eq!(result.interfaces.len(), usize::from(kind != 1), "kind {kind}");
    }
}

#[test]
fn valid_blocks_and_unknown_blocks_still_work() {
    let original = pcapng(&sample_frames());
    let mut data = original[..60].to_vec();
    data.extend(block(0x0000_1234, b"ignored"));
    data.extend(&original[60..]);
    let result = summarize(&data, &|| false).unwrap();
    assert_eq!(result.packets, 7);
    assert!(result.warnings.is_empty());
    assert_eq!(result.interfaces.len(), 1);
}

#[test]
fn big_endian_block_footers_use_the_sections_byte_order() {
    let be_block = |kind: u32, body: &[u8]| {
        let size = (body.len() + 12) as u32;
        let mut out = kind.to_be_bytes().to_vec();
        out.extend(size.to_be_bytes());
        out.extend(body);
        out.extend(size.to_be_bytes());
        out
    };
    let mut section = 0x1a2b3c4du32.to_be_bytes().to_vec();
    section.extend([0, 1, 0, 0]);
    section.extend((-1i64).to_be_bytes());
    let mut data = be_block(0x0a0d0d0a, &section);
    data.extend(be_block(1, &[0, 1, 0, 0, 0, 0, 0, 0]));
    let result = summarize(&data, &|| false).unwrap();
    assert!(result.warnings.is_empty());
    assert_eq!(result.interfaces[0].link_type, 1);
    let end = data.len();
    data[end - 4..].copy_from_slice(&20u32.to_le_bytes());
    assert!(!summarize(&data, &|| false).unwrap().warnings.is_empty());
}

#[test]
fn interface_ids_cannot_wrap_into_a_previous_section() {
    let section = pcapng(&[]);
    let mut data = section.clone();
    data.extend(&section);
    let mut body = [0; 20];
    body[..4].copy_from_slice(&u32::MAX.to_le_bytes());
    data.extend(block(6, &body));
    data.extend(block(6, &[0; 20]));
    let mut cap = craft_anything_formats::pcap::Capture::new(&data).unwrap();
    let frame = cap.next().unwrap();
    assert_eq!((frame.number, frame.interface), (1, 1));
    assert!(cap.next().is_none());
    assert!(cap.warnings.iter().any(|w| w.contains("undeclared interface")));
}

#[test]
fn captured_lengths_cannot_wrap_the_body_end() {
    for kind in [2, 6] {
        let mut data = pcapng(&[]);
        let mut body = [0; 20];
        body[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
        data.extend(block(kind, &body));
        data.extend(block(kind, &[0; 20]));
        let s = summarize(&data, &|| false).unwrap();
        assert_eq!(s.packets, 1);
        assert_eq!(s.warnings.len(), 1);
    }
}

#[test]
fn fine_decimal_resolutions_apply_the_offset_after_rounding() {
    let header = pcapng(&[]);
    for (exponent, nanos) in [(28, 1), (29, 0), (48, 0), (127, 0)] {
        let mut data = header[..28].to_vec();
        let mut idb = vec![1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 1, 0, exponent, 0, 0, 0];
        idb.extend([14, 0, 8, 0]);
        idb.extend(2i64.to_le_bytes());
        data.extend(block(1, &idb));
        let mut body = [0; 20];
        body[4..12].fill(0xff);
        data.extend(block(6, &body));
        let result = summarize(&data, &|| false).unwrap();
        assert_eq!(result.first_ts_ns, Some(2_000_000_000 + nanos), "10^-{exponent} s ticks");
    }
}
