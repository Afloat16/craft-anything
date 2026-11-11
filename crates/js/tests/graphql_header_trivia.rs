use craft_anything_js::api_ops::{HOLE, read_document};
use craft_anything_js::{Limits, analyze};

fn operations(text: &str) -> Vec<(&'static str, Option<String>, bool)> {
    read_document(text, false).unwrap().operations.into_iter().map(|operation| (operation.kind, operation.name, operation.literal)).collect()
}

#[test]
fn header_comments_do_not_hide_operation_names() {
    for kind in ["query", "mutation", "subscription"] {
        for newline in ["\n", "\r", "\r\n"] {
            let text = format!("{kind} # a different label{newline} Actual {{ field }}");
            assert_eq!(operations(&text), [(kind, Some("Actual".into()), true)]);
        }
    }
}

#[test]
fn comment_punctuation_cannot_open_selection_sets() {
    let text = "query Actual # ignore ) } { \" punctuation\n { field }";
    assert_eq!(operations(text), [("query", Some("Actual".into()), true)]);
}

#[test]
fn comments_inside_variable_definitions_do_not_change_parenthesis_depth() {
    let text = "query Actual($id: ID! # ignore ) { \"\n, $limit: Int = 1) { node(id: $id) { id } }";
    assert_eq!(operations(text), [("query", Some("Actual".into()), true)]);
}

#[test]
fn commas_and_byte_order_marks_do_not_hide_operation_names() {
    for kind in ["query", "mutation", "subscription"] {
        for trivia in [",", "\u{feff}", ", \u{feff},\t"] {
            let text = format!("{kind}{trivia}Actual {{ field }}");
            assert_eq!(operations(&text), [(kind, Some("Actual".into()), true)]);
        }
    }
}

#[test]
fn fragment_header_comments_do_not_hide_later_operations() {
    let text = "fragment # ignore ) { \"\n Part on User { id }\nquery Actual { user { ...Part } }";
    assert_eq!(operations(text), [("query", Some("Actual".into()), true)]);
}

#[test]
fn parsed_and_fallback_api_facts_keep_names_after_header_trivia() {
    for document in ["query # heading\n Actual { field }", "query,\u{feff} Actual { field }", "query Actual # ) { \n { field }"] {
        let source = format!("const document = {};", serde_json::to_string(document).unwrap());
        for limits in [Limits::default(), Limits { max_parse_bytes: 0, ..Limits::default() }] {
            let analysis = analyze("client.js", &source, &limits);
            let rows: Vec<_> = analysis.facts.api_operations.iter().filter(|operation| operation.protocol == "graphql").collect();
            assert_eq!(rows.len(), 1, "{:?}", analysis.facts.api_operations);
            assert_eq!(rows[0].kind, "query");
            assert_eq!(rows[0].name.as_deref(), Some("Actual"));
            assert!(rows[0].literal);
        }
    }
}

#[test]
fn runtime_name_holes_after_comments_remain_unknown() {
    let text = format!("query # ignore ) {{\n {HOLE} {{ field }}");
    assert_eq!(operations(&text), [("query", None, false)]);
    let source = "const document = gql`query # ignore ) {\n ${operationName} { field }`;";
    let analysis = analyze("client.js", source, &Limits::default());
    assert!(analysis.facts.api_operations.iter().any(|operation| operation.kind == "query" && operation.name.is_none() && !operation.literal));
    assert!(!analysis.facts.api_operations.iter().any(|operation| operation.name.is_some()));
}

#[test]
fn anonymous_operations_do_not_gain_a_name_from_ignored_tokens() {
    for text in ["query # a label\n { field }", "query , \u{feff} # label\r { field }", "query { field }"] {
        assert_eq!(operations(text), [("query", None, true)]);
    }
}

#[test]
fn comment_characters_inside_strings_stay_inside_the_string() {
    let text = "query Actual($note: String = \"# ) } {, \") { echo(note: $note) }";
    assert_eq!(operations(text), [("query", Some("Actual".into()), true)]);
    assert_eq!(
        operations("mutation Save { save } subscription Watch { changes }"),
        [("mutation", Some("Save".into()), true), ("subscription", Some("Watch".into()), true)]
    );
}

#[test]
fn comments_without_a_following_selection_set_are_not_operations() {
    for text in ["query # no selection", "query Actual # { field }", "query # comment only\r", "# query Actual { field }"] {
        assert!(read_document(text, false).is_none(), "{text}");
    }
}
