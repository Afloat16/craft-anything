//! A run's side of the project conversation: folding workflow events into the feed and keeping
//! it saved as the run goes. The feed itself (its items and file) is `engine::feed`.

pub use craft_anything_engine::feed::*;

use super::activity::{Activity, ActivityFile};
use super::*;

/// Fold one workflow event into the feed.
pub fn apply(feed: &mut Vec<FeedItem>, e: &RunEvent) {
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
            let explained = matches!(feed.last(), Some(FeedItem::Error { .. }));
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
    saved_at: Option<std::time::Instant>,
    activity: Arc<Activity>,
    log: ActivityFile,
}

impl FeedLog {
    pub(super) fn open(ws: &Workspace) -> FeedLog {
        let activity = Arc::new(Activity::new(vec![ws.app_dir(), ws.root.clone()]));
        FeedLog { root: ws.root.clone(), feed: load(ws), saved_at: None, activity, log: ActivityFile::new(&ws.root) }
    }

    pub(super) fn activity(&self) -> Arc<Activity> {
        self.activity.clone()
    }

    pub(super) fn record(&mut self, e: &RunEvent) {
        let lines = self.activity.record(e);
        self.log.append(&lines, &self.activity);
        apply(&mut self.feed, e);
        self.save(matches!(e, RunEvent::Finished { .. }));
    }

    pub(super) fn items(&self) -> &[FeedItem] {
        &self.feed
    }

    pub(super) fn push(&mut self, item: FeedItem) {
        self.feed.push(item);
        self.save(true);
    }

    pub(super) fn answered(&mut self, request: &Request, shown: &str, deliver: impl FnOnce() -> Result<(), ToolError>) -> Result<(), ToolError> {
        let mut candidate = self.feed.clone();
        merge_saved_answers(&self.root, &mut candidate);
        record_pending_answer(&mut candidate, request, shown);
        save(&self.root, &mut candidate)?;
        self.feed = candidate;
        self.saved_at = Some(std::time::Instant::now());
        // The raw saved state (including recovery from damaged bytes) is now committed. The
        // display still offers this request while its inbox delivery is pending or fails.
        deliver()
    }

    fn save(&mut self, now: bool) {
        if !now && self.saved_at.is_some_and(|t| t.elapsed() < Duration::from_secs(2)) {
            return;
        }
        // Answers given elsewhere while it runs (the CLI, an agent's project_inbox_answer) went
        // into the file; keep them.
        merge_saved_answers(&self.root, &mut self.feed);
        if let Err(e) = save(&self.root, &mut self.feed) {
            log::warn!("couldn't save the project's feed: {}", e.message);
            let notice = FeedItem::Error { message: format!("Couldn't save the project's conversation: {}", e.message), remediation: e.remediation, log: None };
            if !self.feed.contains(&notice) {
                self.feed.push(notice);
            }
        }
        self.saved_at = Some(std::time::Instant::now());
    }
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
        assert!(
            log.answered(&question, "Blue", || {
                delivered = true;
                Ok(())
            })
            .is_err(),
            "the old fallback still isn't a loaded conversation"
        );
        assert!(!delivered);
        let mut reopened = FeedLog::open(&ws);
        reopened
            .answered(&question, "Blue", || inbox.answer(&id, &craft_anything_engine::inbox::Answer { choice: Some("Blue".into()), ..Default::default() }))
            .unwrap();
        assert!(inbox.pending().is_empty());
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
