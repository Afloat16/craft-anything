# Refuse incomplete tar engine installs after GNU metadata stops

Upstream PR: https://github.com/storytold/craft-anything/pull/1394

## Problem

A tar listing can retain earlier members after a GNU long-name or long-link record is refused. The managed engine installer previously treated that partial inventory as a successful extraction whenever an earlier regular file existed. That can replace an engine installation with incomplete contents.

This is a follow-up to #1362, prepared from upstream `efddbdeb6868f10a3d8d53f114d47aed72b89612` after that PR merged.

## Change

- Record an explicit typed stop reason for oversized or truncated GNU name metadata, while retaining earlier members for inspection.
- Check that reason in the installer before writing any member. Oversized metadata returns `OverLimit`; truncated metadata returns `GarbledOutput`.
- Keep the existing compatibility for readable tar members without an end-of-archive marker. CPIO behavior and successful bounded GNU names remain unchanged.

## Regression evidence

The test-only revision `24d844ce89c6ea660e41f30be676411b25b06490` keeps production code at upstream `efddbdeb6868f10a3d8d53f114d47aed72b89612`. Its [actual baseline run](https://github.com/Afloat16/claude-mem/actions/runs/42648642576/job/129268658248) reports **4 failed and 2 passed**: oversized/truncated long names and long links each write an earlier file on the old code; missing end markers and valid bounded metadata are passing controls.

The fixed tests also assert that an existing installed file is unchanged when extraction is refused. Formats tests check that partial listing bytes remain readable and that absent end markers do not receive the new stop reason.

## Verification

Exact source revision: `c88767b601aef15b8e2604b607f069734fef5a09`.
The [source-specific verification steps](https://github.com/Afloat16/claude-mem/actions/runs/42653593224/job/129286113135) passed formatting, the complete formats and engines test suites, all-targets clippy with warnings denied, layers, locks, secret scanning, WebAssembly checks and Cargo.lock stability. The workflow checks out this exact craft-anything revision in Afloat16/claude-mem.

Branch head `c88767b601aef15b8e2604b607f069734fef5a09` merges upstream `cdc8a880f1e60bce757ebac59e3f7983f084a1f8` with both parents retained. The exact integrated head passed all verification steps above.

Roadmap: R3.12.
