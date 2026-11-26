# Validation provenance

This directory is a fork-only validation harness, not a production change.

| Material | Source / author | Licence | Use |
| --- | --- | --- | --- |
| Rename/lock probe, generated empty-file fixtures, PowerShell runners and workflow | Afloat16, original validation work for Craft #2183 | MIT OR Apache-2.0 | Generated entirely inside the job's exclusive temporary directory; no third-party or personal fixtures |
| Three child-opening helpers in `src/lib.rs` | Published `crates/platform/src/fs.rs` at `f367fe8fd0f054bbe2320742eac6fd853d6dbf91` | MIT OR Apache-2.0 | Helper bodies and their current platform-specific permission documentation copied byte-for-byte from this repository; only standalone crate attributes/imports added |
| `fs_at` 0.2.1 | Robert Collins; <https://docs.rs/crate/fs_at/0.2.1/source/Cargo.toml> | Apache-2.0 | Unmodified dependency through its public API, already present in production |
| `windows` 0.62.2 | Microsoft; <https://docs.rs/crate/windows/0.62.2/source/Cargo.toml> | MIT OR Apache-2.0 | Unmodified dependency through its public API, already present in production |

No third-party implementation source is copied into the probe. Registry checksums are
recorded in the standalone `Cargo.lock`. Existing production notices remain unchanged.
The validation job preserves SHA-256 hashes of the exact source and scripts with its logs.
