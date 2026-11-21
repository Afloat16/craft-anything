//! Explicit recovery copies of a run's visible conversation. These files are never imported,
//! replaced or removed automatically: retaining one does not claim the main feed was saved.

use super::{FeedItem, LoadedFeed};
use craft_anything_evidence::{ErrorCode, ToolError, sha256_hex};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub const RECOVERY_ARCHIVE_DIR: &str = "feed-recovery";
const FORMAT: &str = "craft-anything.feed-recovery";
const VERSION: u32 = 1;
const SUFFIX: &str = ".recovery.json";
const MAX_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ENTRIES: usize = 256;
const MAX_NOTICES: usize = 16;
const NOTICE_MESSAGE: &str = "An earlier run kept a separate conversation recovery copy.";
const NOTICE_REMEDIATION: &str = "Open the copy to inspect its conversation. It has not been imported automatically, and may overlap the saved conversation.";

/// Version 1: the complete folded conversation, its loaded source's identity, and the
/// caller's recovery context. `metadata` contains `format`, `source` and `details`; source
/// records a decoded/damaged SHA-256 or an unreadable/missing state, never damaged bytes.
/// Details contain only information already kept in the conversation or manifest, not raw
/// tool events. Unknown versions and invalid records are left alone, not imported or removed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoverySnapshot {
    pub version: u32,
    pub items: Vec<FeedItem>,
    pub metadata: Value,
    #[serde(skip)]
    project: Option<Result<std::sync::Arc<super::transaction::ProjectIdentity>, ToolError>>,
}

impl RecoverySnapshot {
    pub fn from_loaded(feed: &LoadedFeed, details: Value) -> Self {
        let source = if let Some(digest) = &feed.decoded_source {
            json!({"state": "decoded", "sha256": digest})
        } else if let Some(bytes) = &feed.damaged {
            json!({"state": "damaged", "sha256": sha256_hex(bytes)})
        } else if let Some(error) = &feed.not_loaded {
            json!({"state": "unreadable", "error_code": format!("{:?}", error.code)})
        } else {
            json!({"state": "missing"})
        };
        Self {
            version: VERSION,
            items: feed.to_vec(),
            metadata: json!({"format": FORMAT, "source": source, "details": details}),
            project: Some(feed.project.clone()),
        }
    }

    fn recognized(&self) -> bool {
        let valid_source = self.metadata.get("source").is_some_and(|source| match source.get("state").and_then(Value::as_str) {
            Some("decoded" | "damaged") => source
                .get("sha256")
                .and_then(Value::as_str)
                .is_some_and(|digest| digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))),
            Some("unreadable") => source.get("error_code").and_then(Value::as_str).is_some_and(|code| !code.is_empty()),
            Some("missing") => true,
            _ => false,
        });
        self.version == VERSION
            && self.metadata.get("format").and_then(Value::as_str) == Some(FORMAT)
            && self.metadata.get("details").is_some_and(Value::is_object)
            && valid_source
    }
}

fn directory(root: &Path, create: bool) -> Result<Option<PathBuf>, ToolError> {
    directory_using(root, create, sync_directory)
}

fn directory_using(root: &Path, create: bool, mut sync: impl FnMut(&Path) -> Result<(), ToolError>) -> Result<Option<PathBuf>, ToolError> {
    let mut path = root.to_path_buf();
    for name in ["inbox", RECOVERY_ARCHIVE_DIR] {
        let parent = path.clone();
        path.push(name);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && !create => return Ok(None),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let mut builder = std::fs::DirBuilder::new();
                #[cfg(unix)]
                std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
                match builder.create(&path) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(ToolError::io(path.display(), error)),
                }
                std::fs::symlink_metadata(&path).map_err(|error| ToolError::io(path.display(), error))?
            }
            Err(error) => return Err(ToolError::io(path.display(), error)),
        };
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(ToolError::invalid(format!("{} isn't a regular conversation-recovery folder.", path.display())));
        }
        if create {
            // A previous attempt may have created this entry and failed before its parent was synced.
            sync(&parent)?;
        }
    }
    Ok(Some(path))
}

#[cfg(unix)]
pub(super) fn sync_directory(path: &Path) -> Result<(), ToolError> {
    std::fs::File::open(path).and_then(|file| file.sync_all()).map_err(|error| ToolError::io(path.display(), error))
}

#[cfg(not(unix))]
pub(super) fn sync_directory(_path: &Path) -> Result<(), ToolError> {
    Ok(())
}

struct CappedWriter<'a> {
    file: &'a mut std::fs::File,
    remaining: u64,
}

impl std::io::Write for CappedWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let length = u64::try_from(bytes.len()).map_err(|_| std::io::Error::other("The recovery copy is too large."))?;
        if length > self.remaining {
            return Err(std::io::Error::other("The conversation recovery copy exceeds 64 MiB."));
        }
        let written = self.file.write(bytes)?;
        self.remaining = self.remaining.saturating_sub(written as u64);
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

/// Keep a distinct owner-private copy, bounded to 64 MiB. Success means the complete file was
/// synced, and on Unix its directory and ancestor directory entries were synced too. Even a
/// partial failed write is left alone; discovery cannot mistake it for a complete record.
/// Never touches the main feed.
pub fn write_recovery_snapshot(root: &Path, snapshot: &RecoverySnapshot) -> Result<PathBuf, ToolError> {
    write_recovery_snapshot_using(root, snapshot, sync_directory)
}

fn write_recovery_snapshot_using(root: &Path, snapshot: &RecoverySnapshot, mut sync: impl FnMut(&Path) -> Result<(), ToolError>) -> Result<PathBuf, ToolError> {
    if !snapshot.recognized() {
        return Err(ToolError::new(ErrorCode::NotSupported, "This conversation recovery-copy format isn't supported."));
    }
    let _guard = super::lock_project(root)?;
    if let Some(project) = &snapshot.project {
        project.as_ref().map_err(Clone::clone)?.check(root)?;
    }
    let directory = directory_using(root, true, &mut sync)?.ok_or_else(|| ToolError::internal("The conversation-recovery folder wasn't created."))?;
    static NEXT: AtomicU64 = AtomicU64::new(0);
    for _ in 0..64 {
        let name = format!("conversation-{}-{}-{}{SUFFIX}", crate::inbox::now_ms(), std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
        let path = directory.join(name);
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
        let mut file = match options.open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(ToolError::io(path.display(), error)),
        };
        serde_json::to_writer(CappedWriter { file: &mut file, remaining: MAX_BYTES }, snapshot).map_err(|error| ToolError::io(path.display(), error.into()))?;
        file.sync_all().map_err(|error| ToolError::io(path.display(), error))?;
        sync(&directory)?;
        return Ok(path);
    }
    Err(ToolError::new(ErrorCode::FileError, "Couldn't reserve a new conversation recovery copy.")
        .with_remediation("Keep the existing recovery copies and retry."))
}

fn recovery_filename(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.starts_with("conversation-") && name.ends_with(SUFFIX))
}

/// Earlier writers saved these display rows as errors. Match their complete generated shape,
/// including paths from before a project move, without removing ordinary errors with a log.
pub(super) fn is_recovery_notice(item: &FeedItem) -> bool {
    let FeedItem::Error { message, remediation: Some(remediation), log: Some(path) } = item else { return false };
    message == NOTICE_MESSAGE
        && remediation == NOTICE_REMEDIATION
        && recovery_filename(path)
        && path.parent().is_some_and(|directory| {
            directory.file_name().is_some_and(|name| name == RECOVERY_ARCHIVE_DIR)
                && directory.parent().and_then(Path::file_name).is_some_and(|name| name == "inbox")
        })
}

/// Read historical recovery notices for a display snapshot. Call when loading a view, not
/// while polling it, and keep the returned rows separate from the feed passed to `save`.
pub fn recovery_notices(root: &Path) -> Vec<FeedItem> {
    let mut items = Vec::new();
    append_recovery_notices(root, &mut items);
    items
}

fn read_snapshot(path: &Path, remaining: &mut u64) -> Option<RecoverySnapshot> {
    let before = std::fs::symlink_metadata(path).ok()?;
    if !before.is_file() || before.file_type().is_symlink() || before.len() > *remaining {
        return None;
    }
    let (mut file, opened) = craft_anything_platform::fs::open_regular_file(path).ok()?;
    // Reject observed path replacements, including a link installed after the directory walk.
    let current = std::fs::symlink_metadata(path).ok()?;
    if !current.is_file() || current.file_type().is_symlink() || current.len() != opened.len() || current.modified().ok() != opened.modified().ok() {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        if current.dev() != opened.dev() || current.ino() != opened.ino() {
            return None;
        }
    }
    let limit = (*remaining).min(MAX_BYTES);
    let mut bytes = Vec::new();
    let read = (&mut file).take(limit).read_to_end(&mut bytes);
    *remaining = remaining.saturating_sub(bytes.len() as u64);
    read.ok()?;
    let mut extra = [0u8; 1];
    if file.read(&mut extra).ok()? != 0 {
        return None;
    }
    let snapshot: RecoverySnapshot = serde_json::from_slice(&bytes).ok()?;
    snapshot.recognized().then_some(snapshot)
}

/// Discover at most 256 directory entries, 16 valid records and 64 MiB in total. This is a
/// historical notice, not a statement that today's conversation failed to save. No feed lock
/// is taken and no contents are replayed; callers keep the loaded source digest unchanged.
pub(super) fn append_recovery_notices(root: &Path, items: &mut Vec<FeedItem>) {
    let Ok(Some(directory)) = directory(root, false) else { return };
    let Ok(entries) = std::fs::read_dir(directory) else { return };
    let mut remaining = MAX_BYTES;
    let mut shown = 0;
    for entry in entries.take(MAX_ENTRIES).flatten() {
        if shown >= MAX_NOTICES || remaining == 0 {
            break;
        }
        let path = entry.path();
        if !recovery_filename(&path) {
            continue;
        }
        if read_snapshot(&path, &mut remaining).is_none() {
            continue;
        }
        shown += 1;
        if !items.iter().any(|item| matches!(item, FeedItem::Error { log: Some(saved), .. } if saved == &path)) {
            items.push(FeedItem::Error { message: NOTICE_MESSAGE.into(), remediation: Some(NOTICE_REMEDIATION.into()), log: Some(path) });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feed::{FEED_FILE, FEED_VERSION, Saved, UserItem, for_display, load, record_answer, save};
    use crate::inbox::{Inbox, RequestKind};
    use crate::workspace::{TargetRef, Workspace};

    #[test]
    fn recovery_copies_are_private_distinct_and_preserve_the_loaded_source() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let original = br#"{"version":5,"items":[{"User":"The saved conversation"}]}"#;
        std::fs::write(ws.root.join(FEED_FILE), original).unwrap();
        let mut feed = load(&ws);
        feed.push(FeedItem::User(UserItem::Text("The unsaved words".into())));
        let snapshot = RecoverySnapshot::from_loaded(&feed, json!({"pending_event_count": 1}));
        let first = write_recovery_snapshot(&ws.root, &snapshot).unwrap();
        let second = write_recovery_snapshot(&ws.root, &snapshot).unwrap();
        assert_ne!(first, second);
        let bytes = std::fs::read(&first).unwrap();
        let kept: RecoverySnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(kept.metadata["source"]["sha256"], sha256_hex(original));
        assert_eq!(kept.items, feed.to_vec());
        assert_eq!(std::fs::read(&second).unwrap(), bytes);
        assert_eq!(std::fs::read(ws.root.join(FEED_FILE)).unwrap(), original);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(std::fs::metadata(&first).unwrap().permissions().mode() & 0o777, 0o600);
        }
        let mut shown = Vec::new();
        append_recovery_notices(&ws.root, &mut shown);
        append_recovery_notices(&ws.root, &mut shown);
        assert_eq!(shown.len(), 2, "each exact archive path gets one historical notice");
        assert!(shown.iter().all(|item| matches!(item, FeedItem::Error { message, .. } if message.contains("earlier run"))));
    }

    #[test]
    fn recovery_discovery_leaves_unknown_damaged_oversized_and_linked_entries_alone() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut snapshot = RecoverySnapshot::from_loaded(&load(&ws), json!({}));
        let valid = write_recovery_snapshot(&ws.root, &snapshot).unwrap();
        let directory = valid.parent().unwrap();
        let damaged = directory.join("conversation-damaged.recovery.json");
        std::fs::write(&damaged, b"Unrelated personal contents").unwrap();
        let unknown = directory.join("conversation-newer.recovery.json");
        snapshot.version = 2;
        let unknown_bytes = serde_json::to_vec(&snapshot).unwrap();
        std::fs::write(&unknown, &unknown_bytes).unwrap();
        assert_eq!(write_recovery_snapshot(&ws.root, &snapshot).unwrap_err().code, ErrorCode::NotSupported);
        let huge = directory.join("conversation-oversized.recovery.json");
        std::fs::File::create(&huge).unwrap().set_len(MAX_BYTES + 1).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&valid, directory.join("conversation-linked.recovery.json")).unwrap();
        let mut shown = Vec::new();
        append_recovery_notices(&ws.root, &mut shown);
        assert_eq!(shown.len(), 1);
        assert!(matches!(&shown[0], FeedItem::Error { log: Some(path), .. } if path == &valid));
        assert_eq!(std::fs::read(&damaged).unwrap(), b"Unrelated personal contents");
        assert_eq!(std::fs::read(&unknown).unwrap(), unknown_bytes);
        assert_eq!(std::fs::metadata(&huge).unwrap().len(), MAX_BYTES + 1);
        // Enforce the same serialization cap while writing, before reporting a kept copy.
        let capped = directory.join("partial");
        let mut file = std::fs::File::create(&capped).unwrap();
        assert!(std::io::Write::write_all(&mut CappedWriter { file: &mut file, remaining: 4 }, b"12345").is_err());
        assert_eq!(std::fs::metadata(capped).unwrap().len(), 0);
    }

    #[test]
    fn recovery_copy_does_not_replace_an_existing_folder_entry_or_follow_a_link() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let snapshot = RecoverySnapshot::from_loaded(&load(&ws), json!({}));
        std::fs::create_dir_all(ws.inbox_dir()).unwrap();
        let entry = ws.inbox_dir().join(RECOVERY_ARCHIVE_DIR);
        std::fs::write(&entry, b"Keep this existing file").unwrap();
        assert!(write_recovery_snapshot(&ws.root, &snapshot).is_err());
        assert_eq!(std::fs::read(&entry).unwrap(), b"Keep this existing file");
        #[cfg(unix)]
        {
            let external = d.path().join("elsewhere");
            std::fs::create_dir(&external).unwrap();
            std::fs::remove_file(&entry).unwrap();
            std::os::unix::fs::symlink(&external, &entry).unwrap();
            assert!(write_recovery_snapshot(&ws.root, &snapshot).is_err());
            assert_eq!(std::fs::read_dir(external).unwrap().count(), 0);
        }
    }

    #[test]
    fn answering_a_question_does_not_persist_recovery_notices() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let snapshot = RecoverySnapshot::from_loaded(&load(&ws), json!({}));
        let archive = write_recovery_snapshot(&ws.root, &snapshot).unwrap();
        assert_eq!(for_display(&ws).iter().filter(|item| is_recovery_notice(item)).count(), 1);

        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(RequestKind::Question, "Which colour?", "", vec![], true, None).unwrap();
        let request = inbox.pending().into_iter().find(|request| request.id == id).unwrap();
        record_answer(&ws, &request, "Blue").unwrap();
        let saved: Saved = serde_json::from_slice(&std::fs::read(ws.root.join(FEED_FILE)).unwrap()).unwrap();
        assert!(saved.items.iter().all(|item| !is_recovery_notice(item)));
        assert!(load(&ws).iter().all(|item| !is_recovery_notice(item)));
        assert!(saved.items.iter().any(|item| matches!(item, FeedItem::Ask { answer: Some(answer), .. } if answer == "Blue")));

        let retained = d.path().join("retained-conversation.json");
        std::fs::rename(&archive, &retained).unwrap();
        assert!(for_display(&ws).iter().all(|item| !is_recovery_notice(item)));
        assert_eq!(serde_json::from_slice::<RecoverySnapshot>(&std::fs::read(retained).unwrap()).unwrap().items, snapshot.items);
    }

    #[test]
    fn legacy_saved_notices_are_removed_without_losing_ordinary_errors() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let archive = ws.inbox_dir().join(RECOVERY_ARCHIVE_DIR).join("conversation-1-2-3.recovery.json");
        let notice = FeedItem::Error {
            message: NOTICE_MESSAGE.into(),
            remediation: Some(NOTICE_REMEDIATION.into()),
            log: Some(d.path().join("previous-project").join("inbox").join(RECOVERY_ARCHIVE_DIR).join("conversation-1-2-3.recovery.json")),
        };
        let expected = vec![
            FeedItem::User(UserItem::Text(NOTICE_MESSAGE.into())),
            FeedItem::Agent { text: NOTICE_MESSAGE.into(), streaming: false },
            FeedItem::Error { message: "The builder stopped.".into(), remediation: Some(NOTICE_REMEDIATION.into()), log: Some(archive.clone()) },
            FeedItem::Error { message: NOTICE_MESSAGE.into(), remediation: Some("Inspect the builder's failure.".into()), log: Some(archive.clone()) },
            FeedItem::Error { message: NOTICE_MESSAGE.into(), remediation: Some(NOTICE_REMEDIATION.into()), log: Some(ws.root.join("build.log")) },
            FeedItem::Error {
                message: NOTICE_MESSAGE.into(),
                remediation: Some(NOTICE_REMEDIATION.into()),
                log: Some(ws.inbox_dir().join(RECOVERY_ARCHIVE_DIR).join("personal.json")),
            },
        ];
        let mut old_items = vec![notice];
        old_items.extend(expected.clone());
        let old = Saved { version: FEED_VERSION, items: old_items, recovery: None };
        let path = ws.root.join(FEED_FILE);
        std::fs::write(&path, serde_json::to_vec(&old).unwrap()).unwrap();

        let mut loaded = load(&ws);
        assert_eq!(loaded.to_vec(), expected);
        assert_eq!(for_display(&ws), expected);
        save(&ws.root, &mut loaded).unwrap();
        let saved: Saved = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(saved.items, expected);
    }

    #[cfg(unix)]
    #[test]
    fn recovery_creation_syncs_both_parent_entries_and_the_archive_directory() {
        let d = tempfile::tempdir().unwrap();
        let root = Workspace::create(d.path(), "Project", "Test", TargetRef::None).unwrap().root;
        let snapshot = RecoverySnapshot {
            version: VERSION,
            items: vec![],
            metadata: json!({"format": FORMAT, "source": {"state": "missing"}, "details": {}}),
            project: None,
        };
        let mut synced = Vec::new();
        let archive = write_recovery_snapshot_using(&root, &snapshot, |path| {
            synced.push(path.to_path_buf());
            sync_directory(path)
        })
        .unwrap();
        assert_eq!(synced, vec![root.clone(), root.join("inbox"), root.join("inbox").join(RECOVERY_ARCHIVE_DIR)]);
        assert!(archive.is_file());
    }

    #[cfg(unix)]
    #[test]
    fn parent_sync_failures_stop_archiving_and_are_retried_for_existing_directories() {
        for failed_component in [None, Some("inbox")] {
            let d = tempfile::tempdir().unwrap();
            let root = Workspace::create(d.path(), "Project", "Test", TargetRef::None).unwrap().root;
            let snapshot = RecoverySnapshot {
                version: VERSION,
                items: vec![],
                metadata: json!({"format": FORMAT, "source": {"state": "missing"}, "details": {}}),
                project: None,
            };
            let failed = failed_component.map_or_else(|| root.clone(), |name| root.join(name));
            for _ in 0..2 {
                let error = write_recovery_snapshot_using(&root, &snapshot, |path| {
                    if path == failed.as_path() {
                        Err(ToolError::io(path.display(), std::io::Error::other("injected parent synchronization failure")))
                    } else {
                        sync_directory(path)
                    }
                })
                .unwrap_err();
                assert!(error.message.contains("injected parent synchronization failure"));
                let archive = root.join("inbox").join(RECOVERY_ARCHIVE_DIR);
                assert!(!archive.exists() || std::fs::read_dir(archive).unwrap().next().is_none());
            }
            let mut synced = Vec::new();
            let archive = write_recovery_snapshot_using(&root, &snapshot, |path| {
                synced.push(path.to_path_buf());
                sync_directory(path)
            })
            .unwrap();
            assert_eq!(synced, vec![root.clone(), root.join("inbox"), root.join("inbox").join(RECOVERY_ARCHIVE_DIR)]);
            assert!(archive.is_file());
        }
    }

    #[test]
    fn recovery_does_not_recreate_a_deleted_projects_inbox() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Project", "Test", TargetRef::None).unwrap();
        let root = ws.root.clone();
        let personal = root.join("personal.txt");
        std::fs::write(&personal, b"Keep these words").unwrap();
        let snapshot = RecoverySnapshot::from_loaded(&load(&ws), json!({}));
        ws.delete().unwrap();
        assert!(write_recovery_snapshot(&root, &snapshot).is_err());
        assert!(!root.join("inbox").exists());
        assert_eq!(std::fs::read(personal).unwrap(), b"Keep these words");
    }

    #[test]
    fn loaded_writers_and_archives_reject_a_recreated_project_at_the_same_path() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "Project", "Old project", TargetRef::None).unwrap();
        let root = ws.root.clone();
        let created_ms = ws.manifest.created_ms;
        let personal = root.join("personal.txt");
        std::fs::write(&personal, b"Keep these words").unwrap();
        let mut old = load(&ws);
        let snapshot = RecoverySnapshot::from_loaded(&old, json!({}));
        ws.delete().unwrap();
        let mut new = Workspace::create_at(&root, "New project", "New words", TargetRef::None, Some(crate::workspace::ExistingFiles::Edit)).unwrap();
        new.manifest.created_ms = created_ms.saturating_add(1);
        new.save().unwrap();
        old.push(FeedItem::User(UserItem::Text("Stale words".into())));
        assert_eq!(save(&root, &mut old).unwrap_err().code, ErrorCode::FileChanged);
        assert_eq!(write_recovery_snapshot(&root, &snapshot).unwrap_err().code, ErrorCode::FileChanged);
        assert!(!root.join(FEED_FILE).exists());
        assert!(!root.join("inbox").join(RECOVERY_ARCHIVE_DIR).exists());
        assert_eq!(load(&new).to_vec(), vec![FeedItem::User(UserItem::Text("New words".into()))]);
        assert_eq!(std::fs::read(personal).unwrap(), b"Keep these words");
    }
}
