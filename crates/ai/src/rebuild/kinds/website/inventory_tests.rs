use super::*;
use crate::rebuild::tests::check_worker;

fn workspace() -> (tempfile::TempDir, Workspace) {
    let dir = tempfile::tempdir().unwrap();
    let ws = Workspace::create(dir.path(), "Recorded site", "Recreate this website", TargetRef::Website { url: "https://example.test/".into() }).unwrap();
    (dir, ws)
}

fn brief(ws: &Workspace) -> String {
    WebsiteKind.brief_sections(&BriefCx { ws, deep: false, ghidra: false, editor: None, assets: false, image_model: None, sees_pictures: true }).join("\n")
}

fn record(ws: &Workspace, name: &str, bytes: &[u8]) -> PathBuf {
    let path = ws.capture_dir().join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, bytes).unwrap();
    path
}

fn rejected(ws: &Workspace, name: &str) {
    let note = brief(ws);
    assert!(note.contains("can't read the website study") && note.contains(name), "the builder must see the unreadable inventory: {note}");
    assert!(note.contains("Keep the recorded files"), "the builder must know how to recover without discarding the study: {note}");
    let error = check_worker(ws, false).run_checks(ws, &WebsiteKind, 1).unwrap_err();
    assert!(error.message.contains(name), "the actual check must reject the same record: {error:?}");
}

#[test]
fn malformed_routes_are_reported_without_rewriting_them() {
    let (_dir, ws) = workspace();
    let bytes = b"[{\"path\":\"/pricing\",\"slug\":\"pricing\",\"url\":";
    let path = record(&ws, ROUTES_FILE, bytes);
    rejected(&ws, ROUTES_FILE);
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}

#[test]
fn malformed_state_inventories_are_reported() {
    for bytes in [b"{not JSON".as_slice(), b"{}", b"{\"states\":null}", b"{\"states\":[false]}"] {
        let (_dir, ws) = workspace();
        let path = record(&ws, "states-390/states.json", bytes);
        rejected(&ws, "states.json");
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}

#[test]
fn an_existing_state_directory_requires_its_inventory() {
    let (_dir, ws) = workspace();
    std::fs::create_dir_all(ws.capture_dir().join(STATES_DIR)).unwrap();
    rejected(&ws, "states.json");
}

#[test]
fn non_file_inventories_are_not_empty_studies() {
    for name in [ROUTES_FILE, "states/states.json"] {
        let (_dir, ws) = workspace();
        std::fs::create_dir_all(ws.capture_dir().join(name)).unwrap();
        rejected(&ws, name);
    }
}

#[test]
fn an_oversized_inventory_is_reported_without_rewriting_it() {
    for (name, json) in [(ROUTES_FILE, "[]"), ("states/states.json", r#"{"states":[]}"#)] {
        let (_dir, ws) = workspace();
        let mut bytes = json.as_bytes().to_vec();
        bytes.resize((1 << 20) + 1, b' ');
        let path = record(&ws, name, &bytes);
        rejected(&ws, name);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}

#[test]
fn the_check_reports_invalid_inventory_before_replacing_earlier_results() {
    let (_dir, ws) = workspace();
    record(&ws, ROUTES_FILE, b"{broken");
    std::fs::create_dir_all(ws.app_dir()).unwrap();
    std::fs::write(ws.app_dir().join("index.html"), "<!doctype html><p>Existing build</p>").unwrap();
    let saved = ws.checks_dir().join("round-1/previous.txt");
    std::fs::create_dir_all(saved.parent().unwrap()).unwrap();
    std::fs::write(&saved, b"previous comparison").unwrap();
    let error = check_worker(&ws, false).run_checks(&ws, &WebsiteKind, 1).unwrap_err();
    assert!(error.message.contains(ROUTES_FILE), "{error:?}");
    assert_eq!(std::fs::read(saved).unwrap(), b"previous comparison");
}

#[cfg(unix)]
#[test]
fn a_dangling_route_link_is_not_an_absent_optional_inventory() {
    let (_dir, ws) = workspace();
    std::fs::create_dir_all(ws.capture_dir()).unwrap();
    let path = ws.capture_dir().join(ROUTES_FILE);
    std::os::unix::fs::symlink("missing-routes.json", &path).unwrap();
    rejected(&ws, ROUTES_FILE);
    assert!(std::fs::symlink_metadata(path).unwrap().file_type().is_symlink());
}

#[test]
fn absent_optional_records_and_empty_recorded_states_still_work() {
    let (_dir, ws) = workspace();
    assert_eq!(brief(&ws), HOW_TO_RUN);
    record(&ws, "states/states.json", br#"{"width":1440,"states":[]}"#);
    assert_eq!(brief(&ws), HOW_TO_RUN);
    let checks = check_worker(&ws, false).run_checks(&ws, &WebsiteKind, 1).unwrap();
    assert_eq!(checks.len(), 1);
    assert!(!checks[0].passed && checks[0].note.contains("No built site"));
}

#[test]
fn a_route_inventory_at_the_byte_limit_is_accepted() {
    let (_dir, ws) = workspace();
    let mut bytes = b"[]".to_vec();
    bytes.resize(1 << 20, b' ');
    record(&ws, ROUTES_FILE, &bytes);
    assert_eq!(brief(&ws), HOW_TO_RUN);
}

#[test]
fn recorded_pages_and_pictured_controls_remain_in_the_brief() {
    let (_dir, ws) = workspace();
    record(&ws, ROUTES_FILE, br#"[{"path":"/pricing","slug":"pricing","url":"https://example.test/pricing"}]"#);
    record(&ws, "states/states.json", br#"{"width":1440,"states":[{"kind":"menu","label":"Products","file":"state-1.png"},{"kind":"dialog","label":"Sign in","changed":["address changed"]}]}"#);
    let note = brief(&ws);
    assert!(note.contains("/pricing (capture/routes/pricing/)"), "{note}");
    assert!(note.contains("states/state-1.png (Products, menu, 1440 px)"), "{note}");
    assert!(!note.contains("can't read") && !note.contains("Sign in"), "unpictured controls stay out of the comparison: {note}");
}
