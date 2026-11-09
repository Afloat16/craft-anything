use craft_anything_formats::der::{name, read};
fn tlv(tag: u8, b: &[u8]) -> Vec<u8> {
    let mut out = vec![tag, b.len() as u8];
    out.extend(b);
    out
}
fn attribute(id: u8, tag: u8, value: &[u8]) -> Vec<u8> {
    let mut b = tlv(6, &[0x55, 4, id]);
    b.extend(tlv(tag, value));
    tlv(0x30, &b)
}
fn render(sets: &[Vec<u8>]) -> String {
    let payload = sets.iter().flat_map(|s| tlv(0x31, s)).collect::<Vec<_>>();
    let root = tlv(0x30, &payload);
    name(&root, &read(&root, 0).unwrap())
}
#[test]
fn attribute_text_cannot_impersonate_a_second_attribute() {
    assert_eq!(render(&[attribute(3, 0x0c, b" org, O=other+CN=x\0 ")]), r"CN=\ org\, O\=other\+CN\=x\00\ ");
    assert_eq!(render(&[attribute(3, 0x0c, b"#literal")]), r"CN=\#literal");
}
#[test]
fn joint_and_separate_rdns_are_distinguishable() {
    let cn = attribute(3, 0x0c, b"Desk");
    let ou = attribute(11, 0x13, b"Lab");
    assert_eq!(render(&[[cn.clone(), ou.clone()].concat()]), "CN=Desk+OU=Lab");
    assert_eq!(render(&[cn, ou]), "CN=Desk, OU=Lab");
}
#[test]
fn unicode_directory_string_values_are_lossless_or_explicitly_binary() {
    assert_eq!(render(&[attribute(3, 0x1c, &[0, 1, 0xf6, 0x80])]), "CN=🚀");
    assert_eq!(render(&[attribute(3, 0x1e, &[0, 65, 0])]), "CN=#1e03004100");
    assert_eq!(render(&[attribute(3, 4, &[0, 255])]), "CN=#040200ff");
}
