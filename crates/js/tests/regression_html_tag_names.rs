use craft_anything_js::html::scan_html;

#[test]
fn regression_custom_tag_names_are_not_builtin_tags() {
    let f = scan_html(
        r#"<script:widget src="ghost.js"></script:widget><script_widget src="other.js"></script_widget><link:widget rel="modulepreload" href="ghost.js"><meta:widget http-equiv="content-security-policy" content="bad"><title:widget>wrong</title:widget><script src="real.js"></script>"#,
    );
    assert_eq!(f.scripts.iter().filter_map(|s| s.src.as_deref()).collect::<Vec<_>>(), ["real.js"]);
    assert!(f.preloads.is_empty());
    assert!(f.csp.is_none());
    assert!(f.title.is_none());
    assert_eq!(scan_html("<SCRIPT src='upper.js'></SCRIPT>").scripts[0].src.as_deref(), Some("upper.js"));
}
