#![cfg(unix)]

use craft_anything_engines::frida::{Attach, Frida, Selector};
use craft_anything_evidence::ErrorCode;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn local() -> Selector {
    Selector { device_id: Some("local".into()), remote: None }
}

fn spawn(argv: &[&str]) -> Attach {
    Attach::Spawn { program: PathBuf::from("/fixture/target program"), argv: argv.iter().map(|s| (*s).to_string()).collect() }
}

fn capture(selector: &Selector, attach: &Attach) -> Vec<String> {
    let dir = tempfile::tempdir().unwrap();
    let cli = dir.path().join("frida");
    std::fs::write(&cli, "#!/bin/sh\nprintf '%s\\000' \"$@\"\n").unwrap();
    std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o755)).unwrap();
    let output = Frida::new(cli).run_script(selector, attach, Path::new("/fixture/script file.js")).unwrap();
    assert!(output.success);
    assert_eq!(output.stdout.last(), Some(&0));
    output.stdout[..output.stdout.len() - 1].split(|b| *b == 0).map(|part| String::from_utf8(part.to_vec()).unwrap()).collect()
}

fn target_arguments(arguments: &[String]) -> &[String] {
    let boundary = arguments.iter().position(|arg| arg == "--").expect("a CLI option boundary before target arguments");
    assert_eq!(&arguments[..boundary], ["-D", "local", "-f", "/fixture/target program", "-l", "/fixture/script file.js", "-q"]);
    &arguments[boundary + 1..]
}

#[test]
fn spawned_program_flags_are_not_frida_options() {
    let expected = ["--version", "--help", "-q", "--target-only", "-l", "a different.js"];
    let actual = capture(&local(), &spawn(&expected));
    assert_eq!(target_arguments(&actual), expected);
}

#[test]
fn a_literal_separator_is_kept_after_the_cli_boundary() {
    let expected = ["--", "-q", "trailing"];
    let actual = capture(&local(), &spawn(&expected));
    assert_eq!(target_arguments(&actual), expected);
}

#[test]
fn empty_unicode_and_whitespace_arguments_are_preserved() {
    let expected = ["", "", "two words", "café", "two  spaces", "tab\tinside", "line\nbreak"];
    let actual = capture(&local(), &spawn(&expected));
    assert_eq!(target_arguments(&actual), expected);
}

#[test]
fn options_file_arguments_are_refused_before_launch() {
    for spelling in ["-O", "--options-file", "--options-file=", "--options-file=/fixture/options.txt"] {
        let error = Frida::new("/nonexistent/definitely/not/frida")
            .run_script(&local(), &spawn(&["before", spelling, "after"]), Path::new("/fixture/script.js"))
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::BadInput, "{spelling}");
        assert!(error.message.contains("options-file"));
        assert!(!error.message.contains("/fixture/options.txt"));
    }
}

#[test]
fn similar_option_names_and_joined_short_forms_are_not_refused() {
    let expected = ["--options-file-not", "--options-files", "-Options", "-Ojoined", "plain-O", "value=--options-file"];
    let actual = capture(&local(), &spawn(&expected));
    assert_eq!(target_arguments(&actual), expected);
}

#[test]
fn attaching_by_pid_keeps_its_cli_arguments() {
    let actual = capture(&local(), &Attach::Pid(42));
    assert_eq!(actual, ["-D", "local", "-p", "42", "-l", "/fixture/script file.js", "-q"]);
}

#[test]
fn spawning_without_target_arguments_keeps_its_cli_arguments() {
    let actual = capture(&local(), &spawn(&[]));
    assert_eq!(actual, ["-D", "local", "-f", "/fixture/target program", "-l", "/fixture/script file.js", "-q"]);
}

#[test]
fn an_invalid_selector_is_still_refused_before_launch() {
    let error = Frida::new("/nonexistent/definitely/not/frida")
        .run_script(&Selector::default(), &spawn(&["--help"]), Path::new("/fixture/script.js"))
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::BadInput);
    assert!(error.message.contains("device"));
}
