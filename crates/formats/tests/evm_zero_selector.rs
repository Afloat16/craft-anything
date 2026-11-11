//! Fixtures generated with solc 0.8.30 from:
//! contract Probe {
//!   function execute_44g58pv() external pure returns (uint256) { return 7; }
//!   function regular() external pure returns (uint256) { return 9; }
//! }
//! The compiler methodIdentifiers are 00000000 and 03703a30. The optimizer uses 200 runs.

use craft_anything_formats::evm::{Encoding, analyze, decode_input};

fn compiler_dispatch(hex: &str, pattern: &str) {
    let (code, _) = decode_input(hex.as_bytes(), Encoding::Hex).unwrap();
    let result = analyze(&code).unwrap();
    let selectors: Vec<_> = result.functions.iter().map(|f| f.selector.as_str()).collect();
    assert_eq!(selectors, ["0x00000000", "0x03703a30"]);
    let zero = &result.functions[0];
    assert!(zero.jumpdest_valid);
    assert_eq!(code[zero.bytecode_offset], 0x5b);
    assert_eq!(zero.pattern, pattern);
    assert_eq!(result.style, "solc-like");
}

#[test]
fn unoptimized_paris_keeps_the_zero_selector() {
    compiler_dispatch(
        "6080604052348015600f57600080fd5b5060043610602f5760003560e01c80600014603457806303703a3014604e575b600080fd5b603a6068565b604051604591906091565b60405180910390f35b60546071565b604051605f91906091565b60405180910390f35b60006007905090565b60006009905090565b6000819050919050565b608b81607a565b82525050565b600060208201905060a460008301846084565b9291505056fea264697066735822122096323582fe45ea80ad30baf043246b46d90ac4277e5ae74e3508b62354eb507264736f6c634300081e0033",
        "eq-jumpi",
    );
}

#[test]
fn unoptimized_shanghai_keeps_the_zero_selector() {
    compiler_dispatch(
        "6080604052348015600e575f5ffd5b5060043610602c575f3560e01c805f14603057806303703a3014604a575b5f5ffd5b60366064565b60405160419190608a565b60405180910390f35b6050606c565b604051605b9190608a565b60405180910390f35b5f6007905090565b5f6009905090565b5f819050919050565b6084816074565b82525050565b5f602082019050609b5f830184607d565b9291505056fea2646970667358221220dd6d5d637e250b507395a48b62d009c87c6c24f915eca81195d3fd247981b7cc64736f6c634300081e0033",
        "eq-jumpi",
    );
}

#[test]
fn optimized_paris_keeps_the_zero_selector() {
    compiler_dispatch(
        "6080604052348015600f57600080fd5b5060043610602d5760003560e01c8015603257806303703a30146047575b600080fd5b60075b60405190815260200160405180910390f35b6009603556fea26469706673582212201c182c50aa629fc3b5f7b292ea23b08daac53e44de7d38bb5f624d667308b31364736f6c634300081e0033",
        "iszero-jumpi",
    );
}

#[test]
fn optimized_shanghai_keeps_the_zero_selector() {
    compiler_dispatch(
        "6080604052348015600e575f5ffd5b5060043610602b575f3560e01c8015602f57806303703a30146044575b5f5ffd5b60075b60405190815260200160405180910390f35b6009603256fea2646970667358221220b14d1a5d05c74e6aef99e9d29210d8961b9bab8a0c3512024ace9917f397984864736f6c634300081e0033",
        "iszero-jumpi",
    );
}

#[test]
fn optimized_via_ir_paris_keeps_the_zero_selector() {
    compiler_dispatch(
        "6080806040526004361015601257600080fd5b60003560e01c908115604c57506303703a3014602d57600080fd5b346047576000366003190112604757602060405160098152f35b600080fd5b34604757600036600319011260475780600760209252f3fea2646970667358221220d8e0abcb68a71530c79a02e1146c8a97bb1ab842314a1a4243f3b6e747f51a0564736f6c634300081e0033",
        "iszero-jumpi",
    );
}

#[test]
fn optimized_via_ir_shanghai_keeps_the_zero_selector() {
    compiler_dispatch(
        "60808060405260043610156011575f80fd5b5f3560e01c908115604757506303703a3014602a575f80fd5b346043575f366003190112604357602060405160098152f35b5f80fd5b346043575f36600319011260435780600760209252f3fea26469706673582212208cd8c30f1d873bf2a8f13716dd1d6198bafc0c97e95809241ef984c9cc6ee07164736f6c634300081e0033",
        "iszero-jumpi",
    );
}

fn jump_to_body(prefix: &[u8]) -> Vec<u8> {
    let mut code = prefix.to_vec();
    let destination = code.len() + 4;
    code.extend([0x60, u8::try_from(destination).unwrap(), 0x57, 0x00, 0x5b, 0x00]);
    code
}

#[test]
fn zero_requires_the_extracted_selector_as_its_operand() {
    for prefix in [
        vec![0x34, 0x15],                                                                   // CALLVALUE is not a selector.
        vec![0x60, 4, 0x35, 0x60, 0xe0, 0x1c, 0x80, 0x15],                                  // Argument data at offset 4.
        vec![0x5f, 0x35, 0x60, 0xdf, 0x1c, 0x80, 0x15],                                     // A different shift.
        vec![0x60, 0x80, 0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x81, 0x15],                         // DUP2 copies the other value.
        vec![0x60, 0x80, 0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x90, 0x80, 0x15],                   // SWAP1 then DUP1 copies the other value.
        vec![0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x60, 1, 0x15],                                  // The selector was covered by a constant.
        vec![0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x00, 0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x80, 0x15], // A later code region.
    ] {
        let result = analyze(&jump_to_body(&prefix)).unwrap();
        assert!(result.functions.is_empty(), "{prefix:02x?}: {:?}", result.functions);
    }
}

#[test]
fn a_direct_zero_test_needs_no_duplicate() {
    let result = analyze(&jump_to_body(&[0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x15])).unwrap();
    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.functions[0].selector, "0x00000000");
    assert!(result.functions[0].jumpdest_valid);
}

#[test]
fn invalid_destinations_remain_uncertain() {
    let prefix = [0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x80, 0x15];
    let mut code = jump_to_body(&prefix);
    code[prefix.len() + 1] = 0;
    let result = analyze(&code).unwrap();
    assert_eq!(result.functions.len(), 1);
    assert!(!result.functions[0].jumpdest_valid);
    assert_eq!(result.functions[0].mutability, "unknown");
}

#[test]
fn incomplete_conditions_do_not_invent_a_zero_function() {
    let prefix = [0x5f, 0x35, 0x60, 0xe0, 0x1c, 0x80];
    for suffix in [&[0x15, 0x60][..], &[0x15, 0x61, 0x01, 0x57], &[0x5f, 0x60, 0, 0x57]] {
        let mut code = prefix.to_vec();
        code.extend(suffix);
        assert!(analyze(&code).unwrap().functions.is_empty());
    }
}
