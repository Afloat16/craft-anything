use craft_anything_engine::{CallOptions, Caller, Engine, ToolOutput};
use craft_anything_testkit::sqlite::{PAGE_SIZE as PAGE, schema_tree_db};
use serde_json::{Value, json};

fn evidence(data: &[u8], mut params: Value) -> Value {
    let directory = craft_anything_engine::testing::temp_dir();
    let path = directory.join("schema.db");
    std::fs::write(&path, data).unwrap();
    params["path"] = json!(path);
    let engine = Engine::new(craft_anything_engine::testing::session());
    let output = engine.call("db_sqlite_show", params, CallOptions::new(Caller::Mcp)).unwrap();
    let ToolOutput::Evidence(record) = output else { panic!("expected SQLite evidence") };
    assert!(record.verify());
    let value = serde_json::to_value(&*record).unwrap();
    std::fs::remove_dir_all(directory).unwrap();
    value
}

fn has_schema_limitation(value: &Value) -> bool {
    value["limitations"].as_array().unwrap().iter().any(|limitation| limitation["code"] == "schema_incomplete")
}

#[test]
fn complete_schema_and_table_pages_keep_existing_coverage() {
    let data = schema_tree_db();
    let listing = evidence(&data, json!({}));
    assert_eq!(listing["coverage"]["status"], "complete");
    assert_eq!(listing["coverage"]["examined"], 1);
    assert_eq!(listing["coverage"]["total"], 1);
    assert_eq!(listing["result"]["schema_truncated"], false);
    assert_eq!(listing["result"]["tables"][0]["name"], "items");
    assert_eq!(listing["result"]["other_objects"][0]["name"], "item_names");
    assert!(!has_schema_limitation(&listing));
    assert!(!listing["summary"].as_str().unwrap().contains("incomplete"));
    for (offset, limit, partial, next) in [(0, 1, true, Some(1)), (2, 2, false, None), (3, 1, false, None), (0, 10, false, None)] {
        let rows = evidence(&data, json!({"table": "items", "offset": offset, "limit": limit}));
        assert_eq!(rows["coverage"]["status"], if partial { "partial" } else { "complete" });
        assert_eq!(rows["coverage"]["total"], 3);
        assert_eq!(rows["coverage"]["next_offset"], json!(next));
        assert!(!has_schema_limitation(&rows));
    }
}

#[test]
fn a_missing_schema_leaf_is_partial_without_an_invented_total() {
    let mut data = schema_tree_db();
    data.truncate(3 * PAGE);
    let listing = evidence(&data, json!({}));
    assert_eq!(listing["result"]["tables"][0]["name"], "items");
    assert_eq!(listing["result"]["tables"][0]["row_count"], 3);
    assert_eq!(listing["result"]["other_objects"], json!([]));
    assert_eq!(listing["result"]["schema_truncated"], true);
    assert_eq!(listing["coverage"]["status"], "partial");
    assert_eq!(listing["coverage"]["examined"], 1);
    assert!(listing["coverage"]["total"].is_null());
    assert!(listing["coverage"]["next_offset"].is_null());
    assert!(has_schema_limitation(&listing));
    assert!(listing["summary"].as_str().unwrap().contains("incomplete schema"));
}

#[test]
fn readable_table_pages_do_not_complete_an_incomplete_schema() {
    let mut data = schema_tree_db();
    data[108..112].copy_from_slice(&0u32.to_be_bytes());
    for (offset, limit, shown, truncated, next) in [(0, 1, 1, true, Some(1)), (2, 2, 1, false, None), (3, 1, 0, false, None), (0, 10, 3, false, None)] {
        let rows = evidence(&data, json!({"table": "items", "offset": offset, "limit": limit}));
        assert_eq!(rows["coverage"]["status"], "partial", "offset {offset}");
        assert_eq!(rows["result"]["schema_truncated"], true);
        assert_eq!(rows["coverage"]["total"], 3, "the readable table still has an exact row count");
        assert_eq!(rows["coverage"]["next_offset"], json!(next));
        assert_eq!(rows["result"]["rows"]["returned"], shown);
        assert_eq!(rows["result"]["rows"]["truncated"], truncated);
        assert!(has_schema_limitation(&rows));
    }
}

#[test]
fn an_unreadable_schema_root_is_partial_even_with_no_objects() {
    let data = schema_tree_db();
    let listing = evidence(&data[..100], json!({}));
    assert_eq!(listing["result"]["tables"], json!([]));
    assert_eq!(listing["result"]["schema_truncated"], true);
    assert_eq!(listing["coverage"]["status"], "partial");
    assert_eq!(listing["coverage"]["examined"], 0);
    assert!(listing["coverage"]["total"].is_null());
    assert!(has_schema_limitation(&listing));
}

#[test]
fn an_incomplete_schema_cannot_establish_that_a_table_is_absent() {
    let directory = craft_anything_engine::testing::temp_dir();
    let path = directory.join("schema.db");
    let engine = Engine::new(craft_anything_engine::testing::session());
    for incomplete in [false, true] {
        let mut data = schema_tree_db();
        if incomplete {
            data[PAGE - 5..PAGE - 1].copy_from_slice(&0u32.to_be_bytes());
        }
        std::fs::write(&path, data).unwrap();
        let error = engine
            .call("db_sqlite_show", json!({"path": path, "table": "missing"}), CallOptions::new(Caller::Mcp))
            .expect_err("an unknown table must return an error");
        assert_eq!(error.message.contains("incomplete schema"), incomplete);
        assert_eq!(error.message.contains("in this database"), !incomplete);
    }
    std::fs::remove_dir_all(directory).unwrap();
}
