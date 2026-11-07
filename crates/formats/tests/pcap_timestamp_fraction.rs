use craft_anything_formats::pcap::{Capture, summarize};
use craft_anything_testkit::pcap::pcap;

#[test]
fn out_of_range_fractions_cannot_move_a_frame_into_the_next_second() {
    for (magic, per_second) in [(0xa1b2_c3d4u32, 1_000_000u32), (0xa1b2_3c4d, 1_000_000_000)] {
        let mut data = pcap(&[vec![], vec![]]);
        data[..4].copy_from_slice(&magic.to_le_bytes());
        data[28..32].copy_from_slice(&per_second.to_le_bytes());
        data[44..48].copy_from_slice(&(per_second - 1).to_le_bytes());
        let frames: Vec<_> = Capture::new(&data).unwrap().collect();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].ts_ns, None);
        let valid = 1_700_000_002_000_000_000 - 1_000_000_000 / u64::from(per_second);
        assert_eq!(frames[1].ts_ns, Some(valid));
        let summary = summarize(&data, &|| false).unwrap();
        assert_eq!((summary.packets, summary.first_ts_ns, summary.last_ts_ns), (2, Some(valid), Some(valid)));
    }
}
