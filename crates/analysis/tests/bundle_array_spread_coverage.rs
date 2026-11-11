use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};

fn graph(files: &[(&str, String)]) -> AppGraph {
    let files: Vec<_> = files.iter().map(|(path, source)| AppFile { path: (*path).into(), bytes: source.as_bytes().to_vec() }).collect();
    let listing = files.iter().map(|file| (file.path.clone(), file.bytes.len() as u64)).collect();
    let input = AppInput { files, listing, ..AppInput::default() };
    analyze_application(&input, &AppOptions::default(), &|| false).unwrap()
}

fn spread_source() -> String {
    let tail = (3..=10).map(|value| format!("function(e){{e.exports={value}}}")).collect::<Vec<_>>().join(",");
    format!("var __webpack_modules__=[function(e,t,r){{r(1)}},function(e){{e.exports=32}},...more,{tail}];")
}

#[test]
fn array_spread_unknowns_reach_the_application_graph() {
    let result = graph(&[("bundle.js", spread_source())]);
    let module = result.module("bundle.js").unwrap();
    assert_eq!(module.bundle_modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["0", "1"]);
    assert!(result.limitations.iter().any(|(code, message)| code == "truncated" && message.contains("bundle.js")));
    assert!(!result.nodes.iter().any(|node| node.kind == "bundle-module" && node.label == "3"));
}

#[test]
fn one_partial_bundle_does_not_discard_other_files_modules() {
    let result = graph(&[
        ("partial.js", spread_source()),
        ("ordinary.js", "var __webpack_modules__=[function(e){e.exports=31},function(e){e.exports=32}];".into()),
    ]);
    assert_eq!(result.module("ordinary.js").unwrap().bundle_modules.len(), 2);
    assert!(result.limitations.iter().any(|(code, message)| code == "truncated" && message.contains("partial.js")));
    assert!(!result.limitations.iter().any(|(code, message)| code == "truncated" && message.contains("ordinary.js")));
}

#[test]
fn ordinary_array_tables_have_no_incomplete_recovery_note() {
    let result = graph(&[("bundle.js", "var __webpack_modules__=[function(e,t,r){r(1)},function(e){e.exports=32}];".into())]);
    assert_eq!(result.module("bundle.js").unwrap().bundle_modules.len(), 2);
    assert!(!result.limitations.iter().any(|(code, _)| code == "truncated"));
}
