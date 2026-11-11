use craft_anything_analysis::js_source::Ignore;

fn check(rules: &str, cases: &[(&str, bool)]) {
    let mut ignore = Ignore::default();
    ignore.add_file("", rules);
    for (path, expected) in cases {
        assert_eq!(ignore.is_ignored(path, false), *expected, "rules={rules:?}, path={path:?}");
    }
}

#[test]
fn leading_escaped_glob_characters_remain_literal() {
    check("\\*.js\n", &[("*.js", true), ("app.js", false), ("nested/*.js", true), ("nested/app.js", false)]);
    check("\\?.js\n", &[("?.js", true), ("a.js", false), ("ab.js", false)]);
    check("\\[ab].js\n", &[("[ab].js", true), ("a.js", false), ("b.js", false)]);
}

#[test]
fn leading_literal_backslashes_are_not_consumed_twice() {
    check("\\\\cache.js\n", &[("\\cache.js", true), ("cache.js", false)]);
}

#[test]
fn escaped_trailing_spaces_survive_padding_removal() {
    for rules in ["trail\\ \n", "trail\\   \n"] {
        check(rules, &[("trail ", true), ("trail", false), ("trail  ", false), ("nested/trail ", true)]);
    }
    check("\\ \n", &[(" ", true), ("", false)]);
    check("trail\\\\  \n", &[("trail\\", true), ("trail", false), ("trail\\ ", false)]);
}

#[test]
fn non_space_trailing_whitespace_is_part_of_the_filename() {
    check("tab\t\n", &[("tab\t", true), ("tab", false)]);
    check("unicode\u{a0}\n", &[("unicode\u{a0}", true), ("unicode", false)]);
}

#[test]
fn an_unfinished_escape_never_matches_or_negates_a_rule() {
    check("trailing\\\n", &[("trailing", false), ("trailing\\", false)]);
    check("*.js\n!keep.js\\\n", &[("keep.js", true), ("other.js", true)]);
    check("trailing\\\\\n", &[("trailing\\", true), ("trailing", false)]);
}

#[test]
fn line_endings_remove_only_one_carriage_return() {
    check("line\r", &[("line", true), ("line\r", false)]);
    check("line\r\n", &[("line", true), ("line\r", false)]);
    check("line\r\r\n", &[("line\r", true), ("line", false)]);
    check("line\\ \r\n", &[("line ", true), ("line", false)]);
}

#[test]
fn escaped_names_respect_the_nested_ignore_file_base() {
    let mut ignore = Ignore::default();
    ignore.add_file("pkg", "\\*.js\n");
    assert!(ignore.is_ignored("pkg/*.js", false));
    assert!(!ignore.is_ignored("pkg/app.js", false));
    assert!(!ignore.is_ignored("*.js", false));
    assert!(!ignore.is_ignored("app.js", false));
}

#[test]
fn escaped_comment_and_negation_markers_still_name_literal_files() {
    check("\\#note.js\n", &[("#note.js", true), ("note.js", false)]);
    check("\\!keep.js\n", &[("!keep.js", true), ("keep.js", false)]);
    check("*.js\n!\\*.js\n", &[("*.js", false), ("app.js", true)]);
    check("\\a.js\n", &[("a.js", true), ("\\a.js", false)]);
}

#[test]
fn ordinary_globs_anchors_directory_rules_and_negation_are_unchanged() {
    check("*.js\n!keep.js\n", &[("app.js", true), ("keep.js", false), ("app.txt", false), ("nested/app.js", true), ("nested/keep.js", false)]);
    check("/\\*.js\n", &[("*.js", true), ("app.js", false), ("nested/*.js", false)]);
    check("# comment\nplain   \n", &[("plain", true), ("plain ", false), ("# comment", false)]);
    let mut ignore = Ignore::default();
    ignore.add_file("", "node_modules/\n/dist\nsrc/**/gen/*.js\n");
    assert!(ignore.is_ignored("node_modules", true));
    assert!(!ignore.is_ignored("node_modules", false));
    assert!(ignore.is_ignored("dist", true));
    assert!(!ignore.is_ignored("nested/dist", true));
    assert!(ignore.is_ignored("src/gen/app.js", false));
    assert!(ignore.is_ignored("src/a/b/gen/app.js", false));
}
