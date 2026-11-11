use craft_anything_js::{Limits, analyze};

fn fingerprint(source: &str) -> String {
    analyze("module.js", source, &Limits::default()).facts.fingerprint.unwrap()
}

fn different(left: &str, right: &str) {
    assert_ne!(fingerprint(left), fingerprint(right), "{left} / {right}");
}

#[test]
fn bigint_values_remain_distinct_without_float_conversion() {
    different("globalThis.result = 1n;", "globalThis.result = 2n;");
    different("globalThis.result = 9007199254740992n;", "globalThis.result = 9007199254740993n;");
}

#[test]
fn regular_expression_patterns_and_flags_change_fingerprints() {
    different("globalThis.result = /cat/.test('cat');", "globalThis.result = /dog/.test('cat');");
    different("globalThis.result = /a/.test('A');", "globalThis.result = /a/i.test('A');");
}

#[test]
fn every_template_text_position_changes_the_fingerprint() {
    different("globalThis.result = `left`;", "globalThis.result = `right`;");
    different("const value = 3; globalThis.result = `left${value}`;", "const value = 3; globalThis.result = `right${value}`;");
    different("const value = 3; globalThis.result = `prefix${value}left`;", "const value = 3; globalThis.result = `prefix${value}right`;");
}

#[test]
fn tagged_template_raw_text_and_invalid_escapes_remain_observable() {
    different(r#"globalThis.result = String.raw`\n`;"#, "globalThis.result = String.raw`\n`;");
    different(r#"globalThis.result = String.raw`\u{}`;"#, r#"globalThis.result = String.raw`\xG0`;"#);
}

#[test]
fn identifier_renaming_preserves_literal_fingerprints() {
    let original = "function original(parameter) { return [9007199254740993n, /x/ig, `prefix${parameter}tail`]; }";
    let renamed = "function a(b) { return [9007199254740993n, /x/ig, `prefix${b}tail`]; }";
    assert_eq!(fingerprint(original), fingerprint(renamed));
}

#[test]
fn equivalent_bigint_spellings_keep_the_same_fingerprint() {
    let expected = fingerprint("globalThis.result = 10n;");
    for spelling in ["0xan", "0XAn", "0b1010n", "0o12n", "1_0n"] {
        assert_eq!(fingerprint(&format!("globalThis.result = {spelling};")), expected, "{spelling}");
    }
}

#[test]
fn regular_expression_flag_order_does_not_change_the_fingerprint() {
    assert_eq!(fingerprint("globalThis.result = /a/gi;"), fingerprint("globalThis.result = /a/ig;"));
}

#[test]
fn existing_literal_and_operator_behavior_is_preserved() {
    assert_eq!(fingerprint("globalThis.result = 10;"), fingerprint("globalThis.result = 1e1;"));
    assert_eq!(fingerprint("globalThis.result = 'x';"), fingerprint(r#"globalThis.result = '\u0078';"#));
    different("globalThis.result = 10;", "globalThis.result = 11;");
    different("globalThis.result = true;", "globalThis.result = false;");
    different("globalThis.result = 'left';", "globalThis.result = 'right';");
    different("globalThis.result = 1 + 2;", "globalThis.result = 1 - 2;");
}
