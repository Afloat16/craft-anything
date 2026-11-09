# Archive, compression and package contributions

Ten independent improvements are published in Afloat16/craft-anything initially based on upstream `1b5f516b07277036e9e6cca5e75c0a5ebdfcd9f1`. Each branch contains an earlier roadmap claim, a focused implementation, regression fixtures and corresponding roadmap/status updates.

Confirmed upstream pull requests: **10**. Merged: **6**. Fork validation PRs and material/validation branches are excluded from contribution counts.

## Source revisions and checks

| # | Improvement | Branch head | Tested source revision | Verification | Upstream contribution |
|---|---|---|---|---|---|
| 1 | Keep PyInstaller CArchive members inside the payload | [a574cde](https://github.com/Afloat16/craft-anything/commit/a574cde8b266b8a07f84b0ad70a809a08e8f4d95) | [a574cde](https://github.com/Afloat16/craft-anything/commit/a574cde8b266b8a07f84b0ad70a809a08e8f4d95) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1363](https://github.com/storytold/craft-anything/pull/1363) |
| 2 | Reject ASAR manifest nodes with invalid file metadata | [2c5df53](https://github.com/Afloat16/craft-anything/commit/2c5df5307f4733355a4e88be1ebf58bbfb0b95d1) | [1613476](https://github.com/Afloat16/craft-anything/commit/1613476ae0e8af07bf09e69f6454705047ec1e90) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1364](https://github.com/storytold/craft-anything/pull/1364) |
| 3 | Validate gzip header checksums without rejecting informational fields | [7f58bf1](https://github.com/Afloat16/craft-anything/commit/7f58bf1574fae2edfd2fa71de2176639cb9562e1) | [7f58bf1](https://github.com/Afloat16/craft-anything/commit/7f58bf1574fae2edfd2fa71de2176639cb9562e1) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1371](https://github.com/storytold/craft-anything/pull/1371) |
| 4 | Read complete gzip packed members and verify their trailers | [aadb52e](https://github.com/Afloat16/craft-anything/commit/aadb52ee71b387e58d0d090bded25d8f6b6fea8d) | [aadb52e](https://github.com/Afloat16/craft-anything/commit/aadb52ee71b387e58d0d090bded25d8f6b6fea8d) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1372](https://github.com/storytold/craft-anything/pull/1372) |
| 5 | Extract ZIP entries compressed with bzip2 or XZ | [2ca31fb](https://github.com/Afloat16/craft-anything/commit/2ca31fb11dc780e9a96d9fa12beadf2e80ad4e53) | [2ca31fb](https://github.com/Afloat16/craft-anything/commit/2ca31fb11dc780e9a96d9fa12beadf2e80ad4e53) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1374](https://github.com/storytold/craft-anything/pull/1374) |
| 6 | Extract ZIP LZMA members with checked framing and output limits | [fc220b1](https://github.com/Afloat16/craft-anything/commit/fc220b1a29572e2b2c17ed2b948e915c54fa6de6) | [a298987](https://github.com/Afloat16/craft-anything/commit/a298987d966e4e58f84afd6ed81d46d88dd4ec69) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1375](https://github.com/storytold/craft-anything/pull/1375) |
| 7 | Enforce standalone LZMA budgets before large dictionary growth | [f57923e](https://github.com/Afloat16/craft-anything/commit/f57923ea32733aa2d4e383bb44dd4b3892b9dfe5) | [f57923e](https://github.com/Afloat16/craft-anything/commit/f57923ea32733aa2d4e383bb44dd4b3892b9dfe5) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1381](https://github.com/storytold/craft-anything/pull/1381) |
| 8 | Preserve GNU tar member identities at the long-name limit | [ffe055c](https://github.com/Afloat16/craft-anything/commit/ffe055c297b98df5ac7c329f74d0bd86807a83bd) | [de2c639](https://github.com/Afloat16/craft-anything/commit/de2c63908909cab675887103921ed2601b1bae9b) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42640688213/job/129240669663) | [#1362](https://github.com/storytold/craft-anything/pull/1362) |
| 9 | Keep GNU sparse tar payloads out of regular-file extraction | [5d18e4e](https://github.com/Afloat16/craft-anything/commit/5d18e4ee2a33fc75e84d2322160b0a02100b755f) | [576b1d0](https://github.com/Afloat16/craft-anything/commit/576b1d0bc3b9ac26b00bfa748de20ca7b44bb702) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1382](https://github.com/storytold/craft-anything/pull/1382) |
| 10 | Decode concatenated XZ streams and standard stream padding | [f25796a](https://github.com/Afloat16/craft-anything/commit/f25796a28ead5c1f0b5233778a5578a8bd944684) | [f25796a](https://github.com/Afloat16/craft-anything/commit/f25796a28ead5c1f0b5233778a5578a8bd944684) | [Formats checks passed](https://github.com/Afloat16/claude-mem/actions/runs/42649592140/job/129272016386) | [#1383](https://github.com/storytold/craft-anything/pull/1383) |

All ten original implementations passed formatting, the complete formats crate tests including their added regressions, and all-targets clippy with warnings denied. The table links the latest verified source for each change; current integration heads are tracked separately below. The original supplementary verification also passed layers, locks, secret scanning, WebAssembly checks and Cargo.lock stability for all ten source revisions. [Supplementary verification](https://github.com/Afloat16/claude-mem/actions/runs/42640688213) uses one runner and shares compilation output across separate exact-revision checkouts.

## Before and after evidence

The [test-only baseline](https://github.com/Afloat16/craft-anything/commit/9415e3d2c7aa457474d691aab342203830a34ccb) keeps production code byte-for-byte at upstream `1b5f516b07277036e9e6cca5e75c0a5ebdfcd9f1` and adds the public-reader regressions. Its [actual run](https://github.com/Afloat16/claude-mem/actions/runs/42636776430/job/129226895582) completed with **530 passed and 16 failed**. The 16 failures cover all ten improvements; the four added valid-input and robustness controls passed. Every added regression passes on its corresponding fixed branch.

Three assertions calling the new private XZ VLI helper are excluded from the baseline because that helper does not exist in the old production code; all public API regression cases remain.

| # | Regression cases failing on the unchanged production code |
|---|---|
| 1 | `members_cannot_read_the_table_cookie_or_signed_tail`, `a_pyz_in_the_signed_tail_does_not_supply_nested_members` |
| 2 | `malformed_nodes_do_not_become_empty_files` |
| 3 | `gzip_checks_the_optional_header_checksum`, `gzip_accepts_informational_bytes_but_rejects_reserved_flags` |
| 4 | `gzip_members_require_complete_valid_trailers`, `gzip_concatenation_shares_the_output_limit` |
| 5 | `bzip2_and_xz_zip_entries_keep_size_crc_and_output_checks` |
| 6 | `zip_lzma_reads_size_delimited_and_end_marked_streams`, `zip_lzma_rejects_bad_properties_without_unbounded_dictionaries` |
| 7 | `standalone_lzma_checks_declared_sizes_before_decoding`, `standalone_lzma_stops_before_a_large_window_reaches_a_damaged_tail` |
| 8 | `oversized_gnu_names_do_not_alias_their_truncated_prefix` |
| 9 | `gnu_sparse_pax_members_are_not_regular_file_bytes`, `sparse_metadata_survives_other_extension_headers` |
| 10 | `xz_concatenated_streams_and_padding_share_one_budget` |

## Scope and history review

The ten branches address separate CArchive payload ranges, ASAR node grammar, gzip header checksums, packed gzip member integrity, ZIP bzip2/XZ extraction, ZIP LZMA extraction, standalone LZMA budgets, GNU long-name identities, GNU sparse member handling, and XZ concatenation. Candidate behavior was checked against the full upstream pull-request corpus and prior Afloat16 contributions. The manifest records the relevant nearby prior work for each change.

The XZ concatenation improvement retains the existing backing decoder and does not introduce a separate hard limit for its internal XZ block allocation. The standalone LZMA budget improvement targets the public reader, distinct from the already protected Unity decoder path.

[manifest.json](manifest.json) contains exact source revisions, regression names, deduplication notes, PR descriptions, before/after verification URLs and complete submission links.

## Integration with upstream main

The ordinary merge commits preserve both parents and all upstream changes. Their comparisons against the integrated main contain only each original improvement and its corresponding documentation.

| # | Current head | Integrated main | Current-head verification |
|---|---|---|---|
| 1 | [a574cde](https://github.com/Afloat16/craft-anything/commit/a574cde8b266b8a07f84b0ad70a809a08e8f4d95) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |
| 2 | [2c5df53](https://github.com/Afloat16/craft-anything/commit/2c5df5307f4733355a4e88be1ebf58bbfb0b95d1) | `cdc8a88` | [in_progress](https://github.com/Afloat16/claude-mem/actions/runs/42653593224) |
| 3 | [7f58bf1](https://github.com/Afloat16/craft-anything/commit/7f58bf1574fae2edfd2fa71de2176639cb9562e1) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |
| 4 | [aadb52e](https://github.com/Afloat16/craft-anything/commit/aadb52ee71b387e58d0d090bded25d8f6b6fea8d) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |
| 5 | [2ca31fb](https://github.com/Afloat16/craft-anything/commit/2ca31fb11dc780e9a96d9fa12beadf2e80ad4e53) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |
| 6 | [fc220b1](https://github.com/Afloat16/craft-anything/commit/fc220b1a29572e2b2c17ed2b948e915c54fa6de6) | `cdc8a88` | [in_progress](https://github.com/Afloat16/claude-mem/actions/runs/42653593224) |
| 7 | [f57923e](https://github.com/Afloat16/craft-anything/commit/f57923ea32733aa2d4e383bb44dd4b3892b9dfe5) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |
| 9 | [5d18e4e](https://github.com/Afloat16/craft-anything/commit/5d18e4ee2a33fc75e84d2322160b0a02100b755f) | `cdc8a88` | [in_progress](https://github.com/Afloat16/claude-mem/actions/runs/42653593224) |
| 10 | [f25796a](https://github.com/Afloat16/craft-anything/commit/f25796a28ead5c1f0b5233778a5578a8bd944684) | `7073462` | [success](https://github.com/Afloat16/claude-mem/actions/runs/42649592140) |

## Follow-up after merge

The GNU tar identity contribution #1362 has merged. Its installer follow-up, **Refuse incomplete tar engine installs after GNU metadata stops**, is tracked separately and does not add to the ten planned contributions. The unchanged-production baseline reproduces four failures while two compatibility controls pass. Source `3e588c5589ee56ab50004dd0e87cc47f4f2455da` has verification status **success**. [Follow-up PR](https://github.com/storytold/craft-anything/pull/1394). [Verification](https://github.com/Afloat16/claude-mem/actions/runs/42649935938).
