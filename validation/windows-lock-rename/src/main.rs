#![forbid(unsafe_code)]

#[cfg(windows)]
mod native {
    use craft_2183_windows_helper_check::{open_child_directory, open_child_file_for_update};
    use std::fs::{self, File, OpenOptions, TryLockError};
    use std::io;
    use std::os::windows::fs::OpenOptionsExt as _;
    use std::path::{Path, PathBuf};
    use windows::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;

    struct Paths {
        root: PathBuf,
        inbox: PathBuf,
        private: PathBuf,
        claimed: PathBuf,
    }

    fn setup(base: &Path, case: &str) -> io::Result<Paths> {
        let root = base.join(case);
        fs::create_dir(&root)?;
        let inbox = root.join("inbox");
        let private = root.join("private");
        fs::create_dir(&inbox)?;
        fs::create_dir(&private)?;
        let claimed = private.join("reservation");
        Ok(Paths { root, inbox, private, claimed })
    }

    fn directory(path: &Path) -> io::Result<File> {
        // Standard directory handle with default READ|WRITE|DELETE sharing. Child handles
        // and file opens below use the exact published fs_at production helpers.
        OpenOptions::new().read(true).custom_flags(FILE_FLAG_BACKUP_SEMANTICS.0).open(path)
    }

    fn observe(case: &str, operation: &str, result: io::Result<()>) -> bool {
        match result {
            Ok(()) => {
                println!("OBS\t{case}\t{operation}\tok");
                true
            }
            Err(error) => {
                println!("OBS\t{case}\t{operation}\terror\tkind={:?}\tos={:?}\t{error}", error.kind(), error.raw_os_error());
                false
            }
        }
    }

    fn exclusive(file: &File) -> io::Result<()> {
        file.try_lock().map_err(|error| io::Error::other(format!("could not establish exclusive lock: {error:?}")))
    }

    fn prove_exclusion(case: &str, path: &Path) -> io::Result<()> {
        let other = OpenOptions::new().read(true).write(true).open(path)?;
        match other.try_lock() {
            Err(TryLockError::WouldBlock) => {
                println!("PREMISE\t{case}\texclusive lock blocks a second lock acquisition\tok");
                Ok(())
            }
            Err(TryLockError::Error(error)) => Err(error),
            Ok(()) => Err(io::Error::other("the second open acquired the supposedly held exclusive lock")),
        }
    }

    fn baseline(base: &Path) -> io::Result<()> {
        let case = "empty-directory-baseline";
        let p = setup(base, case)?;
        let moved = fs::rename(&p.inbox, &p.claimed);
        if !observe(case, "rename inbox without handles", moved) {
            return Err(io::Error::other("baseline directory rename failed"));
        }
        observe(case, "remove claimed without handles", fs::remove_dir(&p.claimed));
        observe(case, "remove private without handles", fs::remove_dir(&p.private));
        observe(case, "remove root without handles", fs::remove_dir(&p.root));
        Ok(())
    }

    fn directory_handles(base: &Path) -> io::Result<()> {
        let case = "directory-identity-handles";
        let p = setup(base, case)?;
        let root = directory(&p.root)?;
        let inbox = open_child_directory(&root, Path::new("inbox"))?;
        let private = open_child_directory(&root, Path::new("private"))?;
        let moved = observe(case, "rename inbox while directory handles remain", fs::rename(&p.inbox, &p.claimed));
        let current = if moved { &p.claimed } else { &p.inbox };
        observe(case, "remove inbox while its directory handle remains", fs::remove_dir(current));
        drop(inbox);
        observe(case, "remove inbox after directory handle drop", fs::remove_dir(current));
        observe(case, "remove private while its directory handle remains", fs::remove_dir(&p.private));
        drop(private);
        observe(case, "remove private after directory handle drop", fs::remove_dir(&p.private));
        drop(root);
        observe(case, "remove root after all handles drop", fs::remove_dir(&p.root));
        Ok(())
    }

    fn open_file(base: &Path, locked: bool) -> io::Result<()> {
        let case = if locked { "exclusive-file-lock" } else { "open-file-no-lock" };
        let p = setup(base, case)?;
        let root = directory(&p.root)?;
        let inbox = open_child_directory(&root, Path::new("inbox"))?;
        let private = open_child_directory(&root, Path::new("private"))?;
        let file = open_child_file_for_update(&inbox, Path::new("feed.lock"), true)?;
        if locked {
            exclusive(&file)?;
            prove_exclusion(case, &p.inbox.join("feed.lock"))?;
        }
        let moved = observe(case, "rename inbox while file handle remains", fs::rename(&p.inbox, &p.claimed));
        let current = if moved { &p.claimed } else { &p.inbox };
        observe(case, "remove file while its handle remains", fs::remove_file(current.join("feed.lock")));
        observe(case, "remove inbox before file handle drop", fs::remove_dir(current));
        drop(file);
        observe(case, "remove file after its handle drop", fs::remove_file(current.join("feed.lock")));
        observe(case, "remove inbox with only its directory handle", fs::remove_dir(current));
        drop(inbox);
        observe(case, "remove inbox after directory handle drop", fs::remove_dir(current));
        observe(case, "remove private while its directory handle remains", fs::remove_dir(&p.private));
        drop(private);
        observe(case, "remove private after its handle drop", fs::remove_dir(&p.private));
        drop(root);
        observe(case, "remove root after all handles drop", fs::remove_dir(&p.root));
        Ok(())
    }

    fn detach_file(base: &Path, late_file: bool) -> io::Result<()> {
        let case = if late_file { "detached-lock-with-late-initializer" } else { "detached-held-lock" };
        let p = setup(base, case)?;
        let root = directory(&p.root)?;
        let inbox = open_child_directory(&root, Path::new("inbox"))?;
        let private = open_child_directory(&root, Path::new("private"))?;
        let file = open_child_file_for_update(&inbox, Path::new("feed.lock"), true)?;
        exclusive(&file)?;
        prove_exclusion(case, &p.inbox.join("feed.lock"))?;
        let detached = p.private.join("held.lock");
        if !observe(case, "move still-held file to private slot", fs::rename(p.inbox.join("feed.lock"), &detached)) {
            println!("NOT_RUN\t{case}\tlater operations require a successful file move");
            return Ok(());
        }
        prove_exclusion(case, &detached)?;
        let late = if late_file {
            // Reproduce only the filesystem interleaving of a caller delayed after its
            // project check. Never restore the old lock over this newly created entry.
            let late = open_child_file_for_update(&inbox, Path::new("feed.lock"), true)?;
            exclusive(&late)?;
            println!("PREMISE\t{case}\tnew public lock acquired while old private lock remains held\tok");
            Some(late)
        } else {
            None
        };
        let mut moved = observe(case, "rename inbox while detached lock remains held", fs::rename(&p.inbox, &p.claimed));
        drop(late);
        if late_file && !moved {
            moved = observe(case, "rename inbox after late file handle drop", fs::rename(&p.inbox, &p.claimed));
        }
        let current = if moved { &p.claimed } else { &p.inbox };
        if late_file {
            let metadata = fs::metadata(current.join("feed.lock"))?;
            println!("PRESERVED\t{case}\tnew public lock is retained at {}\tbytes={}", current.join("feed.lock").display(), metadata.len());
        }
        observe(case, "remove detached file while lock remains held", fs::remove_file(&detached));
        observe(case, "remove claimed inbox before file and directory drops", fs::remove_dir(current));
        observe(case, "remove private before file and directory drops", fs::remove_dir(&p.private));
        drop(file);
        observe(case, "remove detached file after lock handle drop", fs::remove_file(&detached));
        observe(case, "remove claimed inbox with its directory handle", fs::remove_dir(current));
        drop(inbox);
        observe(case, "remove claimed inbox after its directory handle drop", fs::remove_dir(current));
        observe(case, "remove private with only its own directory handle", fs::remove_dir(&p.private));
        drop(private);
        observe(case, "remove private after all descendant and own handles drop", fs::remove_dir(&p.private));
        drop(root);
        observe(case, "remove root after all handles drop", fs::remove_dir(&p.root));
        Ok(())
    }

    fn root_with_descendants(base: &Path) -> io::Result<()> {
        let case = "root-with-open-directory-descendants";
        let p = setup(base, case)?;
        let root = directory(&p.root)?;
        let inbox = open_child_directory(&root, Path::new("inbox"))?;
        let private = open_child_directory(&root, Path::new("private"))?;
        let claimed = base.join("claimed-root");
        let mut moved = observe(case, "rename root with inbox and cleanup directory handles", fs::rename(&p.root, &claimed));
        drop(inbox);
        drop(private);
        if !moved {
            moved = observe(case, "rename root after descendant handles drop", fs::rename(&p.root, &claimed));
        }
        let current = if moved { &claimed } else { &p.root };
        observe(case, "remove inbox after descendant handles drop", fs::remove_dir(current.join("inbox")));
        observe(case, "remove private after descendant handles drop", fs::remove_dir(current.join("private")));
        observe(case, "remove root with only its own handle", fs::remove_dir(current));
        drop(root);
        observe(case, "remove root after its own handle drop", fs::remove_dir(current));
        Ok(())
    }

    pub fn run() -> io::Result<()> {
        let base = std::env::var_os("CRAFT_PROBE_ROOT").ok_or_else(|| io::Error::other("CRAFT_PROBE_ROOT is required"))?;
        let base = PathBuf::from(base);
        // Exclusive root creation: never reuse or recursively clean an existing directory.
        fs::create_dir(&base)?;
        println!("PROBE\tRust 1.100 native Windows\troot={}", base.display());
        let cases: &[(&str, fn(&Path) -> io::Result<()>)] = &[
            ("baseline", baseline),
            ("directory-handles", directory_handles),
            ("open-file", |base| open_file(base, false)),
            ("exclusive-lock", |base| open_file(base, true)),
            ("detached-held-lock", |base| detach_file(base, false)),
            ("late-initializer", |base| detach_file(base, true)),
            ("root-descendants", root_with_descendants),
        ];
        let mut failed = false;
        for (name, case) in cases {
            match case(&base) {
                Ok(()) => println!("CASE\t{name}\tcompleted observations"),
                Err(error) => {
                    failed = true;
                    println!("CASE_SETUP_FAILURE\t{name}\tkind={:?}\tos={:?}\t{error}", error.kind(), error.raw_os_error());
                }
            }
        }
        println!("PROBE\tfinished\tsetup_failures={failed}\tresiduals are retained in the job-owned temporary directory");
        if failed { Err(io::Error::other("one or more case prerequisites failed")) } else { Ok(()) }
    }
}

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    native::run()
}

#[cfg(not(windows))]
fn main() {
    eprintln!("This probe requires native Windows; a cross-compile or Unix run is not runtime evidence.");
    std::process::exit(2);
}
