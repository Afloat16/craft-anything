# Thirty contribution follow-ups

Snapshot: **2026-11-03 18:35:31 UTC** / **2026-11-04 02:35:31 UTC+08:00**.

All thirty changes have upstream pull requests. The links below replace the earlier creation links. Each listed head includes main at `f5d1f275a597b434b4bc36d3af27f64bc0fd2773` through a normal merge. GitHub reports all thirty as mergeable, and no unresolved inline review threads were found at this snapshot.

Current CI at these exact heads: **19 successful, 1 failed, 10 in progress**. Earlier successful runs are listed separately and do not cover the final main synchronization.

## Current pull requests

| Item | Pull request | Change | Current head | Current CI |
|---|---|---|---|---|
| 1 | [#688](https://github.com/storytold/craft-anything/pull/688) | Reject signed LEB128 values outside the 64-bit range | [`26b1ab79`](https://github.com/Afloat16/craft-anything/commit/26b1ab7978c0eeb7baa258bfe839ff13fef2dbd0) | [success](https://github.com/storytold/craft-anything/actions/runs/41692721275) |
| 2 | [#690](https://github.com/storytold/craft-anything/pull/690) | Preserve valid text around damaged modified UTF-8 | [`14e95f3e`](https://github.com/Afloat16/craft-anything/commit/14e95f3e7365aa266edc33c527b525949b9ae493) | [success](https://github.com/storytold/craft-anything/actions/runs/41692739762) |
| 3 | [#691](https://github.com/storytold/craft-anything/pull/691) | Reject invalid widened Java instructions | [`ad488410`](https://github.com/Afloat16/craft-anything/commit/ad488410ebc9dc745aaea510f5ffd84ee47789ca) | [success](https://github.com/storytold/craft-anything/actions/runs/41692753004) |
| 4 | [#692](https://github.com/storytold/craft-anything/pull/692) | Validate Java invocation operands before decoding calls | [`5a073e46`](https://github.com/Afloat16/craft-anything/commit/5a073e46febddc75e169e83c90a7c1cdd1437527) | [success](https://github.com/storytold/craft-anything/actions/runs/41692767850) |
| 5 | [#693](https://github.com/storytold/craft-anything/pull/693) | Reject impossible DEX invocation register lists | [`390df5f8`](https://github.com/Afloat16/craft-anything/commit/390df5f83642d4aa9b7964b709079e2c411c5c58) | [success](https://github.com/storytold/craft-anything/actions/runs/41692778074) |
| 6 | [#694](https://github.com/storytold/craft-anything/pull/694) | Reject invalid DEX array payload element widths | [`7d5f4770`](https://github.com/Afloat16/craft-anything/commit/7d5f4770cab5f30dee6c24fd745c35428256ae17) | [success](https://github.com/storytold/craft-anything/actions/runs/41692792746) |
| 7 | [#695](https://github.com/storytold/craft-anything/pull/695) | Read SQLite text using the database encoding | [`9c49f69a`](https://github.com/Afloat16/craft-anything/commit/9c49f69a9032be69896d45419328fdac850eece6) | [success](https://github.com/storytold/craft-anything/actions/runs/41692801601) |
| 8 | [#696](https://github.com/storytold/craft-anything/pull/696) | Keep SQLite cell reads within their page | [`b5b47a5d`](https://github.com/Afloat16/craft-anything/commit/b5b47a5d3d4bd39d8b396dce5d7779199091689a) | [success](https://github.com/storytold/craft-anything/actions/runs/41692814058) |
| 9 | [#697](https://github.com/storytold/craft-anything/pull/697) | Stop repeated SQLite overflow pages from fabricating data | [`97b03a6f`](https://github.com/Afloat16/craft-anything/commit/97b03a6f9022a688daabc91dea259f153dad87cc) | [success](https://github.com/storytold/craft-anything/actions/runs/41692830030) |
| 10 | [#698](https://github.com/storytold/craft-anything/pull/698) | Keep Android string pools within declared chunks | [`cddf8441`](https://github.com/Afloat16/craft-anything/commit/cddf8441b395105fbb452c9d060250dfb8cf448b) | [success](https://github.com/storytold/craft-anything/actions/runs/41692842389) |
| 11 | [#699](https://github.com/storytold/craft-anything/pull/699) | Apply ZIP output limits to stored entries | [`1e593fbe`](https://github.com/Afloat16/craft-anything/commit/1e593fbe41ad275ab2f0f0709f4bd44e034ce962) | [success](https://github.com/storytold/craft-anything/actions/runs/41692159741) |
| 12 | [#700](https://github.com/storytold/craft-anything/pull/700) | Check ZIP member sizes after decompression | [`e9db1bad`](https://github.com/Afloat16/craft-anything/commit/e9db1bad36cd6b70e6835fdce465573824765efa) | [success](https://github.com/storytold/craft-anything/actions/runs/41692181984) |
| 13 | [#701](https://github.com/storytold/craft-anything/pull/701) | Keep ZIP64 values within their extra field | [`aa701bdf`](https://github.com/Afloat16/craft-anything/commit/aa701bdf34486de20ddaf5cc951df8b4ee017586) | [success](https://github.com/storytold/craft-anything/actions/runs/41692195763) |
| 14 | [#702](https://github.com/storytold/craft-anything/pull/702) | Validate ASAR header framing | [`0b9f5d7c`](https://github.com/Afloat16/craft-anything/commit/0b9f5d7c8b3799d087f23ca03ca1c6fee1b54f88) | [success](https://github.com/storytold/craft-anything/actions/runs/41692289599) |
| 15 | [#703](https://github.com/storytold/craft-anything/pull/703) | Traverse ROMFS dot and empty root directories | [`2bef9409`](https://github.com/Afloat16/craft-anything/commit/2bef9409336d42f1a0f764848670dfd6b69fdb87) | [success](https://github.com/storytold/craft-anything/actions/runs/41692309447) |
| 16 | [#704](https://github.com/storytold/craft-anything/pull/704) | Report incomplete cramfs block output | [`8328b4e5`](https://github.com/Afloat16/craft-anything/commit/8328b4e549330a09b4eb787857089a5263d0966d) | [success](https://github.com/storytold/craft-anything/actions/runs/41692327401) |
| 17 | [#705](https://github.com/storytold/craft-anything/pull/705) | Preserve sparse ext positions across indirect levels | [`13d210d6`](https://github.com/Afloat16/craft-anything/commit/13d210d63b21c1edf93d7b02d3a2f15adde544e3) | [failure](https://github.com/storytold/craft-anything/actions/runs/41692378272) |
| 18 | [#706](https://github.com/storytold/craft-anything/pull/706) | Distinguish missing ext data from sparse holes | [`178d7897`](https://github.com/Afloat16/craft-anything/commit/178d789785ffbba3dc7bbce71a1547fb2e904544) | [success](https://github.com/storytold/craft-anything/actions/runs/41692394230) |
| 19 | [#707](https://github.com/storytold/craft-anything/pull/707) | Keep PyInstaller modules within their PYZ archive | [`8612a9c7`](https://github.com/Afloat16/craft-anything/commit/8612a9c73d08d3273589679e079d293c8c3d17d8) | [success](https://github.com/storytold/craft-anything/actions/runs/41692411966) |
| 20 | [#708](https://github.com/storytold/craft-anything/pull/708) | Preserve JFFS2 sizes when recovery budgets are exhausted | [`1e4d6f36`](https://github.com/Afloat16/craft-anything/commit/1e4d6f36353d2da27f0f13f0957d9c8205b7809c) | [success](https://github.com/storytold/craft-anything/actions/runs/41692427030) |
| 21 | [#709](https://github.com/storytold/craft-anything/pull/709) | Keep comments when adding the first JSON configuration value | [`8af65ca7`](https://github.com/Afloat16/craft-anything/commit/8af65ca72d40c5284d15cf96e1ecc29ab41ca9cc) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693676646) |
| 22 | [#710](https://github.com/storytold/craft-anything/pull/710) | Remove every duplicate JSON configuration key | [`7a32fc54`](https://github.com/Afloat16/craft-anything/commit/7a32fc5486c96358ae52a6f5494ca4f7b0244300) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693809704) |
| 23 | [#711](https://github.com/storytold/craft-anything/pull/711) | Keep UDP payloads within declared datagram lengths | [`2f653396`](https://github.com/Afloat16/craft-anything/commit/2f6533961eaeecfd8ca94337ffedd681a39b6acc) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693826718) |
| 24 | [#712](https://github.com/storytold/craft-anything/pull/712) | Reject invalid TCP header lengths | [`83487ac2`](https://github.com/Afloat16/craft-anything/commit/83487ac2fe99f0bc9dfab5230366bb6e7fd5e30d) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693841558) |
| 25 | [#713](https://github.com/storytold/craft-anything/pull/713) | Respect DNS-over-TCP frame lengths | [`67f0cfb7`](https://github.com/Afloat16/craft-anything/commit/67f0cfb73ae21e2e8a9aff0bec5280792d967680) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693932774) |
| 26 | [#714](https://github.com/storytold/craft-anything/pull/714) | Validate pcapng block framing | [`d9ffd0cd`](https://github.com/Afloat16/craft-anything/commit/d9ffd0cd0fd4467b042de22c9755dc665830610f) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693917149) |
| 27 | [#715](https://github.com/storytold/craft-anything/pull/715) | Keep TLS metadata within declared lengths | [`a3f8763f`](https://github.com/Afloat16/craft-anything/commit/a3f8763f4aa84cdeeba2dd0a05b1230b388dd241) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693905229) |
| 28 | [#716](https://github.com/storytold/craft-anything/pull/716) | Preserve distinct response metadata in replay samples | [`53a902a7`](https://github.com/Afloat16/craft-anything/commit/53a902a772d30e78951e2b8af5e301f4e1738052) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693894074) |
| 29 | [#717](https://github.com/storytold/craft-anything/pull/717) | Apply replay body limits after redaction | [`f6c86482`](https://github.com/Afloat16/craft-anything/commit/f6c86482cab1d888aacc8f932591585a7fd80a61) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693880980) |
| 30 | [#718](https://github.com/storytold/craft-anything/pull/718) | Create atomic-write temporary files exclusively | [`fb8a7df0`](https://github.com/Afloat16/craft-anything/commit/fb8a7df04af3ba1b84414a997b0b3bde00d3b5e3) | [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693868110) |

## Verification of the synchronization

Every branch was checked against the fixed main snapshot. Its first parent is the previous contribution head and its second parent is `f5d1f275`. The branch-owned production and regression blobs were preserved. The only conflict resolutions combine ROADMAP and STATUS entries; each contribution's STATUS row is at the end of the first table. Main's conversation-width and desktop-app records remain present.

Equivalent commits already present on the account were retained. No force pushes were used. The author and committer remain Afloat. These checks establish the source and ancestry of each published head; the linked CI runs establish execution status.

## Follow-up corrections

- **#692:** propagate incomplete Java bytecode scans through class references and the references tool, retaining earlier references and subsequent methods. Two class-level regressions accompany the correction. The review thread is resolved.
- **#695:** use typed two-byte chunks for UTF-16 decoding to satisfy strict Clippy, preserve odd-byte replacement, and retain repository formatting.
- **#716–#718:** the original implementation heads now have successful CI records. Their source and test blobs remain unchanged in the synchronized heads.

| PR | Earlier tested implementation head | Successful CI |
|---|---|---|
| [#692](https://github.com/storytold/craft-anything/pull/692) | [`e7cd6da5`](https://github.com/Afloat16/craft-anything/commit/e7cd6da59d2858dc9ab5e9fba1fcd2ceaaea434f) | [41688827573](https://github.com/storytold/craft-anything/actions/runs/41688827573) |
| [#695](https://github.com/storytold/craft-anything/pull/695) | [`8357dced`](https://github.com/Afloat16/craft-anything/commit/8357dced53cb153764d503c8cee312499d457919) | [41689288428](https://github.com/storytold/craft-anything/actions/runs/41689288428) |
| [#716](https://github.com/storytold/craft-anything/pull/716) | [`ec385c38`](https://github.com/Afloat16/craft-anything/commit/ec385c38b69e021617bd2b93f226d891cefb0c9a) | [41686996208](https://github.com/storytold/craft-anything/actions/runs/41686996208) |
| [#717](https://github.com/storytold/craft-anything/pull/717) | [`b9586247`](https://github.com/Afloat16/craft-anything/commit/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd) | [41687009280](https://github.com/storytold/craft-anything/actions/runs/41687009280) |
| [#718](https://github.com/storytold/craft-anything/pull/718) | [`5f0f1ce2`](https://github.com/Afloat16/craft-anything/commit/5f0f1ce228af21a6ccc6494f6dacf392357f56a1) | [41687019068](https://github.com/storytold/craft-anything/actions/runs/41687019068) |

## Current CI failure

[#705, run 41692378272, attempt 1](https://github.com/storytold/craft-anything/actions/runs/41692378272/job/126113975177) failed in the existing NeedBuilder UI smoke test. UI smoke: 76 passed, 1 failed, 0 ignored. Both ext_sparse_indirect regressions passed; the executed formats tests totalled 467 passed, 0 failed, 0 ignored. Formatting, strict workspace Clippy, layers, locks and secrets checks passed before the failure. Later gates did not run.

The entire smoke.rs is unchanged from f5d1f275. Existing PR #681 targets this fixture and has a successful full CI run; this log does not establish the precise asynchronous event order. [#681](https://github.com/storytold/craft-anything/pull/681) passed [CI 41670991884](https://github.com/storytold/craft-anything/actions/runs/41670991884), including all 75 smoke tests on its tested merge. No ext change is justified by this failure.

## Earlier CI failures

These runs belong to the previous heads, before the final main synchronization. Their failure logs remain useful context; they are not the status of the current heads in the table above.

| PR and earlier run | Observed failure | Follow-up |
|---|---|---|
| [#693, 41687565823](https://github.com/storytold/craft-anything/actions/runs/41687565823/job/126097243842) | Expected OverLimit, observed EngineMissing; server reported BrokenPipe. This fixture already reads the complete request body. The log does not establish the transport or classification root cause. | Keep the error-classification assertion; no speculative parser or fixture change. |
| [#700, 41687698727](https://github.com/storytold/craft-anything/actions/runs/41687698727/job/126097714866) | Existing single-read HTTP fixture returned None and reported BrokenPipe. The two ZIP regression tests were not reached. | The separate loopback fixture branch 35a32fdb consumes the complete bounded request header; upstream PR creation remains pending. |
| [#702, 41687774538](https://github.com/storytold/craft-anything/actions/runs/41687774538/job/126097979711) | The cleanup-path assertion failed. Source inspection shows a deleted timestamp-based name can be reused within one millisecond; the log does not print the paths, so that mechanism is not an executed reproduction. | The separate paste-cleanup fixture gives the old image a fixed historical name and retains the deletion/retention assertions; upstream PR creation remains pending. |
| [#704, 41687797277](https://github.com/storytold/craft-anything/actions/runs/41687797277/job/126098059280) | Expected Transient, observed Unknown. The unchanged test passed in other contemporaneous runs; the first error/event is absent from this log. | Preserve the Transient assertion. A startup-versus-transport root cause is not established. |
| [#711, 41687928153](https://github.com/storytold/craft-anything/actions/runs/41687928153/job/126098521460) | Existing NeedBuilder UI smoke assertion failed after all four UDP regressions and the 399 formats unit tests passed. | Existing PR #681 isolates discovery in this fixture and has a successful full CI run; it is not yet in the fixed base. |

The ready loopback fixture is on [`test/loopback-fixture-stability-20261103`](https://github.com/Afloat16/craft-anything/tree/test/loopback-fixture-stability-20261103). The paste-cleanup fixture is on [`test/r2.9-paste-cleanup-fixture-20261104`](https://github.com/Afloat16/craft-anything/tree/test/r2.9-paste-cleanup-fixture-20261104). Both are separate test follow-ups, with current execution pending.

## Validation record

The [earlier validation record](https://github.com/Afloat16/craft-anything/blob/ae2855a83af28a1050b43c5be00428f7b34c53ee/docs/contributions/20261103-thirty-improvements.md#validation-record) reported affected formats tests, strict Clippy and formatting for items 1–27, and a combined result of 563 passed with 0 ignored at tree `450d06683b305748efc07415a05d0edd7e9ed374`. That combined tree was local and was not published as a Git object; the record does not provide an Actions run or job ID.

The earlier record marked shared WASM checks passed for items 1–19 and 22, pending for 20–21 and 23–29, and not applicable to item 30. These are historical values. No new standalone shared-target execution is claimed by this synchronization.

The accompanying JSON preserves each original head and validation summary alongside current commit, tree, file hashes and CI identities. A successful earlier run must not be presented as execution of a newer synchronized head.

## Change and regression details

### 1. Reject signed LEB128 values outside the 64-bit range — [#688](https://github.com/storytold/craft-anything/pull/688)

Reject invalid sign extension in the tenth byte so malformed exception counts cannot become valid values.

**Regression coverage:** Signed-limit and overflowing tenth-byte regressions.

Scope: the current signed 64-bit LEB128 helper; broader DEX 32-bit operand rules are unchanged.

**Current head:** [`26b1ab7978c0eeb7baa258bfe839ff13fef2dbd0`](https://github.com/Afloat16/craft-anything/commit/26b1ab7978c0eeb7baa258bfe839ff13fef2dbd0). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692721275).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687513072) at `47dd4c4b7edee5fbec50cf7f47343779623677fd`.

**Files:** `crates/formats/src/dex.rs`, `crates/formats/tests/dex_signed_leb128.rs`.

### 2. Preserve valid text around damaged modified UTF-8 — [#690](https://github.com/storytold/craft-anything/pull/690)

Validate continuation and overlong encodings in the shared DEX/Java decoder while preserving valid following text, encoded NUL and surrogate pairs.

**Regression coverage:** Malformed encodings and valid surrounding text controls.

**Current head:** [`14e95f3e7365aa266edc33c527b525949b9ae493`](https://github.com/Afloat16/craft-anything/commit/14e95f3e7365aa266edc33c527b525949b9ae493). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692739762).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687531781) at `47c038446f08c8d58f8baa830f513eed17e7458c`.

**Files:** `crates/formats/src/bytes.rs`, `crates/formats/src/dex.rs`, `crates/formats/src/java_class.rs`, `crates/formats/tests/modified_utf8_validation.rs`.

### 3. Reject invalid widened Java instructions — [#691](https://github.com/storytold/craft-anything/pull/691)

Allow wide only for the defined load, store, ret and iinc instructions so malformed bytecode cannot be decoded as a valid widened operation.

**Regression coverage:** Allowed widened operations and invalid opcode regressions.

**Current head:** [`ad488410ebc9dc745aaea510f5ffd84ee47789ca`](https://github.com/Afloat16/craft-anything/commit/ad488410ebc9dc745aaea510f5ffd84ee47789ca). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692753004).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687543272) at `a0e0cb8cd3ea69a10f586cb0119cea527f3d0f5a`.

**Files:** `crates/formats/src/java_ops.rs`, `crates/formats/tests/java_wide_opcodes.rs`.

### 4. Validate Java invocation operands before decoding calls — [#692](https://github.com/storytold/craft-anything/pull/692)

Require a nonzero invokeinterface count and zero reserved operand bytes before accepting interface or dynamic calls. Propagate an incomplete instruction scan through class references and the Java references tool while retaining earlier references and subsequent methods.

**Regression coverage:** Invalid counts, reserved bytes and valid invocation controls. Two class-level regressions cover all four malformed invocation operands, earlier references and later methods.

**Current head:** [`5a073e46febddc75e169e83c90a7c1cdd1437527`](https://github.com/Afloat16/craft-anything/commit/5a073e46febddc75e169e83c90a7c1cdd1437527). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692767850).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41688827573) at `e7cd6da59d2858dc9ab5e9fba1fcd2ceaaea434f`.

**Files:** `crates/engine/src/tools/java_tools.rs`, `crates/formats/src/java_class.rs`, `crates/formats/src/java_ops.rs`, `crates/formats/tests/java_invoke_operands.rs`.

### 5. Reject impossible DEX invocation register lists — [#693](https://github.com/storytold/craft-anything/pull/693)

Reject fixed invocation lists larger than five registers and range operands extending beyond the register index space.

**Regression coverage:** Oversized fixed lists, overflowing ranges and valid boundaries.

**Current head:** [`390df5f83642d4aa9b7964b709079e2c411c5c58`](https://github.com/Afloat16/craft-anything/commit/390df5f83642d4aa9b7964b709079e2c411c5c58). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692778074).

**Previous-head CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41687565823) at `b00334a40f08ef932eb4543338d017db84da5ea4`.

**Files:** `crates/formats/src/dex_ops.rs`, `crates/formats/tests/dex_invoke_registers.rs`.

### 6. Reject invalid DEX array payload element widths — [#694](https://github.com/storytold/craft-anything/pull/694)

Accept only 1, 2, 4 or 8 byte array elements, rejecting unsupported and zero widths before payload-size calculations.

**Regression coverage:** Invalid element widths, large counts and valid width controls.

**Current head:** [`7d5f4770cab5f30dee6c24fd745c35428256ae17`](https://github.com/Afloat16/craft-anything/commit/7d5f4770cab5f30dee6c24fd745c35428256ae17). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692792746).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687573832) at `c3b33665568d369f3a0e94f0eb054d15a7ef4eed`.

**Files:** `crates/formats/src/dex_ops.rs`, `crates/formats/tests/dex_array_widths.rs`.

### 7. Read SQLite text using the database encoding — [#695](https://github.com/storytold/craft-anything/pull/695)

Use the database text encoding for schema and row text so UTF-16 databases are decoded correctly. Decode UTF-16 through typed two-byte chunks while preserving replacement of a trailing odd byte.

**Regression coverage:** UTF-8, UTF-16LE and UTF-16BE schema and row fixtures.

**Current head:** [`9c49f69a9032be69896d45419328fdac850eece6`](https://github.com/Afloat16/craft-anything/commit/9c49f69a9032be69896d45419328fdac850eece6). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692801601).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41689288428) at `8357dced53cb153764d503c8cee312499d457919`.

**Files:** `crates/formats/src/sqlite.rs`, `crates/formats/tests/sqlite_text_encoding.rs`.

### 8. Keep SQLite cell reads within their page — [#696](https://github.com/storytold/craft-anything/pull/696)

Constrain cell headers, pointer arrays and payload reads to the current usable page instead of borrowing bytes from a neighboring page.

**Regression coverage:** Malformed page pointers and truncated page-local data.

**Current head:** [`b5b47a5d3d4bd39d8b396dce5d7779199091689a`](https://github.com/Afloat16/craft-anything/commit/b5b47a5d3d4bd39d8b396dce5d7779199091689a). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692814058).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687598373) at `bccf303604b895a4dde98079977c4468c3a1064a`.

**Files:** `crates/formats/src/sqlite.rs`, `crates/formats/tests/sqlite_page_boundaries.rs`.

### 9. Stop repeated SQLite overflow pages from fabricating data — [#697](https://github.com/storytold/craft-anything/pull/697)

Track visited overflow pages and stop repeated pages while retaining only the actually recovered partial payload.

**Regression coverage:** Repeated-page and cyclic overflow-chain regressions.

**Current head:** [`97b03a6f9022a688daabc91dea259f153dad87cc`](https://github.com/Afloat16/craft-anything/commit/97b03a6f9022a688daabc91dea259f153dad87cc). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692830030).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687609537) at `06e4b5075189afdc48cbcdac4707869ea3d833dc`.

**Files:** `crates/formats/src/sqlite.rs`, `crates/formats/tests/sqlite_overflow_cycles.rs`.

### 10. Keep Android string pools within declared chunks — [#698](https://github.com/storytold/craft-anything/pull/698)

Validate chunk framing, offsets, string/style regions and terminators before decoding string pool entries.

**Regression coverage:** Out-of-chunk offsets, malformed regions and termination checks.

**Current head:** [`cddf8441b395105fbb452c9d060250dfb8cf448b`](https://github.com/Afloat16/craft-anything/commit/cddf8441b395105fbb452c9d060250dfb8cf448b). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692842389).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687681072) at `788fcd9f4e5e7dfab0f0cc4ddecc96713c017d56`.

**Files:** `crates/formats/src/axml.rs`, `crates/formats/tests/android_string_pool_bounds.rs`.

### 11. Apply ZIP output limits to stored entries — [#699](https://github.com/storytold/craft-anything/pull/699)

Check the actual stored entry length before allocation so stored members obey the same extraction output cap.

**Regression coverage:** Oversized stored members and exact-limit controls.

**Current head:** [`1e593fbe41ad275ab2f0f0709f4bd44e034ce962`](https://github.com/Afloat16/craft-anything/commit/1e593fbe41ad275ab2f0f0709f4bd44e034ce962). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692159741).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687686886) at `777a631bcd8a2d425872d6e1d4256f6f0301e6f7`.

**Files:** `crates/formats/src/zip.rs`, `crates/formats/tests/zip_stored_cap.rs`.

### 12. Check ZIP member sizes after decompression — [#700](https://github.com/storytold/craft-anything/pull/700)

Compare the decoded member size with the advertised uncompressed size before returning recovered content.

**Regression coverage:** Decoded-size mismatches and valid member controls.

**Current head:** [`e9db1bad36cd6b70e6835fdce465573824765efa`](https://github.com/Afloat16/craft-anything/commit/e9db1bad36cd6b70e6835fdce465573824765efa). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692181984).

**Previous-head CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41687698727) at `a570fdff89498a8ed87182d77c728d0c889b60ee`.

**Files:** `crates/formats/src/zip.rs`, `crates/formats/tests/zip_decoded_size.rs`.

### 13. Keep ZIP64 values within their extra field — [#701](https://github.com/storytold/craft-anything/pull/701)

Read ZIP64 metadata only inside its own extra field and reject missing metadata for sentinel size or offset values.

**Regression coverage:** Truncated ZIP64 fields, adjacent fields and missing sentinel metadata.

**Current head:** [`aa701bdf34486de20ddaf5cc951df8b4ee017586`](https://github.com/Afloat16/craft-anything/commit/aa701bdf34486de20ddaf5cc951df8b4ee017586). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692195763).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687762890) at `6f1b326e455f2e4709e23c06130e217da41f02d9`.

**Files:** `crates/formats/src/zip.rs`, `crates/formats/tests/zip64_extra_boundaries.rs`.

### 14. Validate ASAR header framing — [#702](https://github.com/storytold/craft-anything/pull/702)

Validate nested header lengths and ensure the complete declared header lies within the archive before parsing it.

**Regression coverage:** Inconsistent nested lengths and truncated archive headers.

**Current head:** [`0b9f5d7c8b3799d087f23ca03ca1c6fee1b54f88`](https://github.com/Afloat16/craft-anything/commit/0b9f5d7c8b3799d087f23ca03ca1c6fee1b54f88). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692289599).

**Previous-head CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41687774538) at `5862e25ba261301ba37c1152010fe91e52bf0997`.

**Files:** `crates/formats/src/asar.rs`, `crates/formats/tests/asar_header_bounds.rs`.

### 15. Traverse ROMFS dot and empty root directories — [#703](https://github.com/storytold/craft-anything/pull/703)

Traverse children for supported dot and empty root names so valid root layouts expose their contained files.

**Regression coverage:** Root layout, recovered contents and entry-limit fixtures.

Scope: supported root-directory layouts; existing nested-directory handling is preserved.

**Current head:** [`2bef9409336d42f1a0f764848670dfd6b69fdb87`](https://github.com/Afloat16/craft-anything/commit/2bef9409336d42f1a0f764848670dfd6b69fdb87). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692309447).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687783673) at `61f722e4cc93ea908b2fcf03c6eb599fdb292ed7`.

**Files:** `crates/formats/src/fsread/romfs.rs`, `crates/formats/tests/romfs_root_directory.rs`.

### 16. Report incomplete cramfs block output — [#704](https://github.com/storytold/craft-anything/pull/704)

Require full-sized decoded blocks and the exact final remainder before claiming complete file recovery.

**Regression coverage:** Short intermediate blocks, final block lengths and valid controls.

**Current head:** [`8328b4e549330a09b4eb787857089a5263d0966d`](https://github.com/Afloat16/craft-anything/commit/8328b4e549330a09b4eb787857089a5263d0966d). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692327401).

**Previous-head CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41687797277) at `c1eb80975e90b07ee012903b155843c58d732718`.

**Files:** `crates/formats/src/fsread/cramfs.rs`, `crates/formats/tests/cramfs_block_lengths.rs`.

### 17. Preserve sparse ext positions across indirect levels — [#705](https://github.com/storytold/craft-anything/pull/705)

Advance by the logical span of a missing indirect block so later data stays at the correct file offset.

**Regression coverage:** Missing indirect spans and later mapped data regressions.

**Current head:** [`13d210d63b21c1edf93d7b02d3a2f15adde544e3`](https://github.com/Afloat16/craft-anything/commit/13d210d63b21c1edf93d7b02d3a2f15adde544e3). **Current CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41692378272).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687810980) at `562189ef24cbc2aa44df3013b285cdd13693b7b4`.

**Files:** `crates/formats/src/fsread/extfs.rs`, `crates/formats/tests/ext_sparse_indirect.rs`.

### 18. Distinguish missing ext data from sparse holes — [#706](https://github.com/storytold/craft-anything/pull/706)

Report mapped data blocks beyond the captured image as unavailable instead of silently returning zero-filled bytes.

**Regression coverage:** Out-of-image mapped blocks and real sparse-hole controls.

Scope: missing mapped data blocks. Indirect and extent metadata recovery remains separate roadmap work.

**Current head:** [`178d789785ffbba3dc7bbce71a1547fb2e904544`](https://github.com/Afloat16/craft-anything/commit/178d789785ffbba3dc7bbce71a1547fb2e904544). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692394230).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687825216) at `3e17fe135982574bb80d6ff33f48c04a673746fb`.

**Files:** `crates/formats/src/fsread/extfs.rs`, `crates/formats/tests/ext_missing_blocks.rs`.

### 19. Keep PyInstaller modules within their PYZ archive — [#707](https://github.com/storytold/craft-anything/pull/707)

Reject listed module payloads outside the PYZ archive rather than reading bytes from neighboring outer members.

**Regression coverage:** A module starting at the PYZ end with a valid adjacent outer member.

Scope: module entries produced by the archive listing.

**Current head:** [`8612a9c73d08d3273589679e079d293c8c3d17d8`](https://github.com/Afloat16/craft-anything/commit/8612a9c73d08d3273589679e079d293c8c3d17d8). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692411966).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687886220) at `6e9b69c9b500ef4c6805404365df23ede2dfeee8`.

**Files:** `crates/formats/src/pyinstaller.rs`, `crates/formats/tests/pyz_module_bounds.rs`.

### 20. Preserve JFFS2 sizes when recovery budgets are exhausted — [#708](https://github.com/storytold/craft-anything/pull/708)

Keep the latest inode logical size when per-file or total recovery budgets prevent retaining the file contents.

**Regression coverage:** Per-file and total-budget exhaustion fixtures.

Scope: recovery-budget failures; unsupported compression handling is unchanged.

**Current head:** [`1e4d6f36353d2da27f0f13f0957d9c8205b7809c`](https://github.com/Afloat16/craft-anything/commit/1e4d6f36353d2da27f0f13f0957d9c8205b7809c). **Current CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41692427030).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687897097) at `9d4a307186b662fb46b2790ff26edcf63c262c6f`.

**Files:** `crates/formats/src/fsread/jffs2.rs`, `crates/formats/tests/jffs2_unrecovered_size.rs`.

### 21. Keep comments when adding the first JSON configuration value — [#709](https://github.com/storytold/craft-anything/pull/709)

Preserve leading comments, a byte-order mark and newline style when inserting the first configuration value.

**Regression coverage:** Comment-only inputs, BOM and newline-style controls.

**Current head:** [`8af65ca72d40c5284d15cf96e1ecc29ab41ca9cc`](https://github.com/Afloat16/craft-anything/commit/8af65ca72d40c5284d15cf96e1ecc29ab41ca9cc). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693676646).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687903093) at `6a97e4fc7654ab295c1ce35729a60a0d46f144f7`.

**Files:** `crates/formats/src/jsonc.rs`, `crates/formats/tests/regression_jsonc_comment_only.rs`.

### 22. Remove every duplicate JSON configuration key — [#710](https://github.com/storytold/craft-anything/pull/710)

Remove every occurrence of the requested key while keeping valid separators and independent comments between entries.

**Regression coverage:** Adjacent duplicates, separators and retained line/block comments.

**Current head:** [`7a32fc5486c96358ae52a6f5494ca4f7b0244300`](https://github.com/Afloat16/craft-anything/commit/7a32fc5486c96358ae52a6f5494ca4f7b0244300). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693809704).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687915897) at `bc10d3c99f543c254943b59852aa76f66c03a020`.

**Files:** `crates/formats/src/jsonc.rs`, `crates/formats/tests/regression_jsonc_duplicate_removal.rs`.

### 23. Keep UDP payloads within declared datagram lengths — [#711](https://github.com/storytold/craft-anything/pull/711)

Diagnose invalid or truncated datagrams and prevent trailing capture bytes from being interpreted as UDP payload or DNS data.

**Regression coverage:** Invalid lengths, truncation, trailing padding and valid datagrams.

Scope: ordinary UDP datagrams; IPv6 jumbograms are not added.

**Current head:** [`2f6533961eaeecfd8ca94337ffedd681a39b6acc`](https://github.com/Afloat16/craft-anything/commit/2f6533961eaeecfd8ca94337ffedd681a39b6acc). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693826718).

**Previous-head CI:** [failure](https://github.com/storytold/craft-anything/actions/runs/41687928153) at `2d2baecfed3602167b3a03098f94af19aeaa4ecb`.

**Files:** `crates/formats/src/pcap.rs`, `crates/formats/tests/regression_udp_length.rs`.

### 24. Reject invalid TCP header lengths — [#712](https://github.com/storytold/craft-anything/pull/712)

Reject data offsets below the minimum header or beyond captured bytes before exposing a TCP payload.

**Regression coverage:** Undersized headers, offsets beyond capture and valid headers.

**Current head:** [`83487ac2fe99f0bc9dfab5230366bb6e7fd5e30d`](https://github.com/Afloat16/craft-anything/commit/83487ac2fe99f0bc9dfab5230366bb6e7fd5e30d). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693841558).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687937314) at `7a4398fed392767d91240df2a2b309a712eb9a04`.

**Files:** `crates/formats/src/pcap.rs`, `crates/formats/tests/regression_tcp_header_length.rs`.

### 25. Respect DNS-over-TCP frame lengths — [#713](https://github.com/storytold/craft-anything/pull/713)

Limit decoding to the first complete length-prefixed DNS message and diagnose truncated frames.

**Regression coverage:** Incomplete frames, trailing data and valid first-message controls.

Scope: one complete captured DNS/TCP message; stream reassembly is not added.

**Current head:** [`67f0cfb73ae21e2e8a9aff0bec5280792d967680`](https://github.com/Afloat16/craft-anything/commit/67f0cfb73ae21e2e8a9aff0bec5280792d967680). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693932774).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687948685) at `f8254f89da9cf67745f27f49c098a74e6f9e12c5`.

**Files:** `crates/formats/src/pcap.rs`, `crates/formats/tests/regression_tcp_dns_framing.rs`.

### 26. Validate pcapng block framing — [#714](https://github.com/storytold/craft-anything/pull/714)

Validate repeated block lengths and each supported fixed header before reading interfaces or packets; update section byte order only after validation.

**Regression coverage:** Mismatched trailers, short supported blocks, both byte orders and valid capture controls.

Remaining roadmap work: checked 32-bit arithmetic for captured lengths and section-relative interface ids.

**Current head:** [`d9ffd0cd0fd4467b042de22c9755dc665830610f`](https://github.com/Afloat16/craft-anything/commit/d9ffd0cd0fd4467b042de22c9755dc665830610f). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693917149).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687957586) at `f079d69c1672035ace96fb4e2aab52e188bf7803`.

**Files:** `crates/formats/src/pcap.rs`, `crates/formats/tests/regression_pcapng_framing.rs`.

### 27. Keep TLS metadata within declared lengths — [#715](https://github.com/storytold/craft-anything/pull/715)

Bound nested record, handshake and extension slices before reading TLS hello metadata.

**Regression coverage:** Ten fixtures cover truncation, injected trailing bytes and valid SNI/ALPN and TLS 1.2 hellos.

**Current head:** [`a3f8763f4aa84cdeeba2dd0a05b1230b388dd241`](https://github.com/Afloat16/craft-anything/commit/a3f8763f4aa84cdeeba2dd0a05b1230b388dd241). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693905229).

**Previous-head CI:** [success](https://github.com/storytold/craft-anything/actions/runs/41687968387) at `140a75d2aa7f23fd831a419831f7d6f3c8006096`.

**Files:** `crates/formats/src/pcap.rs`, `crates/formats/tests/regression_tls_bounds.rs`.

### 28. Preserve distinct response metadata in replay samples — [#716](https://github.com/storytold/craft-anything/pull/716)

Consecutive samples compare all response fields, keeping distinct headers, content types and omitted-body metadata.

**Regression coverage:** Five regressions cover changing Location headers, MIME types, omitted-body reasons, cached 304 markers and identical-sample deduplication.

**Current head:** [`53a902a772d30e78951e2b8af5e301f4e1738052`](https://github.com/Afloat16/craft-anything/commit/53a902a772d30e78951e2b8af5e301f4e1738052). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693894074).

**Previous-head CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41689978704) at `06f77d6b71f68bab669a982a892a0cbffdc90248`.

**Files:** `crates/analysis/src/standin.rs`, `crates/analysis/tests/regression_standin_sample_metadata.rs`.

### 29. Apply replay body limits after redaction — [#717](https://github.com/storytold/craft-anything/pull/717)

Check the redacted text against each body and capture budget before retaining it or charging its bytes.

**Regression coverage:** Four regressions cover expanding redactions, per-body and total limits, exact retained size and rejected-body budget accounting.

Scope: retained text bytes. Existing raw-input prechecks remain in place; temporary redaction allocations are not bounded by this change.

**Current head:** [`f6c86482cab1d888aacc8f932591585a7fd80a61`](https://github.com/Afloat16/craft-anything/commit/f6c86482cab1d888aacc8f932591585a7fd80a61). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693880980).

**Previous-head CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41690055300) at `bc6dde7873027e4e744af2afcfc4ad6b0ea44dd1`.

**Files:** `crates/analysis/src/standin.rs`, `crates/analysis/tests/regression_standin_redacted_budget.rs`.

### 30. Create atomic-write temporary files exclusively — [#718](https://github.com/storytold/craft-anything/pull/718)

Reserve temporary files exclusively with bounded collision retries, and only clean up a file reserved by the current write.

**Regression coverage:** One filesystem regression covers existing files, Unix symlinks, private and inherited permissions, exhausted retries and cleanup after rename failure.

**Current head:** [`fb8a7df04af3ba1b84414a997b0b3bde00d3b5e3`](https://github.com/Afloat16/craft-anything/commit/fb8a7df04af3ba1b84414a997b0b3bde00d3b5e3). **Current CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41693868110).

**Previous-head CI:** [in_progress](https://github.com/storytold/craft-anything/actions/runs/41690070641) at `298797e039f29c87526e69ff0be4455dbf91acae`.

**Files:** `crates/platform/src/fs.rs`, `crates/platform/tests/regression_atomic_temp_creation.rs`.
