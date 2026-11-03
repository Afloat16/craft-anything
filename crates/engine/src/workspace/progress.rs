//! File progress kept between builder turns and runs. File contents, rather than timestamps,
//! decide whether the app changed; a successful write of identical bytes is not progress.

use super::{OWN_NAMES, Workspace, rebuilt};
use crate::CancelToken;
use craft_anything_evidence::{ErrorCode, ToolError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

/// A builder gets two consecutive turns without changing the app before it needs attention.
pub const UNCHANGED_TURN_LIMIT: u32 = 2;
// Keep state inside an already reserved Craft directory. A project created by an older
// version may have its own app data named build-progress.json at the root.
pub const BUILD_PROGRESS_FILE: &str = "inbox/build-progress.json";
pub const BUILD_PROGRESS_VERSION: u32 = 1;
const MAX_RECORD_BYTES: u64 = 16 * 1024;
const MAX_ENTRIES: usize = 100_000;
const MAX_DEPTH: usize = 64;

const STEPS: &[crate::versioned::Step<Value>] = &[v0_to_v1];
const _: () = assert!(STEPS.len() == BUILD_PROGRESS_VERSION as usize);

fn v0_to_v1(doc: &mut Value) -> Result<(), String> {
    if doc.is_object() { Ok(()) } else { Err("the progress record isn't a JSON object".into()) }
}

/// The app as the builder last left it, and turns since it last changed. Only the rebuild run
/// writes this record. An older project without one has no counted turns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BuildProgress {
    pub version: u32,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub unchanged_turns: u32,
}

impl Default for BuildProgress {
    fn default() -> Self {
        Self { version: BUILD_PROGRESS_VERSION, fingerprint: None, unchanged_turns: 0 }
    }
}

impl BuildProgress {
    /// Read a project's progress. Broken or newer records stop the run rather than quietly
    /// giving it another budget. Version 0 has the same shape; version 1 adds the version number.
    pub fn load(root: &Path) -> Result<Self, ToolError> {
        let path = root.join(BUILD_PROGRESS_FILE);
        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(ToolError::io(path.display(), e)),
        };
        let mut bytes = Vec::new();
        file.take(MAX_RECORD_BYTES + 1).read_to_end(&mut bytes).map_err(|e| ToolError::io(path.display(), e))?;
        if bytes.len() as u64 > MAX_RECORD_BYTES {
            return Err(ToolError::new(ErrorCode::OverLimit, format!("{BUILD_PROGRESS_FILE} is too large to read.")));
        }
        let mut doc: Value = serde_json::from_slice(&bytes).map_err(|e| ToolError::invalid(format!("Couldn't read {BUILD_PROGRESS_FILE}: {e}")))?;
        let found = match doc.get("version") {
            None => 0,
            Some(v) => {
                v.as_u64().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| ToolError::invalid(format!("{BUILD_PROGRESS_FILE} has an invalid version.")))?
            }
        };
        if found > BUILD_PROGRESS_VERSION {
            return Err(ToolError::unsupported(format!("{BUILD_PROGRESS_FILE} was written by a newer Craft Anything, so this build is left alone."))
                .with_remediation("Update Craft Anything to continue this build."));
        }
        let version = crate::versioned::upgrade(&mut doc, found, 0, STEPS)
            .map_err(|e| ToolError::invalid(format!("Couldn't bring {BUILD_PROGRESS_FILE} up to date: {e}")))?;
        let obj = doc.as_object_mut().ok_or_else(|| ToolError::invalid(format!("{BUILD_PROGRESS_FILE} isn't a JSON object.")))?;
        obj.insert("version".into(), Value::from(version));
        serde_json::from_value(doc).map_err(|e| ToolError::invalid(format!("Couldn't read {BUILD_PROGRESS_FILE}: {e}")))
    }

    pub fn save(&self, root: &Path) -> Result<(), ToolError> {
        if self.version > BUILD_PROGRESS_VERSION {
            return Err(ToolError::unsupported("The newer build progress record can't be overwritten."));
        }
        // Recheck on disk: a run must not overwrite a newer record written since it started.
        Self::load(root)?;
        let mut current = self.clone();
        current.version = BUILD_PROGRESS_VERSION;
        let bytes = serde_json::to_vec_pretty(&current).map_err(|e| ToolError::internal(e.to_string()))?;
        crate::project::write_atomic(&root.join(BUILD_PROGRESS_FILE), &bytes)
    }

    /// A fresh request starts a new attempt; a plain retry keeps the existing count.
    pub fn reset(&mut self) {
        self.unchanged_turns = 0;
    }

    /// An edit made between turns or while the project was closed is progress too.
    pub fn before_turn(&mut self, fingerprint: String) {
        if self.fingerprint.as_deref() != Some(fingerprint.as_str()) {
            self.reset();
        }
        self.fingerprint = Some(fingerprint);
    }

    /// Count only completed builder turns. Checks and conversation records never count as a turn.
    pub fn after_turn(&mut self, fingerprint: String) {
        self.unchanged_turns = if self.fingerprint.as_deref() == Some(fingerprint.as_str()) { self.unchanged_turns.saturating_add(1) } else { 0 };
        self.fingerprint = Some(fingerprint);
    }

    pub fn stalled(&self) -> bool {
        self.unchanged_turns >= UNCHANGED_TURN_LIMIT
    }
}

/// The published schema of the progress record (`cargo xtask schemas`).
pub fn build_progress_schema() -> String {
    let schema = schemars::generate::SchemaSettings::draft2020_12().into_generator().into_root_schema_for::<BuildProgress>();
    serde_json::to_string_pretty(&schema).map(|s| s + "\n").unwrap_or_default()
}

impl Workspace {
    /// A stable digest of the app's file names and contents. Dependency and build caches follow
    /// the same exclusions as project copies; Craft's own records and git metadata are left out.
    /// Symlinks are recorded as links, never followed. Reads are streamed and cancellable.
    pub fn app_fingerprint(&self, stop: &CancelToken) -> Result<String, ToolError> {
        let root = self.app_dir();
        let mut hash = Sha256::new();
        hash.update(b"craft-build-files-v1\0");
        let mut visited = 0;
        fingerprint_tree(self, &root, &root, &mut hash, &mut visited, 0, stop)?;
        Ok(hex::encode(hash.finalize()))
    }
}

fn fingerprint_tree(
    ws: &Workspace,
    root: &Path,
    dir: &Path,
    hash: &mut Sha256,
    visited: &mut usize,
    depth: usize,
    stop: &CancelToken,
) -> Result<(), ToolError> {
    stop.check()?;
    if depth > MAX_DEPTH {
        return Err(ToolError::new(
            ErrorCode::OverLimit,
            format!("Can't check build progress: {} is nested more than {MAX_DEPTH} folders deep.", dir.display()),
        ));
    }
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| ToolError::io(dir.display(), e))? {
        stop.check()?;
        let entry = entry.map_err(|e| ToolError::io(dir.display(), e))?;
        let path = entry.path();
        let name = entry.file_name();
        if name == ".git" || (ws.manifest.app_in_root && dir == root && OWN_NAMES.iter().any(|n| name == *n)) {
            continue;
        }
        let kind = entry.file_type().map_err(|e| ToolError::io(path.display(), e))?;
        if kind.is_dir() && rebuilt(&path) {
            continue;
        }
        *visited = visited.saturating_add(1);
        if *visited > MAX_ENTRIES {
            return Err(ToolError::new(
                ErrorCode::OverLimit,
                format!("Can't check build progress: {} contains more than {MAX_ENTRIES} files and folders.", root.display()),
            ));
        }
        entries.push((path, kind));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (path, kind) in entries {
        stop.check()?;
        if kind.is_dir() {
            fingerprint_tree(ws, root, &path, hash, visited, depth + 1, stop)?;
            continue;
        }
        let relative = path.strip_prefix(root).map_err(|_| ToolError::internal("A build file is outside its app folder."))?;
        for component in relative.components() {
            let name = component.as_os_str().as_encoded_bytes();
            hash.update((name.len() as u64).to_le_bytes());
            hash.update(name);
        }
        hash.update(0u64.to_le_bytes());
        if kind.is_symlink() {
            hash.update(b"link\0");
            let target = std::fs::read_link(&path).map_err(|e| ToolError::io(path.display(), e))?;
            let target = target.as_os_str().as_encoded_bytes();
            hash.update((target.len() as u64).to_le_bytes());
            hash.update(target);
        } else if kind.is_file() {
            hash.update(b"file\0");
            hash.update(file_fingerprint(&path, stop)?);
        } else {
            return Err(ToolError::unsupported(format!("Can't check build progress for the special file {}.", path.display())));
        }
    }
    Ok(())
}

fn file_fingerprint(path: &Path, stop: &CancelToken) -> Result<[u8; 32], ToolError> {
    let mut file = std::fs::File::open(path).map_err(|e| ToolError::io(path.display(), e))?;
    let before = file.metadata().map_err(|e| ToolError::io(path.display(), e))?;
    // A growing log can't keep the scan running forever. A changed file is checked again on the
    // next run instead of being mistaken for an unchanged app.
    let mut reader = (&mut file).take(before.len());
    let mut bytes = 0u64;
    let mut hash = Sha256::new();
    // Executability changes what can be built or launched. Other permission and timestamp
    // changes alone do not count as progress. Non-Unix platforms have no execute mode bits.
    hash.update(executable_mode(&before).to_le_bytes());
    let mut buffer = [0u8; 64 * 1024];
    loop {
        stop.check()?;
        let n = reader.read(&mut buffer).map_err(|e| ToolError::io(path.display(), e))?;
        if n == 0 {
            break;
        }
        bytes = bytes.saturating_add(n as u64);
        hash.update(buffer.get(..n).ok_or_else(|| ToolError::internal("A file read exceeded its buffer."))?);
    }
    let after = file.metadata().map_err(|e| ToolError::io(path.display(), e))?;
    if bytes != before.len()
        || before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
        || executable_mode(&before) != executable_mode(&after)
    {
        return Err(ToolError::new(
            ErrorCode::FileChanged,
            format!("{} changed while checking build progress. Try again after its write finishes.", path.display()),
        ));
    }
    Ok(hash.finalize().into())
}

#[cfg(unix)]
fn executable_mode(metadata: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111
}

#[cfg(not(unix))]
fn executable_mode(_metadata: &std::fs::Metadata) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ExistingFiles, TargetRef};

    fn fingerprint(ws: &Workspace) -> String {
        ws.app_fingerprint(&CancelToken::default()).unwrap()
    }

    #[test]
    fn progress_counts_identical_bytes_and_resets_for_edits() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Notes", "A notes app", TargetRef::None).unwrap();
        std::fs::create_dir_all(ws.app_dir()).unwrap();
        let file = ws.app_dir().join("main.txt");
        std::fs::write(&file, "first").unwrap();
        let first = fingerprint(&ws);
        let mut progress = BuildProgress::load(&ws.root).unwrap();
        progress.before_turn(first.clone());
        std::fs::write(&file, "first").unwrap();
        progress.after_turn(fingerprint(&ws));
        assert_eq!(progress.unchanged_turns, 1);
        progress.save(&ws.root).unwrap();
        let mut progress = BuildProgress::load(&Workspace::open(&ws.root).unwrap().root).unwrap();
        progress.before_turn(fingerprint(&ws));
        progress.after_turn(fingerprint(&ws));
        assert!(progress.stalled());
        std::fs::write(&file, "other").unwrap();
        assert_ne!(first, fingerprint(&ws), "same length, different contents");
        progress.before_turn(fingerprint(&ws));
        assert_eq!(progress.unchanged_turns, 0);
        std::fs::remove_file(file).unwrap();
        progress.after_turn(fingerprint(&ws));
        assert_eq!(progress.unchanged_turns, 0, "deleting an app file is a change too");
    }

    #[test]
    fn root_apps_ignore_craft_records_and_caches_but_keep_runtime_assets() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("mine");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "the app").unwrap();
        let ws = Workspace::create_at(&root, "Mine", "A page", TargetRef::None, Some(ExistingFiles::Edit)).unwrap();
        let before = fingerprint(&ws);
        for dir in ["checks", "node_modules", "target", ".git"] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            std::fs::write(root.join(dir).join("changed.txt"), "changed outside the app's source").unwrap();
        }
        std::fs::write(root.join("feed.json"), "changed feed").unwrap();
        BuildProgress::default().save(&ws.root).unwrap();
        assert_eq!(before, fingerprint(&ws));
        std::fs::create_dir_all(root.join("assets/obj")).unwrap();
        std::fs::write(root.join("assets/obj/shape.obj"), "runtime model").unwrap();
        assert_ne!(before, fingerprint(&ws), "an obj asset folder is not a managed build cache");
    }

    #[test]
    fn progress_records_migrate_and_newer_or_broken_records_are_left_alone() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join(BUILD_PROGRESS_FILE);
        assert_eq!(BuildProgress::load(d.path()).unwrap(), BuildProgress::default());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, r#"{"fingerprint":"previous","unchanged_turns":2}"#).unwrap();
        let old = BuildProgress::load(d.path()).unwrap();
        assert_eq!(old.version, BUILD_PROGRESS_VERSION);
        assert!(old.stalled());
        old.save(d.path()).unwrap();
        assert!(std::fs::read_to_string(&path).unwrap().contains("\"version\": 1"));
        for bad in [r#"{"version":99,"future":true}"#, "not json", r#"{"version":-1}"#, r#"{"app_data":"not a Craft progress record"}"#] {
            std::fs::write(&path, bad).unwrap();
            assert!(BuildProgress::load(d.path()).is_err());
            assert!(old.save(d.path()).is_err());
            assert_eq!(std::fs::read_to_string(&path).unwrap(), bad);
        }
        std::fs::write(&path, vec![b' '; MAX_RECORD_BYTES as usize + 1]).unwrap();
        assert_eq!(BuildProgress::load(d.path()).unwrap_err().code, ErrorCode::OverLimit);
    }

    #[test]
    fn a_duplicate_has_a_new_attempt_and_progress_is_never_app_source() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("mine");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "the app").unwrap();
        let ws = Workspace::create_at(&root, "Mine", "A page", TargetRef::None, Some(ExistingFiles::Edit)).unwrap();
        let progress = BuildProgress { fingerprint: Some(fingerprint(&ws)), unchanged_turns: 2, ..BuildProgress::default() };
        progress.save(&ws.root).unwrap();
        let copy = ws.duplicate("Another attempt").unwrap();
        assert_eq!(BuildProgress::load(&copy.root).unwrap(), BuildProgress::default());
        assert!(!copy.app_dir().join(BUILD_PROGRESS_FILE).exists());
        assert_eq!(fingerprint(&ws), fingerprint(&copy));
        assert_eq!(BuildProgress::load(&ws.root).unwrap(), progress, "the original keeps its limit");
    }

    #[test]
    fn an_existing_root_build_progress_name_remains_the_persons_app_data() {
        for (data, directory) in
            [(r#"{"player":"saved"}"#, false), ("{}", false), (r#"{"version":1,"fingerprint":null,"unchanged_turns":2}"#, false), ("a runtime asset", true)]
        {
            let d = tempfile::tempdir().unwrap();
            let root = d.path().join("mine");
            std::fs::create_dir_all(&root).unwrap();
            std::fs::write(root.join("index.html"), "the app").unwrap();
            let ws = Workspace::create_at(&root, "Mine", "A page", TargetRef::None, Some(ExistingFiles::Edit)).unwrap();
            let before = fingerprint(&ws);
            // Simulate an existing in-place project from before progress was recorded. Even a
            // JSON object matching the new record's shape does not prove Craft owns it.
            let relative = if directory { "build-progress.json/state.txt" } else { "build-progress.json" };
            let app_file = root.join(relative);
            std::fs::create_dir_all(app_file.parent().unwrap()).unwrap();
            std::fs::write(&app_file, data).unwrap();
            assert!(!ws.is_own_record(&root.join("build-progress.json")));
            let with_data = fingerprint(&ws);
            assert_ne!(before, with_data, "the root name belongs to the app");
            let progress = BuildProgress { fingerprint: Some(with_data.clone()), unchanged_turns: 2, ..BuildProgress::default() };
            progress.save(&root).unwrap();
            assert_eq!(std::fs::read_to_string(&app_file).unwrap(), data, "save must not claim or migrate the root file");
            assert_eq!(fingerprint(&ws), with_data, "the inbox record is not app progress");
            std::fs::write(&app_file, format!("{data}\n")).unwrap();
            assert_ne!(fingerprint(&ws), with_data, "changes to that app file count");
            std::fs::write(&app_file, data).unwrap();
            let copy = ws.duplicate("Another attempt").unwrap();
            assert_eq!(std::fs::read_to_string(copy.app_dir().join(relative)).unwrap(), data);
            assert_eq!(BuildProgress::load(&copy.root).unwrap(), BuildProgress::default());
            assert!(!copy.app_dir().join(BUILD_PROGRESS_FILE).exists());
            ws.delete().unwrap();
            assert_eq!(std::fs::read_to_string(&app_file).unwrap(), data, "removing Craft's records must keep the app's data");
            assert!(!root.join(BUILD_PROGRESS_FILE).exists());
        }
    }

    #[test]
    fn inbox_messages_do_not_consume_the_progress_record() {
        use crate::inbox::{Answer, Inbox, RequestKind};
        let d = tempfile::tempdir().unwrap();
        let progress = BuildProgress { unchanged_turns: 2, ..BuildProgress::default() };
        progress.save(d.path()).unwrap();
        let inbox = Inbox::new(d.path().join("inbox"));
        assert!(inbox.pending().is_empty());
        let id = inbox.post(RequestKind::Question, "Continue?", "", Vec::new(), true, None).unwrap();
        assert_eq!(inbox.pending().len(), 1);
        inbox.answer(&id, &Answer { text: Some("Yes".into()), ..Default::default() }).unwrap();
        let answer = inbox.wait(&id, std::time::Duration::from_millis(100), &|| false).unwrap();
        assert_eq!(answer.text.as_deref(), Some("Yes"));
        assert!(inbox.pending().is_empty());
        assert_eq!(BuildProgress::load(d.path()).unwrap(), progress);
    }

    #[test]
    fn moving_the_project_folder_keeps_its_unchanged_turn_count() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Notes", "A notes app", TargetRef::None).unwrap();
        std::fs::create_dir_all(ws.app_dir()).unwrap();
        std::fs::write(ws.app_dir().join("index.html"), "the app").unwrap();
        let progress = BuildProgress { fingerprint: Some(fingerprint(&ws)), unchanged_turns: 2, ..BuildProgress::default() };
        progress.save(&ws.root).unwrap();
        let moved = d.path().join("moved-project");
        std::fs::rename(&ws.root, &moved).unwrap();
        let ws = Workspace::open(&moved).unwrap();
        let mut loaded = BuildProgress::load(&ws.root).unwrap();
        loaded.before_turn(fingerprint(&ws));
        assert_eq!(loaded, progress, "the fingerprint is relative to the app, not its original folder");
        assert!(loaded.stalled());
    }

    #[cfg(unix)]
    #[test]
    fn executable_mode_is_progress_but_other_permission_bits_are_not() {
        use std::os::unix::fs::PermissionsExt;
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Script", "A script", TargetRef::None).unwrap();
        std::fs::create_dir_all(ws.app_dir()).unwrap();
        let file = ws.app_dir().join("build.sh");
        std::fs::write(&file, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        let before = fingerprint(&ws);
        let mut progress = BuildProgress { fingerprint: Some(before.clone()), unchanged_turns: 2, ..BuildProgress::default() };
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        progress.before_turn(fingerprint(&ws));
        assert!(progress.stalled(), "read/write mode alone is not source progress");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o700)).unwrap();
        progress.before_turn(fingerprint(&ws));
        assert_ne!(progress.fingerprint.as_deref(), Some(before.as_str()));
        assert_eq!(progress.unchanged_turns, 0, "making the script runnable is progress");
    }

    #[test]
    fn deeply_nested_apps_stop_without_recursing_indefinitely() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Nested", "A nested app", TargetRef::None).unwrap();
        let mut dir = ws.app_dir();
        for _ in 0..=MAX_DEPTH {
            dir.push("d");
        }
        std::fs::create_dir_all(dir).unwrap();
        let error = ws.app_fingerprint(&CancelToken::default()).unwrap_err();
        assert_eq!(error.code, ErrorCode::OverLimit);
        assert!(error.message.contains("nested"));
    }

    #[test]
    fn the_progress_schema_is_current() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/schemas/build-progress.schema.json");
        let saved = std::fs::read_to_string(path).unwrap_or_default();
        assert_eq!(saved.replace("\r\n", "\n"), build_progress_schema());
    }

    #[cfg(unix)]
    #[test]
    fn links_are_not_followed_and_cancel_stops_the_scan() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Notes", "A notes app", TargetRef::None).unwrap();
        std::fs::create_dir_all(ws.app_dir()).unwrap();
        let outside = d.path().join("outside");
        std::fs::write(&outside, "one").unwrap();
        std::os::unix::fs::symlink(&outside, ws.app_dir().join("link")).unwrap();
        std::os::unix::fs::symlink(ws.app_dir(), ws.app_dir().join("cycle")).unwrap();
        let before = fingerprint(&ws);
        std::fs::write(outside, "two").unwrap();
        assert_eq!(before, fingerprint(&ws));
        let stop = CancelToken::default();
        stop.cancel();
        assert_eq!(ws.app_fingerprint(&stop).unwrap_err().code, ErrorCode::Cancelled);
    }
}
