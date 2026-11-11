use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};

fn graph(files: &[(&str, &str)]) -> AppGraph {
    let files: Vec<_> = files.iter().map(|(path, source)| AppFile { path: (*path).into(), bytes: source.as_bytes().to_vec() }).collect();
    let listing = files.iter().map(|file| (file.path.clone(), file.bytes.len() as u64)).collect();
    let input = AppInput { files, listing, ..AppInput::default() };
    analyze_application(&input, &AppOptions::default(), &|| false).unwrap()
}

fn require_target<'a>(result: &'a AppGraph, importer: &str) -> Option<&'a str> {
    let imports = &result.module(importer).unwrap().imports;
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].kind, "require");
    imports[0].resolved.as_deref()
}

#[test]
fn commonjs_imports_keep_literal_query_and_fragment_characters() {
    for suffix in ["#release", "?raw"] {
        let file = format!("worker{suffix}.js");
        let source = format!("require('./{file}');");
        let result = graph(&[
            ("package.json", r#"{"main":"main.js"}"#),
            ("main.js", &source),
            (&file, "exports.correct = true;"),
            ("worker.js", "exports.wrong = true;"),
        ]);
        assert_eq!(require_target(&result, "main.js"), Some(file.as_str()));
        assert!(result.edges.iter().any(|edge| edge.from == "module:main.js" && edge.to == format!("module:{file}") && edge.kind == "require"));
        assert!(!result.edges.iter().any(|edge| edge.from == "module:main.js" && edge.to == "module:worker.js" && edge.kind == "require"));
        assert_eq!(result.module(&file).unwrap().role, "package");
        assert_eq!(result.module("worker.js").unwrap().role, "unknown");
    }
}

#[test]
fn extensionless_commonjs_imports_append_to_the_literal_file_name() {
    for suffix in ["#release", "?raw"] {
        let file = format!("worker{suffix}.js");
        let source = format!("require('./worker{suffix}');");
        let result = graph(&[("main.js", &source), (&file, "exports.correct = true;"), ("worker.js", "exports.wrong = true;")]);
        assert_eq!(require_target(&result, "main.js"), Some(file.as_str()));
    }
}

#[test]
fn commonjs_imports_keep_literal_directory_names() {
    for suffix in ["#release", "?raw"] {
        let file = format!("src/dir{suffix}/child.js");
        let source = format!("require('./dir{suffix}/child.js');");
        let result = graph(&[("src/main.js", &source), (&file, "exports.correct = true;"), ("src/dir.js", "exports.wrong = true;")]);
        assert_eq!(require_target(&result, "src/main.js"), Some(file.as_str()));
    }
}

#[test]
fn a_missing_literal_commonjs_file_does_not_resolve_to_a_stripped_name() {
    for suffix in ["#release", "?raw"] {
        let source = format!("require('./missing{suffix}');");
        let result = graph(&[("main.js", &source), ("missing.js", "exports.wrong = true;")]);
        assert_eq!(require_target(&result, "main.js"), None);
        assert!(!result.edges.iter().any(|edge| edge.from == "module:main.js" && edge.kind == "require"));
        assert_eq!(result.module("missing.js").unwrap().role, "unknown");
    }
}

#[test]
fn package_main_is_a_literal_file_path() {
    for suffix in ["#release", "?raw"] {
        let file = format!("app/entry{suffix}.js");
        let package = format!(r#"{{"main":"entry{suffix}.js"}}"#);
        let result = graph(&[("app/package.json", &package), (&file, "exports.correct = true;"), ("app/entry.js", "exports.wrong = true;")]);
        assert_eq!(result.entry_points.main.as_deref(), Some(file.as_str()));
        assert_eq!(result.module(&file).unwrap().role, "package");
        assert_eq!(result.module("app/entry.js").unwrap().role, "unknown");
    }
}

#[test]
fn esm_imports_and_reexports_keep_url_suffix_handling() {
    let source =
        "import { value } from './actual.mjs?cache#part'; export { value as again } from './actual.mjs?second#part'; import('./actual.mjs?third#part');";
    let result = graph(&[("main.mjs", source), ("actual.mjs", "export const value = 7;")]);
    let imports = &result.module("main.mjs").unwrap().imports;
    assert_eq!(imports.len(), 3);
    for kind in ["import", "re-export", "dynamic-import"] {
        let import = imports.iter().find(|import| import.kind == kind).unwrap();
        assert_eq!(import.resolved.as_deref(), Some("actual.mjs"));
    }
}

#[test]
fn ordinary_commonjs_paths_keep_parent_normalization_and_extension_lookup() {
    let result = graph(&[("src/main.js", "require('../shared/./actual');"), ("shared/actual.js", "exports.value = 7;")]);
    assert_eq!(require_target(&result, "src/main.js"), Some("shared/actual.js"));
}

#[test]
fn commonjs_percent_spelling_is_not_url_decoded() {
    let result = graph(&[
        ("main.js", "require('./worker%23release.js');"),
        ("worker%23release.js", "exports.correct = true;"),
        ("worker#release.js", "exports.wrong = true;"),
    ]);
    assert_eq!(require_target(&result, "main.js"), Some("worker%23release.js"));
}

#[test]
fn literal_commonjs_paths_still_cannot_escape_the_application_root() {
    let result = graph(&[("main.js", "require('../outside#release.js');"), ("outside#release.js", "exports.wrong = true;")]);
    assert_eq!(require_target(&result, "main.js"), None);
}

#[test]
fn html_script_urls_keep_query_and_fragment_handling() {
    let result = graph(&[("index.html", r#"<script src="./actual.js?cache#part"></script>"#), ("actual.js", "exports.value = 7;")]);
    let imports = &result.module("index.html").unwrap().imports;
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].kind, "html-script");
    assert_eq!(imports[0].resolved.as_deref(), Some("actual.js"));
}
