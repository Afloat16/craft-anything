use craft_anything_analysis::js_source::Ignore;

fn check(rules: &str, paths: &[(&str, bool)]) {
    let mut ignore = Ignore::default();
    ignore.add_file("", rules);
    for (path, expected) in paths {
        assert_eq!(ignore.is_ignored(path, false), *expected, "{rules:?} against {path:?}");
    }
}

#[test]
fn globstar_slash_matches_zero_or_more_complete_directories() {
    check("**/file.js", &[("file.js", true), ("src/file.js", true), ("src/deep/file.js", true), ("other.ts", false)]);
    check("src/**/file.js", &[("src/file.js", true), ("src/a/b/file.js", true), ("file.js", false), ("other/src/file.js", false)]);
}

#[test]
fn suffix_reuse_does_not_start_a_match_inside_a_path_component() {
    check("**/file.js", &[("prefixfile.js", false), ("src/prefixfile.js", false), ("file.jsx", false), ("src/file.js/extra", false), ("file.js", true)]);
    check("**/src/file.js", &[("prefixsrc/file.js", false), ("other/src/file.js", true), ("src/file.js", true)]);
}

#[test]
fn multiple_globstar_rows_keep_each_directory_boundary() {
    check(
        "a/**/b/**/file.js",
        &[
            ("a/b/file.js", true),
            ("a/one/two/b/three/file.js", true),
            ("ab/file.js", false),
            ("a/one/bprefix/file.js", false),
            ("a/one/b/prefixfile.js", false),
            ("other/a/b/file.js", false),
        ],
    );
}

#[test]
fn ordinary_wildcards_and_classes_still_restrict_their_components() {
    check(
        "**/[ab]*/**/file.?s",
        &[("a/file.js", true), ("deep/ab/nested/file.ts", true), ("c/file.js", false), ("a/file.jsx", false), ("deep/a/file.js", true)],
    );
    check("*/**/file.js", &[("file.js", false), ("src/file.js", true), ("src/deep/file.js", true)]);
}

#[test]
fn later_negations_and_reexclusions_keep_their_order() {
    check(
        "**/file.js\n!**/keep/**/file.js\nsrc/keep/private/**/file.js",
        &[
            ("file.js", true),
            ("src/file.js", true),
            ("keep/file.js", false),
            ("src/keep/deep/file.js", false),
            ("src/keep/private/file.js", true),
            ("src/keep/private/deep/file.js", true),
        ],
    );
}

#[test]
fn nested_ignore_files_match_only_their_own_relative_paths() {
    let mut ignore = Ignore::default();
    ignore.add_file("src", "**/file.js\n!keep/**/file.js");
    for (path, expected) in [
        ("src/file.js", true),
        ("src/deep/file.js", true),
        ("src/keep/file.js", false),
        ("src/keep/deep/file.js", false),
        ("src-prefix/file.js", false),
        ("other/src/file.js", false),
    ] {
        assert_eq!(ignore.is_ignored(path, false), expected, "{path}");
    }
}

#[test]
fn unicode_names_and_literal_patterns_keep_their_results() {
    check("**/雪.js", &[("雪.js", true), ("é/雪.js", true), ("é/prefix雪.js", false), ("é/雪.jsx", false)]);
    check("literal.js", &[("literal.js", true), ("src/literal.js", true), ("prefixliteral.js", false)]);
    check("src/*.js", &[("src/file.js", true), ("src/deep/file.js", false)]);
}

#[test]
fn long_realistic_paths_preserve_matching_and_nonmatching_results() {
    let prefix = (0..36).map(|i| format!("{i:02}{}", "d".repeat(97))).collect::<Vec<_>>().join("/");
    let matching = format!("{prefix}/file.js");
    let different = format!("{prefix}/file.ts");
    let mut ignore = Ignore::default();
    ignore.add_file("", "**/file.js");
    assert!(ignore.is_ignored(&matching, false));
    assert!(!ignore.is_ignored(&different, false));
    let mut absent = Ignore::default();
    absent.add_file("", "**/never.js");
    assert!(!absent.is_ignored(&matching, false));
}
