# fix(formats): reject incomplete GNU tar record extents

## Problem
A GNU long-name or long-link record can contain its complete body but end before its 512-byte padding. The parser stopped with an earlier member list without setting the typed metadata stop reason, allowing the installer to overwrite an existing file and report success for a partial archive. An overflowing padded extent took the same unclassified stop path.

This follows the remaining record-extent case reported on #1394 after that PR was merged; #1394 already covers oversized identities and truncated metadata bodies.

## Change
Require the complete padded extent of GNU `L` and `K` records and set `GnuMetadataTruncated` when padding is missing or checked extent arithmetic fails. The existing installer guard then rejects the archive before writing any member. Earlier members remain available for inspection, and ordinary files with complete data remain readable without trailing padding or an end marker.

## Validation
The [exact-revision run](https://github.com/Afloat16/claude-mem/actions/runs/42659466852/job/129306793039) completed successfully and includes both the reproduction and the correction:

- Test-only baseline `bec699d80a3f4d9ae977d20e6ceecc50af1e5dfe` leaves production code at upstream `69e7f5773d48368b18f22c2e7abe79d115589174`. Four new installer guards fail while seven controls pass; each failing case returns `Ok(1)` and replaces the pre-existing file.
- Corrected source `25c45ef05d81bfee699dd3c7d351aefc27d83224` passes the complete formats and engines test suites: **1,200 passed, 0 failed, 1 existing network-download test ignored**. Added cases cover both GNU record kinds, missing padding, overflowing extents, retained existing files, fully padded records, and ordinary-member compatibility.
- Formatting, all-targets clippy with warnings denied, layers, locks, secret scanning, WebAssembly checks and Cargo.lock stability all pass for that exact source.

The workflow runs in Afloat16/claude-mem and checks out the named craft-anything revisions. Final head `541e27bacfb18cb7b060f25cb6f37b48daadd700` adds only ROADMAP and STATUS verification notes after the tested source; source and test files are unchanged.

Roadmap: R3.12.
