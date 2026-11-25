//! One project's feed writers and recovery publication share a bounded, cross-process lock.

use craft_anything_evidence::{ErrorCode, ToolError};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const WAIT: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(20);
pub(crate) const PROJECT_LOCK_FILE: &str = "feed.lock";

/// A non-reentrant project feed transaction. Dropping the owning file releases the OS lock.
/// Acquire request locks before this guard; never acquire a request lock while holding it.
/// The empty lock file is retained until project deletion removes its manifest first.
#[derive(Debug)]
pub struct ProjectFeedGuard {
    pub(super) root: PathBuf,
    project: ProjectIdentity,
    _file: same_file::Handle,
}

impl ProjectFeedGuard {
    pub(super) fn check_project(&self) -> Result<(), ToolError> {
        self.project.check(&self.root)
    }

    /// A lock acquired by pathname also has to match the workspace the caller loaded.
    pub(crate) fn check_workspace(&self, ws: &crate::workspace::Workspace) -> Result<(), ToolError> {
        self.check_project()?;
        if self.project.same(&ProjectIdentity::from_workspace(ws)?) { Ok(()) } else { Err(project_changed()) }
    }

    /// Preserve the held coordination inode even when the filesystem spells its name differently.
    pub(crate) fn is_lock_entry(&self, path: &Path) -> Result<bool, ToolError> {
        check_entry(&self.root.join("inbox").join(PROJECT_LOCK_FILE), &self._file)?;
        let metadata = std::fs::symlink_metadata(path).map_err(|error| ToolError::io(path.display(), error))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() != 0 {
            return Ok(false);
        }
        let (file, _) = craft_anything_platform::fs::open_regular_file(path)?;
        let file = same_file::Handle::from_file(file).map_err(|error| ToolError::io(path.display(), error))?;
        Ok(file == self._file)
    }

    pub(crate) fn finish_deletion(&self, inbox: &same_file::Handle, after_claim: impl FnOnce()) -> Result<(), ToolError> {
        crate::workspace::finish_deleted_inbox(&self.root, &self.project.directory, inbox, &self._file, after_claim)
    }
}

/// A read-only copy holds a shared lock, but cannot supply a writer's transaction guard.
#[derive(Debug)]
pub(crate) struct ProjectCopyGuard(ProjectFeedGuard);

impl ProjectCopyGuard {
    pub(crate) fn check_workspace(&self, ws: &crate::workspace::Workspace) -> Result<(), ToolError> {
        self.0.check_workspace(ws)
    }
}

#[derive(Debug)]
pub(super) struct ProjectIdentity {
    directory: same_file::Handle,
    created_ms: u64,
    generation: Option<String>,
}

impl ProjectIdentity {
    fn read(root: &Path) -> Result<Self, ToolError> {
        Self::from_workspace(&crate::workspace::Workspace::open(root)?)
    }

    pub(super) fn from_workspace(ws: &crate::workspace::Workspace) -> Result<Self, ToolError> {
        let root = &ws.root;
        if !root.is_dir() {
            return Err(ToolError::new(ErrorCode::FileError, "The project's folder isn't available, so its conversation couldn't be locked.")
                .with_remediation("Restore the project folder, then reopen the project before retrying."));
        }
        let directory = same_file::Handle::from_path(root).map_err(|error| ToolError::io(root.display(), error))?;
        let created_ms = ws.manifest.created_ms;
        let generation = ws.manifest.generation.clone();
        Ok(Self { directory, created_ms, generation })
    }

    pub(super) fn same(&self, other: &Self) -> bool {
        self.directory == other.directory && self.created_ms == other.created_ms && self.generation == other.generation
    }

    pub(super) fn check(&self, root: &Path) -> Result<(), ToolError> {
        let current = Self::read(root)?;
        if !self.same(&current) {
            return Err(project_changed());
        }
        Ok(())
    }
}

fn project_changed() -> ToolError {
    ToolError::new(ErrorCode::FileChanged, "The project changed while its conversation operation was waiting.")
        .with_remediation("Open the current project before retrying. This operation has not changed its conversation.")
}

/// Coordinate feed comparison, replacement and backup publication across application processes.
/// Do not call another locking feed operation while this guard is held.
pub fn lock_project(root: &Path) -> Result<ProjectFeedGuard, ToolError> {
    lock_for(root, WAIT)
}

/// Read an existing coordination file without creating or writing source entries.
pub(crate) fn lock_project_for_copy(root: &Path) -> Result<ProjectCopyGuard, ToolError> {
    copy_lock_for(root, WAIT)
}

fn copy_lock_for(root: &Path, wait: Duration) -> Result<ProjectCopyGuard, ToolError> {
    let project = ProjectIdentity::read(root)?;
    let directory = root.join("inbox");
    let metadata = std::fs::symlink_metadata(&directory).map_err(|error| ToolError::io(directory.display(), error))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(invalid(&directory));
    }
    let path = directory.join(PROJECT_LOCK_FILE);
    let metadata = std::fs::symlink_metadata(&path).map_err(|error| ToolError::io(path.display(), error))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() != 0 {
        return Err(invalid(&path));
    }
    let (file, _) = craft_anything_platform::fs::open_regular_file(&path)?;
    let file = same_file::Handle::from_file(file).map_err(|error| ToolError::io(path.display(), error))?;
    check_entry(&path, &file)?;
    wait_for_lock(file.as_file(), &path, Instant::now(), wait, true)?;
    check_entry(&path, &file)?;
    project.check(root)?;
    Ok(ProjectCopyGuard(ProjectFeedGuard { root: root.to_path_buf(), project, _file: file }))
}

fn lock_for(root: &Path, wait: Duration) -> Result<ProjectFeedGuard, ToolError> {
    lock_for_using(root, wait, || {})
}

fn lock_for_using(root: &Path, wait: Duration, after_identity: impl FnOnce()) -> Result<ProjectFeedGuard, ToolError> {
    lock_for_using_after_mkdir(root, wait, after_identity, || {})
}

fn lock_for_using_after_mkdir(root: &Path, wait: Duration, after_identity: impl FnOnce(), after_mkdir: impl FnOnce()) -> Result<ProjectFeedGuard, ToolError> {
    // Reject a project already gone before creating coordination entries. If deletion
    // overlaps initialization, only an inbox created by this attempt can be reclaimed.
    let project = ProjectIdentity::read(root)?;
    after_identity();
    let directory = root.join("inbox");
    let created_inbox = match std::fs::symlink_metadata(&directory) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // Generation-bearing projects reserve their inbox before publication. A missing
            // reservation is never recreated by a possibly stale writer.
            if project.generation.is_some() {
                return Err(project_changed());
            }
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
                Ok(()) => true,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
                Err(error) => return Err(ToolError::io(directory.display(), error)),
            }
        }
        Ok(_) => false,
        Err(error) => return Err(ToolError::io(directory.display(), error)),
    };
    let owned_inbox = if created_inbox {
        after_mkdir();
        let inbox = same_file::Handle::from_path(&directory).map_err(|error| ToolError::io(directory.display(), error))?;
        // mkdir does not return a handle. Deletion and another creator may have replaced
        // the entry before this first capture; failed identity confirmation grants no cleanup.
        project.check(root).map_err(|error| {
            let guidance = format!(
                "The new coordination directory could not be bound to the original project. Inspect {} before retrying; its current contents were kept.",
                directory.display()
            );
            let guidance = match &error.remediation {
                Some(previous) => format!("{previous} {guidance}"),
                None => guidance,
            };
            error.with_remediation(guidance)
        })?;
        Some(inbox)
    } else {
        None
    };
    let mut opened = None;
    let mut created_lock = false;
    let mut held_lock = false;
    let acquired = (|| {
        let metadata = std::fs::symlink_metadata(&directory).map_err(|error| ToolError::io(directory.display(), error))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(invalid(&directory));
        }
        // A known stale initializer must not add a lock to another creator's reservation.
        // This early refusal is not a replacement for the check under the acquired lock.
        project.check(root)?;
        let path = directory.join(PROJECT_LOCK_FILE);
        let started = Instant::now();
        let (file, absent) = loop {
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
                Ok(file) => break (file, absent),
                Err(error) if absent && error.kind() == std::io::ErrorKind::AlreadyExists && started.elapsed() < wait => {}
                Err(error) => return Err(ToolError::io(path.display(), error)),
            }
        };
        let file = opened.insert(same_file::Handle::from_file(file).map_err(|error| ToolError::io(path.display(), error))?);
        created_lock = absent;
        check_entry(&path, file)?;
        wait_for_lock(file.as_file(), &path, started, wait, false)?;
        held_lock = true;
        check_entry(&path, file)?;
        project.check(root)
    })();
    if let Err(error) = acquired {
        return Err(match owned_inbox {
            Some(inbox) => match crate::workspace::release_created_inbox(
                root,
                project.directory,
                inbox,
                opened.as_ref().filter(|_| created_lock),
                opened.as_ref().filter(|_| held_lock),
            ) {
                Ok(()) => error,
                Err(cleanup) => {
                    let guidance = format!(
                        "The initializer's coordination entry could not be fully released: {} Inspect {} and any retained path before retrying.",
                        cleanup.message,
                        directory.display()
                    );
                    let guidance = match &error.remediation {
                        Some(previous) => format!("{previous} {guidance}"),
                        None => guidance,
                    };
                    error.with_remediation(guidance)
                }
            },
            None => error,
        });
    }
    let file = opened.ok_or_else(|| ToolError::internal("The conversation lock was acquired without retaining its file."))?;
    Ok(ProjectFeedGuard { root: root.to_path_buf(), project, _file: file })
}

fn wait_for_lock(file: &std::fs::File, path: &Path, started: Instant, wait: Duration, shared: bool) -> Result<(), ToolError> {
    loop {
        let locked = if shared { file.try_lock_shared() } else { file.try_lock() };
        match locked {
            Ok(()) => return Ok(()),
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

/// Cleanup must not move an inbox out from under a deleter that still owns its lock.
/// `held` is supplied only by an initializer that has already acquired that exclusive handle.
pub(crate) fn lock_inbox_for_cleanup(root: &Path, held: Option<&same_file::Handle>) -> Result<Option<same_file::Handle>, ToolError> {
    let path = root.join("inbox").join(PROJECT_LOCK_FILE);
    if let Some(file) = held {
        check_entry(&path, file)?;
        return Ok(None);
    }
    match std::fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(ToolError::io(path.display(), error)),
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() == 0 => {}
        Ok(_) => return Err(invalid(&path)),
    }
    let file = std::fs::File::options().read(true).write(true).open(&path).map_err(|error| ToolError::io(path.display(), error))?;
    let file = same_file::Handle::from_file(file).map_err(|error| ToolError::io(path.display(), error))?;
    check_entry(&path, &file)?;
    wait_for_lock(file.as_file(), &path, Instant::now(), Duration::ZERO, false)?;
    check_entry(&path, &file)?;
    Ok(Some(file))
}

pub(crate) fn check_entry(path: &Path, file: &same_file::Handle) -> Result<(), ToolError> {
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
    use crate::workspace::{ExistingFiles, TargetRef, Workspace};

    #[test]
    fn feed_lock_child() {
        let Ok(root) = std::env::var("CRAFT_TEST_FEED_LOCK_ROOT") else { return };
        let result = lock_for(Path::new(&root), Duration::from_millis(50));
        assert_eq!(result.unwrap_err().code, ErrorCode::TimedOut);
    }

    #[test]
    fn the_feed_lock_coordinates_processes_and_releases_on_drop() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let guard = lock_project(&ws.root).unwrap();
        craft_anything_platform::Process::new(std::env::current_exe().unwrap())
            .args(["--exact", "feed::transaction::tests::feed_lock_child", "--test-threads=1"])
            .env("CRAFT_TEST_FEED_LOCK_ROOT", &ws.root)
            .timeout(Duration::from_secs(5))
            .stdout_cap(64 * 1024)
            .stderr_cap(64 * 1024)
            .run_checked()
            .unwrap();
        assert_eq!(lock_for(&ws.root, Duration::ZERO).unwrap_err().code, ErrorCode::TimedOut);
        let path = ws.root.join("inbox/feed.lock");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
        }
        drop(guard);
        assert!(path.is_file(), "releasing the lock does not replace the coordination file");
        drop(lock_project(&ws.root).unwrap());
    }

    #[test]
    fn a_read_only_copy_lock_excludes_writers_and_never_creates_entries() {
        use std::io::Write as _;
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let path = ws.inbox_dir().join(PROJECT_LOCK_FILE);
        assert!(copy_lock_for(&ws.root, Duration::ZERO).is_err());
        assert!(!path.exists(), "a read-only lock does not initialize coordination");
        drop(lock_project(&ws.root).unwrap());
        let identity = same_file::Handle::from_path(&path).unwrap();

        let first = copy_lock_for(&ws.root, Duration::ZERO).unwrap();
        let second = copy_lock_for(&ws.root, Duration::ZERO).unwrap();
        first.check_workspace(&ws).unwrap();
        assert!(first.0._file.as_file().write_all(b"not writable").is_err());
        craft_anything_platform::Process::new(std::env::current_exe().unwrap())
            .args(["--exact", "feed::transaction::tests::feed_lock_child", "--test-threads=1"])
            .env("CRAFT_TEST_FEED_LOCK_ROOT", &ws.root)
            .timeout(Duration::from_secs(5))
            .stdout_cap(64 * 1024)
            .stderr_cap(64 * 1024)
            .run_checked()
            .unwrap();
        assert_eq!(lock_for(&ws.root, Duration::ZERO).unwrap_err().code, ErrorCode::TimedOut);
        drop(first);
        assert_eq!(lock_for(&ws.root, Duration::ZERO).unwrap_err().code, ErrorCode::TimedOut);
        drop(second);
        let writer = lock_for(&ws.root, Duration::ZERO).unwrap();
        assert_eq!(copy_lock_for(&ws.root, Duration::ZERO).unwrap_err().code, ErrorCode::TimedOut);
        drop(writer);

        assert_eq!(identity, same_file::Handle::from_path(&path).unwrap());
        assert_eq!(std::fs::metadata(&path).unwrap().len(), 0);
    }

    #[test]
    fn coordination_preserves_unrecognized_entries_and_missing_projects() {
        let directory = tempfile::tempdir().unwrap();
        let missing = directory.path().join("missing");
        assert!(lock_project(&missing).is_err());
        assert!(!missing.exists());
        assert!(lock_project(directory.path()).is_err());
        assert!(!directory.path().join("inbox").exists());
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let inbox = ws.root.join("inbox");
        assert!(inbox.is_dir(), "initial creation retains its coordination reservation");
        let path = inbox.join("feed.lock");
        std::fs::write(&path, b"Keep these words").unwrap();
        assert!(lock_project(&ws.root).is_err());
        assert!(copy_lock_for(&ws.root, Duration::ZERO).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"Keep these words");
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        assert!(lock_project(&ws.root).is_err());
        assert!(copy_lock_for(&ws.root, Duration::ZERO).is_err());
        assert!(path.is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn coordination_does_not_follow_existing_links() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let inbox = ws.root.join("inbox");
        assert!(inbox.is_dir(), "initial creation retains its coordination reservation");
        let path = inbox.join("feed.lock");
        let target = directory.path().join("personal");
        std::fs::write(&target, b"").unwrap();
        std::os::unix::fs::symlink(&target, &path).unwrap();
        assert!(lock_project(&ws.root).is_err());
        assert!(copy_lock_for(&ws.root, Duration::ZERO).is_err());
        assert_eq!(std::fs::read_link(path).unwrap(), target);
        assert_eq!(std::fs::read(target).unwrap(), b"");
    }

    #[test]
    fn guards_recheck_the_project_after_locking() {
        let directory = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let guard = lock_project(&ws.root).unwrap();
        let manifest = ws.root.join(crate::workspace::MANIFEST);
        std::fs::remove_file(&manifest).unwrap();
        assert!(guard.check_project().is_err());
        ws.manifest.created_ms = ws.manifest.created_ms.saturating_add(1);
        // Construct an external replacement; ordinary workspace updates keep the identity
        // and acquire this same non-reentrant guard.
        std::fs::write(&manifest, serde_json::to_vec_pretty(&ws.manifest).unwrap()).unwrap();
        assert_eq!(guard.check_project().unwrap_err().code, ErrorCode::FileChanged);
        let replacement = directory.path().join("replacement");
        std::fs::rename(&ws.root, &replacement).unwrap();
        std::fs::create_dir(&ws.root).unwrap();
        ws.manifest.created_ms = ws.manifest.created_ms.saturating_sub(1);
        std::fs::write(&manifest, serde_json::to_vec_pretty(&ws.manifest).unwrap()).unwrap();
        assert_eq!(guard.check_project().unwrap_err().code, ErrorCode::FileChanged);
    }

    #[test]
    fn deletion_during_lock_creation_still_refuses_payload_writes() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let root = ws.root.clone();
        std::fs::write(root.join("personal.txt"), b"Keep my app").unwrap();
        assert!(lock_for_using(&root, WAIT, || ws.delete().unwrap()).is_err());
        assert!(!root.join(crate::workspace::MANIFEST).exists());
        assert!(!root.join(super::super::FEED_FILE).exists());
        assert!(!root.join("inbox").exists(), "the rejected initializer releases only its newly created empty coordination directory");
        assert_eq!(std::fs::read(root.join("personal.txt")).unwrap(), b"Keep my app");
        Workspace::create_at(&root, "Replacement", "Retry", TargetRef::None, Some(ExistingFiles::Edit)).unwrap();
    }

    #[test]
    fn a_stale_initializer_preserves_an_active_creators_existing_inbox() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let root = ws.root.clone();
        std::fs::write(root.join("personal.txt"), b"Keep my app").unwrap();
        let mut active = None;

        let result = lock_for_using(&root, WAIT, || {
            ws.delete().unwrap();
            std::fs::create_dir(root.join("inbox")).unwrap();
            active = Some(same_file::Handle::from_path(root.join("inbox")).unwrap());
        });

        assert!(result.is_err());
        assert_eq!(active.unwrap(), same_file::Handle::from_path(root.join("inbox")).unwrap());
        assert_eq!(
            std::fs::read_dir(root.join("inbox")).unwrap().count(),
            0,
            "the stale initializer neither claims nor populates another creator's reservation"
        );
        assert!(Workspace::create_at(&root, "Other", "Retry", TargetRef::None, Some(ExistingFiles::Edit)).is_err());
    }

    #[test]
    fn a_stale_initializer_preserves_a_published_replacement() {
        let directory = tempfile::tempdir().unwrap();
        let ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let root = ws.root.clone();
        std::fs::write(root.join("personal.txt"), b"Keep my app").unwrap();
        let mut published = Vec::new();

        let result = lock_for_using(&root, WAIT, || {
            ws.delete().unwrap();
            Workspace::create_at(&root, "Replacement", "New request", TargetRef::None, Some(ExistingFiles::Edit)).unwrap();
            published = std::fs::read(root.join(crate::workspace::MANIFEST)).unwrap();
        });

        assert_eq!(result.unwrap_err().code, ErrorCode::FileChanged);
        assert_eq!(std::fs::read(root.join(crate::workspace::MANIFEST)).unwrap(), published);
        assert_eq!(std::fs::read_dir(root.join("inbox")).unwrap().count(), 0);
        assert_eq!(std::fs::read(root.join("personal.txt")).unwrap(), b"Keep my app");
    }

    #[test]
    fn a_legacy_initializers_late_first_capture_preserves_a_new_creators_reservation() {
        let directory = tempfile::tempdir().unwrap();
        let mut ws = Workspace::create(directory.path(), "Project", "Test", TargetRef::None).unwrap();
        let root = ws.root.clone();
        std::fs::write(root.join("personal.txt"), b"Keep my app").unwrap();
        ws.manifest.generation = None;
        std::fs::write(root.join(crate::workspace::MANIFEST), serde_json::to_vec_pretty(&ws.manifest).unwrap()).unwrap();
        std::fs::remove_dir(ws.inbox_dir()).unwrap();
        let mut active = None;

        let result = lock_for_using_after_mkdir(
            &root,
            WAIT,
            || {},
            || {
                ws.delete().unwrap();
                std::fs::create_dir(root.join("inbox")).unwrap();
                active = Some(same_file::Handle::from_path(root.join("inbox")).unwrap());
            },
        );

        let error = result.unwrap_err();
        assert_eq!(active.unwrap(), same_file::Handle::from_path(root.join("inbox")).unwrap());
        assert_eq!(std::fs::read_dir(root.join("inbox")).unwrap().count(), 0, "an unconfirmed first capture cannot release another creator's reservation");
        assert!(error.remediation.as_deref().unwrap().contains(&root.join("inbox").display().to_string()));
        assert!(Workspace::create_at(&root, "Other", "Retry", TargetRef::None, Some(ExistingFiles::Edit)).is_err());
        assert_eq!(std::fs::read(root.join("personal.txt")).unwrap(), b"Keep my app");
    }
}
