# Validation provenance

This directory is a fork-only validation harness, not a production change.

| Material | Source / author | Licence | Use |
| --- | --- | --- | --- |
| Rename/lock probe, generated empty-file fixtures, PowerShell runners and workflow | Afloat16, original validation work for Craft #2183 | MIT OR Apache-2.0 | Generated entirely inside the job's exclusive temporary directory; no third-party or personal fixtures |
| Three child-opening helpers in `src/lib.rs` | Published `crates/platform/src/fs.rs` at `f3715a5539b28ae24ba244c7dc5f6f865257f3cf` | MIT OR Apache-2.0 | Helper bodies copied byte-for-byte from this repository; only standalone crate attributes/imports added |
| `fs_at` 0.2.1 | Robert Collins; <https://docs.rs/crate/fs_at/0.2.1/source/Cargo.toml> | Apache-2.0 | Unmodified dependency through its public API, already present in production |
| `windows` 0.62.2 | Microsoft; <https://docs.rs/crate/windows/0.62.2/source/Cargo.toml> | MIT OR Apache-2.0 | Unmodified dependency through its public API, already present in production |

No third-party implementation source is copied into the probe. Registry checksums are
recorded in the standalone `Cargo.lock`. Existing production notices remain unchanged.
Both jobs preserve SHA-256 hashes of the exact source and scripts with their logs.
