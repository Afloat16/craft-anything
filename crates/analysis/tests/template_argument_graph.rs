use craft_anything_analysis::js_app::{AppFile, AppGraph, AppInput, AppOptions, analyze_application};

fn graph(sources: &[(&str, &str)]) -> AppGraph {
    let files: Vec<_> = sources.iter().map(|(path, source)| AppFile { path: (*path).into(), bytes: source.as_bytes().to_vec() }).collect();
    let listing = files.iter().map(|file| (file.path.clone(), file.bytes.len() as u64)).collect();
    let input = AppInput { label: "template values".into(), format: "directory".into(), files, listing, notes: vec![] };
    analyze_application(&input, &AppOptions::default(), &|| false).unwrap()
}

#[test]
fn escaped_template_requires_resolve_to_the_actual_source_module() {
    let graph = graph(&[
        ("main.js", r#"const helper = require(`./mod\u0075le.js`); helper();"#),
        ("module.js", "module.exports = () => 1;"),
    ]);
    let imports = &graph.module("main.js").unwrap().imports;
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].specifier, "./module.js");
    assert_eq!(imports[0].resolved.as_deref(), Some("module.js"));
    assert!(graph.edges.iter().any(|edge| edge.from == "module:main.js" && edge.to == "module:module.js" && edge.kind == "require"));
}

#[test]
fn escaped_channels_pair_with_strings_and_keep_the_exposed_bridge_name() {
    let graph = graph(&[
        ("main.js", r#"const {ipcMain} = require("electron"); ipcMain.handle(`a\u0070p:load`, () => {});"#),
        ("preload.js", r#"const {contextBridge, ipcRenderer} = require("electron"); contextBridge.exposeInMainWorld(`a\x70i`, {load: () => ipcRenderer.invoke("app:load")});"#),
    ]);
    assert_eq!(graph.ipc.len(), 1);
    let channel = &graph.ipc[0];
    assert_eq!(channel.channel, "app:load");
    assert!(channel.paired);
    assert_eq!(channel.handlers.len(), 1);
    assert_eq!(channel.invokers.len(), 1);
    assert_eq!(channel.bridge_members, ["window.api.load"]);
    assert_eq!(graph.exposed[0]["key"], "api");
}

#[test]
fn ordinary_channel_pairs_and_imports_are_unchanged() {
    let graph = graph(&[
        ("main.js", r#"require("./module.js"); ipcMain.handle("app:load", () => {});"#),
        ("module.js", r#"ipcRenderer.invoke("app:load");"#),
    ]);
    assert_eq!(graph.module("main.js").unwrap().imports[0].resolved.as_deref(), Some("module.js"));
    assert_eq!(graph.ipc.len(), 1);
    assert!(graph.ipc[0].paired);
}
