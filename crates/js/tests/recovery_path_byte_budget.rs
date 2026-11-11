use craft_anything_js::recover::{RewriteLevel, recover_modules, safe_module_path};
use craft_anything_js::{BundleModule, Limits};
use std::collections::HashSet;

fn assert_component_lengths(path: &str) {
    assert!(path.split('/').all(|component| component.len() <= 255), "{path:?}");
}

#[test]
fn multibyte_module_filenames_fit_a_native_component() {
    for id in ["界".repeat(120), "𐐀".repeat(80)] {
        let path = safe_module_path(&id, ".js");
        assert_component_lengths(&path);
        let stem = path.strip_suffix(".js").unwrap();
        assert!(!stem.is_empty());
        assert!(id.starts_with(stem));
    }
}

#[test]
fn multibyte_parent_components_are_bounded_too() {
    let directory = "界".repeat(100);
    let path = safe_module_path(&format!("{directory}/entry.js"), ".js");
    assert_component_lengths(&path);
    assert!(path.ends_with("/entry.js"));
    assert!(directory.starts_with(path.split('/').next().unwrap()));
}

#[test]
fn nested_multibyte_paths_do_not_expand_past_the_native_path_budget() {
    let mut parts = vec!["界".repeat(100); 15];
    parts.push("entry.js".into());
    let path = safe_module_path(&parts.join("/"), ".js");
    assert!(path.len() < 4096);
    assert_eq!(path.split('/').count(), 16);
    assert!(path.ends_with("/entry.js"));
    assert_component_lengths(&path);
}

#[test]
fn recovered_collisions_keep_writable_names_and_original_identities() {
    let source = "globalThis.answer = 1;";
    let prefix = "界".repeat(120);
    let ids = [format!("{prefix}-first.js"), format!("{prefix}-second.js"), format!("{prefix}-third.js")];
    let modules: Vec<_> = ids
        .iter()
        .map(|id| BundleModule {
            id: id.clone(),
            format: "webpack".into(),
            chunk_ids: vec![],
            start: 0,
            end: source.len() as u32,
            body: Some((0, source.len() as u32)),
            line: 1,
            params: vec![],
            deps: vec![],
            sha256: "0123456789abcdef".into(),
            fingerprint: String::new(),
        })
        .collect();
    let recovered = recover_modules(source, &modules, RewriteLevel::None, &Limits::default());
    assert_eq!(recovered.len(), ids.len());
    assert_eq!(recovered.iter().map(|module| &module.file_name).collect::<HashSet<_>>().len(), ids.len());
    for (module, id) in recovered.iter().zip(ids) {
        assert_component_lengths(&module.file_name);
        assert_eq!(module.id, id);
        assert!(module.text.contains(&format!("{id:?}")));
    }
}

#[test]
fn source_map_paths_without_a_fallback_extension_are_bounded() {
    let id = "界".repeat(120);
    let path = safe_module_path(&id, "");
    assert_component_lengths(&path);
    assert!(!path.is_empty());
    assert!(id.starts_with(&path));
    assert!(!path.ends_with(".js"));
}

#[test]
fn ascii_and_short_unicode_paths_keep_their_existing_names() {
    for (id, expected) in [
        ("module", "module.js"),
        ("folder/entry.mjs", "folder/entry.mjs"),
        ("模块/数据.ts", "模块/数据.ts"),
        ("webpack://./src/app.js", "src/app.js"),
        (".././keep/entry", "keep/entry.js"),
    ] {
        assert_eq!(safe_module_path(id, ".js"), expected);
    }
    assert_eq!(safe_module_path(&"a".repeat(200), ".js"), format!("{}.js", "a".repeat(120)));
}

#[test]
fn a_multibyte_character_at_the_budget_boundary_stays_whole() {
    for id in [format!("{}界x", "a".repeat(119)), format!("{}𐐀x", "a".repeat(118)), format!("{}éx", "a".repeat(119))] {
        let path = safe_module_path(&id, ".js");
        let stem = path.strip_suffix(".js").unwrap();
        assert!(!stem.is_empty());
        assert!(id.starts_with(stem));
        assert!(!path.contains('\u{fffd}'));
        assert_component_lengths(&path);
    }
}
