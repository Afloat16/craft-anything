#![cfg(windows)]
#![forbid(unsafe_code)]

use std::path::Path;

fn check_child_name(name: &Path) -> std::io::Result<()> {
    let mut components = name.components();
    if !matches!(components.next(), Some(std::path::Component::Normal(component)) if component == name.as_os_str()) || components.next().is_some() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "expected a single child name"));
    }
    Ok(())
}

/// Open one existing child directory relative to its retained parent, without following links.
pub fn open_child_directory(parent: &std::fs::File, name: &Path) -> std::io::Result<std::fs::File> {
    check_child_name(name)?;
    #[cfg(any(unix, windows))]
    {
        #[cfg(unix)]
        let file = {
            use nix::fcntl::{OFlag, openat};
            use nix::sys::stat::Mode;
            let flags = OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_NONBLOCK | OFlag::O_CLOEXEC;
            std::fs::File::from(openat(parent, name, flags, Mode::empty()).map_err(std::io::Error::from)?)
        };
        #[cfg(windows)]
        let file = {
            use fs_at::os::windows::OpenOptionsExt as _;
            use windows::Win32::Storage::FileSystem::{FILE_LIST_DIRECTORY, FILE_READ_ATTRIBUTES, FILE_TRAVERSE};
            // Keep the no-follow handle in a File before rejecting links below: fs_at 0.2.1's
            // own link check can fail before taking ownership. open_path_at needs explicit access.
            fs_at::OpenOptions::default().desired_access((FILE_READ_ATTRIBUTES | FILE_LIST_DIRECTORY | FILE_TRAVERSE).0).open_path_at(parent, name)?
        };
        let metadata = file.metadata()?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the child is not a directory"));
        }
        Ok(file)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = parent;
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "directory-relative opening is unavailable"))
    }
}

/// Open one regular child for updating, optionally creating it exclusively with private permissions.
/// The parent handle is authoritative even if its former pathname now names another directory.
/// Existing special files cannot block a Unix open before their type is checked.
pub fn open_child_file_for_update(parent: &std::fs::File, name: &Path, create_new: bool) -> std::io::Result<std::fs::File> {
    check_child_name(name)?;
    #[cfg(any(unix, windows))]
    {
        #[cfg(unix)]
        let file = {
            use nix::fcntl::{OFlag, openat};
            use nix::sys::stat::Mode;
            let mut flags = OFlag::O_RDWR | OFlag::O_NOFOLLOW | OFlag::O_NONBLOCK | OFlag::O_CLOEXEC;
            if create_new {
                flags |= OFlag::O_CREAT | OFlag::O_EXCL;
            }
            std::fs::File::from(openat(parent, name, flags, Mode::S_IRUSR | Mode::S_IWUSR).map_err(std::io::Error::from)?)
        };
        #[cfg(windows)]
        let file = {
            use fs_at::os::windows::OpenOptionsExt as _;
            use windows::Win32::Storage::FileSystem::{FILE_GENERIC_READ, FILE_GENERIC_WRITE};
            // As above, validate the owned no-follow File and retain read/write and exclusive creation.
            fs_at::OpenOptions::default().desired_access((FILE_GENERIC_READ | FILE_GENERIC_WRITE).0).create_new(create_new).open_path_at(parent, name)?
        };
        let metadata = file.metadata()?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the child is not a regular file"));
        }
        Ok(file)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (parent, create_new);
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "directory-relative opening is unavailable"))
    }
}

// The three helper bodies above are unchanged from the published baseline.
