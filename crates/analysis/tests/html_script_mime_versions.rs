use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};

fn graph(files: &[(&str, &str)]) -> AppGraph {
    let files: Vec<_> = files.iter().map(|(path, source)| AppFile { path: (*path).into(), bytes: source.as_bytes().to_vec() }).collect();
    let listing = files.iter().map(|file| (file.path.clone(), file.bytes.len() as u64)).collect();
    analyze_application(&AppInput { files, listing, ..Default::default() }, &AppOptions::default(), &|| false).unwrap()
}

#[test]
fn unsupported_script_versions_do_not_invent_inline_endpoints() {
    let html = r#"<script type="text/javascript1evil">fetch("/data-only");</script>
<script>fetch("/real");</script>"#;
    let result = graph(&[("index.html", html)]);
    let urls: Vec<_> = result.endpoints.iter().filter_map(|endpoint| endpoint["url"].as_str()).collect();
    assert_eq!(urls, ["/real"]);
    assert_eq!(result.modules.len(), 2);
    assert!(result.module("index.html#inline-0").is_some());
}

#[test]
fn unsupported_script_versions_do_not_create_page_imports() {
    let html = r#"<script type="application/javascript1.0" src="./data.js"></script>
<script type="text/javascript1.5" src="./app.js"></script>"#;
    let result = graph(&[("index.html", html), ("data.js", "fetch('/data-file');"), ("app.js", "fetch('/app');")]);
    let page = result.module("index.html").unwrap();
    assert_eq!(page.imports.len(), 1);
    assert_eq!(page.imports[0].kind, "html-script");
    assert_eq!(page.imports[0].specifier, "./app.js");
    assert_eq!(page.imports[0].resolved.as_deref(), Some("app.js"));
    assert_eq!(result.module("app.js").unwrap().role, "page");
    assert_eq!(result.module("data.js").unwrap().role, "unknown");
    // The input still contains data.js: its standalone facts survive without a false page import.
    assert!(result.endpoints.iter().any(|endpoint| endpoint["url"] == "/data-file"));
}

#[test]
fn supported_legacy_versions_still_produce_application_facts() {
    let html: String = (0..=5).map(|version| format!(r#"<script type="text/javascript1.{version}">fetch("/version-{version}");</script>"#)).collect();
    let result = graph(&[("index.html", &html)]);
    let urls: Vec<_> = result.endpoints.iter().filter_map(|endpoint| endpoint["url"].as_str()).collect();
    assert_eq!(urls, ["/version-0", "/version-1", "/version-2", "/version-3", "/version-4", "/version-5"]);
    assert_eq!(result.modules.len(), 7);
}
