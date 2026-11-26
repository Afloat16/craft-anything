#![forbid(unsafe_code)]

#[cfg(windows)]
mod native {
    use craft_2183_windows_helper_check::{open_child_directory, open_child_file_for_update};
    use same_file::Handle;
    use std::fs::{self, File, OpenOptions, TryLockError};
    use std::io;
    use std::os::windows::fs::OpenOptionsExt as _;
    use std::path::{Path, PathBuf};
    use windows::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;

    const FOREIGN: &[u8] = b"Keep this occupied target";

    fn directory(path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS.0)
            .open(path)
    }

    fn native_path(path: &Path) -> io::Result<PathBuf> {
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::other("missing parent"))?;
        let name = path
            .file_name()
            .ok_or_else(|| io::Error::other("missing filename"))?;
        Ok(fs::canonicalize(parent)?.join(name))
    }

    fn prove_exclusion(path: &Path) -> io::Result<()> {
        let other = OpenOptions::new().read(true).write(true).open(path)?;
        match other.try_lock() {
            Err(TryLockError::WouldBlock) => Ok(()),
            Err(TryLockError::Error(error)) => Err(error),
            Ok(()) => Err(io::Error::other(
                "the supposedly held source lock was acquired",
            )),
        }
    }

    fn run_case(base: &Path, keep_target_open: bool, replace: bool) -> io::Result<()> {
        let case = format!(
            "{}-{}",
            if keep_target_open {
                "held-target"
            } else {
                "closed-target"
            },
            if replace { "replace" } else { "no-replace" }
        );
        let root_path = base.join(&case);
        fs::create_dir(&root_path)?;
        fs::create_dir(root_path.join("inbox"))?;
        fs::create_dir(root_path.join("private"))?;
        let root = directory(&root_path)?;
        let inbox = open_child_directory(&root, Path::new("inbox"))?;
        let _private = open_child_directory(&root, Path::new("private"))?;
        let file = open_child_file_for_update(&inbox, Path::new("feed.lock"), true)?;
        file.try_lock()
            .map_err(|error| io::Error::other(format!("initial lock failed: {error:?}")))?;
        let original = Handle::from_file(file)?;
        let public = root_path.join("inbox/feed.lock");
        let retained = root_path.join("private/coordination-0");
        let witness = root_path.join("target-witness");
        fs::write(&retained, FOREIGN)?;
        let held_target = if keep_target_open {
            Some(Handle::from_path(&retained)?)
        } else {
            // Keep an identity witness without opening either name during the move.
            fs::hard_link(&retained, &witness)?;
            None
        };
        prove_exclusion(&public)?;
        let native_public = native_path(&public)?;
        let native_retained = native_path(&retained)?;
        println!(
            "PATH\t{case}\tinput={}\tnative={}",
            retained.display(),
            native_retained.display()
        );
        let result = if replace {
            atomicwrites::replace_atomic(&native_public, &native_retained)
        } else {
            atomicwrites::move_atomic(&native_public, &native_retained)
        };
        let succeeded = result.is_ok();
        match result {
            Ok(()) => println!("OBS\t{case}\tmove\tok"),
            Err(error) => println!(
                "OBS\t{case}\tmove\terror\tkind={:?}\tos={:?}\t{error}",
                error.kind(),
                error.raw_os_error()
            ),
        }
        let witness_after = if keep_target_open {
            None
        } else {
            Some(Handle::from_path(&witness)?)
        };
        let expected_target = held_target
            .as_ref()
            .or(witness_after.as_ref())
            .expect("one target identity witness");
        let retained_after = Handle::from_path(&retained)?;
        let target_preserved = &retained_after == expected_target;
        let target_is_source = retained_after == original;
        let public_is_source = Handle::from_path(&public).is_ok_and(|current| current == original);
        if target_preserved {
            assert_eq!(
                fs::read(&retained)?,
                FOREIGN,
                "{case}: target bytes changed"
            );
        }
        if !keep_target_open {
            assert_eq!(
                fs::read(&witness)?,
                FOREIGN,
                "{case}: witness bytes changed"
            );
        }
        if succeeded {
            assert!(
                !public.exists() && target_is_source,
                "{case}: successful move did not move the original source"
            );
            assert_eq!(
                fs::metadata(&retained)?.len(),
                0,
                "{case}: moved source changed"
            );
            prove_exclusion(&retained)?;
        } else {
            assert!(
                public_is_source && target_preserved,
                "{case}: failed move changed one of the entries"
            );
            assert_eq!(fs::metadata(&public)?.len(), 0, "{case}: source changed");
            prove_exclusion(&public)?;
        }
        println!(
            "STATE\t{case}\tpublic_is_source={public_is_source}\ttarget_preserved={target_preserved}\ttarget_is_source={target_is_source}\texclusive_lock_preserved=true"
        );
        if !replace {
            assert!(
                !succeeded,
                "no-replace move must reject the occupied target"
            );
        } else if !keep_target_open {
            assert!(
                succeeded,
                "closed-target overwrite control must reach actual replacement"
            );
        }
        Ok(())
    }

    pub fn run() -> io::Result<()> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        let base = std::env::temp_dir().join(format!(
            "craft-2183-target-probe-{}-{stamp}",
            std::process::id()
        ));
        fs::create_dir(&base)?;
        println!("FIXTURES\t{}", base.display());
        for keep_target_open in [false, true] {
            for replace in [false, true] {
                run_case(&base, keep_target_open, replace)?;
            }
        }
        println!(
            "PROBE_COMPLETE\t4 cases\tclosed-target replacement reached\tno-replace preservation and held-source exclusion verified"
        );
        Ok(())
    }
}

fn main() {
    #[cfg(windows)]
    native::run().expect("native target replacement probe");
    #[cfg(not(windows))]
    panic!("this probe requires native Windows");
}
