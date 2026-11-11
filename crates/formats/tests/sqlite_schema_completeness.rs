use craft_anything_formats::sqlite::SqliteDb;
use craft_anything_testkit::sqlite::{PAGE_SIZE as PAGE, schema_tree_db, simple_db};

fn check(data: &[u8], names: &[&str], incomplete: bool) {
    let db = SqliteDb::parse(data).unwrap();
    let (objects, truncated) = db.schema();
    assert_eq!(objects.iter().map(|object| object.name.as_str()).collect::<Vec<_>>(), names);
    assert_eq!(truncated, incomplete, "schema() must disclose an incomplete schema walk");
    let overview = db.overview();
    assert_eq!(overview.schema_truncated, incomplete);
    assert_eq!(serde_json::to_value(overview.schema).unwrap(), serde_json::to_value(objects).unwrap());
}

#[test]
fn complete_schema_keeps_objects_and_table_rows() {
    let data = schema_tree_db();
    check(&data, &["items", "item_names"], false);
    let db = SqliteDb::parse(&data).unwrap();
    let (objects, _) = db.schema();
    assert_eq!(objects[0].rootpage, 2);
    assert_eq!(objects[0].columns.len(), 4);
    assert_eq!(objects[1].obj_type, "view");
    let rows = db.read_rows(2, 0, 10);
    assert_eq!(rows.rows.len(), 3);
    assert!(!rows.truncated);
    assert!(rows.notes.is_empty());
}

#[test]
fn a_genuinely_empty_schema_is_complete() {
    let mut data = simple_db();
    data[100..PAGE].fill(0);
    data[100] = 0x0d;
    data[105..107].copy_from_slice(&(PAGE as u16).to_be_bytes());
    check(&data, &[], false);
}

#[test]
fn null_cyclic_and_missing_schema_children_keep_readable_siblings() {
    for bad in [0u32, 1, 5] {
        let mut left = schema_tree_db();
        left[PAGE - 5..PAGE - 1].copy_from_slice(&bad.to_be_bytes());
        check(&left, &["item_names"], true);

        let mut right = schema_tree_db();
        right[108..112].copy_from_slice(&bad.to_be_bytes());
        check(&right, &["items"], true);
    }
}

#[test]
fn a_reused_schema_leaf_is_retained_once_and_marked_incomplete() {
    let mut data = schema_tree_db();
    data[108..112].copy_from_slice(&3u32.to_be_bytes());
    check(&data, &["items"], true);
}

#[test]
fn wrong_kind_and_unreadable_schema_cells_are_disclosed() {
    let mut wrong_kind = schema_tree_db();
    wrong_kind[3 * PAGE] = 0x0a;
    check(&wrong_kind, &["items"], true);

    let mut bad_cell = schema_tree_db();
    bad_cell[3 * PAGE + 8..3 * PAGE + 10].copy_from_slice(&u16::MAX.to_be_bytes());
    check(&bad_cell, &["items"], true);

    let mut bad_child_pointer = schema_tree_db();
    bad_child_pointer[112..114].copy_from_slice(&u16::MAX.to_be_bytes());
    check(&bad_child_pointer, &["item_names"], true);
}

#[test]
fn truncated_schema_pages_do_not_look_like_a_complete_prefix() {
    let mut data = schema_tree_db();
    data.truncate(3 * PAGE);
    check(&data, &["items"], true);

    data.truncate(100);
    check(&data, &[], true);
}

#[test]
fn an_invalid_schema_root_does_not_look_empty_and_complete() {
    let mut data = schema_tree_db();
    data[103..105].copy_from_slice(&u16::MAX.to_be_bytes());
    check(&data, &[], true);
}

#[test]
fn a_schema_depth_limit_keeps_the_readable_prefix_but_is_incomplete() {
    let mut data = schema_tree_db();
    let leaf = data[3 * PAGE..4 * PAGE].to_vec();
    let pages = 70usize;
    data.resize(pages * PAGE, 0);
    data[28..32].copy_from_slice(&(pages as u32).to_be_bytes());
    for page in 4..pages {
        let start = (page - 1) * PAGE;
        data[start..start + PAGE].fill(0);
        data[start] = 0x05;
        data[start + 5..start + 7].copy_from_slice(&(PAGE as u16).to_be_bytes());
        data[start + 8..start + 12].copy_from_slice(&((page + 1) as u32).to_be_bytes());
    }
    data[(pages - 1) * PAGE..].copy_from_slice(&leaf);
    check(&data, &["items"], true);
}
