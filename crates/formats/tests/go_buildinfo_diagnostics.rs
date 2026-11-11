use craft_anything_formats::{bytes::Reader, go};
use craft_anything_testkit::formats as fixture;

const DATA_VA: u64 = 0x40_3000;
const FAILURE: &str = "found a pointer-encoded build information header but could not follow its version pointer";

#[derive(Clone, Copy)]
enum Encoding {
    Inline,
    Pointer { size: usize, big: bool },
}

fn broken_pointer(size: u8, big: bool) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[..14].copy_from_slice(b"\xff Go buildinf:");
    bytes[14] = size;
    bytes[15] = u8::from(big);
    bytes
}

fn integer(bytes: &mut [u8], at: usize, value: u64, size: usize, big: bool) {
    if big {
        bytes[at..at + size].copy_from_slice(&value.to_be_bytes()[8 - size..]);
    } else {
        bytes[at..at + size].copy_from_slice(&value.to_le_bytes()[..size]);
    }
}

fn build_info(encoding: Encoding, address: u64) -> Vec<u8> {
    let version = "go1.17.13";
    let inline = fixture::go_buildinfo_inline(version, &fixture::go_module_text());
    let Encoding::Pointer { size, big } = encoding else { return inline };
    let reader = Reader::new(&inline);
    let (version_len, version_prefix) = reader.uleb128(32).unwrap();
    let module_prefix = 32 + version_prefix + version_len as usize;
    let (module_len, module_prefix_len) = reader.uleb128(module_prefix).unwrap();
    let module = reader.slice(module_prefix + module_prefix_len, module_len as usize).unwrap();
    let version_header = 32;
    let module_header = version_header + 2 * size;
    let version_data = module_header + 2 * size;
    let module_data = version_data + version.len();
    let mut bytes = vec![0; module_data + module.len()];
    bytes[..32].copy_from_slice(&broken_pointer(size as u8, big));
    for (at, value) in [
        (16, address + version_header as u64),
        (16 + size, address + module_header as u64),
        (version_header, address + version_data as u64),
        (version_header + size, version.len() as u64),
        (module_header, address + module_data as u64),
        (module_header + size, module.len() as u64),
    ] {
        integer(&mut bytes, at, value, size, big);
    }
    bytes[version_data..module_data].copy_from_slice(version.as_bytes());
    bytes[module_data..].copy_from_slice(module);
    bytes
}

// Relocate the fixture's writable build-information region without changing its
// real function table. Every inserted candidate has an unreadable zero pointer.
fn executable(prefix: &[u8], valid: Option<Encoding>) -> Vec<u8> {
    assert_eq!(prefix.len() % 16, 0);
    let mut bytes = fixture::go_elf();
    let sections = Reader::new(&bytes).u64_le(40).unwrap() as usize;
    bytes[0x3000..0x300e].fill(0);
    let offset = (bytes.len() + 4095) & !4095;
    bytes.resize(offset, 0);
    bytes.extend(prefix);
    if let Some(encoding) = valid {
        bytes.extend(build_info(encoding, DATA_VA + prefix.len() as u64));
    }
    let size = (bytes.len() - offset) as u64;
    let data_program_header = 0x40 + 56;
    for (at, value) in [
        (data_program_header + 8, offset as u64),
        (data_program_header + 32, size),
        (data_program_header + 40, size),
        (sections + 3 * 64 + 24, offset as u64),
        (sections + 3 * 64 + 32, size),
    ] {
        integer(&mut bytes, at, value, 8, false);
    }
    bytes
}

fn repeated(count: usize) -> Vec<u8> {
    broken_pointer(8, false).repeat(count)
}

fn facts(mut info: go::GoBuildInfo) -> go::GoBuildInfo {
    info.offset = 0;
    info.address = 0;
    info
}

fn assert_count(warning: &str, count: usize) {
    assert!(warning.contains(&format!("found {count} pointer-encoded build information headers")), "{warning}");
    assert!(warning.contains("could not follow their version pointers"), "{warning}");
    assert!(warning.contains("candidate probes in this scan"), "{warning}");
}

#[test]
fn a_clean_region_keeps_complete_inline_metadata_and_functions() {
    let report = go::inspect(&executable(&[], Some(Encoding::Inline)), None).unwrap();
    assert!(report.warnings.is_empty());
    let info = report.buildinfo.unwrap();
    assert!(info.complete);
    assert_eq!(info.go_version.as_deref(), Some("go1.17.13"));
    assert_eq!(info.main.as_ref().unwrap().path, "example.com/hello");
    assert_eq!(info.deps.len(), 2);
    assert_eq!(report.pclntab.unwrap().functions.len(), fixture::GO_FUNCS.len());
}

#[test]
fn one_unreadable_pointer_keeps_the_existing_diagnostic_per_scan() {
    for valid in [None, Some(Encoding::Inline)] {
        let report = go::inspect(&executable(&repeated(1), valid), None).unwrap();
        assert_eq!(report.warnings, vec![FAILURE; if valid.is_some() { 1 } else { 2 }]);
        assert_eq!(report.buildinfo.is_some(), valid.is_some());
        assert_eq!(report.pclntab.unwrap().functions.len(), fixture::GO_FUNCS.len());
    }
}

#[test]
fn many_unreadable_pointers_do_not_hide_later_inline_metadata() {
    let baseline = go::inspect(&executable(&[], Some(Encoding::Inline)), None).unwrap();
    let report = go::inspect(&executable(&repeated(2048), Some(Encoding::Inline)), None).unwrap();
    assert_eq!(report.warnings.len(), 1);
    assert_count(&report.warnings[0], 2048);
    assert_eq!(facts(report.buildinfo.unwrap()), facts(baseline.buildinfo.unwrap()));
    assert_eq!(report.pclntab, baseline.pclntab);
    assert_eq!(report.packages, baseline.packages);
}

#[test]
fn region_and_whole_file_fallback_each_report_their_probe_count() {
    let report = go::inspect(&executable(&repeated(2048), None), None).unwrap();
    assert!(report.buildinfo.is_none());
    assert_eq!(report.warnings.len(), 2);
    for warning in &report.warnings {
        assert_count(warning, 2048);
    }
    assert_eq!(report.pclntab.unwrap().functions.len(), fixture::GO_FUNCS.len());
    assert!(report.is_go);
}

#[test]
fn later_pointer_metadata_survives_in_both_widths_and_byte_orders() {
    for size in [4, 8] {
        for big in [false, true] {
            let encoding = Encoding::Pointer { size, big };
            let baseline = go::inspect(&executable(&[], Some(encoding)), None).unwrap();
            let report = go::inspect(&executable(&repeated(257), Some(encoding)), None).unwrap();
            assert_eq!(report.warnings.len(), 1);
            assert_count(&report.warnings[0], 257);
            let info = report.buildinfo.unwrap();
            assert_eq!(info.encoding, "pointer");
            assert_eq!(info.pointer_size, size as u8);
            assert_eq!(info.big_endian, big);
            assert_eq!(facts(info), facts(baseline.buildinfo.unwrap()));
            assert_eq!(report.pclntab, baseline.pclntab);
        }
    }
}

#[test]
fn only_aligned_candidates_with_an_unreadable_version_pointer_are_counted() {
    let mut prefix = vec![0; 192];
    for (offset, size, big) in [(0, 4, false), (32, 8, false), (64, 8, true), (96, 16, false), (129, 8, false)] {
        prefix[offset..offset + 32].copy_from_slice(&broken_pointer(size, big));
    }
    let report = go::inspect(&executable(&prefix, Some(Encoding::Inline)), None).unwrap();
    assert_eq!(report.warnings.len(), 1);
    assert_count(&report.warnings[0], 3);
    assert!(report.buildinfo.unwrap().complete);
}
