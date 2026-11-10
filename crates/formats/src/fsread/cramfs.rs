//! cramfs (`Compressed ROMFS`, magic `0x28cd3d45`): a compressed read-only filesystem in either
//! byte order. Inodes are 12 bytes; a
//! directory's children sit consecutively in its data span; a regular file's data is a run of
//! zlib-compressed 4 KiB blocks preceded by their end-offset pointers. Bounded against cycles,
//! huge sizes and corrupt offsets.

use super::{FsEntry, FsImage, FsKind, Limits, join};
use crate::bytes::Reader;

const BLK: usize = 4096;

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u16(self, reader: &Reader<'_>, off: usize) -> Option<u16> {
        match self {
            Self::Little => reader.u16_le(off),
            Self::Big => reader.u16_be(off),
        }
    }

    fn u32(self, reader: &Reader<'_>, off: usize) -> Option<u32> {
        match self {
            Self::Little => reader.u32_le(off),
            Self::Big => reader.u32_be(off),
        }
    }
}

struct Inode {
    mode: u16,
    size: usize,
    namelen: usize,
    offset: usize,
}

fn inode_at(r: &Reader, off: usize, endian: Endian) -> Option<Inode> {
    let mode = endian.u16(r, off)?;
    let sz = endian.u32(r, off.checked_add(4)?)?;
    let no = endian.u32(r, off.checked_add(8)?)?;
    // These are native-order bitfields, not simply byte-swapped little-endian words.
    let (size, name_units, offset_units) = match endian {
        Endian::Little => (sz & 0x00ff_ffff, no & 0x3f, no >> 6),
        Endian::Big => (sz >> 8, no >> 26, no & 0x03ff_ffff),
    };
    Some(Inode { mode, size: size as usize, namelen: name_units as usize * 4, offset: offset_units as usize * 4 })
}

pub fn read(data: &[u8], limits: &Limits) -> FsImage {
    let mut img = FsImage::new("cramfs");
    let r = Reader::new(data);
    let endian = if r.u32_le(0) == Some(0x28cd_3d45) {
        Endian::Little
    } else if r.u32_be(0) == Some(0x28cd_3d45) {
        Endian::Big
    } else {
        img.warn("not a cramfs image");
        img.complete = false;
        return img;
    };
    let Some(root) = inode_at(&r, 64, endian) else {
        img.warn("root inode unreadable");
        img.complete = false;
        return img;
    };
    walk(data, root.offset, root.size, "", limits, &mut img, 0, endian);
    img
}

fn trim(name: &[u8]) -> String {
    let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    String::from_utf8_lossy(name.get(..end).unwrap_or(&[])).into_owned()
}

#[allow(clippy::too_many_arguments)]
fn walk(data: &[u8], dir_off: usize, dir_size: usize, parent: &str, limits: &Limits, img: &mut FsImage, depth: usize, endian: Endian) {
    if depth > limits.max_depth {
        img.warn("directory nesting limit reached");
        img.complete = false;
        return;
    }
    let rest = data.get(dir_off..).unwrap_or_default();
    let directory = rest.get(..dir_size).unwrap_or(rest);
    let r = Reader::new(directory);
    let mut pos = 0;
    while pos < directory.len() {
        if img.entries.len() >= limits.max_entries {
            img.complete = false;
            return;
        }
        let Some(ino) = inode_at(&r, pos, endian) else { break };
        let name_off = pos + 12;
        let Some(full_name) = r.slice(name_off, ino.namelen) else { break };
        let name = trim(full_name.get(..ino.namelen.min(limits.max_name.max(1))).unwrap_or_default());
        let advance = 12usize.checked_add(ino.namelen);
        let Some(advance) = advance.filter(|&a| a > 12) else { break };
        let path = if name.is_empty() { parent.to_string() } else { join(parent, &name) };
        let typ = ino.mode & 0xf000;
        if !name.is_empty() {
            match typ {
                0x4000 => {
                    img.entries.push(FsEntry::dir(path.clone()));
                    walk(data, ino.offset, ino.size, &path, limits, img, depth + 1, endian);
                }
                0x8000 => {
                    let bytes = file_data(data, &ino, limits, img, endian);
                    img.entries.push(FsEntry::file(path, ino.size as u64, bytes));
                }
                0xa000 => {
                    let target = file_data(data, &ino, limits, img, endian).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();
                    img.entries.push(FsEntry::symlink(path, target));
                }
                _ => img.entries.push(FsEntry { path, kind: FsKind::Other, size: ino.size as u64, symlink_target: None, data: None }),
            }
        }
        pos = match pos.checked_add(advance) {
            Some(n) if n > pos => n,
            _ => break,
        };
    }
    if pos != dir_size {
        img.warn("cramfs: incomplete directory entry");
        img.complete = false;
    }
}

/// Decompress a regular file's (or symlink's) zlib block run.
fn file_data(data: &[u8], ino: &Inode, limits: &Limits, img: &mut FsImage, endian: Endian) -> Option<Vec<u8>> {
    if ino.size == 0 {
        return Some(Vec::new());
    }
    if ino.size as u64 > limits.max_file_bytes || img.bytes.saturating_add(ino.size as u64) > limits.max_total_bytes {
        img.warn("file skipped (size budget)");
        img.complete = false;
        return None;
    }
    let r = Reader::new(data);
    let nblocks = ino.size.div_ceil(BLK);
    let mut out = Vec::with_capacity(ino.size);
    let mut block_start = ino.offset.checked_add(nblocks.checked_mul(4)?)?;
    for i in 0..nblocks {
        let end = endian.u32(&r, ino.offset.checked_add(i.checked_mul(4)?)?)? as usize;
        if end < block_start || end > data.len() {
            img.warn("corrupt cramfs block pointer");
            img.complete = false;
            return None;
        }
        let comp = data.get(block_start..end)?;
        match crate::firmware::inflate(comp, true, BLK as u64, true) {
            Ok(inf) => {
                let expected = ino.size.saturating_sub(i.saturating_mul(BLK)).min(BLK);
                if inf.produced != expected as u64 {
                    img.warn("cramfs block output does not match the file size");
                    img.complete = false;
                    return None;
                }
                out.extend_from_slice(&inf.data.unwrap_or_default());
            }
            Err(_) => {
                img.warn("cramfs block did not inflate");
                img.complete = false;
                return None;
            }
        }
        block_start = end;
    }
    out.truncate(ino.size);
    img.bytes = img.bytes.saturating_add(out.len() as u64);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big_endian_fixture() -> Vec<u8> {
        let mut data = craft_anything_testkit::firmware::cramfs_image();
        for off in [0, 4, 8, 12, 32, 36, 40, 44] {
            let value = u32::from_le_bytes(data[off..off + 4].try_into().unwrap());
            data[off..off + 4].copy_from_slice(&value.to_be_bytes());
        }
        for off in [64, 76, 100, 116] {
            let mode = u16::from_le_bytes(data[off..off + 2].try_into().unwrap());
            let size = u32::from_le_bytes(data[off + 4..off + 8].try_into().unwrap());
            let name_offset = u32::from_le_bytes(data[off + 8..off + 12].try_into().unwrap());
            data[off..off + 2].copy_from_slice(&mode.to_be_bytes());
            data[off + 4..off + 8].copy_from_slice(&size.rotate_left(8).to_be_bytes());
            data[off + 8..off + 12].copy_from_slice(&(((name_offset & 63) << 26) | (name_offset >> 6)).to_be_bytes());
            if mode & 0xf000 == 0x8000 {
                let pointer = (name_offset >> 6) as usize * 4;
                let end = u32::from_le_bytes(data[pointer..pointer + 4].try_into().unwrap());
                data[pointer..pointer + 4].copy_from_slice(&end.to_be_bytes());
            }
        }
        data
    }

    #[test]
    fn big_endian_cramfs_recovers_the_same_entries_and_bytes() {
        let le = read(&craft_anything_testkit::firmware::cramfs_image(), &Limits::default());
        let be = read(&big_endian_fixture(), &Limits::default());
        assert!(be.complete, "{:?}", be.warnings);
        assert_eq!(be, le);
    }

    #[test]
    fn big_endian_cramfs_keeps_size_budgets_and_pointer_checks() {
        let mut data = big_endian_fixture();
        let limits = Limits { max_file_bytes: 6, ..Limits::default() };
        let img = read(&data, &limits);
        assert!(!img.complete);
        assert_eq!(img.entries.iter().find(|e| e.path == "hello.txt").unwrap().data, None);
        assert_eq!(img.entries.iter().find(|e| e.path == "dir/inner").unwrap().data.as_deref(), Some(b"nested".as_slice()));
        data[136..140].copy_from_slice(&u32::MAX.to_be_bytes());
        let corrupt = read(&data, &Limits::default());
        assert!(!corrupt.complete);
        assert!(corrupt.warnings.iter().any(|w| w.contains("block pointer")));
        for len in 0..data.len() {
            let _ = read(&data[..len], &Limits::default());
        }
    }

    #[test]
    fn reads_a_small_image() {
        let img = read(&craft_anything_testkit::firmware::cramfs_image(), &Limits::default());
        assert!(img.complete, "{:?}", img.warnings);
        assert_eq!(img.entries.iter().find(|e| e.path == "hello.txt").unwrap().data.as_deref(), Some(b"Hello from cramfs!\n".as_slice()));
        assert!(img.entries.iter().any(|e| e.path == "dir" && e.kind == FsKind::Dir));
        assert_eq!(img.entries.iter().find(|e| e.path == "dir/inner").unwrap().data.as_deref(), Some(b"nested".as_slice()));
    }

    #[test]
    fn hostile_never_panics() {
        let base = craft_anything_testkit::firmware::cramfs_image();
        for m in craft_anything_testkit::mutations(&base) {
            let _ = read(&m, &Limits::default());
        }
        for len in 0..base.len().min(400) {
            let _ = read(&base[..len], &Limits::default());
        }
    }
}
