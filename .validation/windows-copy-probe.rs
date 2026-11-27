#![forbid(unsafe_code)]

use std::fs::{self, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const SHARE_READ_WRITE: u32 = 1 | 2;

fn copy_new(from: &Path, to: &Path, after_reserve: impl FnOnce()) -> io::Result<u64> {
    let _source = OpenOptions::new().read(true).share_mode(SHARE_READ_WRITE).open(from)?;
    let _destination = OpenOptions::new().write(true).create_new(true).share_mode(SHARE_READ_WRITE).open(to)?;
    after_reserve();
    fs::copy(from, to)
}

struct Directory(PathBuf);

impl Directory {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("craft-copy-probe-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn stream(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(":Zone.Identifier");
    name.into()
}

#[test]
fn native_copy_keeps_streams_and_pins_both_names() {
    let directory = Directory::new();
    let from = directory.0.join("source.txt");
    let to = directory.0.join("copy.txt");
    fs::write(&from, b"Generated ordinary content").unwrap();
    fs::write(stream(&from), b"Generated alternate stream").unwrap();
    let copied = copy_new(&from, &to, || {
        assert!(fs::rename(&from, directory.0.join("moved-source.txt")).is_err());
        assert!(fs::remove_file(&from).is_err());
        assert!(fs::rename(&to, directory.0.join("moved-copy.txt")).is_err());
        assert!(fs::remove_file(&to).is_err());
        assert!(fs::rename(&directory.0, directory.0.with_extension("moved")).is_err());
    }).unwrap();
    assert_eq!(copied, b"Generated ordinary content".len() as u64);
    assert_eq!(fs::read(&to).unwrap(), b"Generated ordinary content");
    assert_eq!(fs::read(stream(&to)).unwrap(), b"Generated alternate stream");
}

#[test]
fn occupied_destination_keeps_its_data_and_streams() {
    let directory = Directory::new();
    let from = directory.0.join("source.txt");
    let to = directory.0.join("personal.txt");
    fs::write(&from, b"Generated ordinary content").unwrap();
    fs::write(stream(&from), b"Generated alternate stream").unwrap();
    fs::write(&to, b"Keep personal bytes").unwrap();
    fs::write(stream(&to), b"Keep personal alternate stream").unwrap();
    assert_eq!(copy_new(&from, &to, || panic!("an occupied destination must stop before copying")).unwrap_err().kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(&to).unwrap(), b"Keep personal bytes");
    assert_eq!(fs::read(stream(&to)).unwrap(), b"Keep personal alternate stream");
}
