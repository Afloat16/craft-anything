use craft_anything_analysis::js_source::Ignore;

fn ignored(rule: &str, path: &str) -> bool {
    let mut ignore = Ignore::default();
    ignore.add_file("", &format!("{rule}\n"));
    ignore.is_ignored(path, false)
}

#[test]
fn trailing_filename_stars_do_not_remove_the_required_separator() {
    for rule in ["root/a**/leaf", "root/a***/leaf"] {
        assert!(!ignored(rule, "root/aleaf"), "{rule}");
        assert!(!ignored(rule, "root/a/x/leaf"), "{rule}");
        assert!(ignored(rule, "root/a/leaf"), "{rule}");
        assert!(ignored(rule, "root/ax/leaf"), "{rule}");
    }
}

#[test]
fn embedded_filename_star_runs_do_not_cross_directories() {
    for rule in ["root/a**b/leaf", "root/a***b/leaf", "root/a****b/leaf"] {
        assert!(!ignored(rule, "root/a/x/b/leaf"), "{rule}");
        assert!(!ignored(rule, "root/a/工具/b/leaf"), "{rule}");
        assert!(ignored(rule, "root/ab/leaf"), "{rule}");
        assert!(ignored(rule, "root/axb/leaf"), "{rule}");
    }
}

#[test]
fn leading_filename_star_runs_stay_in_their_component() {
    for rule in ["root/**a/leaf", "root/***a/leaf"] {
        assert!(!ignored(rule, "root/x/a/leaf"), "{rule}");
        assert!(ignored(rule, "root/a/leaf"), "{rule}");
        assert!(ignored(rule, "root/xa/leaf"), "{rule}");
    }
}

#[test]
fn complete_star_components_match_zero_or_more_directories() {
    for rule in ["root/***/leaf", "root/****/leaf"] {
        assert!(ignored(rule, "root/leaf"), "{rule}");
        assert!(ignored(rule, "root/x/leaf"), "{rule}");
        assert!(ignored(rule, "root/x/y/leaf"), "{rule}");
        assert!(!ignored(rule, "root/aleaf"), "{rule}");
        assert!(!ignored(rule, "root/x/aleaf"), "{rule}");
    }
}

#[test]
fn ordinary_recursive_and_single_star_rules_keep_their_meaning() {
    for path in ["root/leaf", "root/x/leaf", "root/x/y/leaf"] {
        assert!(ignored("root/**/leaf", path));
        assert!(ignored("**/leaf", path));
        assert!(ignored("root/**", path));
    }
    assert!(ignored("root/*/leaf", "root/x/leaf"));
    assert!(!ignored("root/*/leaf", "root/leaf"));
    assert!(!ignored("root/*/leaf", "root/x/y/leaf"));
    let mut nested = Ignore::default();
    nested.add_file("src", "root/**/leaf\n!root/keep/leaf\n");
    assert!(nested.is_ignored("src/root/x/leaf", false));
    assert!(!nested.is_ignored("src/root/keep/leaf", false));
    assert!(!nested.is_ignored("other/root/x/leaf", false));
}

#[test]
fn escaped_stars_remain_literal_parts_of_a_filename() {
    assert!(ignored(r"root/\**/leaf", "root/*x/leaf"));
    assert!(!ignored(r"root/\**/leaf", "root/x/leaf"));
    assert!(!ignored(r"root/\**/leaf", "root/*/x/leaf"));
    assert!(ignored(r"root/*\*/leaf", "root/x*/leaf"));
    assert!(!ignored(r"root/*\*/leaf", "root/x/leaf"));
    assert!(ignored(r"root/\*\*/leaf", "root/**/leaf"));
    assert!(!ignored(r"root/\*\*/leaf", "root/x/leaf"));
}

#[test]
fn escaped_directory_separators_do_not_promote_filename_stars() {
    assert!(!ignored(r"root\/a**/leaf", "root/aleaf"));
    assert!(!ignored(r"root\/a**/leaf", "root/a/x/leaf"));
    assert!(ignored(r"root\/a**/leaf", "root/ax/leaf"));
    assert!(ignored(r"root\/**/leaf", "root/leaf"));
    assert!(ignored(r"root\/**/leaf", "root/x/y/leaf"));
}

#[test]
fn an_escaped_slash_after_recursive_stars_remains_required() {
    assert!(!ignored(r"root/**\/leaf", "root/leaf"));
    assert!(!ignored(r"root/**\/leaf", "root/aleaf"));
    assert!(ignored(r"root/**\/leaf", "root/x/leaf"));
    assert!(ignored(r"root/**\/leaf", "root/x/y/leaf"));
}

#[test]
fn recursive_and_filename_stars_keep_their_separate_roles() {
    assert!(ignored("root/**/a**/leaf", "root/a/leaf"));
    assert!(ignored("root/**/a**/leaf", "root/x/ax/leaf"));
    assert!(!ignored("root/**/a**/leaf", "root/x/aleaf"));
    assert!(!ignored("root/**/a**/leaf", "root/a/x/leaf"));
    assert!(ignored("root/***/a**b/leaf", "root/ab/leaf"));
    assert!(ignored("root/***/a**b/leaf", "root/x/axb/leaf"));
    assert!(!ignored("root/***/a**b/leaf", "root/x/a/x/b/leaf"));
}
