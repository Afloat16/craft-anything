use super::*;
use craft_anything_engine::workspace::WebsiteLanguage;

fn note(id: u64, text: &str) -> Note {
    Note { id, text: text.into(), shown: text.into(), pictures: Vec::new(), questions: Vec::new() }
}

#[test]
fn website_language_follows_delivered_notes_and_survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let mut ws = Workspace::create(dir.path(), "Site", "Rebuild this website in French", TargetRef::None).unwrap();
    let website = kind("website");
    website.record_requests(&mut ws, &[]).unwrap();
    let mut worker = check_worker(&ws, false);
    worker.self_check = Some((website.clone(), 90.0));
    let inbox = notes::notes_dir(&ws.inbox_dir());
    let german = "Set the page language to de";
    worker.notes.lock().unwrap().push(note(71, german));
    notes::hand_over(&inbox, 71, german, None).unwrap();
    worker.relay_read_notes(&mut ws).unwrap();
    assert_eq!(ws.manifest.website_language, Some(WebsiteLanguage::Requested { tag: "fr".into() }), "queued notes are still retractable");
    assert_eq!(notes::claim(&inbox), [german]);
    worker.relay_read_notes(&mut ws).unwrap();
    assert_eq!(Workspace::open(&ws.root).unwrap().manifest.website_language, Some(WebsiteLanguage::Requested { tag: "de".into() }));
    assert!(worker.notes.lock().unwrap().is_empty());
    // A following turn consumes these together, in delivery order; the final explicit request wins.
    for (id, text) in [(72, "Translate the site into Italian"), (73, "Restore the original language"), (74, "Make the header blue")] {
        worker.notes.lock().unwrap().push(note(id, text));
        notes::hand_over(&inbox, id, text, None).unwrap();
    }
    let (prompt, _) = worker.added(&mut ws, false, true).unwrap().unwrap();
    assert!(prompt.contains("Restore the original language"));
    assert!(notes::claim(&inbox).is_empty(), "a hook cannot deliver the same notes again");
    ws.reload().unwrap();
    website.record_requests(&mut ws, &[]).unwrap();
    assert_eq!(ws.manifest.website_language, Some(WebsiteLanguage::Original));
}

#[test]
fn website_language_write_failure_keeps_notes_and_refuses_the_check() {
    for claimed in [false, true] {
        let dir = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(dir.path(), "Site", "Rebuild this website in French", TargetRef::None).unwrap();
        let website = kind("website");
        website.record_requests(&mut ws, &[]).unwrap();
        let before = ws.manifest.website_language.clone();
        let mut worker = check_worker(&ws, false);
        worker.self_check = Some((website, 90.0));
        let inbox = notes::notes_dir(&ws.inbox_dir());
        worker.notes.lock().unwrap().push(note(71, "Translate the site into German"));
        notes::hand_over(&inbox, 71, "Translate the site into German", None).unwrap();
        if claimed {
            notes::claim(&inbox);
        }
        // A deterministic write refusal, including on privileged test runners.
        ws.manifest.version = craft_anything_engine::workspace::MANIFEST_VERSION + 1;
        let error = if claimed { worker.relay_read_notes(&mut ws).unwrap_err() } else { worker.added(&mut ws, false, true).unwrap_err() };
        assert_eq!(error.code, ErrorCode::NotSupported);
        assert_eq!(ws.manifest.website_language, before);
        assert_eq!(Workspace::open(&ws.root).unwrap().manifest.website_language, before);
        assert_eq!(worker.notes.lock().unwrap().len(), 1, "the failed save must not discard the note");
        assert!(if claimed { notes::was_read(&inbox, 71) } else { notes::claim(&inbox).len() == 1 });
    }
}
