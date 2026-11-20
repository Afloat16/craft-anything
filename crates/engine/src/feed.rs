//! A project's conversation, as data: what the person asked, what the builder did and said, its
//! checklist, questions and checks. It's kept in the project folder (`feed.json`); a rebuild run
//! appends to it as it goes (`ai::rebuild::feed`), and every client reads the same file.

use crate::inbox::{DeliveryState, Inbox, Request};
use crate::workspace::{CheckResult, Status, Step, Workspace};
use craft_anything_evidence::{ErrorCode, ToolError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::io::Write as _;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

mod backups;
pub(crate) use backups::owned_backups;

/// Where the conversation is kept in the project folder.
pub const FEED_FILE: &str = "feed.json";

/// Recovery copies are project records too: the writer and workspace ownership rules share
/// this finite set so copies are protected and kept out of the app. Deletion also requires a
/// matching ownership receipt; a legacy personal file with the same name is left alone.
pub(crate) const DAMAGED_BACKUP_NAMES: &[&str] = &[
    "feed.json.broken",
    "feed.json.broken.1",
    "feed.json.broken.2",
    "feed.json.broken.3",
    "feed.json.broken.4",
    "feed.json.broken.5",
    "feed.json.broken.6",
    "feed.json.broken.7",
    "feed.json.broken.8",
    "feed.json.broken.9",
    "feed.json.broken.10",
    "feed.json.broken.11",
    "feed.json.broken.12",
    "feed.json.broken.13",
    "feed.json.broken.14",
    "feed.json.broken.15",
];

/// The format written today (6 retains the history projected during recovery; 5 marks open plan
/// steps stopped at a run's end; 4 saves a dismissed
/// plan card with it; 3 saved a prompt's pictures); files without a version are the earlier bare
/// list. An older Craft Anything won't rewrite a file it can't read (`save` refuses a newer version).
pub const FEED_VERSION: u32 = 6;

/// What the person sent: as the feed shows it, and the pictures attached to it (their copies in
/// the project's `attachments/`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserItem {
    /// No pictures: saved as a plain string, as every earlier feed was.
    Text(String),
    WithPictures {
        text: String,
        pictures: Vec<PathBuf>,
    },
}

impl UserItem {
    pub fn new(text: String, pictures: Vec<PathBuf>) -> Self {
        if pictures.is_empty() { UserItem::Text(text) } else { UserItem::WithPictures { text, pictures } }
    }

    pub fn text(&self) -> &str {
        match self {
            UserItem::Text(text) | UserItem::WithPictures { text, .. } => text,
        }
    }

    /// The copies in the project's `attachments/`, in the order they were attached.
    pub fn pictures(&self) -> &[PathBuf] {
        match self {
            UserItem::Text(_) => &[],
            UserItem::WithPictures { pictures, .. } => pictures,
        }
    }
}

impl From<String> for UserItem {
    fn from(text: String) -> Self {
        UserItem::Text(text)
    }
}

/// One step of the builder's own checklist (from its TodoWrite list).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanStep {
    pub text: String,
    /// What it says while doing this step ("Drawing the map").
    pub doing: String,
    pub status: PlanStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Todo,
    Doing,
    Done,
    /// A run ended with the step still open: stopped, not done (#504).
    Stopped,
}

/// One entry in a project's activity feed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedItem {
    /// The builder's checklist, kept up to date in place (shown pinned above the prompt box).
    Plan(Vec<PlanStep>),
    User(UserItem),
    Step {
        step: Step,
        text: String,
    },
    /// The agent's words (streamed).
    Agent {
        text: String,
        streaming: bool,
    },
    /// A run of tool calls, collapsed in Simple mode: (tool, finished ok?).
    Work {
        tools: Vec<(String, Option<bool>)>,
        notes: Vec<String>,
    },
    /// A question or approval; `answer` is the person's reply once given.
    Ask {
        request: Request,
        answer: Option<String>,
    },
    Check(CheckResult),
    Error {
        message: String,
        remediation: Option<String>,
        /// A log with everything known about the failure, to open and inspect.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        log: Option<std::path::PathBuf>,
    },
    /// The person paused the run (not a failure); a message or Continue carries on.
    Paused,
    /// The person folded the plan card above the prompt box (#504). It marks the plan before it as
    /// hidden; the steps are kept, and a later run's checklist shows again.
    PlanDismissed,
}

/// The plan card to show: the checklist, unless the person folded it after it was last updated.
pub fn visible_plan(feed: &[FeedItem]) -> Option<&[PlanStep]> {
    let at = feed.iter().rposition(|i| matches!(i, FeedItem::Plan(_)))?;
    if feed[at + 1..].iter().any(|i| matches!(i, FeedItem::PlanDismissed)) {
        return None;
    }
    match &feed[at] {
        FeedItem::Plan(steps) => Some(steps.as_slice()),
        _ => None,
    }
}

/// Fold the plan card in a feed. Returns false when there's no plan to fold or it's folded already.
pub fn dismiss_plan(feed: &mut Vec<FeedItem>) -> bool {
    if visible_plan(feed).is_none() {
        return false;
    }
    feed.push(FeedItem::PlanDismissed);
    true
}

/// Fold the plan card in a project's saved conversation (the UI's write, while no run is going).
pub fn dismiss_plan_saved(ws: &Workspace) -> Result<bool, ToolError> {
    let mut feed = load(ws);
    let folded = dismiss_plan(&mut feed);
    if folded {
        save(&ws.root, &mut feed)?;
    }
    Ok(folded)
}

/// Mark a question or approval answered, with the words shown for the answer.
pub fn answered(feed: &mut [FeedItem], id: &str, shown: &str) {
    for item in feed {
        if let FeedItem::Ask { request, answer: a @ None } = item
            && request.id == id
        {
            *a = Some(shown.to_string());
        }
    }
}

/// The manifest fields that produced a recovery's history prefix. Later manifest updates must
/// not change the identity of that projected occurrence when a live writer replays its events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryProjection {
    source_sha256: String,
    pub request: String,
    pub checks: Vec<CheckResult>,
    pub last_reply: String,
    pub failure_note: Option<String>,
}

impl RecoveryProjection {
    fn from_workspace(ws: &Workspace, damaged: &[u8]) -> Self {
        let m = &ws.manifest;
        Self {
            source_sha256: craft_anything_evidence::sha256_hex(damaged),
            request: m.request.clone(),
            checks: m.checks.clone(),
            last_reply: m.last_reply.clone(),
            failure_note: (m.status == Status::NeedsYou && !m.note.is_empty()).then(|| m.note.clone()),
        }
    }
}

/// The saved file. Version 6 records a recovery's projected history; older files and the
/// original bare list have no such provenance and are read without inventing it.
#[derive(Serialize, Deserialize)]
struct Saved {
    version: u32,
    items: Vec<FeedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recovery: Option<RecoveryProjection>,
}

/// A conversation together with whether its saved contents were actually read. A display
/// fallback must keep that fact through edits: repairing the file later doesn't make the
/// fallback safe to write over it. Load it again after fixing a read error.
#[derive(Debug, Clone)]
pub struct LoadedFeed {
    items: Vec<FeedItem>,
    not_loaded: Option<ToolError>,
    damaged: Option<Vec<u8>>,
    decoded: bool,
    recovery: Option<RecoveryProjection>,
}

impl LoadedFeed {
    /// Whether loading found no read or decoding error. A confirmed absent file also starts
    /// a new conversation; recovery callers use `was_decoded` to require saved contents.
    pub fn is_readable(&self) -> bool {
        self.not_loaded.is_none() && self.damaged.is_none()
    }

    /// True only when this load actually decoded saved bytes, never for an absent-file fallback.
    pub fn was_decoded(&self) -> bool {
        self.decoded
    }

    /// A live writer may try a fresh load after a repair. The original fallback remains
    /// protected: this does not clear the error or make it directly writable.
    pub fn had_read_error(&self) -> bool {
        self.not_loaded.is_some()
    }

    pub fn recovery_projection(&self) -> Option<&RecoveryProjection> {
        self.recovery.as_ref()
    }

    /// Only the damaged bytes this writer actually loaded can identify its recovery. A plain
    /// read failure, or an unrelated old recovery, cannot authorize deleting saved history.
    pub fn was_recovered_from(&self, earlier: &LoadedFeed) -> bool {
        match (&self.recovery, &earlier.damaged) {
            (Some(projection), Some(damaged)) => projection.source_sha256 == craft_anything_evidence::sha256_hex(damaged),
            _ => false,
        }
    }

    /// A writer that has reconciled the projected occurrences keeps its replayed history.
    pub fn clear_recovery_projection(&mut self) {
        self.recovery = None;
    }

    /// Take the items for a read-only view. Writers keep the `LoadedFeed` and pass it to `save`.
    pub fn into_items(self) -> Vec<FeedItem> {
        self.items
    }
}

impl Deref for LoadedFeed {
    type Target = Vec<FeedItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for LoadedFeed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

const DAMAGED_NOTICE: &str = "This project's saved conversation couldn't be read. Showing what the project could recover.";

fn newer_version(bytes: &[u8]) -> bool {
    serde_json::from_slice::<Value>(bytes).ok().and_then(|v| v.get("version").and_then(Value::as_u64)).is_some_and(|v| v > u64::from(FEED_VERSION))
}

fn newer_error() -> ToolError {
    ToolError::new(ErrorCode::NotSupported, "This project's conversation was saved by a newer Craft Anything, so it isn't changed.")
        .with_remediation("Update Craft Anything and reopen the project to keep its conversation up to date.")
}

fn decode(bytes: &[u8]) -> Option<Saved> {
    serde_json::from_slice::<Saved>(bytes)
        .ok()
        .filter(|saved| saved.version <= FEED_VERSION)
        .or_else(|| serde_json::from_slice::<Vec<FeedItem>>(bytes).ok().map(|items| Saved { version: 0, items, recovery: None }))
}

/// Only a path with nothing at it is a new conversation. In particular, a dangling link is
/// still the person's file and must not be replaced with a fallback.
fn read_previous(path: &Path) -> Result<Option<Vec<u8>>, ToolError> {
    match std::fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(read_error) if read_error.kind() == std::io::ErrorKind::NotFound => match std::fs::symlink_metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(ToolError::io(path.display(), e)),
            Ok(_) => Err(ToolError::io(path.display(), read_error)),
        },
        Err(e) => Err(ToolError::io(path.display(), e)),
    }
}

/// The feed for a project: what was saved, else rebuilt from its manifest. Questions still
/// waiting in its inbox are always there.
pub fn load(ws: &Workspace) -> LoadedFeed {
    let saved = read_previous(&ws.root.join(FEED_FILE)).and_then(|bytes| match bytes {
        Some(bytes) if newer_version(&bytes) => Err(newer_error()),
        bytes => Ok(bytes),
    });
    let saved = match saved {
        Ok(None) => return LoadedFeed { items: history(ws), not_loaded: None, damaged: None, decoded: false, recovery: None },
        Err(error) => {
            let error = if error.remediation.is_some() {
                error
            } else {
                error.with_remediation("The original is unchanged. Make it readable, then reopen the project before saving or answering a question.")
            };
            let mut items = history(ws);
            items.push(FeedItem::Error {
                message: format!("This project's saved conversation couldn't be read: {}", error.message),
                remediation: error.remediation.clone(),
                log: None,
            });
            return LoadedFeed { items, not_loaded: Some(error), damaged: None, decoded: false, recovery: None };
        }
        Ok(Some(bytes)) => match decode(&bytes) {
            Some(saved) => saved,
            None => {
                let mut items = history(ws);
                items.push(FeedItem::Error {
                    message: DAMAGED_NOTICE.into(),
                    remediation: Some("The original is unchanged. A complete copy will be kept beside feed.json before saving a new conversation.".into()),
                    log: None,
                });
                let recovery = Some(RecoveryProjection::from_workspace(ws, &bytes));
                return LoadedFeed { items, not_loaded: None, damaged: Some(bytes), decoded: false, recovery };
            }
        },
    };
    let mut feed = saved.items;
    // Nothing is streaming any more after a restart.
    for item in &mut feed {
        if let FeedItem::Agent { streaming, .. } = item {
            *streaming = false;
        }
    }
    let shown: HashSet<String> = feed.iter().filter_map(|i| if let FeedItem::Ask { request, .. } = i { Some(request.id.clone()) } else { None }).collect();
    feed.extend(Inbox::new(ws.inbox_dir()).pending().into_iter().filter(|r| !shown.contains(&r.id)).map(|request| FeedItem::Ask { request, answer: None }));
    LoadedFeed { items: feed, not_loaded: None, damaged: None, decoded: true, recovery: saved.recovery }
}

/// A read-only conversation view, including retryable questions whose answer was saved but
/// couldn't yet be delivered. Writers use `load` so a save between recording and delivery
/// cannot erase the recorded answer.
pub fn for_display(ws: &Workspace) -> Vec<FeedItem> {
    let mut items = load(ws).into_items();
    show_pending(&mut items, &Inbox::new(ws.inbox_dir()));
    items
}

/// Offer undelivered answers for retry in a cloned display list, never in a writer's feed.
/// If the request expired, keep its saved words visible without claiming they were delivered.
pub fn show_pending(items: &mut [FeedItem], inbox: &Inbox) {
    let waiting: HashSet<String> = inbox.pending().into_iter().map(|request| request.id).collect();
    for item in items {
        if let FeedItem::Ask { request, answer } = item {
            let delivery = inbox.delivery_state(&request.id);
            if matches!(delivery, Ok(DeliveryState::Delivered)) {
                continue;
            }
            if waiting.contains(&request.id) {
                if let Some(shown) = answer.take() {
                    request.detail.push_str("\n\nYour saved reply has not been delivered. You can retry it:\n");
                    request.detail.push_str(&shown);
                }
            } else if let Some(shown) = answer
                && !matches!(delivery, Ok(DeliveryState::Untracked))
            {
                let reason = match delivery {
                    Err(error) => format!("The delivery record could not be read: {}", error.message),
                    _ => "This question is no longer waiting.".into(),
                };
                *item = FeedItem::Error {
                    message: format!("Your reply to \"{}\" was saved but has not been delivered.\n\nSaved reply:\n{shown}", request.title),
                    remediation: Some(format!("{reason} Keep these words and copy them into a new request from the builder.")),
                    log: None,
                };
            }
        }
    }
}

/// A feed rebuilt from what a project saved: the request, the checks, the agent's last words,
/// and anything still waiting for an answer.
pub fn history(ws: &Workspace) -> Vec<FeedItem> {
    let m = &ws.manifest;
    let mut feed = vec![FeedItem::User(UserItem::new(m.request.clone(), Vec::new()))];
    feed.extend(m.checks.iter().cloned().map(FeedItem::Check));
    if !m.last_reply.is_empty() {
        feed.push(FeedItem::Agent { text: m.last_reply.clone(), streaming: false });
    }
    if matches!(m.status, Status::NeedsYou) && !m.note.is_empty() {
        feed.push(FeedItem::Error { message: m.note.clone(), remediation: None, log: None });
    }
    feed.extend(Inbox::new(ws.inbox_dir()).pending().into_iter().map(|request| FeedItem::Ask { request, answer: None }));
    feed
}

/// Write the feed into the project folder (only one that exists; never creates one). A feed
/// saved by a newer Craft Anything is never written over (docs/file-formats.md). A read failure
/// also leaves the path unchanged; only a missing path can be created. A readable damaged
/// conversation is copied byte-for-byte to an unused backup before it can be replaced.
pub fn save(root: &Path, feed: &mut LoadedFeed) -> Result<(), ToolError> {
    if let Some(error) = &feed.not_loaded {
        return Err(error.clone());
    }
    if !root.is_dir() {
        return Err(ToolError::new(ErrorCode::FileError, "The project's folder isn't available, so its conversation wasn't saved.")
            .with_remediation("Restore the project folder, then reopen the project before retrying."));
    }
    let path = root.join(FEED_FILE);
    let previous = read_previous(&path)?;
    if previous.as_deref().is_some_and(newer_version) {
        return Err(newer_error());
    }
    if let Some(original) = &feed.damaged
        && previous.as_deref() != Some(original.as_slice())
    {
        return Err(ToolError::new(ErrorCode::FileChanged, "The saved conversation changed after it couldn't be read.")
            .with_remediation("Reopen the project to read the current conversation before retrying. Nothing was written."));
    }
    let mut items = feed.items.clone();
    if let Some(bytes) = previous.filter(|bytes| decode(bytes).is_none()) {
        let backup = keep_damaged(root, &bytes)?;
        let notice = FeedItem::Error {
            message: DAMAGED_NOTICE.into(),
            remediation: Some(format!(
                "The complete original is kept at {}. This conversation continues from the project's recoverable history.",
                backup.display()
            )),
            log: None,
        };
        if let Some(item) = items.iter_mut().find(|item| matches!(item, FeedItem::Error { message, .. } if message == DAMAGED_NOTICE)) {
            *item = notice;
        } else {
            items.insert(0, notice);
        }
    }
    let saved = Saved { version: FEED_VERSION, items, recovery: feed.recovery.clone() };
    let json = serde_json::to_vec(&saved).map_err(|e| ToolError::internal(e.to_string()))?;
    crate::project::write_atomic(&path, &json)?;
    feed.items = saved.items;
    feed.damaged = None;
    Ok(())
}

/// Every backup recovery can create. Deletion additionally verifies a generated receipt and
/// the current regular file's bytes, rather than taking ownership from its name alone.
fn damaged_backups(root: &Path) -> impl Iterator<Item = PathBuf> + '_ {
    DAMAGED_BACKUP_NAMES.iter().map(move |name| root.join(name))
}

/// Keep an exact copy without ever opening an existing backup for replacement. A failed copy
/// leaves the source untouched; a failed later save can try the next unused name.
fn keep_damaged(root: &Path, bytes: &[u8]) -> Result<PathBuf, ToolError> {
    for backup in damaged_backups(root) {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut file = match options.open(&backup) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(ToolError::io(backup.display(), error)),
        };
        let result = file.write_all(bytes).and_then(|()| file.sync_all());
        drop(file);
        if let Err(error) = result {
            let _ = std::fs::remove_file(&backup);
            return Err(ToolError::io(backup.display(), error));
        }
        if let Err(error) = backups::record(root, &backup, bytes) {
            let _ = std::fs::remove_file(&backup);
            return Err(error);
        }
        return Ok(backup);
    }
    Err(ToolError::new(
        ErrorCode::FileError,
        format!("The conversation wasn't saved: all {} backup names beside feed.json are already in use.", DAMAGED_BACKUP_NAMES.len()),
    )
    .with_remediation("Move an earlier .broken backup to another safe location, then retry. The original conversation is unchanged."))
}

/// Record an answer given while no run is going (straight into the saved feed). `request` is the
/// question, taken before it was answered: one the saved conversation doesn't have yet (posted
/// from outside a run) is added with its answer, rather than lost once it stops waiting (#176).
pub fn record_answer(ws: &Workspace, request: &Request, shown: &str) -> Result<(), ToolError> {
    let mut feed = load(ws);
    record_pending_answer(&mut feed, request, shown);
    save(&ws.root, &mut feed)
}

/// Record the latest answer to a request captured from the pending inbox, including a retry
/// after the earlier answer was saved but couldn't be delivered.
pub fn record_pending_answer(feed: &mut Vec<FeedItem>, request: &Request, shown: &str) {
    if let Some(FeedItem::Ask { answer, .. }) = feed.iter_mut().find(|item| matches!(item, FeedItem::Ask { request: r, .. } if r.id == request.id)) {
        *answer = Some(shown.to_owned());
    } else {
        feed.push(FeedItem::Ask { request: request.clone(), answer: Some(shown.to_owned()) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inbox::RequestKind;
    use crate::workspace::TargetRef;

    /// Seed a fixture directly; production writers can only save a value returned by `load`.
    fn save(root: &Path, items: &[FeedItem]) -> Result<(), ToolError> {
        super::save(root, &mut LoadedFeed { items: items.to_vec(), not_loaded: None, damaged: None, decoded: true, recovery: None })
    }

    #[test]
    fn only_decoded_saved_bytes_establish_a_recovery_base() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        let missing = load(&ws);
        assert!(missing.is_readable());
        assert!(!missing.was_decoded());
        std::fs::create_dir(&path).unwrap();
        let unreadable = load(&ws);
        assert!(!unreadable.was_decoded());
        assert!(unreadable.had_read_error());
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, b"damaged").unwrap();
        let damaged = load(&ws);
        assert!(!damaged.was_decoded());
        assert!(!damaged.is_readable());
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Actual saved words"}]}"#).unwrap();
        let decoded = load(&ws);
        assert!(decoded.was_decoded());
        assert!(decoded.is_readable());
        assert!(decoded.recovery_projection().is_none());
        std::fs::write(&path, br#"{"version":999,"items":[]}"#).unwrap();
        let future = load(&ws);
        assert!(!future.was_decoded());
        assert!(future.had_read_error());
    }

    #[test]
    fn recovery_projection_survives_a_later_manifest_update() {
        let directory = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(directory.path(), "x", "Original request", TargetRef::None).unwrap();
        ws.manifest.last_reply = "Turn A".into();
        std::fs::write(ws.root.join(FEED_FILE), b"damaged").unwrap();
        let mut recovered = load(&ws);
        ws.manifest.last_reply = "Turn B".into();
        super::save(&ws.root, &mut recovered).unwrap();
        let saved = load(&ws);
        assert!(saved.was_decoded());
        assert_eq!(saved.recovery_projection().unwrap().last_reply, "Turn A");
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A")));
    }

    #[test]
    fn old_feed_formats_keep_their_items_without_inventing_recovery_provenance() {
        for original in [br#"[{"User":"Saved before recovery metadata"}]"#.as_slice(), br#"{"version":5,"items":[{"User":"Saved before recovery metadata"}]}"#]
        {
            let directory = tempfile::tempdir().unwrap();
            let ws = Workspace::create(directory.path(), "x", "Fallback", TargetRef::None).unwrap();
            let path = ws.root.join(FEED_FILE);
            std::fs::write(&path, original).unwrap();
            let mut feed = load(&ws);
            let before = feed.items.clone();
            assert!(feed.was_decoded());
            assert!(feed.recovery_projection().is_none());
            super::save(&ws.root, &mut feed).unwrap();
            let updated: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            assert_eq!(updated["version"], FEED_VERSION);
            assert!(updated.get("recovery").is_none());
            assert_eq!(load(&ws).items, before);
        }
    }

    #[test]
    fn a_feed_that_was_not_loaded_cannot_replace_a_later_readable_file() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        let mut feed = load(&ws);
        std::fs::remove_dir(&path).unwrap();
        let original = br#"{"version":5,"items":[{"User":"Keep the actual conversation"}]}"#;
        std::fs::write(&path, original).unwrap();
        feed.push(FeedItem::Paused);
        assert!(super::save(&ws.root, &mut feed).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), original);
    }

    #[test]
    fn malformed_feed_is_kept_under_an_unused_name_and_shown_in_the_conversation() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        let original = b"{\"version\":5,\"items\":[\xff\x00";
        std::fs::write(&path, original).unwrap();
        let previous_backup = ws.root.join("feed.json.broken");
        std::fs::write(&previous_backup, b"an earlier conversation").unwrap();
        let mut feed = load(&ws);
        feed.push(FeedItem::Paused);
        super::save(&ws.root, &mut feed).unwrap();
        let backup = ws.root.join("feed.json.broken.1");
        assert_eq!(std::fs::read(&backup).unwrap(), original);
        assert_eq!(std::fs::read(&previous_backup).unwrap(), b"an earlier conversation");
        let saved = load(&ws);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Error { remediation: Some(message), .. } if message.contains(&backup.display().to_string()))));
        assert!(matches!(saved.last(), Some(FeedItem::Paused)));
    }

    #[test]
    fn a_failed_backup_keeps_the_original_and_can_be_retried() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        let original = b"{\"version\":";
        std::fs::write(&path, original).unwrap();
        for index in 0..16 {
            let backup = path.with_extension(if index == 0 { "json.broken".into() } else { format!("json.broken.{index}") });
            std::fs::write(backup, b"keep an earlier backup").unwrap();
        }
        let mut feed = load(&ws);
        let before = feed.items.clone();
        assert!(super::save(&ws.root, &mut feed).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), original);
        assert_eq!(feed.items, before, "a failed save must not claim that a backup was kept");
        let free = path.with_extension("json.broken.15");
        std::fs::remove_file(&free).unwrap();
        super::save(&ws.root, &mut feed).unwrap();
        assert_eq!(std::fs::read(&free).unwrap(), original);
        assert_eq!(std::fs::read(path.with_extension("json.broken")).unwrap(), b"keep an earlier backup");
        feed.push(FeedItem::Paused);
        super::save(&ws.root, &mut feed).unwrap();
        assert_eq!(std::fs::read(&free).unwrap(), original, "later saves keep the recovery copy");
    }

    #[test]
    fn a_damaged_feed_replaced_before_saving_must_be_loaded_again() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::write(&path, b"broken").unwrap();
        let mut feed = load(&ws);
        let repaired = br#"{"version":5,"items":[{"User":"Recovered elsewhere"}]}"#;
        std::fs::write(&path, repaired).unwrap();
        assert_eq!(super::save(&ws.root, &mut feed).unwrap_err().code, ErrorCode::FileChanged);
        assert_eq!(std::fs::read(&path).unwrap(), repaired);
        assert!(!path.with_extension("json.broken").exists());
    }

    #[test]
    fn an_answer_saved_before_inbox_delivery_stays_available_for_retry() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(RequestKind::Question, "Blue or red?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        record_answer(&ws, &question, "Blue").unwrap();
        // Delivery hasn't happened (or failed): reopening must still offer this request.
        assert!(for_display(&ws).iter().any(|item| matches!(item, FeedItem::Ask { request, answer: None } if request.id == id)));
        // A concurrent writer keeps the recorded answer during the delivery interval.
        let mut other = load(&ws);
        other.push(FeedItem::Paused);
        super::save(&ws.root, &mut other).unwrap();
        assert!(load(&ws).iter().any(|item| matches!(item, FeedItem::Ask { answer: Some(answer), .. } if answer == "Blue")));
        record_answer(&ws, &question, "Red").unwrap();
        inbox.answer(&id, &crate::inbox::Answer { choice: Some("Red".into()), ..Default::default() }).unwrap();
        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == id)).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Red")));
    }

    #[cfg(windows)]
    #[test]
    fn a_feed_open_without_sharing_is_preserved_after_the_handle_closes() {
        use std::os::windows::fs::OpenOptionsExt as _;
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Fallback", TargetRef::None).unwrap();
        let path = ws.root.join(FEED_FILE);
        let original = br#"{"version":5,"items":[{"User":"Keep the actual conversation"}]}"#;
        std::fs::write(&path, original).unwrap();
        let held = std::fs::OpenOptions::new().read(true).share_mode(0).open(&path).unwrap();
        assert!(std::fs::read(&path).is_err());
        let mut feed = load(&ws);
        assert!(super::save(&ws.root, &mut feed).is_err());
        drop(held);
        assert!(super::save(&ws.root, &mut feed).is_err(), "the fallback still wasn't loaded");
        assert_eq!(std::fs::read(&path).unwrap(), original);
        let mut reread = load(&ws);
        reread.push(FeedItem::Paused);
        super::save(&ws.root, &mut reread).unwrap();
        assert!(matches!(&load(&ws)[0], FeedItem::User(user) if user.text() == "Keep the actual conversation"));
    }

    #[test]
    fn file_formats_doc_lists_current_manifest_and_feed_versions() {
        let doc = include_str!("../../../docs/file-formats.md");
        let manifest = format!("`MANIFEST_VERSION` = {} ", crate::workspace::MANIFEST_VERSION);
        let feed = format!("`FEED_VERSION` = {} ", FEED_VERSION);
        assert!(doc.contains(&manifest), "docs/file-formats.md must list {manifest}");
        assert!(doc.contains(&feed), "docs/file-formats.md must list {feed}");
    }

    #[test]
    fn history_rebuilds_from_the_manifest() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "Clone it", TargetRef::None).unwrap();
        ws.manifest.checks.push(CheckResult { similarity: Some(70.0), ..CheckResult::new(CheckResult::WEB_VISUAL, 1, true, "") });
        ws.manifest.last_reply = "Built a page.".into();
        ws.save().unwrap();
        Inbox::new(ws.inbox_dir()).post(RequestKind::Question, "Blue or red?", "", vec![], true, None).unwrap();
        let f = history(&ws);
        assert!(matches!(&f[0], FeedItem::User(r) if r.text() == "Clone it"));
        assert!(matches!(f.last(), Some(FeedItem::Ask { answer: None, .. })), "pending questions come back");
        assert_eq!(f.len(), 4);
    }

    #[test]
    fn a_prompt_keeps_its_pictures_with_it() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Clone it", TargetRef::None).unwrap();
        let pictures = vec![ws.root.join("attachments/picture-1.png"), ws.root.join("attachments/picture-2.webp")];
        save(&ws.root, &[FeedItem::User(UserItem::new("Make it blue".into(), pictures.clone()))]).unwrap();
        let loaded = load(&ws);
        assert!(matches!(&loaded[0], FeedItem::User(u) if u.text() == "Make it blue" && u.pictures() == pictures.as_slice()));
        // A prompt with no pictures is still saved as the plain string older feeds hold.
        save(&ws.root, &[FeedItem::User(UserItem::new("Add tags".into(), Vec::new()))]).unwrap();
        let raw: Value = serde_json::from_slice(&std::fs::read(ws.root.join(FEED_FILE)).unwrap()).unwrap();
        assert_eq!(raw["items"][0]["User"], "Add tags");
        assert!(raw["items"][0]["User"].is_string());
    }

    /// #504: a plan card can be folded, the steps stay in the conversation, and a folded plan stays
    /// folded after a restart. A file from before folding (version 3) still shows its plan.
    #[test]
    fn a_folded_plan_stays_folded_and_its_steps_are_kept() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let steps = vec![PlanStep { text: "Lay out the arena".into(), doing: "Laying out the arena".into(), status: PlanStatus::Doing }];
        save(&ws.root, &[FeedItem::User(UserItem::Text("A game".into())), FeedItem::Plan(steps.clone())]).unwrap();
        assert!(visible_plan(&load(&ws)).is_some(), "the plan shows before it's folded");
        assert!(dismiss_plan_saved(&ws).unwrap(), "the plan folds");
        let folded = load(&ws);
        assert!(visible_plan(&folded).is_none(), "folded after a restart too");
        assert!(folded.iter().any(|i| matches!(i, FeedItem::Plan(p) if p == &steps)), "the steps are kept");
        assert!(!dismiss_plan_saved(&ws).unwrap(), "folding again changes nothing");
        save(&ws.root, &[FeedItem::User(UserItem::Text("Hi".into()))]).unwrap();
        assert!(!dismiss_plan_saved(&ws).unwrap(), "a conversation with no plan has nothing to fold");
        std::fs::write(ws.root.join(FEED_FILE), br#"{"version":3,"items":[{"Plan":[{"text":"Lay out","doing":"Laying out","status":"Doing"}]}]}"#).unwrap();
        assert!(visible_plan(&load(&ws)).is_some(), "a version 3 plan still shows");
    }

    /// #504: a step marked stopped when a run ended stays in the saved conversation (it's part of
    /// the plan card's data, not a display flourish).
    #[test]
    fn a_stopped_step_is_kept_across_saves() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let steps = vec![
            PlanStep { text: "Lay out the arena".into(), doing: "Laying out the arena".into(), status: PlanStatus::Done },
            PlanStep { text: "Add the snake".into(), doing: "Adding the snake".into(), status: PlanStatus::Stopped },
            PlanStep { text: "Keep score".into(), doing: "Keeping score".into(), status: PlanStatus::Todo },
        ];
        save(&ws.root, &[FeedItem::User(UserItem::Text("A game".into())), FeedItem::Plan(steps)]).unwrap();
        let loaded = load(&ws);
        assert!(matches!(visible_plan(&loaded), Some(p) if p.iter().any(|s| s.status == PlanStatus::Stopped)));
    }

    #[test]
    fn older_files_load_answers_are_kept_and_newer_files_are_left_alone() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Clone it", TargetRef::None).unwrap();
        save(&ws.root, &[FeedItem::User(UserItem::Text("Clone it".into())), FeedItem::Agent { text: "Done.".into(), streaming: true }]).unwrap();
        assert!(matches!(load(&ws).as_slice(), [FeedItem::User(_), FeedItem::Agent { streaming: false, .. }]), "nothing streams after a restart");
        let raw: Value = serde_json::from_slice(&std::fs::read(ws.root.join(FEED_FILE)).unwrap()).unwrap();
        assert_eq!(raw["version"], FEED_VERSION);
        // A feed saved before versioning (the bare list) still loads.
        std::fs::write(ws.root.join(FEED_FILE), br#"[{"User":"Old request"}]"#).unwrap();
        assert!(matches!(load(&ws).as_slice(), [FeedItem::User(r)] if r.text() == "Old request"));
        // Answers given with no run going are saved too.
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(RequestKind::Question, "Blue or red?", "", vec![], true, None).unwrap();
        let question = inbox.pending().into_iter().find(|r| r.id == id).unwrap();
        inbox.answer(&id, &crate::inbox::Answer { choice: Some("Blue".into()), ..Default::default() }).unwrap();
        record_answer(&ws, &question, "Blue").unwrap();
        assert!(std::fs::read_to_string(ws.root.join(FEED_FILE)).unwrap().contains("\"Blue\""));
        // #176: one the saved conversation never had (posted from outside a run, answered before
        // anything saved it) is kept with its answer, once, and still after the inbox forgets it.
        save(&ws.root, &[FeedItem::User(UserItem::Text("Clone it".into()))]).unwrap();
        let id = inbox.post(RequestKind::Question, "Which scope?", "", vec![], true, None).unwrap();
        let question = inbox.pending().into_iter().find(|r| r.id == id).unwrap();
        inbox.answer(&id, &crate::inbox::Answer { choice: Some("Minimal".into()), ..Default::default() }).unwrap();
        let _ = std::fs::remove_file(inbox.dir().join(format!("{id}.request.json")));
        let _ = std::fs::remove_file(inbox.dir().join(format!("{id}.answer.json")));
        record_answer(&ws, &question, "Minimal").unwrap();
        let asks = |f: &[FeedItem]| f.iter().filter(|i| matches!(i, FeedItem::Ask { request, answer: Some(a) } if request.id == id && a == "Minimal")).count();
        assert_eq!(asks(&load(&ws)), 1, "kept with its answer");
        record_answer(&ws, &question, "Minimal").unwrap();
        assert_eq!(asks(&load(&ws)), 1, "not twice");
        // One from a newer Craft Anything is shown as far as we can, never written over.
        let newer = br#"{"version":99,"items":[{"User":"From the future"}],"extra":1}"#;
        std::fs::write(ws.root.join(FEED_FILE), newer).unwrap();
        assert!(save(&ws.root, &[FeedItem::User(UserItem::Text("x".into()))]).is_err());
        assert_eq!(std::fs::read(ws.root.join(FEED_FILE)).unwrap(), newer);
    }

    /// A writable parent lets the atomic writer replace even a write-only feed. The read must
    /// actually fail before these tests exercise the save, rather than mocking the filesystem.
    #[cfg(unix)]
    fn deny_feed_read(path: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o200)).unwrap();
        match std::fs::read(path) {
            Err(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied);
                true
            }
            Ok(_) => {
                std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
                eprintln!("permission test skipped: this process can read a write-only file");
                false // Run as root: nothing is unreadable.
            }
        }
    }

    #[cfg(unix)]
    fn assert_unreadable_feed_is_preserved(version: u32) {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FEED_FILE);
        let original = serde_json::to_vec(&serde_json::json!({
            "version": version,
            "items": [{"User": "Keep the saved conversation"}],
            "extra": {"keep": true}
        }))
        .unwrap();
        std::fs::write(&path, &original).unwrap();
        if !deny_feed_read(&path) {
            return;
        }
        let replacement = [FeedItem::User(UserItem::Text("New message".into()))];
        let saved = save(dir.path(), &replacement);
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), original, "save must not replace a feed it could not read");
        assert_eq!(saved.unwrap_err().code, ErrorCode::NoPermission);
        // Restoring access allows a normal save, but never a downgrade from a newer version.
        if version > FEED_VERSION {
            assert_eq!(save(dir.path(), &replacement).unwrap_err().code, ErrorCode::NotSupported);
            assert_eq!(std::fs::read(&path).unwrap(), original);
        } else {
            save(dir.path(), &replacement).unwrap();
            let saved: Saved = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            assert_eq!(saved.items, replacement);
        }
    }

    #[cfg(unix)]
    #[test]
    fn save_keeps_an_unreadable_feed() {
        assert_unreadable_feed_is_preserved(FEED_VERSION);
    }

    #[cfg(unix)]
    #[test]
    fn save_keeps_an_unreadable_newer_feed() {
        assert_unreadable_feed_is_preserved(FEED_VERSION + 1);
    }

    #[cfg(unix)]
    #[test]
    fn record_answer_keeps_an_unreadable_feed_and_can_retry() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let ws = Workspace::create(dir.path(), "x", "Original request", TargetRef::None).unwrap();
        let conversation =
            vec![FeedItem::User(UserItem::Text("Original request".into())), FeedItem::Agent { text: "Keep this completed work.".into(), streaming: false }];
        save(&ws.root, &conversation).unwrap();
        let path = ws.root.join(FEED_FILE);
        let original = std::fs::read(&path).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(RequestKind::Question, "Blue or red?", "", vec![], true, None).unwrap();
        let question = inbox.pending().into_iter().find(|r| r.id == id).unwrap();
        inbox.answer(&id, &crate::inbox::Answer { choice: Some("Blue".into()), ..Default::default() }).unwrap();
        if !deny_feed_read(&path) {
            return;
        }
        let saved = record_answer(&ws, &question, "Blue");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), original, "the display fallback must not replace the saved conversation");
        assert_eq!(saved.unwrap_err().code, ErrorCode::NoPermission);
        record_answer(&ws, &question, "Blue").unwrap();
        let loaded = load(&ws);
        assert_eq!(&loaded[..conversation.len()], conversation.as_slice());
        assert_eq!(loaded.len(), conversation.len() + 1);
        assert!(matches!(loaded.last(), Some(FeedItem::Ask { request, answer: Some(answer) }) if request.id == id && answer == "Blue"));
    }

    #[cfg(unix)]
    #[test]
    fn save_leaves_a_dangling_feed_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FEED_FILE);
        let target = Path::new("missing-feed.json");
        std::os::unix::fs::symlink(target, &path).unwrap();
        assert_eq!(std::fs::read(&path).unwrap_err().kind(), std::io::ErrorKind::NotFound);
        let saved = save(dir.path(), &[FeedItem::Paused]);
        assert!(std::fs::symlink_metadata(&path).unwrap().file_type().is_symlink(), "save replaced the dangling link");
        assert_eq!(std::fs::read_link(&path).unwrap(), target);
        assert!(!dir.path().join(target).exists());
        assert_eq!(saved.unwrap_err().code, ErrorCode::NotFound);
    }

    #[cfg(unix)]
    #[test]
    fn save_leaves_a_looping_feed_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FEED_FILE);
        std::os::unix::fs::symlink(FEED_FILE, &path).unwrap();
        assert!(std::fs::read(&path).is_err());
        let saved = save(dir.path(), &[FeedItem::Paused]);
        assert!(std::fs::symlink_metadata(&path).unwrap().file_type().is_symlink(), "save replaced the looping link");
        assert_eq!(std::fs::read_link(&path).unwrap(), Path::new(FEED_FILE));
        assert_eq!(saved.unwrap_err().code, ErrorCode::FileError);
    }

    #[test]
    fn save_creates_absent_feeds_and_recovers_readable_malformed_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FEED_FILE);
        let conversation = [FeedItem::User(UserItem::Text("Keep building".into()))];
        assert_eq!(std::fs::symlink_metadata(&path).unwrap_err().kind(), std::io::ErrorKind::NotFound);
        save(dir.path(), &conversation).unwrap();
        let saved: Saved = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(saved.items, conversation);
        // A readable parse failure can recover only after its exact bytes have been kept.
        std::fs::write(&path, b"{\"version\":").unwrap();
        save(dir.path(), &conversation).unwrap();
        let saved: Saved = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(saved.version, FEED_VERSION);
        assert_eq!(std::fs::read(path.with_extension("json.broken")).unwrap(), b"{\"version\":");
        assert!(matches!(saved.items.first(), Some(FeedItem::Error { .. })));
        assert_eq!(&saved.items[1..], &conversation);
    }
}
