# Other format contributions — 2026-11-09

Status: 10 formal upstream PRs; 9 merged and 1 open; all independent checks and the combined full Linux gate passed. All ten independent changes have formal upstream PRs, with 48 new integration test functions. The table records the exact source and documentation heads, successful independent checks, and current upstream state. Latest verification: 2026-11-09 15:54:59 UTC.

**Upstream PRs created: 10. Merged PRs: 9.** Each independent upstream PR counts as one merged improvement only after it is merged. Fork verification PRs and test-only branches are excluded.

## Source branches

| # | Improvement | Current head | New tests | Verification | Upstream PR | State |
|---|---|---|---:|---|---|---|
| 1 | Reject Java constants that exceed the constant-pool slots (R3.12) | [`8a5332a89480`](https://github.com/Afloat16/craft-anything/commit/8a5332a89480f1f354f404860afe5433a07cd87a) | 4 | independent source checks passed; merged current main to resolve shared documentation conflicts | [#1373](https://github.com/storytold/craft-anything/pull/1373) | merged |
| 2 | Keep embedded Godot pack reads before the trailer (R3.12) | [`ae10e66be638`](https://github.com/Afloat16/craft-anything/commit/ae10e66be638852663f6191d767c2705db6de5b3) | 4 | independent source checks passed; merged current main to resolve shared documentation conflicts | [#1379](https://github.com/storytold/craft-anything/pull/1379) | open |
| 3 | Bound retained metadata when listing Node SEA programs (R3.12) | [`a91f5e8afecf`](https://github.com/Afloat16/craft-anything/commit/a91f5e8afecf0e88a23d7dc906a7aaf863e88666) | 7 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1377](https://github.com/storytold/craft-anything/pull/1377) | merged |
| 4 | Respect the UTF-8 output budget for IL2CPP literals (R3.12) | [`43279b281e6c`](https://github.com/Afloat16/craft-anything/commit/43279b281e6c3dbe7592896d2bfa39f9c2261fcc) | 4 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1380](https://github.com/storytold/craft-anything/pull/1380) | merged |
| 5 | Decode Unity R16 and RG16 textures (R3.9) | [`ab2f5b83f33a`](https://github.com/Afloat16/craft-anything/commit/ab2f5b83f33ae9a309bb7265055314e70ed84208) | 4 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1378](https://github.com/storytold/craft-anything/pull/1378) | merged |
| 6 | Preserve DEX boundaries when member or handler lists are capped (R3.12) | [`eeab41fd938c`](https://github.com/Afloat16/craft-anything/commit/eeab41fd938c2f26d599a4ba9682f8525b8ee67b) | 5 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1388](https://github.com/storytold/craft-anything/pull/1388) | merged |
| 7 | Keep Android resource children inside their parent chunks (R3.12) | [`5eb384a87f45`](https://github.com/Afloat16/craft-anything/commit/5eb384a87f453156e37025f878ddfb427d6ceaa2) | 7 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1389](https://github.com/storytold/craft-anything/pull/1389) | merged |
| 8 | Bound WebAssembly metadata across repeated custom sections (R3.12) | [`aa825077c910`](https://github.com/Afloat16/craft-anything/commit/aa825077c910288caabcc5276de68ab86b778485) | 5 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1390](https://github.com/storytold/craft-anything/pull/1390) | merged |
| 9 | Validate Unity serialized metadata ranges before reading (R3.12) | [`27af3aedf647`](https://github.com/Afloat16/craft-anything/commit/27af3aedf6478a1d902b2b14c298b1c037759cad) | 4 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1391](https://github.com/storytold/craft-anything/pull/1391) | merged |
| 10 | Decode Unreal floating-point texture formats (R3.9) | [`4977803bebe9`](https://github.com/Afloat16/craft-anything/commit/4977803bebe940990854ad87ae1d8144e592cf0f) | 4 | independent formats and compatibility gates passed; source unchanged by documentation completion | [#1392](https://github.com/storytold/craft-anything/pull/1392) | merged |

## CI evidence

- Java passed the full existing Linux gate and unchanged-lockfile check: https://github.com/Afloat16/craft-anything/actions/runs/42629757277
- Combined verification of the ten source changes: https://github.com/Afloat16/craft-anything/actions/runs/42642365482/job/129246683182 (success).
- Independent checks record exact checkout SHAs, case counts and job URLs below. Totals from separate full crate suites share the same existing tests and must not be added as distinct coverage.
- Test-only unchanged-parser baseline: `ba8dcfa622a8354149542d8405c9158805548fc5`, 46 regression functions, result: partial. The combined 46-test baseline job at ebd0ef7 was cancelled before completion and provides no complete before-fix result. Focused exact-baseline checks did reproduce the Node argument count defect (1 failed test) and four WebAssembly inventory defects (1 compatibility test passed). Unity integer baseline excludes its two tests that require new Layout variants.

## Prior-work review

The candidate search covered 927 upstream PR titles/bodies and 206 PRs by Afloat16, with related diffs checked for overlapping parser work. New upstream PRs through #1357 and 280 fork branch names were reviewed. PRs #1353–#1357 concern Swift and do not duplicate these changes. Relevant prior PR numbers and scope differences are recorded per item. Current source branch SHAs were independently confirmed through the branch API.

## 1. Reject Java constants that exceed the constant-pool slots (R3.12)

Branch: `bugfix/r3.12-java-wide-pool-slot-20261109`. Current head: `8a5332a89480f1f354f404860afe5433a07cd87a`. Source head: `e0dbc3a5b60b140f26f3622f34c11035e3c1cb7d`.

A Long or Double in the final declared pool slot was accepted without its required second slot. At constant_pool_count = 65535 the u16 cursor overflowed while advancing past that entry.

Check the next pool index before retaining each constant. Reject a wide entry whose reserved slot does not fit, while preserving correctly formed maximum-size pools.

Regression coverage: Long and Double entries at ordinary and maximum u16 boundaries, valid two-slot tail entries, ordinary constants and an existing class fixture.

Prior-work distinction: Earlier Java work covered bytecode-wide opcodes and method descriptors; this change validates constant-pool slot accounting.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42633003467/job/129213811541): source `e0dbc3a5b60b140f26f3622f34c11035e3c1cb7d`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1373) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-java-wide-pool-slot-20261109)

## 2. Keep embedded Godot pack reads before the trailer (R3.12)

Branch: `bugfix/r3.12-godot-trailer-boundary-20261109`. Current head: `ae10e66be638852663f6191d767c2705db6de5b3`. Source head: `9744a0569a7c51d8938ce645e52f9a66965fb1e2`.

The embedded PCK directory and member reader used the executable's complete length as the pack boundary. This let damaged entries consume the twelve-byte embedded-pack trailer as table fields or member data.

Exclude the embedded trailer from directory parsing and stored-member reads, mark members crossing that boundary unreadable, and enforce it when an entry is passed directly to the reader.

Regression coverage: PCK versions 1, 2 and 3; member ranges in the footer, directory counts and records crossing it, direct reads of forged entries, and valid embedded/standalone packs.

Prior-work distinction: Earlier Godot contributions covered installation and process paths; this change bounds the embedded PCK parser and extractor.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42633003467/job/129213811429): source `9744a0569a7c51d8938ce645e52f9a66965fb1e2`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1379) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-godot-trailer-boundary-20261109)

## 3. Bound retained metadata when listing Node SEA programs (R3.12)

Branch: `bugfix/r3.12-node-sea-retained-budget-20261109`. Current head: `a91f5e8afecf0e88a23d7dc906a7aaf863e88666`. Source head: `ce34c2446499f8740c9c0ad7855af845f8e8355c`.

SEA parsing retained every declared asset and argument before applying the output entry cap. Argument strings and asset keys could also expand without a shared decoded text budget.

Retain only the bounded asset/argument inventory and at most 8 MiB of decoded metadata text, including replacement-character expansion. Omit oversized fields whole, preserve declared totals, mark truncation, and continue validating every skipped binary field.

Regression coverage: Entry limits including empty arguments, oversized paths/keys, invalid UTF-8 expansion, malformed bytes after the retained prefix and ordinary metadata.

Prior-work distinction: No matching Node SEA retained-inventory or decoded-text budget change appeared in the checked histories.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42642329977/job/129246378958): source `ce34c2446499f8740c9c0ad7855af845f8e8355c`, 960 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1377) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-node-sea-retained-budget-20261109)

## 4. Respect the UTF-8 output budget for IL2CPP literals (R3.12)

Branch: `bugfix/r3.12-il2cpp-literal-byte-budget-20261109`. Current head: `43279b281e6c3dbe7592896d2bfa39f9c2261fcc`. Source head: `d81b7d5705461ef4425c33a03de4252f5f096f13`.

Literal extraction limited input bytes, then used lossy UTF-8 conversion. Replacement characters could make the returned string larger than the advertised maximum, and a cut multi-byte character could be reported as damaged text.

Build literal text within the output byte budget, preserve Unicode boundaries and charge the full replacement-character width. Keep the completeness flag tied to consumed original bytes.

Regression coverage: ASCII and empty values, multi-byte UTF-8 across every short boundary, invalid bytes, incomplete input sequences and replacement expansion.

Prior-work distinction: The existing IL2CPP owner checks validate declaring types. This change bounds literal text output.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42634877637/job/129220281761): source `d81b7d5705461ef4425c33a03de4252f5f096f13`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1380) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-il2cpp-literal-byte-budget-20261109)

## 5. Decode Unity R16 and RG16 textures (R3.9)

Branch: `feature/r3.9-unity-integer-textures-20261109`. Current head: `ab2f5b83f33ae9a309bb7265055314e70ed84208`. Source head: `540b4947aa1bc7a7195d67eabcbff5582b8a3127`.

The texture exporter rejected Unity's R16 and RG16 formats even though they can be decoded using the existing bounded pixel pipeline.

Add little-endian unsigned R16 grayscale decoding and RG16's two eight-bit red/green channels. Preserve Unity row inversion, first-mip handling and output allocation limits.

Regression coverage: Unsigned extrema and quantization boundaries, red/green channel order, Unity row inversion, first-mip length, truncated pixels, zero/huge dimensions and output budgets.

Prior-work distinction: Existing float-texture support uses different layouts; this adds the missing integer formats.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42633003467/job/129213811395): source `540b4947aa1bc7a7195d67eabcbff5582b8a3127`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1378) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:feature/r3.9-unity-integer-textures-20261109)

## 6. Preserve DEX boundaries when member or handler lists are capped (R3.12)

Branch: `bugfix/r3.12-dex-capped-table-alignment-20261109`. Current head: `eeab41fd938c2f26d599a4ba9682f8525b8ee67b`. Source head: `149f060b9959e8906327e799088728820659513c`.

A capped class-data group left its unread records at the cursor, then interpreted them as the following group. Capped or incomplete typed-handler lists could likewise turn their remaining bytes into a fabricated catch-all address.

Return retained class members as incomplete before crossing an unconsumed group, and read a catch-all only after every declared typed handler was decoded.

Regression coverage: Over-cap and exact-cap class members; oversized, truncated, empty and exact-cap typed-handler lists with real catch-all controls.

Prior-work distinction: Earlier DEX work covered LEB values, MUTF-8, register lists and array widths. This change preserves record alignment when retention limits are reached.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42633003467/job/129213811327): source `149f060b9959e8906327e799088728820659513c`, 958 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1388) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-dex-capped-table-alignment-20261109)

## 7. Keep Android resource children inside their parent chunks (R3.12)

Branch: `bugfix/r3.12-arsc-parent-chunk-boundaries-20261109`. Current head: `5eb384a87f453156e37025f878ddfb427d6ceaa2`. Source head: `ea680eeb9e9267e54e304448596fd947cc18f3d5`.

A resource child could fit the physical input while crossing the size declared by its enclosing table or package. String-pool offsets, library records and short child headers could therefore borrow unrelated bytes.

Bound each reader to its declared parent, require complete package/known-child headers, keep package string pools inside the package payload, and retain only complete fixed-width library records.

Regression coverage: Table/package crossings, escaped package pools, retained earlier entries, sibling library data, short headers, truncated bag parents and both legacy 284-byte and current 288-byte package headers.

Prior-work distinction: PR #698 bounded a string pool by its own chunk. This change enforces the enclosing table/package and sibling-child boundaries; it does not replace that pool implementation.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42636776430/job/129226895431): source `ea680eeb9e9267e54e304448596fd947cc18f3d5`, 960 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1389) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-arsc-parent-chunk-boundaries-20261109)

## 8. Bound WebAssembly metadata across repeated custom sections (R3.12)

Branch: `bugfix/r3.12-wasm-custom-inventory-budget-20261109`. Current head: `aa825077c910288caabcc5276de68ab86b778485`. Source head: `232100030acee41d80523bdcc7998fd0a1998aa3`.

Every repeated name or producers custom section received a new allowance. Their combined retained inventories could exceed MAX_ITEMS, and the existing local cutoffs silently omitted metadata without setting the report's truncation flag.

Share the function-name, producer-field and producer-value limits across the entire module. Permit updates to retained function names at the cap and report both local and global omissions.

Regression coverage: Repeated function-name sections, updates at the cap, producer values and empty fields across sections, each local cutoff and an ordinary module fixture.

Prior-work distinction: PR #1126 validates custom debug/source-map strings. This change bounds retained name and producer inventories across repeated sections.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42642340046/job/129246412808): source `232100030acee41d80523bdcc7998fd0a1998aa3`, 958 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1390) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-wasm-custom-inventory-budget-20261109)

## 9. Validate Unity serialized metadata ranges before reading (R3.12)

Branch: `bugfix/r3.12-unity-metadata-range-20261109`. Current head: `27af3aedf6478a1d902b2b14c298b1c037759cad`. Source head: `de9d7444f5098a3e6c27b32f3b5249d4bb2743fc`.

The metadata reader accepted a header offset beyond a short input and read past the declared metadata region into object bytes. The string reader could panic on an impossible starting offset.

Validate metadata offset and size with checked arithmetic against the object-data boundary, declared file length and available input. Parse only the resulting metadata slice.

Regression coverage: Every prefix before the metadata end, shortened declared metadata, arithmetic overflow and impossible public Header ranges, plus equality of full-file and exact-prefix results.

Prior-work distinction: PR #890 validates serialized object extents. This change validates the separate metadata header and reader range.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42633003467/job/129213811357): source `de9d7444f5098a3e6c27b32f3b5249d4bb2743fc`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1391) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-unity-metadata-range-20261109)

## 10. Decode Unreal floating-point texture formats (R3.9)

Branch: `feature/r3.9-unreal-float-textures-20261109`. Current head: `4977803bebe940990854ad87ae1d8144e592cf0f`. Source head: `75b507c0b96fe22c8f8a1e517c31ef6cee207a01`.

Cooked Unreal textures with supported half- and full-precision channel layouts were reported as unsupported because their pixel-format names were not mapped to the shared float decoder.

Route scalar, red/green and RGBA half/full precision formats, including the filtered half aliases, through the bounded pixel reader. RGB converts from linear values to 8-bit sRGB; alpha stays linear and values are clipped for PNG output.

Regression coverage: Cooked package fixtures reaching all six channel layouts, filtered aliases, negative/non-finite values, alpha behavior, stored row order, short mips, zero/huge dimensions and output budgets.

Prior-work distinction: The existing float decoder was connected to Unity formats. This adds Unreal format dispatch and cooked-package regression coverage.

- [Independent check](https://github.com/Afloat16/claude-mem/actions/runs/42634877637/job/129220281335): source `75b507c0b96fe22c8f8a1e517c31ef6cee207a01`, 957 passed / 0 failed / 0 ignored; success.

[Upstream PR](https://github.com/storytold/craft-anything/pull/1392) · [Review the branch](https://github.com/storytold/craft-anything/compare/main...Afloat16:feature/r3.9-unreal-float-textures-20261109)
