//! A run's side of the project conversation: folding workflow events into the feed and keeping
//! it saved as the run goes. The feed itself (its items and file) is `engine::feed`.

pub use craft_anything_engine::feed::*;

use super::activity::{Activity, ActivityFile};
use super::*;

/// Fold one workflow event into the feed.
pub fn apply(feed: &mut Vec<FeedItem>, e: &RunEvent) {
    let explained = matches!(feed.last(), Some(FeedItem::Error { .. }));
    apply_with_explanation(feed, e, explained);
}

/// A live run and its replay know which errors they supplied. A saved conversation's tail
/// alone cannot explain this run's failure, particularly when a duplicate Ask changes nothing.
fn apply_run_event(feed: &mut Vec<FeedItem>, e: &RunEvent, explained: &mut bool) {
    let already_explained = *explained && matches!(feed.last(), Some(FeedItem::Error { .. }));
    apply_with_explanation(feed, e, already_explained);
    *explained = matches!(feed.last(), Some(FeedItem::Error { .. }))
        && (already_explained || matches!(e, RunEvent::Agent(AgentEvent::Error { .. }) | RunEvent::Finished { ok: false, paused: false, .. }));
}

fn apply_with_explanation(feed: &mut Vec<FeedItem>, e: &RunEvent, explained: bool) {
    let end_stream = |feed: &mut Vec<FeedItem>| {
        if let Some(FeedItem::Agent { streaming, .. }) = feed.last_mut() {
            *streaming = false;
        }
    };
    match e {
        RunEvent::Step { step, text } => {
            end_stream(feed);
            feed.push(FeedItem::Step { step: *step, text: text.clone() });
        }
        RunEvent::Agent(AgentEvent::TextDelta { text }) => match feed.last_mut() {
            Some(FeedItem::Agent { text: t, streaming: true }) => t.push_str(text),
            _ => feed.push(FeedItem::Agent { text: text.clone(), streaming: true }),
        },
        // The builder's checklist: one list, updated in place.
        // Claude Code's TodoWrite, or our build_checklist (same shape) for API builders.
        RunEvent::Agent(AgentEvent::ToolStarted { tool, input, .. }) if tool == "TodoWrite" || tool == "build_checklist" => {
            if let Some(steps) = plan_from(input) {
                if feed.iter().any(|i| matches!(i, FeedItem::Plan(_))) && craft_anything_engine::feed::visible_plan(feed).is_none() {
                    // The person folded the last list (#504): a new list is a new card at the end,
                    // and the folded one goes.
                    feed.retain(|i| !matches!(i, FeedItem::Plan(_) | FeedItem::PlanDismissed));
                    feed.push(FeedItem::Plan(steps));
                } else {
                    match feed.iter_mut().find_map(|i| if let FeedItem::Plan(p) = i { Some(p) } else { None }) {
                        Some(p) => *p = steps,
                        None => feed.push(FeedItem::Plan(steps)),
                    }
                }
            }
        }
        RunEvent::Agent(AgentEvent::ToolStarted { tool, .. }) => {
            end_stream(feed);
            match feed.last_mut() {
                Some(FeedItem::Work { tools, .. }) => tools.push((tool.clone(), None)),
                _ => feed.push(FeedItem::Work { tools: vec![(tool.clone(), None)], notes: Vec::new() }),
            }
        }
        RunEvent::Agent(AgentEvent::ToolFinished { tool, ok, .. }) => {
            for item in feed.iter_mut().rev() {
                if let FeedItem::Work { tools, .. } = item
                    && let Some(slot) = tools.iter_mut().rev().find(|(t, done)| t == tool && done.is_none())
                {
                    slot.1 = Some(*ok);
                    break;
                }
            }
        }
        RunEvent::Agent(AgentEvent::Status { text }) => match feed.last_mut() {
            Some(FeedItem::Work { notes, .. }) => notes.push(text.clone()),
            _ => feed.push(FeedItem::Work { tools: Vec::new(), notes: vec![text.clone()] }),
        },
        // For the activity log; it doesn't interrupt what the builder is saying.
        RunEvent::Agent(AgentEvent::Output { .. }) => {}
        RunEvent::Agent(AgentEvent::Error { message, remediation, log }) => {
            end_stream(feed);
            feed.push(FeedItem::Error { message: message.clone(), remediation: remediation.clone(), log: log.clone() });
        }
        RunEvent::Agent(_) => end_stream(feed),
        RunEvent::Note { shown, pictures } => {
            end_stream(feed);
            feed.push(FeedItem::User(UserItem::new(shown.clone(), pictures.clone())));
        }
        RunEvent::Ask(request) => {
            if !feed.iter().any(|item| matches!(item, FeedItem::Ask { request: saved, .. } if saved.id == request.id)) {
                feed.push(FeedItem::Ask { request: request.clone(), answer: None });
            }
        }
        RunEvent::Checked(c) => feed.push(FeedItem::Check(c.clone())),
        RunEvent::Preview(_) => {}
        RunEvent::Finished { ok, message, paused } => {
            end_stream(feed);
            // #504: a run that ends with steps still open marks them stopped on the plan card,
            // instead of leaving them "Doing"; a later run's checklist updates the list again.
            if let Some(FeedItem::Plan(steps)) = feed.iter_mut().rev().find(|i| matches!(i, FeedItem::Plan(_))) {
                for step in steps {
                    if step.status == PlanStatus::Doing {
                        step.status = PlanStatus::Stopped;
                    }
                }
            }
            // An agent error already explains itself; don't say it twice.
            if *paused {
                feed.push(FeedItem::Paused);
            } else if !ok && !explained {
                feed.push(FeedItem::Error { message: message.clone(), remediation: None, log: None });
            }
        }
    }
}

/// The steps of a TodoWrite call (`{todos: [{content, activeForm, status}]}`).
pub fn plan_from(input: &Value) -> Option<Vec<PlanStep>> {
    input
        .get("todos")?
        .as_array()?
        .iter()
        .map(|t| {
            let text = t.get("content")?.as_str()?.to_string();
            let doing = t.get("activeForm").and_then(Value::as_str).unwrap_or(&text).to_string();
            let status = match t.get("status")?.as_str()? {
                "completed" => PlanStatus::Done,
                "in_progress" => PlanStatus::Doing,
                _ => PlanStatus::Todo,
            };
            Some(PlanStep { text, doing, status })
        })
        .collect()
}

/// A run's feed: every event goes in, and it's saved at most every couple of seconds while it
/// changes and at once when the run ends.
pub(super) struct FeedLog {
    root: PathBuf,
    feed: LoadedFeed,
    history_at_open: HistoryAtOpen,
    pending: Vec<FeedChange>,
    pending_explanation: Option<Vec<FeedItem>>,
    failure_explained: bool,
    save_error: Option<FeedItem>,
    saved_at: Option<std::time::Instant>,
    activity: Arc<Activity>,
    log: ActivityFile,
}

struct HistoryAtOpen {
    checks: Vec<CheckResult>,
    last_reply: String,
    failure_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum FeedChange {
    Event(RunEvent),
    Item(FeedItem),
}

impl FeedChange {
    fn apply(&self, feed: &mut Vec<FeedItem>, explained: &mut bool) {
        match self {
            Self::Event(event) => apply_run_event(feed, event, explained),
            Self::Item(item) => {
                feed.push(item.clone());
                *explained = matches!(item, FeedItem::Error { .. });
            }
        }
    }
}

fn replay_changes(feed: &mut Vec<FeedItem>, pending: &[FeedChange], mut explained: bool) -> bool {
    for change in pending {
        change.apply(feed, &mut explained);
    }
    explained
}

/// A known explanation may survive edits before its saved context. An additional equal
/// external Error is another occurrence, not proof that this run supplied the new tail.
fn retains_explanation(feed: &[FeedItem], origin: &[FeedItem]) -> bool {
    let Some(error @ FeedItem::Error { .. }) = origin.last() else { return false };
    feed.ends_with(origin) && feed.iter().filter(|item| *item == error).count() == origin.iter().filter(|item| *item == error).count()
}

impl FeedLog {
    pub(super) fn open(ws: &Workspace) -> FeedLog {
        let activity = Arc::new(Activity::new(vec![ws.app_dir(), ws.root.clone()]));
        let history_at_open = HistoryAtOpen {
            checks: ws.manifest.checks.clone(),
            last_reply: ws.manifest.last_reply.clone(),
            failure_note: (ws.manifest.status == Status::NeedsYou && !ws.manifest.note.is_empty()).then(|| ws.manifest.note.clone()),
        };
        FeedLog {
            root: ws.root.clone(),
            feed: load(ws),
            history_at_open,
            pending: Vec::new(),
            pending_explanation: None,
            failure_explained: false,
            save_error: None,
            saved_at: None,
            activity,
            log: ActivityFile::new(&ws.root),
        }
    }

    pub(super) fn activity(&self) -> Arc<Activity> {
        self.activity.clone()
    }

    pub(super) fn record(&mut self, e: &RunEvent) {
        let lines = self.activity.record(e);
        self.log.append(&lines, &self.activity);
        let retain = !matches!(e, RunEvent::Preview(_) | RunEvent::Agent(AgentEvent::Output { .. }));
        if retain {
            self.begin_pending();
        }
        apply_run_event(&mut self.feed, e, &mut self.failure_explained);
        if retain {
            self.pending.push(FeedChange::Event(e.clone()));
        }
        let finished = matches!(e, RunEvent::Finished { .. });
        self.save(finished);
        if finished && self.save_error.is_some() && !self.pending.is_empty() {
            self.keep_recovery_copy();
        }
    }

    fn begin_pending(&mut self) {
        if self.pending.is_empty() {
            self.pending_explanation = (self.failure_explained && matches!(self.feed.last(), Some(FeedItem::Error { .. }))).then(|| self.feed.to_vec());
        }
    }

    /// The worker records the final event, including its durable handoff, before publishing it.
    /// Neither polling nor releasing a run performs I/O or retries a failed save.
    pub(super) fn completion_event(&self, mut event: RunEvent) -> RunEvent {
        if let RunEvent::Finished { message, .. } = &mut event
            && let Some(FeedItem::Error { message: error, remediation, log }) = &self.save_error
        {
            message.push_str("\n\n");
            message.push_str(error);
            if let Some(path) = log {
                message.push_str(&format!("\nA conversation recovery copy was kept at {}. It has not been imported automatically.", path.display()));
            } else if let Some(remediation) = remediation {
                message.push('\n');
                message.push_str(remediation);
            }
        }
        event
    }

    fn keep_recovery_copy(&mut self) {
        let Some(FeedItem::Error { message, .. }) = &self.save_error else { return };
        let message = message.clone();
        // The folded feed is the recoverable conversation. Raw tool input/output in RunEvent
        // is deliberately excluded, just as it is from feed.json; no replay is promised here.
        let snapshot = RecoverySnapshot::from_loaded(
            &self.feed,
            json!({
                "pending_event_count": self.pending.len(),
                "history_at_open": {
                    "checks": self.history_at_open.checks,
                    "last_reply": self.history_at_open.last_reply,
                    "failure_note": self.history_at_open.failure_note
                },
                "save_error": message
            }),
        );
        self.save_error = Some(match write_recovery_snapshot(&self.root, &snapshot) {
            Ok(path) => FeedItem::Error {
                message,
                remediation: Some("A separate recovery copy was kept. Open it to inspect the conversation before deciding what to restore.".into()),
                log: Some(path),
            },
            Err(error) => {
                log::warn!("couldn't keep a conversation recovery copy: {}", error.message);
                FeedItem::Error {
                    message: format!("{message}\nCouldn't keep a conversation recovery copy: {}", error.message),
                    remediation: Some(
                        "The unsaved conversation has not been preserved on disk. Keep the visible words and restore write access before retrying.".into(),
                    ),
                    log: None,
                }
            }
        });
    }

    pub(super) fn items(&self) -> Vec<FeedItem> {
        let mut items = self.feed.to_vec();
        // A persistence failure is a live notice, not the builder's explanation for a failed
        // run. Keep it out of `apply` and the saved conversation; a successful retry removes it.
        items.extend(self.save_error.iter().cloned());
        items
    }

    pub(super) fn push(&mut self, item: FeedItem) {
        self.begin_pending();
        self.feed.push(item.clone());
        self.failure_explained = matches!(&item, FeedItem::Error { .. });
        self.pending.push(FeedChange::Item(item));
        self.save(true);
    }

    pub(super) fn answered(&mut self, request: &Request, shown: &str, deliver: impl FnOnce() -> Result<(), ToolError>) -> Result<(), ToolError> {
        let mut candidate = self.feed.clone();
        merge_saved_answers(&self.root, &mut candidate);
        record_pending_answer(&mut candidate, request, shown);
        // The candidate already contains the new answer. Its explanation comes from the
        // committed owner state, before applying that answer, or from the pending prefix.
        let explanation = self.pending_explanation.as_deref().or_else(|| (self.pending.is_empty() && self.failure_explained).then_some(self.feed.as_slice()));
        let replayed_explanation = save_with_recovery(&self.root, &mut candidate, &self.history_at_open, &self.pending, explanation, Some((request, shown)))?;
        self.feed = candidate;
        if let Some(explained) = replayed_explanation {
            self.failure_explained = explained;
        }
        self.pending.clear();
        self.pending_explanation = None;
        self.save_error = None;
        self.saved_at = Some(std::time::Instant::now());
        // The raw saved state (including recovery from damaged bytes) is now committed. The
        // display still offers this request while its inbox delivery is pending or fails.
        deliver()
    }

    fn save(&mut self, now: bool) {
        self.save_using(now, save_if_unchanged);
    }

    fn save_using(&mut self, now: bool, commit_recovery: impl FnOnce(&Path, &mut LoadedFeed) -> Result<(), ToolError>) {
        if !now && self.saved_at.is_some_and(|t| t.elapsed() < Duration::from_secs(2)) {
            return;
        }
        // Answers given elsewhere while it runs (the CLI, an agent's project_inbox_answer) went
        // into the file; keep them.
        merge_saved_answers(&self.root, &mut self.feed);
        match save_with_recovery_using(
            &self.root,
            &mut self.feed,
            &self.history_at_open,
            &self.pending,
            self.pending_explanation.as_deref(),
            None,
            commit_recovery,
        ) {
            Ok(replayed_explanation) => {
                if let Some(explained) = replayed_explanation {
                    self.failure_explained = explained;
                }
                self.pending.clear();
                self.pending_explanation = None;
                self.save_error = None;
            }
            Err(e) => {
                log::warn!("couldn't save the project's feed: {}", e.message);
                self.save_error = Some(FeedItem::Error {
                    message: format!("Couldn't save the project's conversation: {}", e.message),
                    remediation: e.remediation,
                    log: None,
                });
            }
        }
        self.saved_at = Some(std::time::Instant::now());
    }
}

/// A recovery commit supplies the explanation state of its replayed feed. An ordinary save
/// leaves the caller's existing run state unchanged.
fn save_with_recovery(
    root: &Path,
    feed: &mut LoadedFeed,
    opened: &HistoryAtOpen,
    pending: &[FeedChange],
    explanation: Option<&[FeedItem]>,
    answer: Option<(&Request, &str)>,
) -> Result<Option<bool>, ToolError> {
    save_with_recovery_using(root, feed, opened, pending, explanation, answer, save_if_unchanged)
}

fn save_with_recovery_using(
    root: &Path,
    feed: &mut LoadedFeed,
    opened: &HistoryAtOpen,
    pending: &[FeedChange],
    explanation: Option<&[FeedItem]>,
    answer: Option<(&Request, &str)>,
    commit_recovery: impl FnOnce(&Path, &mut LoadedFeed) -> Result<(), ToolError>,
) -> Result<Option<bool>, ToolError> {
    let error = match save(root, feed) {
        Ok(()) => return Ok(None),
        Err(error) => error,
    };
    if error.code != ErrorCode::FileChanged && !feed.had_read_error() {
        return Err(error);
    }
    let Ok(ws) = Workspace::open(root) else {
        return Err(error);
    };
    let mut repaired = load(&ws);
    if !repaired.was_decoded() {
        return Err(error);
    }
    // Another client recovered the file after this run opened it. Its conversation is the new
    // base; replay only this run's unsaved changes, never its fallback history.
    // An older recovery may remain in an otherwise healthy conversation. Only a projection
    // of this writer's actual damaged bytes can overlap its pending events; a read error or
    // repair from another source provides no such identity, even when the words match.
    let consumed_projection = repaired.was_recovered_from(feed) && reconcile_history(&mut repaired, opened, pending);
    // A periodic save can rebase without any new run events. Only an unchanged tail retains
    // the owner's existing explanation; a new external error or archive notice cannot supply it.
    let keep_explanation = pending.is_empty() && retains_explanation(&repaired, feed);
    let initial_explanation = explanation.is_some_and(|origin| retains_explanation(&repaired, origin));
    let explained = replay_changes(&mut repaired, pending, initial_explanation);
    if let Some((request, shown)) = answer {
        record_pending_answer(&mut repaired, request, shown);
    }
    if consumed_projection {
        repaired.clear_recovery_projection();
    }
    // Commit only against the replacement that was actually replayed. A conflict keeps the
    // original writer and journal intact; its next attempt reloads the latest saved bytes.
    commit_recovery(root, &mut repaired)?;
    *feed = repaired;
    Ok((!keep_explanation).then_some(explained && matches!(feed.last(), Some(FeedItem::Error { .. }))))
}

/// A recovered file records the manifest projection used to create it. The current manifest
/// may have advanced to another turn before this writer retries, so only that recorded prefix
/// identifies projected occurrences. Replay every pending event after removing its overlap;
/// equal older records, later external additions and genuinely repeated events all survive.
fn reconcile_history(repaired: &mut LoadedFeed, opened: &HistoryAtOpen, pending: &[FeedChange]) -> bool {
    let Some(projection) = repaired.recovery_projection() else { return false };
    if !matches!(repaired.first(), Some(FeedItem::User(UserItem::Text(request))) if request == &projection.request)
        || !projection.checks.starts_with(&opened.checks)
        || !projection
            .checks
            .iter()
            .enumerate()
            .all(|(index, check)| matches!(repaired.get(index.saturating_add(1)), Some(FeedItem::Check(value)) if value == check))
    {
        return false;
    }
    let checks = projection.checks.len();
    let reply_index = checks.saturating_add(1);
    let has_reply = !projection.last_reply.is_empty();
    if has_reply && !matches!(repaired.get(reply_index), Some(FeedItem::Agent { text, streaming: false }) if text == &projection.last_reply) {
        return false;
    }
    let failure_index = reply_index.saturating_add(usize::from(has_reply));
    if let Some(note) = &projection.failure_note
        && !matches!(repaired.get(failure_index), Some(FeedItem::Error { message, remediation: None, log: None }) if message == note)
    {
        return false;
    }
    let mut removed = std::collections::BTreeSet::new();
    for change in pending {
        let FeedChange::Event(RunEvent::Checked(check)) = change else { continue };
        if let Some((index, _)) = repaired
            .iter()
            .enumerate()
            .skip(opened.checks.len().saturating_add(1))
            .take(checks.saturating_sub(opened.checks.len()))
            .find(|(index, item)| !removed.contains(index) && matches!(item, FeedItem::Check(value) if value == check))
        {
            removed.insert(index);
        }
    }
    if has_reply && projection.last_reply.trim() != opened.last_reply.trim() && has_completed_reply(pending, projection.last_reply.trim()) {
        removed.insert(reply_index);
    }
    if let Some(note) = &projection.failure_note
        && opened.failure_note.as_deref() != Some(note.as_str())
        && replay_recreates_error(repaired, &removed, failure_index, pending)
    {
        removed.insert(failure_index);
    }
    let consumed = !removed.is_empty();
    let mut index = 0usize;
    repaired.retain(|_| {
        let keep = !removed.contains(&index);
        index = index.saturating_add(1);
        keep
    });
    consumed
}

/// Finished can suppress its message when a builder error already explains it. Preview the
/// real replay without this one projected note, rather than duplicate that decision or skip
/// Finished's other effects. An equal external error is retained and is not replay evidence.
fn replay_recreates_error(repaired: &[FeedItem], removed: &std::collections::BTreeSet<usize>, projected: usize, pending: &[FeedChange]) -> bool {
    let Some(error @ FeedItem::Error { .. }) = repaired.get(projected) else { return false };
    let mut preview: Vec<_> =
        repaired.iter().enumerate().filter(|(index, _)| *index != projected && !removed.contains(index)).map(|(_, item)| item.clone()).collect();
    let count = |items: &[FeedItem]| items.iter().filter(|item| *item == error).count();
    let before = count(&preview);
    replay_changes(&mut preview, pending, false);
    count(&preview) > before
}

/// Mirror converse_turn: tool/status events do not split its accumulated reply. A partial or
/// failed turn is not evidence that a same-text manifest summary belongs to this run.
fn has_completed_reply(pending: &[FeedChange], reply: &str) -> bool {
    let mut text = String::new();
    let mut failed = false;
    for change in pending {
        match change {
            FeedChange::Event(RunEvent::Agent(AgentEvent::TextDelta { text: delta })) => text.push_str(delta),
            FeedChange::Event(RunEvent::Agent(AgentEvent::Error { .. })) => failed = true,
            FeedChange::Event(RunEvent::Agent(AgentEvent::TurnDone { .. })) => {
                if !failed && !text.trim().is_empty() && text.trim() == reply {
                    return true;
                }
                text.clear();
                failed = false;
            }
            _ => {}
        }
    }
    false
}

fn merge_saved_answers(root: &Path, feed: &mut LoadedFeed) {
    if let Ok(ws) = Workspace::open(root) {
        for item in load(&ws).into_items() {
            if let FeedItem::Ask { request, answer: Some(answer) } = item {
                record_pending_answer(feed, &request, &answer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_live_feed_with_busy_recovery_names(ws: &Workspace) -> FeedLog {
        std::fs::write(ws.root.join(FEED_FILE), b"damaged conversation").unwrap();
        for index in 0..16 {
            let name = if index == 0 { "feed.json.broken".into() } else { format!("feed.json.broken.{index}") };
            std::fs::write(ws.root.join(name), b"an earlier recovery copy").unwrap();
        }
        let mut log = FeedLog::open(ws);
        log.record(&RunEvent::Step { step: Step::Build, text: "Building".into() });
        assert_eq!(std::fs::read(ws.root.join(FEED_FILE)).unwrap(), b"damaged conversation");
        log
    }

    fn recover_the_conversation_through_the_cli(ws: &Workspace) {
        std::fs::remove_file(ws.root.join("feed.json.broken")).unwrap();
        let id = Inbox::new(ws.inbox_dir()).post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let engine = craft_anything_engine::Engine::new(craft_anything_engine::testing::session());
        engine
            .call(
                "project_inbox_answer",
                json!({"project": ws.root, "id": id, "choice": "Blue"}),
                craft_anything_engine::CallOptions::new(craft_anything_engine::Caller::Cli),
            )
            .unwrap();
        assert_eq!(std::fs::read(ws.root.join("feed.json.broken")).unwrap(), b"damaged conversation");
    }

    fn finished_turn() -> RunEvent {
        RunEvent::Agent(AgentEvent::TurnDone {
            backend: "test".into(),
            input_tokens: 0,
            output_tokens: 0,
            tokens_unknown: false,
            cost_usd: None,
            cost_is_estimate: false,
        })
    }

    fn record_before_next_save(log: &mut FeedLog, event: RunEvent) {
        // Exercise the normal event path while the two-second save gate keeps this writer
        // behind the CLI and manifest updates. Finished below performs the eventual flush.
        log.saved_at = Some(std::time::Instant::now());
        log.record(&event);
    }

    fn record_complete_pending_reply(log: &mut FeedLog, reply: &str) {
        record_before_next_save(log, RunEvent::Step { step: Step::Build, text: "Building".into() });
        record_before_next_save(log, RunEvent::Agent(AgentEvent::TextDelta { text: reply.into() }));
        record_before_next_save(log, finished_turn());
    }

    fn a_recovered_feed_after_a_later_turn(ws: &mut Workspace) -> Vec<u8> {
        ws.manifest.last_reply = "Turn A.".into();
        ws.save().unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::write(&path, b"damaged conversation from an earlier recovery").unwrap();
        save(&ws.root, &mut load(ws)).unwrap();

        // A subsequent normal run keeps the earlier recovered A and records a real B. The
        // unconsumed projection is still on disk, but it does not belong to a future run.
        let mut previous = FeedLog::open(ws);
        record_complete_pending_reply(&mut previous, "Turn B.");
        ws.manifest.last_reply = "Turn B.".into();
        ws.save().unwrap();
        previous.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        std::fs::read(path).unwrap()
    }

    #[test]
    fn a_read_error_repair_does_not_consume_an_older_projection() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let saved = a_recovered_feed_after_a_later_turn(&mut ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        let mut log = FeedLog::open(&ws);
        record_complete_pending_reply(&mut log, "Turn A.");
        ws.manifest.last_reply = "Turn A.".into();
        ws.save().unwrap();
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, saved).unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let feed = load(&ws);
        assert_eq!(
            feed.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A.")).count(),
            2,
            "the older projected reply and this run's genuinely repeated reply are distinct"
        );
        assert_eq!(feed.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn B.")).count(), 1);
        assert!(log.save_error.is_none());
        assert!(log.pending.is_empty());
    }

    #[test]
    fn a_different_damaged_source_does_not_consume_an_older_projection() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let saved = a_recovered_feed_after_a_later_turn(&mut ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::write(&path, b"a different damaged conversation").unwrap();
        let mut log = FeedLog::open(&ws);
        record_complete_pending_reply(&mut log, "Turn A.");
        ws.manifest.last_reply = "Turn A.".into();
        ws.save().unwrap();
        std::fs::write(&path, saved).unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let feed = load(&ws);
        assert_eq!(
            feed.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A.")).count(),
            2,
            "recovery of different bytes cannot identify this run's new reply as an old projected occurrence"
        );
        assert_eq!(feed.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn B.")).count(), 1);
        assert!(log.save_error.is_none());
        assert!(log.pending.is_empty());
    }

    #[test]
    fn a_recovered_reply_uses_its_saved_projection_after_the_manifest_advances() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        ws.manifest.last_reply = "An earlier reply.".into();
        ws.save().unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        for _ in 0..2 {
            record_complete_pending_reply(&mut log, "Turn A.");
            ws.manifest.last_reply = "Turn A.".into();
            ws.save().unwrap();
        }
        recover_the_conversation_through_the_cli(&ws);
        let mut repaired = load(&ws);
        repaired.push(FeedItem::Agent { text: "Turn A.".into(), streaming: false });
        repaired.push(FeedItem::Agent { text: "Turn B.".into(), streaming: false });
        repaired.push(FeedItem::User(UserItem::Text("Keep the external note".into())));
        save(&ws.root, &mut repaired).unwrap();

        // The CLI recovered A, but this writer's first retry is after the manifest records B.
        // Both completed A turns are real; only the one recovered summary overlaps them.
        record_complete_pending_reply(&mut log, "Turn B.");
        ws.manifest.last_reply = "Turn B.".into();
        ws.save().unwrap();
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let saved = load(&ws);
        let count = |reply: &str| saved.iter().filter(|item| matches!(item, FeedItem::Agent { text, streaming: false } if text == reply)).count();
        assert_eq!(count("Turn A."), 3, "two live turns and the external reply survive, without the projected duplicate");
        assert_eq!(count("Turn B."), 2, "the later live turn and same-text external reply are separate occurrences");
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external note")).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { answer: Some(value), .. } if value == "Blue")));
        let disk: Value = serde_json::from_slice(&std::fs::read(ws.root.join(FEED_FILE)).unwrap()).unwrap();
        assert!(disk.get("recovery").is_none_or(Value::is_null), "the consumed projection is not reused after its prefix changes");
    }

    #[test]
    fn a_recovery_prefix_changed_by_another_client_is_not_consumed() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        record_complete_pending_reply(&mut log, "Turn A.");
        ws.manifest.last_reply = "Turn A.".into();
        ws.save().unwrap();
        recover_the_conversation_through_the_cli(&ws);
        let mut repaired = load(&ws);
        let projected = repaired.iter_mut().find(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A.")).unwrap();
        *projected = FeedItem::Agent { text: "Turn B.".into(), streaming: false };
        repaired.push(FeedItem::Agent { text: "Turn B.".into(), streaming: false });
        save(&ws.root, &mut repaired).unwrap();
        record_complete_pending_reply(&mut log, "Turn B.");
        ws.manifest.last_reply = "Turn B.".into();
        ws.save().unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A.")).count(), 1);
        assert_eq!(
            saved.iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn B.")).count(),
            3,
            "the edited prefix, external matching reply and pending turn are all retained"
        );
    }

    #[test]
    fn a_replacement_without_recovery_provenance_keeps_a_matching_prefix() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        record_complete_pending_reply(&mut log, "Turn A.");
        ws.manifest.last_reply = "Turn A.".into();
        ws.save().unwrap();
        std::fs::write(ws.root.join(FEED_FILE), br#"{"version":5,"items":[{"User":"A game"},{"Agent":{"text":"Turn A.","streaming":false}}]}"#).unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        assert_eq!(
            load(&ws).iter().filter(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Turn A.")).count(),
            2,
            "a matching legacy prefix is not evidence that this writer's turn was projected"
        );
    }

    #[test]
    fn a_live_read_error_recovers_on_the_same_writer_and_keeps_both_clients_changes() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        let mut log = FeedLog::open(&ws);
        log.push(FeedItem::User(UserItem::Text("Keep the live run's note".into())));
        assert!(log.save_error.is_some());
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the external conversation"}]}"#).unwrap();

        log.answered(&question, "Blue", || inbox.answer(&id, &craft_anything_engine::inbox::Answer { choice: Some("Blue".into()), ..Default::default() }))
            .unwrap();
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let saved = load(&ws);
        for text in ["Keep the external conversation", "Keep the live run's note"] {
            assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == text)).count(), 1, "{text}");
        }
        assert!(!saved.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "A game")), "the old fallback is never replayed");
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
        assert!(inbox.pending().is_empty());
        assert!(log.pending.is_empty());
        assert!(log.save_error.is_none());
        assert_eq!(log.items(), saved.into_items());
        assert!(!ws.root.join("feed.json.broken").exists(), "a decoded replacement needs no damaged-file recovery");
    }

    #[test]
    fn a_recovery_save_conflict_keeps_the_journal_and_reloads_the_external_answer() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        let mut log = FeedLog::open(&ws);
        log.record(&RunEvent::Ask(question));
        log.push(FeedItem::User(UserItem::Text("Keep the live run's note".into())));
        assert!(log.save_error.is_some());
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the external conversation"}]}"#).unwrap();
        let before = log.feed.to_vec();
        let pending_count = log.pending.len();
        let mut external_bytes = Vec::new();

        let error = save_with_recovery_using(&ws.root, &mut log.feed, &log.history_at_open, &log.pending, None, None, |root, replayed| {
            assert!(replayed.was_decoded());
            assert!(replayed.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the live run's note")));
            assert!(replayed.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: None } if request.id == id)));
            // The exact write ordering is local to this call: another real client answers
            // after the replacement load and before the guarded commit's disk read.
            let engine = craft_anything_engine::Engine::new(craft_anything_engine::testing::session());
            engine
                .call(
                    "project_inbox_answer",
                    json!({"project": ws.root, "id": id, "choice": "Blue"}),
                    craft_anything_engine::CallOptions::new(craft_anything_engine::Caller::Cli),
                )
                .unwrap();
            external_bytes = std::fs::read(&path).unwrap();
            save_if_unchanged(root, replayed)
        })
        .unwrap_err();

        assert_eq!(error.code, ErrorCode::FileChanged);
        assert_eq!(std::fs::read(&path).unwrap(), external_bytes, "the rejected replay leaves the CLI's answer byte-for-byte intact");
        assert_eq!(log.feed.to_vec(), before, "a failed commit does not install its replay candidate");
        assert!(log.feed.had_read_error(), "the original writer's sticky guard remains intact");
        assert_eq!(log.pending.len(), pending_count);
        assert!(!load(&ws).iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the live run's note")));

        log.save(true);

        let saved = load(&ws);
        for text in ["Keep the external conversation", "Keep the live run's note"] {
            assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == text)).count(), 1, "{text}");
        }
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == id)).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
        assert!(inbox.pending().is_empty());
        assert!(log.pending.is_empty());
        assert!(log.save_error.is_none());
        assert_eq!(log.items(), saved.into_items());
        assert!(!ws.root.join("feed.json.broken").exists());
    }

    #[test]
    fn a_live_read_error_does_not_accept_a_missing_damaged_or_newer_replacement() {
        let replacements: [Option<&[u8]>; 3] = [None, Some(b"damaged conversation"), Some(br#"{"version":999,"items":[]}"#)];
        for replacement in replacements {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let inbox = Inbox::new(ws.inbox_dir());
            inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
            let question = inbox.pending().pop().unwrap();
            let path = ws.root.join(FEED_FILE);
            std::fs::create_dir(&path).unwrap();
            let mut log = FeedLog::open(&ws);
            log.push(FeedItem::User(UserItem::Text("Keep the unsaved note".into())));
            std::fs::remove_dir(&path).unwrap();
            if let Some(bytes) = replacement {
                std::fs::write(&path, bytes).unwrap();
            }
            let mut delivered = false;
            assert!(
                log.answered(&question, "Blue", || {
                    delivered = true;
                    Ok(())
                })
                .is_err()
            );
            assert!(!delivered);
            assert_eq!(inbox.pending().len(), 1);
            assert!(!log.pending.is_empty());
            if let Some(bytes) = replacement {
                assert_eq!(std::fs::read(&path).unwrap(), bytes);
            } else {
                assert!(!path.exists(), "manifest fallback is not a decoded replacement");
            }
            assert!(!ws.root.join("feed.json.broken").exists());
        }
    }

    #[test]
    fn a_recovered_manifest_check_is_replayed_once_without_dropping_other_occurrences() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut check = CheckResult::new("build", 1, true, "The build passed.");
        check.at_ms = 17;
        ws.manifest.checks.push(check.clone());
        ws.save().unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        // The worker saves the check in craft.json before emitting its feed event. An equal
        // earlier check and a repeated event are separate occurrences, not a deduplication set.
        ws.manifest.checks.push(check.clone());
        ws.save().unwrap();
        log.record(&RunEvent::Checked(check.clone()));
        log.record(&RunEvent::Checked(check.clone()));
        recover_the_conversation_through_the_cli(&ws);
        let mut repaired = load(&ws);
        repaired.push(FeedItem::Check(check.clone()));
        repaired.push(FeedItem::User(UserItem::Text("Keep the external note".into())));
        save(&ws.root, &mut repaired).unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Check(value) if value == &check)).count(), 4);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external note")).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { answer: Some(value), .. } if value == "Blue")));
    }

    #[test]
    fn a_recovered_complete_reply_keeps_both_turns_and_external_matching_text() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        ws.manifest.last_reply = "An earlier reply.".into();
        ws.save().unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        for _ in 0..2 {
            log.record(&RunEvent::Agent(AgentEvent::TextDelta { text: " Ready".into() }));
            log.record(&RunEvent::Agent(AgentEvent::ToolStarted { call_id: "check".into(), tool: "build_check".into(), input: json!({}) }));
            log.record(&RunEvent::Agent(AgentEvent::Status { text: "Checking".into() }));
            log.record(&RunEvent::Agent(AgentEvent::TextDelta { text: " to ship. ".into() }));
            log.record(&finished_turn());
            // converse_turn concatenates all text deltas, including those separated by tools;
            // its caller stores the trimmed complete reply before the next checking step.
            ws.manifest.last_reply = "Ready to ship.".into();
            ws.save().unwrap();
            log.record(&RunEvent::Step { step: Step::Check, text: "Checking the build".into() });
        }
        recover_the_conversation_through_the_cli(&ws);
        let mut repaired = load(&ws);
        repaired.push(FeedItem::Agent { text: "Ready to ship.".into(), streaming: false });
        repaired.push(FeedItem::User(UserItem::Text("Keep the external reply".into())));
        save(&ws.root, &mut repaired).unwrap();

        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let saved = load(&ws);
        let replies: Vec<_> =
            saved.iter().filter_map(|item| if let FeedItem::Agent { text, streaming: false } = item { Some(text.as_str()) } else { None }).collect();
        assert_eq!(replies.iter().filter(|text| **text == "Ready to ship.").count(), 1, "the external matching reply stays");
        assert_eq!(replies.iter().filter(|text| **text == " Ready").count(), 2, "both genuine turns keep their first segment");
        assert_eq!(replies.iter().filter(|text| **text == " to ship. ").count(), 2, "both turns keep their second segment");
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external reply")).count(), 1);
    }

    #[test]
    fn a_recovered_reply_does_not_consume_old_unfinished_or_failed_matching_text() {
        for (original, complete, failed) in [("Repeat.", true, false), ("Earlier.", false, false), ("Earlier.", true, true)] {
            let d = tempfile::tempdir().unwrap();
            let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            ws.manifest.last_reply = original.into();
            ws.save().unwrap();
            let mut log = a_live_feed_with_busy_recovery_names(&ws);
            log.record(&RunEvent::Agent(AgentEvent::TextDelta { text: "Repeat.".into() }));
            if failed {
                log.record(&RunEvent::Agent(AgentEvent::Error { message: "The turn failed.".into(), remediation: None, log: None }));
            }
            if complete {
                log.record(&finished_turn());
            }
            ws.manifest.last_reply = "Repeat.".into();
            ws.save().unwrap();
            recover_the_conversation_through_the_cli(&ws);

            log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

            let saved = load(&ws);
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Agent { text, streaming: false } if text == "Repeat.")).count(),
                2,
                "matching text alone is not proof of a newly projected completed turn: {original}, {complete}, {failed}"
            );
        }
    }

    #[test]
    fn a_recovered_failure_note_keeps_external_errors_and_finishes_the_plan() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        ws.manifest.last_reply = "The earlier reply".into();
        ws.save().unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        log.push(FeedItem::Plan(vec![PlanStep { text: "Check the build".into(), doing: "Checking".into(), status: PlanStatus::Doing }]));
        let message = "The build check failed.";
        // run_outcome persists NeedsYou and its note before emitting Finished. The CLI can
        // recover that projection while this live writer still has unsaved events.
        ws.set_status(Status::NeedsYou, message).unwrap();
        recover_the_conversation_through_the_cli(&ws);
        let mut repaired = load(&ws);
        for text in [message, "Keep the other client's error"] {
            repaired.push(FeedItem::Error { message: text.into(), remediation: None, log: None });
        }
        save(&ws.root, &mut repaired).unwrap();

        log.record(&RunEvent::Finished { ok: false, message: message.into(), paused: false });

        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message: text, .. } if text == message)).count(), 2);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "Keep the other client's error")).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Plan(steps) if steps.len() == 1 && steps[0].status == PlanStatus::Stopped)));
    }

    #[test]
    fn a_recovered_failure_note_preserves_an_equal_older_failure() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let message = "The build check failed.";
        ws.set_status(Status::NeedsYou, message).unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        ws.set_status(Status::NeedsYou, message).unwrap();
        recover_the_conversation_through_the_cli(&ws);

        log.record(&RunEvent::Finished { ok: false, message: message.into(), paused: false });

        assert_eq!(load(&ws).iter().filter(|item| matches!(item, FeedItem::Error { message: text, .. } if text == message)).count(), 2);
    }

    #[test]
    fn a_recovered_failure_note_stays_when_replay_would_not_recreate_it() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut log = a_live_feed_with_busy_recovery_names(&ws);
        log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
        let message = "The build failed.";
        ws.set_status(Status::NeedsYou, message).unwrap();
        recover_the_conversation_through_the_cli(&ws);

        log.record(&RunEvent::Finished { ok: false, message: message.into(), paused: false });

        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message: text, .. } if text == message)).count(), 1);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The compiler rejected the source.")).count(), 1);
    }

    #[test]
    fn a_conversation_save_error_does_not_hide_the_build_failure() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut log = FeedLog::open(&ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        log.record(&RunEvent::Step { step: Step::Build, text: "Building".into() });
        assert!(log.items().iter().any(|item| matches!(item, FeedItem::Error { .. })));
        std::fs::remove_dir(&path).unwrap();
        log.record(&RunEvent::Finished { ok: false, message: "The build check failed.".into(), paused: false });
        let saved = load(&ws);
        let errors: Vec<_> = saved.iter().filter_map(|item| if let FeedItem::Error { message, .. } = item { Some(message.as_str()) } else { None }).collect();
        assert_eq!(errors, ["The build check failed."]);
        assert_eq!(log.items().to_vec(), saved.into_items(), "a successful save clears the old persistence notice");
    }

    #[test]
    fn a_save_notice_does_not_duplicate_an_explained_build_failure() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let mut log = FeedLog::open(&ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
        std::fs::remove_dir(&path).unwrap();
        log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
        let saved = load(&ws);
        let errors: Vec<_> = saved.iter().filter_map(|item| if let FeedItem::Error { message, .. } = item { Some(message.as_str()) } else { None }).collect();
        assert_eq!(errors, ["The compiler rejected the source."]);
    }

    #[test]
    fn a_run_failure_is_not_explained_by_an_external_error_after_recovery() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        let mut log = FeedLog::open(&ws);
        log.record(&RunEvent::Ask(question.clone()));
        assert!(log.save_error.is_some());
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the external conversation"}]}"#).unwrap();
        record_answer(&ws, &question, "Blue").unwrap();
        let mut external = load(&ws);
        external.push(FeedItem::Error { message: "The other client's check failed.".into(), remediation: None, log: None });
        save(&ws.root, &mut external).unwrap();

        log.record(&RunEvent::Finished { ok: false, message: "This run's build failed.".into(), paused: false });

        let saved = load(&ws);
        let errors: Vec<_> = saved.iter().filter_map(|item| if let FeedItem::Error { message, .. } = item { Some(message.as_str()) } else { None }).collect();
        assert_eq!(errors, ["The other client's check failed.", "This run's build failed."]);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == id)).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
        assert_eq!(log.items(), saved.into_items(), "the live conversation keeps the same failure and saved answer");
        assert!(log.pending.is_empty());
        assert!(log.save_error.is_none());
    }

    #[test]
    fn a_run_failure_projection_is_reconciled_with_run_scoped_evidence() {
        let d = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        std::fs::write(ws.root.join(FEED_FILE), b"damaged conversation").unwrap();
        for index in 0..16 {
            let name = if index == 0 { "feed.json.broken".into() } else { format!("feed.json.broken.{index}") };
            std::fs::write(ws.root.join(name), b"an earlier recovery copy").unwrap();
        }
        let mut log = FeedLog::open(&ws);
        log.record(&RunEvent::Ask(question));
        assert!(log.save_error.is_some());
        let message = "This run's build failed.";
        ws.set_status(Status::NeedsYou, message).unwrap();
        std::fs::remove_file(ws.root.join("feed.json.broken")).unwrap();
        let engine = craft_anything_engine::Engine::new(craft_anything_engine::testing::session());
        engine
            .call(
                "project_inbox_answer",
                json!({"project": ws.root, "id": id, "choice": "Blue"}),
                craft_anything_engine::CallOptions::new(craft_anything_engine::Caller::Cli),
            )
            .unwrap();
        let mut external = load(&ws);
        external.push(FeedItem::Error { message: "The other client's check failed.".into(), remediation: None, log: None });
        save(&ws.root, &mut external).unwrap();

        log.record(&RunEvent::Finished { ok: false, message: message.into(), paused: false });

        let saved = load(&ws);
        let failures: Vec<_> = saved
            .iter()
            .filter_map(|item| match item {
                FeedItem::Error { message, .. } if message == "This run's build failed." || message == "The other client's check failed." => {
                    Some(message.as_str())
                }
                _ => None,
            })
            .collect();
        assert_eq!(failures, ["The other client's check failed.", message], "only the projected note moves to this run's actual Finished event");
        assert!(saved.recovery_projection().is_none(), "the replay consumes the newly projected failure exactly once");
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == id)).count(), 1);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
        assert_eq!(log.items(), saved.into_items());
    }

    #[test]
    fn a_run_error_stays_explained_after_its_successful_save() {
        for next_step in [false, true] {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let mut log = FeedLog::open(&ws);
            log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
            assert!(log.pending.is_empty(), "the explaining error has already been saved");
            if next_step {
                log.record(&RunEvent::Step { step: Step::Build, text: "Trying another build".into() });
            }
            log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
            let saved = load(&ws);
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The compiler rejected the source.")).count(),
                1
            );
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The build failed.")).count(),
                usize::from(next_step)
            );
        }
    }

    fn a_read_error_run_with_an_error_then_a_new_question(ws: &Workspace) -> (FeedLog, Request) {
        std::fs::create_dir(ws.root.join(FEED_FILE)).unwrap();
        let mut log = FeedLog::open(ws);
        log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
        assert!(log.failure_explained);
        let inbox = Inbox::new(ws.inbox_dir());
        inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Try another compiler?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        log.record(&RunEvent::Ask(question.clone()));
        assert!(!log.failure_explained, "the new question follows the error in the original writer");
        assert!(matches!(log.feed.last(), Some(FeedItem::Ask { .. })));
        assert_eq!(log.pending.len(), 2);
        (log, question)
    }

    #[test]
    fn recovery_failure_explanation_is_saved_with_the_replayed_feed() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let (mut log, question) = a_read_error_run_with_an_error_then_a_new_question(&ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the repaired conversation"}]}"#).unwrap();

        log.save(true);

        assert!(log.pending.is_empty());
        assert!(matches!(log.feed.last(), Some(FeedItem::Error { message, .. }) if message == "The compiler rejected the source."));
        log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
        let saved = load(&ws);
        let errors: Vec<_> = saved.iter().filter_map(|item| if let FeedItem::Error { message, .. } = item { Some(message.as_str()) } else { None }).collect();
        assert_eq!(errors, ["The compiler rejected the source."], "the saved replay already explains this failure");
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == question.id)).count(), 1);
        assert_eq!(log.items(), saved.into_items());
    }

    #[test]
    fn recovery_failure_explanation_survives_answer_delivery() {
        for delivery_fails in [false, true] {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let (mut log, question) = a_read_error_run_with_an_error_then_a_new_question(&ws);
            let path = ws.root.join(FEED_FILE);
            std::fs::remove_dir(&path).unwrap();
            std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the repaired conversation"}]}"#).unwrap();
            let inbox = Inbox::new(ws.inbox_dir());

            let result = log.answered(&question, "Yes", || {
                if delivery_fails {
                    Err(ToolError::new(ErrorCode::FileError, "Delivery is unavailable."))
                } else {
                    inbox.answer(&question.id, &craft_anything_engine::inbox::Answer { text: Some("Yes".into()), ..Default::default() })
                }
            });

            assert_eq!(result.is_err(), delivery_fails);
            assert!(log.pending.is_empty(), "the feed commit succeeds before the delivery result");
            assert!(matches!(log.feed.last(), Some(FeedItem::Error { message, .. }) if message == "The compiler rejected the source."));
            log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
            let saved = load(&ws);
            let errors: Vec<_> =
                saved.iter().filter_map(|item| if let FeedItem::Error { message, .. } = item { Some(message.as_str()) } else { None }).collect();
            assert_eq!(errors, ["The compiler rejected the source."], "delivery_fails={delivery_fails}");
            assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == question.id)).count(), 1);
            assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == question.id && answer == "Yes")));
            assert_eq!(log.items(), saved.into_items());
        }
    }

    #[test]
    fn recovery_failure_explanation_is_unchanged_after_a_guarded_conflict() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let (mut log, _) = a_read_error_run_with_an_error_then_a_new_question(&ws);
        let path = ws.root.join(FEED_FILE);
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the repaired conversation"}]}"#).unwrap();
        let before = log.feed.to_vec();
        let pending = log.pending.clone();
        let changed = br#"{"version":5,"items":[{"User":"Keep the concurrent writer's words"}]}"#;
        let mut committed = false;

        // Reach the owner that installs explanation state, with the same local seam used by
        // save. A real write between reload and commit is rejected by the real guarded save.
        log.save_using(true, |root, replayed| {
            committed = true;
            assert!(matches!(replayed.last(), Some(FeedItem::Error { message, .. }) if message == "The compiler rejected the source."));
            std::fs::write(&path, changed).unwrap();
            let result = save_if_unchanged(root, replayed);
            assert_eq!(result.as_ref().unwrap_err().code, ErrorCode::FileChanged);
            result
        });

        assert!(committed && log.save_error.is_some());
        assert!(!log.failure_explained, "a rejected replay must not change the original writer's explanation state");
        assert!(log.feed.had_read_error());
        assert_eq!(log.feed.to_vec(), before);
        assert_eq!(log.pending, pending);
        assert_eq!(std::fs::read(&path).unwrap(), changed);
    }

    #[test]
    fn an_empty_pending_rebase_preserves_only_the_same_explained_tail() {
        for external_error in [false, true] {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let mut log = FeedLog::open(&ws);
            log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
            assert!(log.pending.is_empty() && log.failure_explained);
            let mut external = load(&ws);
            external.insert(0, FeedItem::User(UserItem::Text("Keep the external words".into())));
            if external_error {
                external.push(FeedItem::Error { message: "An unrelated client's error.".into(), remediation: None, log: None });
            }
            save(&ws.root, &mut external).unwrap();

            log.save(true);

            assert!(log.pending.is_empty());
            assert_eq!(log.failure_explained, !external_error);
            log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
            let saved = load(&ws);
            assert!(saved.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external words")));
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The build failed.")).count(),
                usize::from(external_error)
            );
        }
    }

    #[test]
    fn a_direct_finished_rebase_preserves_the_saved_run_explanation() {
        let compiler = "The compiler rejected the source.";
        for external_error in [None, Some("An unrelated client's error."), Some(compiler)] {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let mut log = FeedLog::open(&ws);
            log.record(&RunEvent::Agent(AgentEvent::Error { message: compiler.into(), remediation: None, log: None }));
            assert!(log.pending.is_empty() && log.failure_explained);
            let mut external = load(&ws);
            external.insert(0, FeedItem::User(UserItem::Text("Keep the external words".into())));
            if let Some(message) = external_error {
                external.push(FeedItem::Error { message: message.into(), remediation: None, log: None });
            }
            save(&ws.root, &mut external).unwrap();

            // No intervening empty-journal save: Finished itself discovers the conflict.
            log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });

            assert!(log.pending.is_empty() && log.save_error.is_none());
            let saved = load(&ws);
            assert!(saved.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external words")));
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == compiler)).count(),
                1 + usize::from(external_error == Some(compiler))
            );
            assert_eq!(
                saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The build failed.")).count(),
                usize::from(external_error.is_some()),
                "external_error={external_error:?}"
            );
        }
    }

    #[test]
    fn an_answer_rebase_keeps_the_explanation_from_before_the_new_answer() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Try another compiler?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let mut log = FeedLog::open(&ws);
        log.record(&RunEvent::Agent(AgentEvent::Error { message: "The compiler rejected the source.".into(), remediation: None, log: None }));
        assert!(log.pending.is_empty() && log.failure_explained);
        let mut external = load(&ws);
        external.insert(0, FeedItem::User(UserItem::Text("Keep the external words".into())));
        save(&ws.root, &mut external).unwrap();

        log.answered(&question, "Yes", || inbox.answer(&question.id, &Answer { text: Some("Yes".into()), ..Default::default() })).unwrap();

        assert!(log.pending.is_empty() && log.failure_explained);
        log.record(&RunEvent::Finished { ok: false, message: "The build failed.".into(), paused: false });
        let saved = load(&ws);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the external words")));
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == question.id && answer == "Yes")));
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "The compiler rejected the source.")).count(), 1);
        assert!(!saved.iter().any(|item| matches!(item, FeedItem::Error { message, .. } if message == "The build failed.")));
    }

    #[test]
    fn a_retained_archive_notice_does_not_explain_a_new_runs_failure() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let snapshot = RecoverySnapshot::from_loaded(&load(&ws), json!({}));
        let path = write_recovery_snapshot(&ws.root, &snapshot).unwrap();
        let mut log = FeedLog::open(&ws);
        assert!(matches!(log.feed.last(), Some(FeedItem::Error { log: Some(saved), .. }) if saved == &path));
        assert!(!log.failure_explained);

        log.record(&RunEvent::Finished { ok: false, message: "This new build failed.".into(), paused: false });

        let saved = load(&ws);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { message, .. } if message == "This new build failed.")).count(), 1);
        assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::Error { log: Some(saved), .. } if saved == &path)).count(), 1);
        assert!(log.pending.is_empty() && log.save_error.is_none());
    }

    #[test]
    fn a_live_feed_keeps_run_changes_when_another_client_recovers_the_conversation() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        std::fs::write(ws.root.join(FEED_FILE), b"damaged conversation").unwrap();
        let mut log = FeedLog::open(&ws);
        record_answer(&ws, &question, "Blue").unwrap();
        let mut repaired = load(&ws);
        repaired.push(FeedItem::User(UserItem::Text("Keep the other client's note".into())));
        save(&ws.root, &mut repaired).unwrap();
        log.record(&RunEvent::Note { shown: "Keep the live run's note".into(), pictures: Vec::new() });
        log.record(&RunEvent::Agent(AgentEvent::TextDelta { text: "Still building.".into() }));
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        let saved = load(&ws);
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
        for text in ["Keep the other client's note", "Keep the live run's note"] {
            assert_eq!(saved.iter().filter(|item| matches!(item, FeedItem::User(note) if note.text() == text)).count(), 1, "{text}");
        }
        assert!(saved.iter().any(|item| matches!(item, FeedItem::Agent { text, streaming: false } if text == "Still building.")));
        assert_eq!(std::fs::read(ws.root.join("feed.json.broken")).unwrap(), b"damaged conversation");
        assert!(!ws.root.join("feed.json.broken.1").exists(), "the live writer does not recover the file a second time");
    }

    #[test]
    fn a_live_feed_does_not_replace_an_unreadable_or_missing_recovery() {
        let replacements: [Option<&[u8]>; 3] = [None, Some(b"another damaged conversation"), Some(br#"{"version":999,"items":[]}"#)];
        for replacement in replacements {
            let d = tempfile::tempdir().unwrap();
            let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
            let path = ws.root.join(FEED_FILE);
            std::fs::write(&path, b"damaged conversation").unwrap();
            let mut log = FeedLog::open(&ws);
            if let Some(bytes) = replacement {
                std::fs::write(&path, bytes).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
            log.push(FeedItem::User(UserItem::Text("An unsaved run note".into())));
            if let Some(bytes) = replacement {
                assert_eq!(std::fs::read(&path).unwrap(), bytes);
            } else {
                assert!(!path.exists());
            }
            assert!(!ws.root.join("feed.json.broken").exists(), "only a valid replacement can become the live writer's new base");
        }
    }

    #[test]
    fn a_live_answer_does_not_recreate_a_moved_project_without_its_conversation() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let mut log = FeedLog::open(&ws);
        let moved = d.path().join("moved-project");
        std::fs::rename(&ws.root, &moved).unwrap();
        let mut delivered = false;
        assert!(
            log.answered(&question, "Blue", || {
                delivered = true;
                Ok(())
            })
            .is_err()
        );
        assert!(!delivered);
        assert!(!ws.root.exists(), "a failed save must not recreate a moved project");
        std::fs::rename(&moved, &ws.root).unwrap();
        assert_eq!(inbox.pending().len(), 1);
        log.answered(&question, "Blue", || inbox.answer(&id, &craft_anything_engine::inbox::Answer { choice: Some("Blue".into()), ..Default::default() }))
            .unwrap();
        assert!(inbox.pending().is_empty());
        assert!(load(&ws).iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Blue")));
    }

    #[test]
    fn a_failed_delivery_after_feed_recovery_can_retry_and_keep_an_external_answer() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue or red?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        std::fs::write(ws.root.join(FEED_FILE), b"damaged conversation").unwrap();
        let mut log = FeedLog::open(&ws);
        let failed = || Err(ToolError::new(craft_anything_evidence::ErrorCode::FileError, "delivery failed"));
        assert_eq!(log.answered(&question, "Blue", failed).unwrap_err().message, "delivery failed");
        assert!(for_display(&ws).iter().any(|item| matches!(item, FeedItem::Ask { answer: None, .. })));
        assert_eq!(std::fs::read(ws.root.join("feed.json.broken")).unwrap(), b"damaged conversation");
        // The same live writer has kept the successful recovery state and can save again.
        assert_eq!(log.answered(&question, "Blue", failed).unwrap_err().message, "delivery failed");
        // The person retries through another client. A later live save keeps that new answer.
        record_answer(&ws, &question, "Red").unwrap();
        inbox.answer(&id, &craft_anything_engine::inbox::Answer { choice: Some("Red".into()), ..Default::default() }).unwrap();
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        assert!(load(&ws).iter().any(|item| matches!(item, FeedItem::Ask { request, answer: Some(answer) } if request.id == id && answer == "Red")));
        log.record(&RunEvent::Ask(question.clone()));
        assert_eq!(
            log.items().iter().filter(|item| matches!(item, FeedItem::Ask { request, .. } if request.id == id)).count(),
            1,
            "a delayed relay keeps the recorded answer"
        );
        assert!(!ws.root.join("feed.json.broken.1").exists(), "a delivery retry needs no second recovery copy");
    }

    #[test]
    fn a_live_answer_is_not_delivered_when_its_feed_save_fails() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let question = inbox.pending().pop().unwrap();
        let path = ws.root.join(FEED_FILE);
        std::fs::create_dir(&path).unwrap();
        let mut log = FeedLog::open(&ws);
        let mut delivered = false;
        assert!(
            log.answered(&question, "Blue", || {
                delivered = true;
                Ok(())
            })
            .is_err()
        );
        assert!(!delivered);
        assert_eq!(inbox.pending().len(), 1);
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, br#"{"version":5,"items":[{"User":"Keep the actual conversation"}]}"#).unwrap();
        log.answered(&question, "Blue", || {
            delivered = true;
            inbox.answer(&id, &craft_anything_engine::inbox::Answer { choice: Some("Blue".into()), ..Default::default() })
        })
        .unwrap();
        assert!(delivered, "the same writer reloads the repaired file before recording and delivering the answer");
        assert!(inbox.pending().is_empty());
        assert!(load(&ws).iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the actual conversation")));
    }

    #[test]
    fn every_event_counts_as_activity() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "y", TargetRef::None).unwrap();
        let mut log = FeedLog::open(&ws);
        let activity = log.activity();
        // Well apart: recording saves files, which takes a while on a busy machine.
        std::thread::sleep(Duration::from_millis(1500));
        assert!(activity.quiet_for() >= Duration::from_millis(1500));
        log.record(&RunEvent::Step { step: Step::Study, text: "Looking".into() });
        assert!(activity.quiet_for() < Duration::from_millis(1500));
    }

    #[test]
    fn the_builders_checklist_is_one_list_updated_in_place() {
        let todo = |status: &str| {
            json!({"todos": [
                {"content": "Lay out the arena", "activeForm": "Laying out the arena", "status": "completed"},
                {"content": "Add the snake", "activeForm": "Adding the snake", "status": status},
            ]})
        };
        let call = |status: &str| RunEvent::Agent(AgentEvent::ToolStarted { call_id: "t".into(), tool: "TodoWrite".into(), input: todo(status) });
        let mut feed = vec![FeedItem::User(UserItem::Text("A snake game".into()))];
        apply(&mut feed, &call("in_progress"));
        apply(&mut feed, &RunEvent::Agent(AgentEvent::TextDelta { text: "Working.".into() }));
        apply(&mut feed, &call("completed"));
        let plans: Vec<_> = feed.iter().filter_map(|f| if let FeedItem::Plan(p) = f { Some(p) } else { None }).collect();
        assert_eq!(plans.len(), 1, "one checklist, updated in place");
        assert_eq!(plans[0][0].doing, "Laying out the arena");
        assert!(plans[0].iter().all(|s| s.status == PlanStatus::Done));
        assert!(!feed.iter().any(|f| matches!(f, FeedItem::Work { .. })), "the checklist isn't tool work");
        assert!(plan_from(&json!({"todos": "nope"})).is_none());
        // An API builder's build_checklist is the same list.
        let mut feed2 = Vec::new();
        apply(&mut feed2, &RunEvent::Agent(AgentEvent::ToolStarted { call_id: "c".into(), tool: "build_checklist".into(), input: todo("in_progress") }));
        assert!(matches!(feed2.as_slice(), [FeedItem::Plan(p)] if p.len() == 2));
    }

    /// #504: a folded checklist doesn't come back by being updated; the next one a run writes is
    /// a new card at the end, and there's only ever one checklist in the feed.
    #[test]
    fn a_folded_checklist_gives_way_to_the_next_one() {
        let todo = |status: &str| json!({"todos": [{"content": "Lay out the arena", "activeForm": "Laying out the arena", "status": status}]});
        let call = |status: &str| RunEvent::Agent(AgentEvent::ToolStarted { call_id: "t".into(), tool: "TodoWrite".into(), input: todo(status) });
        let plans = |f: &[FeedItem]| f.iter().filter(|i| matches!(i, FeedItem::Plan(_))).count();
        let mut feed = vec![FeedItem::User(UserItem::Text("A game".into()))];
        apply(&mut feed, &call("in_progress"));
        apply(&mut feed, &call("completed"));
        assert_eq!(plans(&feed), 1, "updates in place while the list isn't folded");
        assert!(craft_anything_engine::feed::dismiss_plan(&mut feed), "the list folds");
        assert!(craft_anything_engine::feed::visible_plan(&feed).is_none());
        apply(&mut feed, &call("in_progress"));
        assert_eq!(plans(&feed), 1, "one checklist, not one folded and one new");
        assert!(!feed.iter().any(|i| matches!(i, FeedItem::PlanDismissed)), "the fold went with the old list");
        assert!(craft_anything_engine::feed::visible_plan(&feed).is_some(), "the new list shows");
        assert!(matches!(feed.last(), Some(FeedItem::Plan(p)) if p[0].status == PlanStatus::Doing));
    }

    /// #504: a run that ends with steps still open marks them stopped on the plan card instead of
    /// leaving them "Doing"; a paused run stops them too, and the next run's checklist takes over.
    #[test]
    fn a_run_that_ends_marks_open_steps_stopped() {
        let todo = |status: &str| {
            json!({"todos": [
                {"content": "Lay out the arena", "activeForm": "Laying out the arena", "status": "completed"},
                {"content": "Add the snake", "activeForm": "Adding the snake", "status": status},
                {"content": "Keep score", "activeForm": "Keeping score", "status": "pending"},
            ]})
        };
        let call = |status: &str| RunEvent::Agent(AgentEvent::ToolStarted { call_id: "t".into(), tool: "TodoWrite".into(), input: todo(status) });
        let steps = |f: &[FeedItem]| craft_anything_engine::feed::visible_plan(f).unwrap().to_vec();
        let mut feed = vec![FeedItem::User(UserItem::Text("A snake game".into()))];
        apply(&mut feed, &call("in_progress"));
        apply(&mut feed, &RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        let after = steps(&feed);
        assert_eq!(after[0].status, PlanStatus::Done, "done steps stay done");
        assert_eq!(after[1].status, PlanStatus::Stopped, "the step left open stops");
        assert_eq!(after[2].status, PlanStatus::Todo, "never-started steps stay as they were");
        // A paused run stops the open step too, and the next run's checklist takes the list over.
        let mut paused = vec![FeedItem::User(UserItem::Text("A snake game".into()))];
        apply(&mut paused, &call("in_progress"));
        apply(&mut paused, &RunEvent::Finished { ok: true, message: "Paused.".into(), paused: true });
        assert!(steps(&paused).iter().any(|s| s.status == PlanStatus::Stopped), "pausing stops the open step too");
        apply(&mut paused, &call("in_progress"));
        assert!(steps(&paused).iter().any(|s| s.status == PlanStatus::Doing), "the next run's checklist takes over");
        // No checklist: nothing to mark, nothing to lose.
        let mut bare = vec![FeedItem::User(UserItem::Text("A game".into()))];
        apply(&mut bare, &RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        assert!(matches!(bare.last(), Some(FeedItem::User(_))));
    }

    #[test]
    fn a_run_keeps_the_feed_in_the_project() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "Clone it", TargetRef::None).unwrap();
        let mut log = FeedLog::open(&ws);
        log.push(FeedItem::User(UserItem::Text("Make it blue".into())));
        log.record(&RunEvent::Agent(AgentEvent::TextDelta { text: "Done.".into() }));
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        assert!(matches!(load(&ws).as_slice(), [FeedItem::User(a), FeedItem::User(b), FeedItem::Agent { text, streaming: false }]
            if a.text() == "Clone it" && b.text() == "Make it blue" && text == "Done."));
        // An answer written to the file by someone else while it runs survives the run's saves.
        let inbox = Inbox::new(ws.inbox_dir());
        let id = inbox.post(craft_anything_engine::inbox::RequestKind::Question, "Blue?", "", vec![], true, None).unwrap();
        let q = inbox.pending().into_iter().find(|r| r.id == id).unwrap();
        log.record(&RunEvent::Ask(q.clone()));
        record_answer(&ws, &q, "Yes").unwrap();
        inbox.answer(&id, &craft_anything_engine::inbox::Answer { text: Some("Yes".into()), ..Default::default() }).unwrap();
        log.record(&RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        assert!(load(&ws).iter().any(|i| matches!(i, FeedItem::Ask { answer: Some(a), .. } if a == "Yes")));
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

    /// The CLI owns this same RebuildRun; events go through the worker's real emit function.
    fn owned_run(ws: &Workspace) -> (RebuildRun, Sender<RunEvent>) {
        let (tx, rx) = std::sync::mpsc::channel();
        let feed = Arc::new(Mutex::new(FeedLog::open(ws)));
        let activity = feed.lock().unwrap_or_else(PoisonError::into_inner).activity();
        let run = RebuildRun {
            rx,
            stop: CancelToken::default(),
            inbox: Inbox::new(ws.inbox_dir()),
            running: Arc::new(AtomicBool::new(true)),
            preview: Arc::default(),
            notes: Arc::default(),
            clean_room: false,
            feed,
            root: ws.root.clone(),
            started: Instant::now(),
            activity,
        };
        (run, tx)
    }

    fn emit(run: &RebuildRun, tx: &Sender<RunEvent>, event: RunEvent) {
        let done = matches!(event, RunEvent::Finished { .. });
        super::super::emit_event(&run.feed, tx, &Wake::default(), event);
        if done {
            run.running.store(false, Ordering::SeqCst);
        }
    }

    fn copies(ws: &Workspace) -> Vec<PathBuf> {
        std::fs::read_dir(ws.inbox_dir().join("feed-recovery"))
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.file_name().unwrap().to_string_lossy().ends_with(".recovery.json"))
            .collect()
    }

    #[test]
    fn final_feed_recovery_survives_cli_owner_release() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let main = ws.root.join(FEED_FILE);
        std::fs::create_dir(&main).unwrap();
        let (mut run, tx) = owned_run(&ws);
        emit(&run, &tx, RunEvent::Agent(AgentEvent::TextDelta { text: "Keep the finished reply.".into() }));
        emit(
            &run,
            &tx,
            RunEvent::Agent(AgentEvent::ToolStarted {
                call_id: "build".into(),
                tool: "build_check".into(),
                input: json!({"private_argument": "Do not retain raw tool input"}),
            }),
        );
        emit(
            &run,
            &tx,
            RunEvent::Agent(AgentEvent::ToolFinished {
                call_id: "build".into(),
                tool: "build_check".into(),
                ok: true,
                summary: "Do not retain the raw result".into(),
                evidence_id: None,
            }),
        );
        emit(&run, &tx, RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });

        let paths = copies(&ws);
        assert_eq!(paths.len(), 1, "the copy is durable before the owner can receive Finished");
        let path = &paths[0];
        let bytes = std::fs::read(path).unwrap();
        let snapshot: Value = serde_json::from_slice(&bytes).unwrap();
        let items: Vec<FeedItem> = serde_json::from_value(snapshot["items"].clone()).unwrap();
        assert_eq!(snapshot["version"], 1);
        assert_eq!(snapshot["metadata"]["source"]["state"], "unreadable");
        assert!(snapshot["metadata"]["details"]["pending_event_count"].as_u64().unwrap() >= 4);
        assert!(items.iter().any(|item| matches!(item, FeedItem::Agent { text, streaming: false } if text == "Keep the finished reply.")));
        let json = String::from_utf8(bytes.clone()).unwrap();
        assert!(!json.contains("Do not retain raw tool input") && !json.contains("Do not retain the raw result"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(std::fs::metadata(path).unwrap().permissions().mode() & 0o777, 0o600);
        }
        assert!(run.poll().iter().any(|event| matches!(event, RunEvent::Finished { message, .. }
            if message.contains("Couldn't save the project's conversation") && message.contains(path.to_str().unwrap()))));
        drop(run);
        drop(tx);

        // The file becomes usable only after the CLI released the run. Opening it exposes the
        // retained copy but never overwrites or imports over a different saved conversation.
        std::fs::remove_dir(&main).unwrap();
        let external = br#"{"version":5,"items":[{"User":"The other client's words"}]}"#;
        std::fs::write(&main, external).unwrap();
        let reopened = load(&Workspace::open(&ws.root).unwrap());
        assert!(reopened.iter().any(|item| matches!(item, FeedItem::Error { log: Some(saved), .. } if saved == path)));
        assert!(!reopened.iter().any(|item| matches!(item, FeedItem::Agent { text, .. } if text == "Keep the finished reply.")));
        assert_eq!(std::fs::read(&main).unwrap(), external);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn final_feed_recovery_survives_a_runset_replacement() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        std::fs::create_dir(ws.root.join(FEED_FILE)).unwrap();
        let mut session = craft_anything_engine::testing::session();
        session.settings.ai = toml::from_str("[[backends]]\nname = \"Agent\"\nkind = \"agent_cli\"\nmodel = \"claude-code\"\n").unwrap();
        let hub = AiHub::new(Arc::new(Engine::new(session)));
        let senders = Arc::new(Mutex::new(Vec::new()));
        let controls = senders.clone();
        let mut set = RunSet::new(1).with_starter(Box::new(move |_, ws, _| {
            let (run, tx) = owned_run(&ws);
            controls.lock().unwrap_or_else(PoisonError::into_inner).push(tx);
            Ok(run)
        }));
        set.start(&hub, ws.clone(), RunOptions::default()).unwrap();
        let run = set.get(&ws.root).unwrap();
        let old_feed = Arc::downgrade(&run.feed);
        let tx = senders.lock().unwrap_or_else(PoisonError::into_inner)[0].clone();
        emit(run, &tx, RunEvent::Note { shown: "Keep the earlier run's words".into(), pictures: Vec::new() });
        emit(run, &tx, RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        assert!(set.poll(&hub).iter().any(|(_, event)| matches!(event, RunEvent::Finished { .. })));
        let paths = copies(&ws);
        assert_eq!(paths.len(), 1);
        let bytes = std::fs::read(&paths[0]).unwrap();

        set.start(&hub, ws.clone(), RunOptions::default()).unwrap();
        assert!(old_feed.upgrade().is_none(), "the replaced run really releases its only in-memory conversation");
        drop(set);
        assert_eq!(std::fs::read(&paths[0]).unwrap(), bytes);
        let snapshot: Value = serde_json::from_slice(&bytes).unwrap();
        let items: Vec<FeedItem> = serde_json::from_value(snapshot["items"].clone()).unwrap();
        assert!(items.iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the earlier run's words")));
    }

    #[test]
    fn final_feed_recovery_reports_an_unavailable_archive() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        std::fs::create_dir(ws.root.join(FEED_FILE)).unwrap();
        std::fs::write(ws.inbox_dir(), b"An unrelated existing file").unwrap();
        let (mut run, tx) = owned_run(&ws);
        emit(&run, &tx, RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false });
        let events = run.poll();
        let message = events.iter().find_map(|event| if let RunEvent::Finished { message, .. } = event { Some(message) } else { None }).unwrap();
        assert!(message.contains("Couldn't keep a conversation recovery copy"), "{message}");
        assert!(message.contains("has not been preserved on disk"), "{message}");
        assert!(!message.contains("was kept at"));
        assert!(copies(&ws).is_empty());
        assert_eq!(std::fs::read(ws.inbox_dir()).unwrap(), b"An unrelated existing file");
    }

    #[test]
    fn final_feed_recovery_does_not_archive_successful_saves() {
        let d = tempfile::tempdir().unwrap();
        let ws = Workspace::create(d.path(), "x", "A game", TargetRef::None).unwrap();
        let (mut run, tx) = owned_run(&ws);
        emit(&run, &tx, RunEvent::Note { shown: "Keep the saved words".into(), pictures: Vec::new() });
        let finished = RunEvent::Finished { ok: true, message: "Ready.".into(), paused: false };
        emit(&run, &tx, finished.clone());
        assert!(run.poll().contains(&finished));
        assert!(copies(&ws).is_empty());
        drop(run);
        assert!(load(&ws).iter().any(|item| matches!(item, FeedItem::User(note) if note.text() == "Keep the saved words")));
    }
}
