#![cfg(unix)]

use craft_anything_engines::gdb::GdbSession;
use std::os::unix::fs::PermissionsExt;
use std::time::{Duration, Instant};
use tempfile::TempDir;

struct Fixture {
    session: GdbSession,
    dir: TempDir,
}

fn fixture_script(script: &str) -> Fixture {
    let dir = tempfile::tempdir().unwrap();
    let executable = dir.path().join("gdb");
    std::fs::write(&executable, script).unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o755)).unwrap();
    let session = GdbSession::start(&executable, None, None, &[]).unwrap();
    Fixture { session, dir }
}

fn fixture(records: &str) -> Fixture {
    fixture_script(&format!(
        "#!/bin/sh\nwhile IFS= read -r line; do\ncase \"$line\" in\n2-second)\ncat <<'RECORDS'\n{records}\nRECORDS\n;;\n*-gdb-exit) exit 0;;\nesac\ndone\n"
    ))
}

fn leave_first_pending(f: &mut Fixture, cancel: bool) {
    let timeout = if cancel { Duration::from_secs(5) } else { Duration::ZERO };
    let (c, waiting) = f.session.exec("-first", timeout, &|| cancel).unwrap();
    assert!(waiting);
    assert!(c.result.is_none());
    assert!(c.error.is_none());
}

#[test]
fn a_late_error_after_a_timeout_does_not_poison_a_successful_command() {
    let mut f = fixture("1^error,msg=\"earlier failure\"\n2^done");
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("done"));
    assert!(c.error.is_none(), "{c:?}");
}

#[test]
fn a_late_error_after_cancellation_does_not_poison_a_successful_command() {
    let mut f = fixture("1^error,msg=\"cancelled command failed later\"\n2^done");
    leave_first_pending(&mut f, true);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("done"));
    assert!(c.error.is_none(), "{c:?}");
}

#[test]
fn an_old_success_is_not_a_result_for_a_command_still_waiting() {
    let mut f = fixture("1^done");
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_millis(300), &|| false).unwrap();
    assert!(waiting);
    assert!(c.result.is_none(), "{c:?}");
    assert!(c.error.is_none());
}

#[test]
fn a_running_result_is_not_rewritten_by_an_older_error_before_the_stop() {
    let mut f = fixture("2^running\n1^error,msg=\"earlier failure\"\n*stopped,reason=\"breakpoint-hit\",frame={func=\"main\"}");
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("running"));
    assert!(c.error.is_none(), "{c:?}");
    assert_eq!(f.session.state, "stopped");
    assert_eq!(f.session.last_stop.as_ref().unwrap()["reason"], "breakpoint-hit");
    assert_eq!(c.events.len(), 1);
}

#[test]
fn an_untokened_error_is_not_attributed_to_the_current_command() {
    let mut f = fixture("^error,msg=\"unattributed failure\"\n2^done");
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("done"));
    assert!(c.error.is_none(), "{c:?}");
}

#[test]
fn the_current_commands_error_is_retained() {
    let mut f = fixture("1^error,msg=\"earlier failure\"\n2^error,msg=\"current failure\"");
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("error"));
    assert_eq!(c.error.as_deref(), Some("current failure"));
}

#[test]
fn stream_and_async_records_keep_their_existing_session_meaning() {
    let mut f = fixture(
        "1^done\n~\"console\\n\"\n&\"log\\n\"\n@\"target\\n\"\n=thread-created,id=\"7\"\n*running,thread-id=\"all\"\n*stopped,reason=\"exited-normally\"\n2^done",
    );
    leave_first_pending(&mut f, false);
    let (c, waiting) = f.session.exec("-second", Duration::from_secs(5), &|| false).unwrap();
    assert!(!waiting);
    assert_eq!(c.result.as_deref(), Some("done"));
    assert!(c.error.is_none());
    assert_eq!(c.console, "console\n");
    assert_eq!(c.log, "log\n");
    assert_eq!(c.target_output, "target\n");
    assert_eq!(c.events.len(), 3);
    assert_eq!(f.session.state, "exited");
    assert_eq!(f.session.last_stop.as_ref().unwrap()["reason"], "exited-normally");
}

#[test]
fn take_output_still_collects_a_late_result_without_a_command_filter() {
    let mut f = fixture_script(
        "#!/bin/sh\nwhile IFS= read -r line; do\ncase \"$line\" in\n1-first)\nwhile [ ! -f \"$(dirname \"$0\")/release\" ]; do sleep 0.01; done\nprintf '1^error,msg=\"late status error\"\\n'\n;;\n*-gdb-exit) exit 0;;\nesac\ndone\n",
    );
    leave_first_pending(&mut f, false);
    std::fs::write(f.dir.path().join("release"), []).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let (c, dropped) = f.session.take_output();
        assert_eq!(dropped, 0);
        if c.result.is_some() {
            assert_eq!(c.result.as_deref(), Some("error"));
            assert_eq!(c.error.as_deref(), Some("late status error"));
            break;
        }
        assert!(Instant::now() < deadline, "the released fixture did not return its result");
        std::thread::sleep(Duration::from_millis(5));
    }
}
