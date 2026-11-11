use craft_anything_formats::dotnet::cil::{decode, normalized_hash};
use std::collections::HashSet;

fn hash(opcode: u8, operand: &[u8]) -> String {
    let mut body = vec![opcode];
    body.extend_from_slice(operand);
    body.push(0x2a); // ret
    let (instructions, issue) = decode(&body, 100);
    assert!(issue.is_none(), "{issue:?}");
    assert_eq!(instructions.len(), 2);
    normalized_hash(&instructions)
}

#[test]
fn single_precision_infinities_and_nan_do_not_share_a_method_hash() {
    let hashes: HashSet<_> = [f32::INFINITY, f32::NEG_INFINITY, f32::NAN].into_iter().map(|v| hash(0x22, &v.to_le_bytes())).collect();
    assert_eq!(hashes.len(), 3);
}

#[test]
fn double_precision_infinities_and_nan_do_not_share_a_method_hash() {
    let hashes: HashSet<_> = [f64::INFINITY, f64::NEG_INFINITY, f64::NAN].into_iter().map(|v| hash(0x23, &v.to_le_bytes())).collect();
    assert_eq!(hashes.len(), 3);
}

#[test]
fn decoded_nan_sign_and_payload_bits_remain_distinct() {
    let patterns = [0x7ff8_0000_0000_0001u64, 0x7ff8_0000_0000_0002, 0xfff8_0000_0000_0001];
    let hashes: HashSet<_> = patterns.into_iter().map(|bits| hash(0x23, &bits.to_le_bytes())).collect();
    assert_eq!(hashes.len(), patterns.len());
}

#[test]
fn finite_operands_keep_the_existing_hash_encoding() {
    assert_eq!(hash(0x23, &1.5f64.to_le_bytes()), "195f2ec0d52206e04f4e58b7cedf0b75469f4054d818265725463f6007b15439");
    assert_ne!(hash(0x23, &0.0f64.to_le_bytes()), hash(0x23, &(-0.0f64).to_le_bytes()));
}
