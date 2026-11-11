use craft_anything_js::{Limits, ModuleFacts, ParseMode, analyze};

fn facts(source: &str) -> ModuleFacts {
    let facts = analyze("fixture.cjs", source, &Limits::default()).facts;
    assert_eq!(facts.mode, ParseMode::Parsed);
    assert_eq!(facts.parse_errors, 0);
    facts
}

#[test]
fn literal_require_templates_use_their_string_value() {
    let cases = [
        (r#"require(`./mod\u0075le.js`);"#, "./module.js"),
        (r#"require(`./m\x6fdule.js`);"#, "./module.js"),
        (r#"require(`./path\\name.js`);"#, "./path\\name.js"),
        ("require(`./mo\\\ndule.js`);", "./module.js"),
        ("require(`./mo\\\r\ndule.js`);", "./module.js"),
        (r#"require(`./a\nb.js`);"#, "./a\nb.js"),
    ];
    for (source, expected) in cases {
        let facts = facts(source);
        assert_eq!(facts.imports.len(), 1, "{source}");
        assert_eq!(facts.imports[0].specifier, expected, "{source}");
        assert!(facts.imports[0].literal);
    }
}

#[test]
fn an_escaped_native_extension_still_identifies_a_native_module() {
    let facts = facts(r#"require(`./addon.n\x6fde`);"#);
    assert_eq!(facts.imports[0].specifier, "./addon.node");
    assert_eq!(facts.native_modules, ["./addon.node"]);
}

#[test]
fn literal_ipc_templates_share_the_same_channel_as_strings() {
    let facts = facts(r#"ipcMain.handle(`a\u0070p:load`, () => {}); ipcRenderer.invoke("app:load"); ipcRenderer.send(`line\tcolumn`);"#);
    let channels: Vec<_> = facts.ipc.iter().map(|site| site.channel.as_deref()).collect();
    assert_eq!(channels, [Some("app:load"), Some("app:load"), Some("line\tcolumn")]);
}

#[test]
fn exposed_template_keys_are_runtime_global_names() {
    let facts = facts(r#"contextBridge.exposeInMainWorld(`a\x70i`, {}); contextBridge.exposeInIsolatedWorld(7, `iso\u006cated`, {});"#);
    let keys: Vec<_> = facts.exposed.iter().map(|item| item.key.as_deref()).collect();
    assert_eq!(keys, [Some("api"), Some("isolated")]);
}

#[test]
fn commonjs_template_export_names_are_decoded_before_filtering() {
    let facts = facts(r#"Object.defineProperty(exports, `v\u0061lue`, {value:1}); Object.defineProperty(exports, `__esMod\u0075le`, {value:true});"#);
    assert_eq!(facts.exports.cjs, ["value"]);
}

#[test]
fn protocol_template_schemes_use_the_decoded_value() {
    let facts = facts(r#"protocol.handle(`a\u0070p`, () => {});"#);
    assert_eq!(facts.routes.len(), 1);
    assert_eq!(facts.routes[0].path, "app://");
    assert_eq!(facts.routes[0].source, "protocol");
}

#[test]
fn native_binding_template_names_are_decoded() {
    let facts = facts(r#"require("bindings")(`n\u0061tive`); const bindings = require("bindings"); bindings(`u\x74il`);"#);
    assert_eq!(facts.native_modules, ["bindings:native", "bindings:util"]);
}

#[test]
fn ordinary_empty_and_escaped_string_arguments_keep_their_values() {
    let facts = facts(r#"require(`./plain.js`); ipcMain.handle("a\u0070p:load", () => {}); ipcRenderer.invoke(`app:load`); ipcRenderer.send(``);"#);
    assert_eq!(facts.imports[0].specifier, "./plain.js");
    let channels: Vec<_> = facts.ipc.iter().map(|site| site.channel.as_deref()).collect();
    assert_eq!(channels, [Some("app:load"), Some("app:load"), Some("")]);
}

#[test]
fn dynamic_and_tagged_templates_do_not_become_exact_arguments() {
    let facts = facts(r#"ipcRenderer.invoke(`topic:${runtime}`); ipcRenderer.send(tag`\unicode`); require(tag`\xZ`);"#);
    assert_eq!(facts.ipc.len(), 2);
    assert!(facts.ipc.iter().all(|site| site.channel.is_none()));
    assert_eq!(facts.imports.len(), 1);
    assert!(!facts.imports[0].literal);
    assert_eq!(facts.imports[0].specifier, "{dynamic}");
}

#[test]
fn static_interpolation_and_constant_aliases_keep_their_existing_values() {
    let facts = facts(r#"const channel = `a\u0070p:load`; ipcRenderer.send(channel); ipcRenderer.send(`a\u0070p:${"load"}`);"#);
    let channels: Vec<_> = facts.ipc.iter().map(|site| site.channel.as_deref()).collect();
    assert_eq!(channels, [Some("app:load"), Some("app:load")]);
}
