# Requested website language: validation

Source: `09af824249f49b3a5a79b07b3873bf447c10325f`, tree
`298ff2b496ce6b7eb252cdeac7a564d1ffaac0fc`. These are validation records,
kept on a separate fork branch and outside the feature pull request.

## Behavior

An explicit request to translate a rebuilt website is retained when handed to its builder.
The check evaluates the built page's declared language, and stops asking for the original
words and headings. Missing measurements and translated controls that cannot be paired are
unknown, not invented success. A matching declaration does not establish translation quality.
The explicit original-language reset survives restarts and later unrelated requests.
Failed saves retain the notes and the previous language choice.

## Results

- The counterfactual content-policy run restores the old text-comparison policy inside the
  new adapter. It fails two behavior cases, with three controls passing. It is not a claim
  that the entire feature's tests compiled against an unmodified old tree.
- Final complete AI crate: 713 unit + 16 integration tests passed; 4 pre-existing ignored.
- Complete engine crate with `--no-fail-fast`: 762 unit + 79 integration tests passed,
  one failure and 13 pre-existing ignored; no tests were filtered from that full run.
- The one failure is `workspace::progress::tests::special_files_contribute_their_name_and_type_without_reading_them`,
  at the existing `UnixListener::bind` with OS error 1 (operation not permitted).
  The exact same test fails on untouched main `73771bf` with the same error. Both source
  blobs are `6c5952a8113d78e22f0c47df4a284802da544551`. The full engine gate is not green.
- CLI build, all-targets AI/engine clippy with `-D warnings`, fmt, schema generation,
  layers, locks, secret scan, normal commit hook and whitespace checks passed.
- An earlier full link stopped for disk space. Its restart did not change code to bypass
  the failure. A new manifest-version documentation mismatch and an iterator lint were
  fixed; the complete affected suites/checks above were then run.

Linux x86_64, Rust 1.100.0. Cargo ran offline, one build job, no incremental compilation;
the engine package used opt-level=0 and debug=0 in dev/test profiles. The CLI was built before
the AI suite, whose integration tests invoke it. No live provider, browser reconstruction,
GUI, other OS or wasm gate was run. The touched crates are L4/L5; no tool or L0-L2 crate changed.

The final rebase onto main changes only the shared agent/contribution documentation from
#2199. Every feature file is byte-identical before and after that rebase. Final added licence
notices and source/migration comments do not change runtime behavior.

## Reproduction commands

`cargo build --offline -p craft-anything-cli`

`cargo test --offline -p craft-anything-ai -- --test-threads=2`

`cargo test --offline --no-fail-fast -p craft-anything-engine -- --test-threads=2`

`cargo clippy --offline -p craft-anything-engine -p craft-anything-ai --all-targets -- -D warnings`

The isolated untouched-main failure used:
`cargo test --offline -p craft-anything-engine --lib workspace::progress::tests::special_files_contribute_their_name_and_type_without_reading_them -- --exact`.

## Limits and integration

Explicit recognition is conservative: simple English command forms, native language aliases,
Chinese translation forms, explicit locale tags and html lang setters. Quoted/fenced/example
or ambiguous requests leave the saved choice alone. It is not general natural-language
understanding. Locale syntax is delegated to unmodified ICU4X 2.3.0 (Unicode-3.0), already locked
transitively. The non-wildcard match follows RFC 4647 section 3.3.1.

New manifest v6 files open read-only in older builds. An old process that already loaded v5
must be closed before the upgrade. #2099 independently adds a programming-toolchain migration
and also uses manifest v6; whichever merges second must advance the migration and regenerate
the schema. Its distinct stale-writer fix is not copied into this PR.

History checks covered all-author/all-state natural-language and changed-function searches,
the current fork branches and recent main changes. #1164 added the roadmap plan only;
#2099 changes programming-language/toolchain selection, not website content language.
