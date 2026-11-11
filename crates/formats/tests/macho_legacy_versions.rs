use craft_anything_formats::{macho_deps, parse_binary};
use craft_anything_testkit::apple;

fn version_command(command: u32, minimum: u32, sdk: u32) -> Vec<u8> {
    [command, 16, minimum, sdk].into_iter().flat_map(u32::to_le_bytes).collect()
}

fn image(is64: bool, commands: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = apple::macho_min(if is64 { apple::CPU_ARM64 } else { 12 }, 1, commands, &[]);
    if !is64 {
        bytes[..4].copy_from_slice(&0xfeed_faceu32.to_le_bytes());
        // The 32-bit header has no reserved word; all following commands move with it.
        bytes.drain(28..32);
    }
    bytes
}

fn check_version(bytes: &[u8], platform: &str, minimum: &str, sdk: &str) {
    let binary = parse_binary(bytes).unwrap();
    let dependencies = macho_deps::parse_thin(bytes).unwrap();
    assert_eq!(binary.info.get("platform").map(String::as_str), Some(platform));
    assert_eq!(binary.info.get("min_os").map(String::as_str), Some(minimum));
    assert_eq!(binary.info.get("sdk").map(String::as_str), Some(sdk));
    assert_eq!(dependencies.build_versions.len(), 1);
    let version = &dependencies.build_versions[0];
    assert_eq!(version.platform, platform);
    assert_eq!(version.minos, minimum);
    assert_eq!(version.sdk.as_deref(), Some(sdk));
}

#[test]
fn legacy_macos_and_ios_keep_their_sdk_in_the_binary_view() {
    for is64 in [false, true] {
        for (command, platform) in [(0x24, "macos"), (0x25, "ios")] {
            for (sdk, expected) in [(0, "0.0.0"), (0x000a_0405, "10.4.5"), (0xffff_abcd, "65535.171.205")] {
                let data = image(is64, &[version_command(command, 0x0009_0302, sdk)]);
                check_version(&data, platform, "9.3.2", expected);
            }
        }
    }
}

#[test]
fn legacy_tv_and_watch_versions_reach_the_binary_view() {
    for is64 in [false, true] {
        for (command, platform) in [(0x2f, "tvos"), (0x30, "watchos")] {
            let data = image(is64, &[version_command(command, 0x0009_0302, 0x000a_0405)]);
            check_version(&data, platform, "9.3.2", "10.4.5");
        }
    }
}

#[test]
fn legacy_command_labels_agree_in_both_views() {
    for is64 in [false, true] {
        for (command, name) in
            [(0x24, "LC_VERSION_MIN_MACOSX"), (0x25, "LC_VERSION_MIN_IPHONEOS"), (0x2f, "LC_VERSION_MIN_TVOS"), (0x30, "LC_VERSION_MIN_WATCHOS")]
        {
            let data = image(is64, &[version_command(command, 0x0009_0302, 0x000a_0405)]);
            let binary = parse_binary(&data).unwrap();
            let dependencies = macho_deps::parse_thin(&data).unwrap();
            assert_eq!(binary.details["load_commands"][0]["cmd"], name);
            assert_eq!(dependencies.commands[0].cmd, name);
        }
    }
}

#[test]
fn short_legacy_commands_do_not_borrow_the_next_commands_fields() {
    for is64 in [false, true] {
        for command in [0x24, 0x25, 0x2f, 0x30] {
            for size in [8u32, 12] {
                let mut short = version_command(command, 0x0009_0302, 0x000a_0405);
                short.truncate(size as usize);
                short[4..8].copy_from_slice(&size.to_le_bytes());
                let data = image(is64, &[short, apple::lc_build_version()]);
                check_version(&data, "macos", "13.0.0", "14.0.0");
                assert!(!parse_binary(&data).unwrap().warnings.is_empty());
                assert!(!macho_deps::parse_thin(&data).unwrap().warnings.is_empty());
            }
        }
    }
}

#[test]
fn legacy_fields_outside_sizeofcmds_remain_unknown() {
    for is64 in [false, true] {
        for command in [0x24, 0x25, 0x2f, 0x30] {
            let mut data = image(is64, &[version_command(command, 0x0009_0302, 0x000a_0405)]);
            data[20..24].copy_from_slice(&12u32.to_le_bytes());
            let binary = parse_binary(&data).unwrap();
            for key in ["platform", "min_os", "sdk"] {
                assert!(!binary.info.contains_key(key), "{key}");
            }
            assert!(!binary.warnings.is_empty());
            let dependencies = macho_deps::parse_thin(&data).unwrap();
            assert!(dependencies.build_versions.is_empty());
            assert!(!dependencies.warnings.is_empty());
        }
    }
}

#[test]
fn modern_build_versions_keep_their_platform_and_sdk() {
    for is64 in [false, true] {
        for (platform, name) in [(1u32, "macos"), (2, "ios"), (3, "tvos"), (4, "watchos"), (10, "driverkit"), (12, "visionos-simulator"), (99, "platform-99")] {
            let mut command = apple::lc_build_version();
            command[8..12].copy_from_slice(&platform.to_le_bytes());
            check_version(&image(is64, &[command]), name, "13.0.0", "14.0.0");
        }
    }
}

#[test]
fn big_endian_enrichment_keeps_its_existing_limitation() {
    for is64 in [false, true] {
        let mut data = image(is64, &[version_command(0x24, 0x0009_0302, 0x000a_0405)]);
        for word in data.as_chunks_mut::<4>().0 {
            word.reverse();
        }
        let binary = parse_binary(&data).unwrap();
        assert!(!binary.info.contains_key("platform"));
        assert!(binary.warnings.iter().any(|warning| warning.contains("big-endian Mach-O")));
        let dependencies = macho_deps::parse_thin(&data).unwrap();
        assert_eq!(dependencies.build_versions[0].platform, "macos");
        assert_eq!(dependencies.build_versions[0].sdk.as_deref(), Some("10.4.5"));
    }
}
