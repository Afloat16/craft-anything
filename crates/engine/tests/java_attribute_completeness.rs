use craft_anything_engine::{CallOptions, Caller, Engine, ToolOutput};
use craft_anything_testkit::{electron::zip_stored, java as fixture};
use serde_json::{Value, json};
use std::path::Path;

fn evidence(engine: &Engine, path: &Path, tool: &str, mut params: Value) -> Value {
    params["path"] = json!(path);
    match engine.call(tool, params, CallOptions::new(Caller::Cli)).unwrap() {
        ToolOutput::Evidence(value) => {
            assert!(value.verify());
            serde_json::to_value(value).unwrap()
        }
        other => panic!("expected Java evidence, got {other:?}"),
    }
}

fn with_file(name: &str, bytes: &[u8], check: impl FnOnce(&Engine, &Path)) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(name);
    std::fs::write(&path, bytes).unwrap();
    let engine = Engine::new(craft_anything_engine::testing::session());
    check(&engine, &path);
}

fn damaged_class() -> Vec<u8> {
    let mut bytes = fixture::hello_class();
    bytes.pop(); // The class-level SourceFile attribute loses its final byte.
    bytes
}

fn assert_partial(value: &Value) {
    assert_eq!(value["coverage"]["status"], "partial");
    assert!(value["coverage"]["total"].is_null());
    assert!(value["limitations"].as_array().unwrap().iter().any(|limitation| limitation["code"] == "parse_warning"));
    assert!(value["summary"].as_str().unwrap().contains("Parsing is incomplete"));
}

#[test]
fn class_view_preserves_readable_members_and_exposes_the_attribute_gap() {
    with_file("Hello.class", &damaged_class(), |engine, path| {
        let value = evidence(engine, path, "java_class_show", json!({"decompile": false}));
        assert_partial(&value);
        assert_eq!(value["result"]["name"], "com.example.Hello");
        assert_eq!(value["result"]["interfaces"][0], "java.lang.Runnable");
        assert!(value["result"]["fields"].as_array().unwrap().iter().any(|field| field["name"] == "BIG"));
        assert!(value["result"]["methods"].as_array().unwrap().iter().any(|method| method["name"] == "run"));
    });
}

#[test]
fn method_view_keeps_actual_instructions_but_not_complete_class_evidence() {
    with_file("Hello.class", &damaged_class(), |engine, path| {
        let value = evidence(engine, path, "java_method_show", json!({"method_name": "run", "decompile": false}));
        assert_partial(&value);
        assert!(value["result"]["listing"].as_str().unwrap().contains("invokevirtual java.io.PrintStream.println"));
        assert!(value["result"]["instructions_total"].as_u64().unwrap() > 0);
    });
}

#[test]
fn class_search_reports_partial_coverage_for_a_readable_damaged_class() {
    with_file("Hello.class", &damaged_class(), |engine, path| {
        let value = evidence(engine, path, "java_classes_find", json!({"query": "hello", "include_methods": true}));
        assert_partial(&value);
        assert_eq!(value["result"]["matches"], 1);
        assert_eq!(value["result"]["classes"][0]["name"], "com.example.Hello");
    });
}

#[test]
fn a_filtered_empty_page_is_not_proof_of_absence_after_attribute_damage() {
    with_file("Hello.class", &damaged_class(), |engine, path| {
        let value = evidence(engine, path, "java_classes_find", json!({"query": "missing", "include_methods": true}));
        assert_partial(&value);
        assert_eq!(value["result"]["matches"], 0);
        assert!(value["result"]["classes"].as_array().unwrap().is_empty());
        let references = evidence(engine, path, "java_class_users_find", json!({"class_name": "example.Missing"}));
        assert_partial(&references);
        assert_eq!(references["result"]["references_total"], 0);
        assert!(!references["summary"].as_str().unwrap().contains("not defined in this archive"));
        for params in [json!({"method_name": "missing"}), json!({"method_name": "run", "descriptor": "()I"})] {
            let mut params = params;
            params["path"] = json!(path);
            let error = engine.call("java_method_show", params, CallOptions::new(Caller::Cli)).unwrap_err();
            let error = serde_json::to_value(error).unwrap();
            assert_eq!(error["details"]["parse_incomplete"], true);
            assert!(error["message"].as_str().unwrap().contains("readable"));
        }
    });
}

#[test]
fn reference_search_retains_positive_calls_with_partial_parse_coverage() {
    with_file("Hello.class", &damaged_class(), |engine, path| {
        let value = evidence(engine, path, "java_class_users_find", json!({"class_name": "java.io.PrintStream", "method_name": "println"}));
        assert_partial(&value);
        assert!(value["result"]["references_total"].as_u64().unwrap() > 0);
        assert!(value["result"]["references"].as_array().unwrap().iter().any(|reference| reference["caller"] == "com.example.Hello.run:()V"));
    });
}

#[test]
fn archive_search_keeps_damaged_and_healthy_readable_classes() {
    let damaged = damaged_class();
    let helper = fixture::helper_class();
    let jar = zip_stored(&[("com/example/Hello.class", damaged.as_slice()), ("com/example/Helper.class", helper.as_slice())]);
    with_file("classes.jar", &jar, |engine, path| {
        let value = evidence(engine, path, "java_classes_find", json!({"include_methods": true}));
        assert_partial(&value);
        assert_eq!(value["result"]["classes_total"], 2);
        assert_eq!(value["result"]["matches"], 2);
        let references = evidence(engine, path, "java_class_users_find", json!({"class_name": "com.example.Hello", "method_name": "run"}));
        assert_partial(&references);
        assert_eq!(references["result"]["references_total"], 1);
        assert_eq!(references["result"]["references"][0]["caller"], "com.example.Helper.go:()V");
    });
}

#[test]
fn complete_class_method_search_and_reference_evidence_remains_complete() {
    with_file("Hello.class", &fixture::hello_class(), |engine, path| {
        for (tool, params) in [
            ("java_class_show", json!({"decompile": false})),
            ("java_method_show", json!({"method_name": "run", "decompile": false})),
            ("java_classes_find", json!({"query": "missing"})),
            ("java_class_users_find", json!({"class_name": "example.Missing"})),
        ] {
            let value = evidence(engine, path, tool, params);
            assert_eq!(value["coverage"]["status"], "complete", "{tool}: {value}");
            assert!(value["limitations"].as_array().unwrap().iter().all(|limitation| limitation["code"] != "parse_warning"));
        }
    });
}
