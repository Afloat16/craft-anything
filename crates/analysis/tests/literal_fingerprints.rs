use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};
use craft_anything_analysis::js_compare::compare_versions;
use serde_json::json;

fn graph(path: &str, source: &str) -> AppGraph {
    let file = AppFile { path: path.into(), bytes: source.as_bytes().to_vec() };
    let listing = vec![(file.path.clone(), file.bytes.len() as u64)];
    analyze_application(&AppInput { files: vec![file], listing, ..Default::default() }, &AppOptions::default(), &|| false).unwrap()
}

fn bundle(source: &str) -> String {
    format!("(self.webpackChunkdemo=self.webpackChunkdemo||[]).push([[1],{{7:function(module,exports,require){{{source}}}}}]);")
}

const CHANGED: [(&str, &str); 3] = [
    ("globalThis.result = 1n;", "globalThis.result = 2n;"),
    ("globalThis.result = /a/;", "globalThis.result = /b/;"),
    ("globalThis.result = `left`;", "globalThis.result = `right`;"),
];

#[test]
fn literal_changes_do_not_match_different_files_as_structural_renames() {
    for (left, right) in CHANGED {
        let result = compare_versions(&graph("left.js", left), &graph("right.js", right));
        assert!(result["modules"]["renamed"].as_array().unwrap().is_empty(), "{result}");
        assert_eq!(result["modules"]["added"], json!(["right.js"]));
        assert_eq!(result["modules"]["removed"], json!(["left.js"]));
    }
}

#[test]
fn changed_bundle_literals_reach_the_changed_module_inventory() {
    for (left, right) in CHANGED {
        let result = compare_versions(&graph("bundle.js", &bundle(left)), &graph("bundle.js", &bundle(right)));
        let modules = &result["bundle_modules"];
        assert_eq!(modules["left"], 1);
        assert_eq!(modules["right"], 1);
        assert_eq!(modules["matched_by"]["id"], 1, "{result}");
        assert_eq!(modules["changed"], json!([{"id": "7", "file": "bundle.js"}]));
        assert!(modules["added"].as_array().unwrap().is_empty());
        assert!(modules["removed"].as_array().unwrap().is_empty());
    }
}

#[test]
fn equivalent_literals_still_match_by_structure_across_files_and_bundles() {
    let pairs = [
        ("globalThis.result = 10n;", "globalThis.result = 0xan;"),
        ("globalThis.result = /a/gi;", "globalThis.result = /a/ig;"),
        ("function original(parameter) { return `value:${parameter}`; }", "function a(b) { return `value:${b}`; }"),
    ];
    for (left, right) in pairs {
        let result = compare_versions(&graph("left.js", left), &graph("right.js", right));
        assert_eq!(result["modules"]["renamed"].as_array().unwrap().len(), 1, "{result}");
        let result = compare_versions(&graph("bundle.js", &bundle(left)), &graph("bundle.js", &bundle(right)));
        assert_eq!(result["bundle_modules"]["left"], 1);
        assert_eq!(result["bundle_modules"]["right"], 1);
        assert_eq!(result["bundle_modules"]["matched_by"]["structure"], 1, "{result}");
        assert!(result["bundle_modules"]["changed"].as_array().unwrap().is_empty());
    }
}
