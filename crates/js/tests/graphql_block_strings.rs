use craft_anything_js::{Limits, analyze, api_ops::read_document};

const ESCAPED_TRIPLE: &str = "\\\"\"\"";

fn block(value: &str) -> String {
    format!("\"\"\"{value}\"\"\"")
}

fn names(text: &str) -> Option<Vec<(&'static str, Option<String>, bool)>> {
    read_document(text, false).map(|doc| doc.operations.into_iter().map(|op| (op.kind, op.name, op.literal)).collect())
}

fn expected(kind: &'static str, name: &str) -> Vec<(&'static str, Option<String>, bool)> {
    vec![(kind, Some(name.to_string()), true)]
}

#[test]
fn block_strings_in_variable_defaults_keep_operations() {
    for kind in ["query", "mutation", "subscription"] {
        let value = block(&format!("before {ESCAPED_TRIPLE} after"));
        let text = format!("{kind} Actual($value: String = {value}) {{ field(arg: $value) }}");
        assert_eq!(names(&text), Some(expected(kind, "Actual")), "{text}");
    }
}

#[test]
fn block_strings_in_selection_sets_keep_operations() {
    for kind in ["query", "mutation", "subscription"] {
        let value = block(&format!("before {ESCAPED_TRIPLE} after"));
        let text = format!("{kind} Actual {{ field(arg: {value}) }}");
        assert_eq!(names(&text), Some(expected(kind, "Actual")), "{text}");
    }
}

#[test]
fn block_strings_in_directives_keep_operations() {
    for kind in ["query", "mutation", "subscription"] {
        let value = block(&format!("before {ESCAPED_TRIPLE} after"));
        let text = format!("{kind} Actual @note(text: {value}) {{ field }}");
        assert_eq!(names(&text), Some(expected(kind, "Actual")), "{text}");
    }
}

#[test]
fn preceding_backslashes_do_not_disable_a_triple_quote_escape() {
    for count in 1..=4 {
        let value = block(&format!("before {}\"\"\" after", "\\".repeat(count)));
        let text = format!("query Slashes {{ field(arg: {value}) }}");
        assert_eq!(names(&text), Some(expected("query", "Slashes")), "{text}");
    }
}

#[test]
fn escaped_quote_runs_and_unicode_stay_inside_the_block() {
    for value in [
        format!("é雪\n{ESCAPED_TRIPLE}\n# still text }} ) {{\n🙂"),
        format!("{ESCAPED_TRIPLE}{ESCAPED_TRIPLE} tail"),
        format!("{ESCAPED_TRIPLE}\" tail"),
        format!("{ESCAPED_TRIPLE}\"\" tail"),
        format!("before \"\"{ESCAPED_TRIPLE} tail"),
        format!("before {ESCAPED_TRIPLE}"),
    ] {
        let text = format!("query Actual {{ field(arg: {}) }}", block(&value));
        assert_eq!(names(&text), Some(expected("query", "Actual")), "{text}");
    }
}

#[test]
fn block_strings_in_fragments_do_not_hide_later_operations() {
    let value = block(&format!("{ESCAPED_TRIPLE} tail"));
    let text = format!("fragment Part on User {{ field(arg: {value}) }} query Actual {{ user {{ ...Part }} }} mutation Save {{ save }}");
    assert_eq!(names(&text), Some(vec![("query", Some("Actual".into()), true), ("mutation", Some("Save".into()), true)]));
}

#[test]
fn an_escaped_triple_quote_cannot_close_an_unterminated_block() {
    for count in [1, 2, 4] {
        let text = format!("query Invalid {{ field(arg: \"\"\"before {}\"\"\" ) }}", "\\".repeat(count));
        assert!(read_document(&text, false).is_none(), "{text}");
    }
    for text in [
        format!("query Invalid($value: String = \"\"\"before {ESCAPED_TRIPLE} ) {{ field(arg: $value) }}"),
        format!("query Invalid @note(text: \"\"\"before {ESCAPED_TRIPLE} ) {{ field }}"),
    ] {
        assert!(read_document(&text, false).is_none(), "{text}");
    }
}

#[test]
fn parsed_and_fallback_api_facts_follow_block_string_boundaries() {
    let value = block(&format!("before {ESCAPED_TRIPLE} after"));
    let valid = format!("query Actual {{ field(arg: {value}) }}");
    let invalid = format!("query Invalid {{ field(arg: \"\"\"before {ESCAPED_TRIPLE} ) }}");
    for max_parse_bytes in [0, Limits::default().max_parse_bytes] {
        for (document, wanted) in [(&valid, expected("query", "Actual")), (&invalid, vec![])] {
            let source = format!("const document = {};", serde_json::to_string(document).unwrap());
            let analysis = analyze("client.js", &source, &Limits { max_parse_bytes, ..Limits::default() });
            let actual: Vec<_> = analysis.facts.api_operations.iter().map(|op| (op.kind.as_str(), op.name.clone(), op.literal)).collect();
            assert_eq!(actual, wanted, "{document}, parse limit {max_parse_bytes}");
        }
    }
}

#[test]
fn ordinary_block_strings_and_literal_backslashes_keep_their_behavior() {
    for value in ["", "hello\nworld", r"\n \t \u0041", "one \" then two \"\" done", r#"one \" then two \"" done"#] {
        let text = format!("query Plain {{ field(arg: {}) }}", block(value));
        assert_eq!(names(&text), Some(expected("query", "Plain")), "{text}");
    }
}

#[test]
fn ordinary_quoted_strings_and_missing_closers_keep_their_behavior() {
    let value = serde_json::to_string("quote \" and backslash \\").unwrap();
    assert_eq!(names(&format!("query Plain {{ field(arg: {value}) }}")), Some(expected("query", "Plain")));
    assert!(read_document("query Invalid { field(arg: \"before ) }", false).is_none());
    assert!(read_document("query Invalid { field(arg: \"\"\"before ) }", false).is_none());
}
