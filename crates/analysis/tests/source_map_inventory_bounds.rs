use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};
use serde_json::{Value, json};

fn graph_with_maps(maps: Vec<Value>) -> AppGraph {
    let mut files = Vec::new();
    for (i, map) in maps.into_iter().enumerate() {
        files.push(AppFile { path: format!("script-{i}.js"), bytes: format!("//# sourceMappingURL=script-{i}.js.map\n").into_bytes() });
        files.push(AppFile { path: format!("script-{i}.js.map"), bytes: serde_json::to_vec(&map).unwrap() });
    }
    let listing = files.iter().map(|file| (file.path.clone(), file.bytes.len() as u64)).collect();
    analyze_application(&AppInput { files, listing, ..Default::default() }, &AppOptions::default(), &|| false).unwrap()
}

#[test]
fn application_inventory_reports_the_entry_limit() {
    let sources: Vec<_> = (0..5001).map(|i| format!("source-{i}.js")).collect();
    let graph = graph_with_maps(vec![json!({"version": 3, "sourceRoot": "src", "sources": sources, "mappings": ""})]);
    let map = &graph.source_maps[0];
    assert_eq!(map.sources.len(), 5000);
    assert_eq!(map.sources.first().map(String::as_str), Some("src/source-0.js"));
    assert_eq!(map.sources.last().map(String::as_str), Some("src/source-4999.js"));
    assert!(map.sources_truncated);
    assert!(graph.limitations.iter().any(|(key, message)| key == "source_map_inventory_truncated" && message.contains("script-0.js")));
}

#[test]
fn expanded_path_bytes_are_bounded_across_application_maps() {
    let map = json!({
        "version": 3, "sourceRoot": "r".repeat(2 * 1024 * 1024),
        "sources": vec!["/x.js"; 20], "mappings": ""
    });
    let graph = graph_with_maps(vec![map.clone(), map]);
    assert_eq!(graph.source_maps.len(), 2);
    assert_eq!(graph.source_maps[0].sources.len(), 20);
    assert!(!graph.source_maps[0].sources_truncated);
    assert_eq!(graph.source_maps[1].sources.len(), 11);
    assert!(graph.source_maps[1].sources_truncated);
    let bytes: usize = graph.source_maps.iter().flat_map(|map| &map.sources).map(String::len).sum();
    assert!(bytes <= 64 * 1024 * 1024);
    assert!(graph.limitations.iter().any(|(key, message)| key == "source_map_inventory_truncated" && message.contains("script-1.js")));
}

#[test]
fn ordinary_inventories_keep_paths_content_counts_and_complete_evidence() {
    let graph = graph_with_maps(vec![json!({
        "version": 3, "sourceRoot": "src", "sources": ["a.js", null, "/b.js"],
        "sourcesContent": ["first();", null, "second();"], "mappings": ""
    })]);
    let map = &graph.source_maps[0];
    assert_eq!(map.sources, ["src/a.js", "src//b.js"]);
    assert_eq!(map.sources_with_content, 2);
    assert!(!map.sources_truncated);
    assert!(!graph.limitations.iter().any(|(key, _)| key == "source_map_inventory_truncated"));
    let mut saved = serde_json::to_value(&graph).unwrap();
    saved["source_maps"][0].as_object_mut().unwrap().remove("sources_truncated");
    let restored: AppGraph = serde_json::from_value(saved).unwrap();
    assert!(!restored.source_maps[0].sources_truncated);
    assert_eq!(restored.source_maps[0].sources, map.sources);
}
