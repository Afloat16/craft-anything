# Refuse incomplete tar engine installs after GNU metadata stops

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

Exact source revision: `3e588c5589ee56ab50004dd0e87cc47f4f2455da`.
The [exact-source validation](https://github.com/Afloat16/claude-mem/actions/runs/42649935938/job/129273248608) passed formatting, all **1,141 tests** (**124 engines + 1,017 formats**), all-targets clippy with warnings denied, layers, locks, secret scanning, WebAssembly checks and Cargo.lock stability. One existing network-download test remains ignored. The workflow checks out this craft-anything revision in Afloat16/claude-mem.

Branch head `1007aae934ea3d17fbe341daa7851a1c79eda7a6` adds only the completed roadmap entry, status evidence and the parser/installer lesson in `docs/pitfalls.md`; source and test files are unchanged from the tested revision.

Roadmap: R3.12.


[Create the upstream follow-up PR](https://github.com/storytold/craft-anything/compare/main...Afloat16:craft-anything:bugfix/r3.12-tar-install-stop-reasons-20261109?expand=1&title=Refuse%20incomplete%20tar%20engine%20installs%20after%20GNU%20metadata%20stops&body=%23%23%20Problem%0A%0AA%20tar%20listing%20can%20retain%20earlier%20members%20after%20a%20GNU%20long-name%20or%20long-link%20record%20is%20refused.%20The%20managed%20engine%20installer%20previously%20treated%20that%20partial%20inventory%20as%20a%20successful%20extraction%20whenever%20an%20earlier%20regular%20file%20existed.%20That%20can%20replace%20an%20engine%20installation%20with%20incomplete%20contents.%0A%0AThis%20is%20a%20follow-up%20to%20%231362%2C%20prepared%20from%20upstream%20%60efddbdeb6868f10a3d8d53f114d47aed72b89612%60%20after%20that%20PR%20merged.%0A%0A%23%23%20Change%0A%0A-%20Record%20an%20explicit%20typed%20stop%20reason%20for%20oversized%20or%20truncated%20GNU%20name%20metadata%2C%20while%20retaining%20earlier%20members%20for%20inspection.%0A-%20Check%20that%20reason%20in%20the%20installer%20before%20writing%20any%20member.%20Oversized%20metadata%20returns%20%60OverLimit%60%3B%20truncated%20metadata%20returns%20%60GarbledOutput%60.%0A-%20Keep%20the%20existing%20compatibility%20for%20readable%20tar%20members%20without%20an%20end-of-archive%20marker.%20CPIO%20behavior%20and%20successful%20bounded%20GNU%20names%20remain%20unchanged.%0A%0A%23%23%20Regression%20evidence%0A%0AThe%20test-only%20revision%20%6024d844ce89c6ea660e41f30be676411b25b06490%60%20keeps%20production%20code%20at%20upstream%20%60efddbdeb6868f10a3d8d53f114d47aed72b89612%60.%20Its%20%5Bactual%20baseline%20run%5D(https%3A%2F%2Fgithub.com%2FAfloat16%2Fclaude-mem%2Factions%2Fruns%2F42648642576%2Fjob%2F129268658248)%20reports%20**4%20failed%20and%202%20passed**%3A%20oversized%2Ftruncated%20long%20names%20and%20long%20links%20each%20write%20an%20earlier%20file%20on%20the%20old%20code%3B%20missing%20end%20markers%20and%20valid%20bounded%20metadata%20are%20passing%20controls.%0A%0AThe%20fixed%20tests%20also%20assert%20that%20an%20existing%20installed%20file%20is%20unchanged%20when%20extraction%20is%20refused.%20Formats%20tests%20check%20that%20partial%20listing%20bytes%20remain%20readable%20and%20that%20absent%20end%20markers%20do%20not%20receive%20the%20new%20stop%20reason.%0A%0A%23%23%20Verification%0A%0AExact%20source%20revision%3A%20%603e588c5589ee56ab50004dd0e87cc47f4f2455da%60.%0AThe%20%5Bexact-source%20validation%5D(https%3A%2F%2Fgithub.com%2FAfloat16%2Fclaude-mem%2Factions%2Fruns%2F42649935938%2Fjob%2F129273248608)%20passed%20formatting%2C%20all%20**1%2C141%20tests**%20(**124%20engines%20%2B%201%2C017%20formats**)%2C%20all-targets%20clippy%20with%20warnings%20denied%2C%20layers%2C%20locks%2C%20secret%20scanning%2C%20WebAssembly%20checks%20and%20Cargo.lock%20stability.%20One%20existing%20network-download%20test%20remains%20ignored.%20The%20workflow%20checks%20out%20this%20craft-anything%20revision%20in%20Afloat16%2Fclaude-mem.%0A%0ABranch%20head%20%601007aae934ea3d17fbe341daa7851a1c79eda7a6%60%20adds%20only%20the%20completed%20roadmap%20entry%2C%20status%20evidence%20and%20the%20parser%2Finstaller%20lesson%20in%20%60docs%2Fpitfalls.md%60%3B%20source%20and%20test%20files%20are%20unchanged%20from%20the%20tested%20revision.%0A%0ARoadmap%3A%20R3.12.%0A)
