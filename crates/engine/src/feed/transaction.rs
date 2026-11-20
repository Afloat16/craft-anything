//! One project's feed writers and recovery publication share a bounded, cross-process lock.

use craft_anything_evidence::{ErrorCode, ToolError};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const WAIT: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(20);

/// A non-reentrant project feed transaction. Dropping the owning file releases the OS lock.
/// Acquire request locks before this guard; never acquire a request lock while holding it.
/// The empty lock file is retained so waiters always coordinate through the same file.
#[derive(Debug)]
pub struct ProjectFeedGuard {
    pub(super) root: PathBuf,
    _file: same_file::Handle,
}

/// Coordinate feed comparison, replacement and backup publication across application processes.
/// Do not call another locking feed operation while this guard is held.
pub fn lock_project(root: &Path) -> Result<ProjectFeedGuard, ToolError> {
    lock_for(root, WAIT)
}

fn lock_for(root: &Path, wait: Duration) -> Result<ProjectFeedGuard, ToolError> {
    if !root.is_dir() {
        return Err(ToolError::new(ErrorCode::FileError, "The project's folder isn't available, so its conversation couldn't be locked.")
            .with_remediation("Restore the project folder, then reopen the project before retrying."));
    }
    let directory = root.join("inbox");
    match std::fs::symlink_metadata(&directory) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            #[cfg(unix)]
            let builder = {
                use std::os::unix::fs::DirBuilderExt as _;
                let mut builder = std::fs::DirBuilder::new();
                builder.mode(0o700);
                builder
            };
            #[cfg(not(unix))]
            let builder = std::fs::DirBuilder::new();
            match builder.create(&directory) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(ToolError::io(directory.display(), error)),
            }
        }
        Ok(_) => {}
        Err(error) => return Err(ToolError::io(directory.display(), error)),
    }
    let metadata = std::fs::symlink_metadata(&directory).map_err(|error| ToolError::io(directory.display(), error))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(invalid(&directory));
    }
    let path = directory.join("feed.lock");
    let started = Instant::now();
    let file = loop {
        let absent = match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() == 0 => false,
            Ok(_) => return Err(invalid(&path)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
            Err(error) => return Err(ToolError::io(path.display(), error)),
        };
        // Exclusive locks may be emulated as write locks on network filesystems.
        let mut options = std::fs::OpenOptions::new();
        options.read(true).write(true).create_new(absent);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(file) => break file,
            Err(error) if absent && error.kind() == std::io::ErrorKind::AlreadyExists && started.elapsed() < wait => {}
            Err(error) => return Err(ToolError::io(path.display(), error)),
        }
    };
    let file = same_file::Handle::from_file(file).map_err(|error| ToolError::io(path.display(), error))?;
    check_entry(&path, &file)?;
    loop {
        match file.as_file().try_lock() {
            Ok(()) => {
                check_entry(&path, &file)?;
                return Ok(ProjectFeedGuard { root: root.to_path_buf(), _file: file });
            }
            Err(std::fs::TryLockError::WouldBlock) if started.elapsed() < wait => std::thread::sleep(POLL.min(wait.saturating_sub(started.elapsed()))),
            Err(std::fs::TryLockError::WouldBlock) => {
                return Err(ToolError::new(ErrorCode::TimedOut, "Another operation is updating or copying this project's conversation.")
                    .with_remediation("Let that operation finish, then retry. This operation has not changed the conversation."));
            }
            Err(std::fs::TryLockError::Error(error)) => return Err(ToolError::io(path.display(), error)),
        }
    }
}

fn invalid(path: &Path) -> ToolError {
    ToolError::new(ErrorCode::FileError, format!("{} is not a regular conversation coordination entry.", path.display()))
        .with_remediation("Keep the existing entry unchanged and restore access to the project's inbox before retrying.")
}

fn check_entry(path: &Path, file: &same_file::Handle) -> Result<(), ToolError> {
    let opened = file.as_file().metadata().map_err(|error| ToolError::io(path.display(), error))?;
    let current = std::fs::symlink_metadata(path).map_err(|error| ToolError::io(path.display(), error))?;
    if !opened.is_file()
        || opened.len() != 0
        || !current.is_file()
        || current.file_type().is_symlink()
        || current.len() != 0
        || opened.created().ok() != current.created().ok()
        || opened.modified().ok() != current.modified().ok()
    {
        return Err(invalid(path));
    }
    let (current, _) = craft_anything_platform::fs::open_regular_file(path)?;
    let current = same_file::Handle::from_file(current).map_err(|error| ToolError::io(path.display(), error))?;
    if &current != file {
        return Err(ToolError::new(ErrorCode::FileChanged, "The conversation's coordination file changed while it was being opened.")
            .with_remediation("Reopen the project before retrying. Do not remove its coordination file while an operation is active."));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feed_lock_child() {
        let Ok(root) = std::env::var("CRAFT_TEST_FEED_LOCK_ROOT") else { return };
        let result = lock_for(Path::new(&root), Duration::from_millis(50));
        assert_eq!(result.unwrap_err().code, ErrorCode::TimedOut);
    }

    #[test]
    fn the_feed_lock_coordinates_processes_and_releases_on_drop() {
        let directory = tempfile::tempdir().unwrap();
        let guard = lock_project(directory.path()).unwrap();
        craft_anything_platform::Process::new(std::env::current_exe().unwrap())
            .args(["--exact", "feed::transaction::tests::feed_lock_child", "--test-threads=1"])
            .env("CRAFT_TEST_FEED_LOCK_ROOT", directory.path())
            .timeout(Duration::from_secs(5))
            .stdout_cap(64 * 1024)
            .stderr_cap(64 * 1024)
            .run_checked()
            .unwrap();
        assert_eq!(lock_for(directory.path(), Duration::ZERO).unwrap_err().code, ErrorCode::TimedOut);
        let path = directory.path().join("inbox/feed.lock");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
        }
        drop(guard);
        assert!(path.is_file(), "releasing the lock does not replace the coordination file");
        drop(lock_project(directory.path()).unwrap());
    }

    #[test]
    fn coordination_preserves_unrecognized_entries_and_missing_projects() {
        let directory = tempfile::tempdir().unwrap();
        let missing = directory.path().join("missing");
        assert!(lock_project(&missing).is_err());
        assert!(!missing.exists());
        let inbox = directory.path().join("inbox");
        std::fs::create_dir(&inbox).unwrap();
        let path = inbox.join("feed.lock");
        std::fs::write(&path, b"Keep these words").unwrap();
        assert!(lock_project(directory.path()).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"Keep these words");
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        assert!(lock_project(directory.path()).is_err());
        assert!(path.is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn coordination_does_not_follow_existing_links() {
        let directory = tempfile::tempdir().unwrap();
        let inbox = directory.path().join("inbox");
        std::fs::create_dir(&inbox).unwrap();
        let path = inbox.join("feed.lock");
        let target = directory.path().join("personal");
        std::fs::write(&target, b"").unwrap();
        std::os::unix::fs::symlink(&target, &path).unwrap();
        assert!(lock_project(directory.path()).is_err());
        assert_eq!(std::fs::read_link(path).unwrap(), target);
        assert_eq!(std::fs::read(target).unwrap(), b"");
    }
}
