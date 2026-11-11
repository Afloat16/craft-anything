use craft_anything_formats::{
    arsc::{Limits, parse},
    axml::chunk,
    bytes::Reader,
};
use craft_anything_testkit::android::resources_arsc;

fn children(data: &[u8], start: usize) -> Vec<(usize, u16)> {
    let r = Reader::new(data);
    let (_, head, size) = chunk(&r, start).unwrap();
    let mut pos = start + head;
    let mut result = Vec::new();
    while pos < start + size {
        let (kind, _, bytes) = chunk(&r, pos).unwrap();
        result.push((pos, kind));
        pos += bytes;
    }
    result
}
fn package(data: &[u8]) -> usize {
    children(data, 0).into_iter().find(|(_, k)| *k == 0x200).unwrap().0
}
fn types(data: &[u8]) -> Vec<usize> {
    children(data, package(data)).into_iter().filter(|(_, k)| *k == 0x201).map(|r| r.0).collect()
}
#[test]
fn offset_table_is_never_parsed_as_resource_data() {
    let mut d = resources_arsc();
    let pos = types(&d)[0];
    let h = u16::from_le_bytes(d[pos + 2..pos + 4].try_into().unwrap());
    d[pos + 16..pos + 20].copy_from_slice(&u32::from(h).to_le_bytes());
    let out = parse(&d, Limits::default()).unwrap();
    assert!(out.entries.iter().all(|e| e.type_name != "string" || e.config != "default"));
    assert!(!out.warnings.is_empty());
}
#[test]
fn a_truncated_bag_does_not_appear_as_a_complete_style() {
    let mut d = resources_arsc();
    let pos = *types(&d).last().unwrap();
    let e = pos + u32::from_le_bytes(d[pos + 16..pos + 20].try_into().unwrap()) as usize;
    d[e + 12..e + 16].copy_from_slice(&2u32.to_le_bytes());
    let out = parse(&d, Limits::default()).unwrap();
    assert!(!out.entries.iter().any(|e| e.type_name == "style"));
    assert!(!out.warnings.is_empty());
}
