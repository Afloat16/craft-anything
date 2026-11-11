use craft_anything_js::html::scan_html;

const INVALID_TYPES: [&str; 10] = [
    "text/javascript1",
    "text/javascript1.6",
    "text/javascript1evil",
    "application/javascript1.0",
    "application/ecmascript1.0",
    "application/x-javascript1.0",
    "application/x-ecmascript1.0",
    "text/ecmascript1.0",
    "text/x-javascript1.0",
    "text/x-ecmascript1.0",
];

const VALID_TYPES: [&str; 16] = [
    "application/ecmascript",
    "application/javascript",
    "application/x-ecmascript",
    "application/x-javascript",
    "text/ecmascript",
    "text/javascript",
    "text/javascript1.0",
    "text/javascript1.1",
    "text/javascript1.2",
    "text/javascript1.3",
    "text/javascript1.4",
    "text/javascript1.5",
    "text/jscript",
    "text/livescript",
    "text/x-ecmascript",
    "text/x-javascript",
];

#[test]
fn unsupported_version_suffixes_are_not_inline_scripts() {
    for mime in INVALID_TYPES {
        let html = format!(r#"<script type="{mime}">fetch("/data-only");</script>"#);
        assert!(scan_html(&html).scripts.is_empty(), "{mime}");
    }
}

#[test]
fn unsupported_version_suffixes_are_not_external_scripts() {
    for mime in INVALID_TYPES {
        let html = format!(r#"<script type="{mime}" src="./data.js"></script>"#);
        assert!(scan_html(&html).scripts.is_empty(), "{mime}");
    }
}

#[test]
fn parameters_and_case_do_not_make_unsupported_versions_executable() {
    for mime in INVALID_TYPES {
        let html = format!(r#"<script type="{}; charset=utf-8">fetch("/data-only");</script>"#, mime.to_ascii_uppercase());
        assert!(scan_html(&html).scripts.is_empty(), "{mime}");
    }
}

#[test]
fn a_following_real_script_keeps_its_source_coordinates() {
    let html = "<title>脚本</title>\n\
        <script type=\"text/javascript1evil\">\n\
        fetch('/data-only');\n\
        </script>\n\
        <script>fetch('/real');</script>";
    let facts = scan_html(html);
    assert_eq!(facts.scripts.len(), 1);
    let script = &facts.scripts[0];
    assert_eq!(script.line, 5);
    let (start, end) = script.inline.unwrap();
    assert_eq!(&html[start as usize..end as usize], "fetch('/real');");
    assert_eq!(start as usize, html.find("fetch('/real');").unwrap());
}

#[test]
fn all_standard_javascript_mime_types_remain_case_insensitive() {
    for mime in VALID_TYPES {
        let html = format!(r#"<script type="{mime}">run();</script><script type="{}" src="./app.js"></script>"#, mime.to_ascii_uppercase());
        let facts = scan_html(&html);
        assert_eq!(facts.scripts.len(), 2, "{mime}");
        assert_eq!(facts.scripts[1].src.as_deref(), Some("./app.js"));
        assert!(facts.scripts.iter().all(|script| !script.module));
    }
}

#[test]
fn existing_module_babel_jsx_and_parameter_policy_is_preserved() {
    let html = r#"<script>run();</script>
<script type="">run();</script>
<script type="module" src="./module.js"></script>
<script type="text/babel">run();</script>
<script type="text/jsx">run();</script>
<script type="text/javascript1.5; charset=utf-8">run();</script>
<script type=" Application/JavaScript ; charset=utf-8 ">run();</script>"#;
    let facts = scan_html(html);
    assert_eq!(facts.scripts.len(), 7);
    assert!(facts.scripts[2].module);
    assert_eq!(facts.scripts[2].src.as_deref(), Some("./module.js"));
}

#[test]
fn existing_non_javascript_data_types_remain_excluded() {
    for mime in ["importmap", "speculationrules", "application/json", "text/html", "text/x-template"] {
        let html = format!(r#"<script type="{mime}">fetch("/data-only");</script><script type="{mime}" src="./data.js"></script>"#);
        assert!(scan_html(&html).scripts.is_empty(), "{mime}");
    }
}
