# Afloat16 开源贡献与合并推进报告

**统计窗口：2026-10-28 00:00:00 UTC 起有更新的 PR。** 完整清单于 **2026-11-04 03:28:25 UTC** 复核，后续变更查询已核对至 **04:00:06 UTC**，账号总数于 **04:01:42 UTC** 再次确认。本报告纳入已确认的提交、CI 与合并变化；各组证据时间见末尾“证据来源与状态口径”。

已确认的窗口内直接合并数已从初始审计的 **47 项增至 51 项**：本轮检查期间，craft-anything #773、#766 以及 claude-mem #5341、#5340 已由各仓库维护者合并。claude-mem #5339 的五类拒绝用例另由 #5336 部分收录，并保留共同作者署名。本轮还更新了 **6 个既有上游 PR**，并发布 **2 个独立修复分支**：mlxtend 的 CI 配置修复已在 fork 的两个工作流验证成功；craft-anything 的 R2.2 模型发现夹具修复已发布，执行验证待完成。此前 craft-anything 的 **27 项改进已通过维护者整合 PR #722 进入主分支**，保留贡献署名，单独计入“整合收录”，不改写原 PR 的 GitHub 合并状态。[合并记录](https://github.com/storytold/craft-anything/pull/773) · [完整清单与证据](20261104-account-merge-audit.json) · [27 项收录证据](20261104-merge-readiness.json)

## 1. 计数与贡献口径

| 统计节点 | 涉及 PR 总数 | Open | Closed | 窗口内直接 merged |
|---|---:|---:|---:|---:|
| 初始审计 | 251 | 123 | 128 | 47 |
| 03:02:07 UTC 清单，已加入 mlxtend fork 验证 #1 | 252 | 124 | 128 | 47 |
| 纳入随后确认的 craft-anything #773 合并 | 252 | 123 | 129 | 48 |
| 03:28:25 UTC 完整清单 | 252 | 120 | 132 | 50 |
| 加入 03:30:58 UTC 的 #766 合并 | 252 | 119 | 133 | 51 |
| 04:01:42 UTC 总数复核，加入独立夹具验证草稿 #3 | 253 | 120 | 133 | 51 |

完整清单已重新分页读取，并纳入逐项核实的后续合并及 fork 验证草稿。最终 **120 个开放 PR = 117 个上游 PR + 3 个 fork 验证 PR**。已关闭项还含 1 个早期 fork 验证 PR；排除全部 4 个 fork 验证后，上游清单为 249 项（117 open、132 closed）。fork 验证单列为验证或待验证记录。

Closed 的组成也应分别看待：

| 类别 | 数量 | 计数依据 |
|---|---:|---|
| 窗口内直接 merged | 51 | 有 `mergedAt`，且合并时间在统计窗口内；本轮新增 #773、#766、#5341、#5340 |
| 窗口前已 merged、在窗口内又有更新 | 1 | LeRobot #4835 于 2026-10-22 已合并，不作为本窗口新增合并 |
| Closed 且未直接 merged | 81 | `mergedAt=null`；27 项通过 #722 整合收录，另 1 项 #5339 的部分测试经 #5336 收录 |
| 合计 Closed | 133 | 51 + 1 + 81 |

这份报告统计的是上述更新窗口中的 PR，不能据此推算账号全部历史贡献。窗口内原有合并与本轮检查期间新增的合并分别列示。

## 2. 本轮实际发布的改进

以下为 **6 个既有 PR 的更新 + 2 个独立修复分支**。每项列出已经发生的提交和可验证的结果；测试仍在执行、失败待定位或需要维护者批准的项保留其实际状态。

| 项目 | 已发布提交 | 改动与作用 | 已有验证与剩余工作 |
|---|---|---|---|
| [craft-anything #766](https://github.com/storytold/craft-anything/pull/766) | 源码 [`29c48e03`](https://github.com/Afloat16/craft-anything/commit/29c48e039a95edb4f003744c36299bf19f1c9c74)；文档 [`220f3b97`](https://github.com/Afloat16/craft-anything/commit/220f3b97bcb001c8d82d24875a64ee6fe774feed)；最新同步 [`c044adf1`](https://github.com/Afloat16/craft-anything/commit/c044adf12004993ca9ceafc63a204aa778abe93b) | 同一 AXML 字符串池共享 65,536 次额外候选探测预算；重复和不同 offset 都受约束；新增 5 项确定性回归。随后同步包含最新上游改动的 main `41d2175`，处理文档冲突 | 源码提交的 [完整 CI 成功](https://github.com/storytold/craft-anything/actions/runs/41762780268)，5 项新增预算回归均成功。最新同步相对 main 恰有 11 个文件，9 个源码/测试 blob 保持原样；对应审查线程已自动 resolved。**c044adf1 的[完整 CI 成功](https://github.com/storytold/craft-anything/actions/runs/41765846157)，PR 于 03:30:58 UTC merged**；主线提交 [eab6a0f0](https://github.com/storytold/craft-anything/commit/eab6a0f01bf9fc3a36798d322e4442e169394dfc) 作者为 Afloat16 |
| [craft-anything #767](https://github.com/storytold/craft-anything/pull/767) | 回归 [`d2dd401e`](https://github.com/Afloat16/craft-anything/commit/d2dd401eef72c33f9990bfede2d5f13a2da66aa0)；同步 [`cd468aca`](https://github.com/Afloat16/craft-anything/commit/cd468aca911dbbefb4655d726008ef3676fc5cbb)；完成记录 [`1d5bd792`](https://github.com/Afloat16/craft-anything/commit/1d5bd7927bbbb67f2c95b5ecb232decedda8fae0) | 增加确定性的分片请求头夹具回归；同步已包含 #773 修复的 main `41d2175`，保留自身 5 个变更文件，没有再增加 smoke 测试改动 | d2dd401e 的新增及相关回归在日志中成功；[旧完整 CI 的唯一失败](https://github.com/storytold/craft-anything/actions/runs/41763091740)为 #773 修复的共享 UI 用例，失败 runner checkout `44533f2` 尚不含该修复。对应审查线程已自动 resolved。**cd468aca 的[完整 CI 成功](https://github.com/storytold/craft-anything/actions/runs/41765850311)**，原 UI 失败用例也通过；随后仅更新 ROADMAP 的完成状态和该验证证据，最新 head `1d5bd792` 的 [CI](https://github.com/storytold/craft-anything/actions/runs/41767797058) 在另一个未改动的模型发现用例启动临时程序时失败（`Text file busy`）；单作业重跑返回 403，独立 R2.2 夹具修复已发布待验证 |
| [geomstats #2147](https://github.com/geomstats/geomstats/pull/2147) | [`fd6e11e2`](https://github.com/Afloat16/geomstats/commit/fd6e11e2320a6dc8e1d696f60da7904bd3c098b5) | Brownian 回归结束后恢复后端 RNG 状态，消除对随后用例的随机状态影响 | 当前 head **14 个检查全部成功**：6 个测试作业、lint、docs 与 6 个 Codecov 检查；[6 个测试作业](https://github.com/geomstats/geomstats/actions/runs/41762910310)中新增回归累计 **27 次执行成功**。autograd 覆盖率 87.36%（+0.07%），未放宽数值容差 |
| [Genesis #3649](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649) | [`fcd621b0`](https://github.com/Afloat16/genesis-world/commit/fcd621b055b05dc6d57ccf88cc9647bf90aa4bb6) | 合入固定上游 main `8650cfb96e70604cf2ae261c2f151c321e465467`，保留已审查补丁和测试 | `behind` 已消除；`mergeable=true / blocked`，4 个工作流仍为 `action_required`；待维护者批准执行并处理审查规则 |
| [Genesis #3650](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650) | [`f2d54cb8`](https://github.com/Afloat16/genesis-world/commit/f2d54cb8b295149c0c8f02a63f967857e2690bdf) | 同步同一固定 main，保留原测试组织方式 | `behind` 已消除；`mergeable=true / blocked`，4 个工作流待批准 |
| [Genesis #3651](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651) | [`0ce2464b`](https://github.com/Afloat16/genesis-world/commit/0ce2464b1a62a8785e361b2b185aa90f75628bc4) | 同步同一固定 main，保留维护者要求的实现与测试结构 | `behind` 已消除；`mergeable=true / blocked`，4 个工作流待批准，审查讨论仍需维护者确认 |
| mlxtend 独立 CI 修复 | [`410abc2b`](https://github.com/Afloat16/mlxtend/commit/410abc2b778310c64cab57dd284171a73747181d) | 将 lint 工作流的 Python 和 `uv sync` 明确固定到 3.11，与已经成功的单元测试环境一致；分支 `chore/pin-lint-python-20261104` | [fork 验证 #1](https://github.com/Afloat16/mlxtend/pull/1) 的 [lint](https://github.com/Afloat16/mlxtend/actions/runs/41763206302) 与 [unit tests（两个 jobs）](https://github.com/Afloat16/mlxtend/actions/runs/41763206340) 均成功。创建上游 PR 返回 **403 Resource not accessible by integration**；分支已发布，待使用预填入口创建上游 PR |
| craft-anything 独立 R2.2 夹具修复 | [`dac9261a`](https://github.com/Afloat16/craft-anything/commit/dac9261afa8323176a98c3a14c9d5512e505cc13)；先发布 claim `5a306104` | 使用所选路径的 shell symlink 解释普通 `api` 脚本，保留参数、目录、输出、退出与超时检查，并增加所选普通文件不可执行的负例；仅一个测试函数和路线图发生变化 | 创建上游 PR 返回 **403**；[fork 草稿 #3](https://github.com/Afloat16/craft-anything/pull/3) 在 04:00:06 未见工作流运行。**源码核对已完成，执行验证待完成**；预填入口见第 7 节 |

### 本轮新增的直接合并

| PR | 合并时间（UTC） | 实际接纳与署名 |
|---|---|---|
| [craft-anything #766](https://github.com/storytold/craft-anything/pull/766) | 2026-11-04 03:30:58 | Java/DEX/AXML/ext 解析器审查修订，包含共享 AXML 探测预算；主线提交 [eab6a0f0](https://github.com/storytold/craft-anything/commit/eab6a0f01bf9fc3a36798d322e4442e169394dfc) 的作者为 Afloat16 |
| [craft-anything #773](https://github.com/storytold/craft-anything/pull/773) | 2026-11-04 03:05:53 | 固定 builder 可用性测试的发现状态；由 bflatastic 合并；主线提交 [686f3429](https://github.com/storytold/craft-anything/commit/686f3429e1b50a6969a85c6e2a2828a1f38540cd) 的作者为 Afloat16 |
| [claude-mem #5341](https://github.com/thedotmack/claude-mem/pull/5341) | 2026-11-04 03:25:45 | 拒绝 URL 末尾空查询/片段分隔符，保留代理路径中合法编码；主线提交 [6ed60fbc](https://github.com/thedotmack/claude-mem/commit/6ed60fbcb220aa43f571f190396ec535d5e6ec94) 的作者为 Afloat16 |
| [claude-mem #5340](https://github.com/thedotmack/claude-mem/pull/5340) | 2026-11-04 03:26:00 | 在错误响应读取结束、达到上限、读取失败及取消失败路径释放 reader lock；主线提交 [4f6ea065](https://github.com/thedotmack/claude-mem/commit/4f6ea0653275e67d1b304632ccb85c2521b83b49) 的作者为 Afloat16 |

维护者分别确认：#5341 的五条拒绝用例在旧实现失败、修复后成功；#5340 是该 helper 的首次直接测试覆盖，四条锁释放用例在旧实现失败。两个 PR 的发布产物由维护者在合并前移除，按项目约定留待 release 时生成。[#5341 确认](https://github.com/thedotmack/claude-mem/pull/5341#issuecomment-6621715033) · [#5340 确认](https://github.com/thedotmack/claude-mem/pull/5340#issuecomment-6621715326)

### 新增的部分测试收录

[claude-mem #5339](https://github.com/thedotmack/claude-mem/pull/5339) 于 03:19:06 关闭，原 PR 未直接 merged。维护者明确将其中 `true`、`null`、`"1.5"`、`"1e2"` 和空白字符串五类拒绝用例收录到 [#5336](https://github.com/thedotmack/claude-mem/pull/5336) 的路由测试中。主线提交 [de134d3b](https://github.com/thedotmack/claude-mem/commit/de134d3be64d2b835b4f8fe933b53fa5fb983198) 含 `Co-authored-by: Afloat <132809511+Afloat16@users.noreply.github.com>`。新 middleware、生成 bundles 等其余改动未采用；本项单列为部分接纳。[维护者说明](https://github.com/thedotmack/claude-mem/pull/5339#issuecomment-6621608877)

[仍开放的 #681](https://github.com/storytold/craft-anything/pull/681) 与这项已合并的修复重复，后续应整理 **#681**。[#768](https://github.com/storytold/craft-anything/pull/768) 已被合并的 [#761](https://github.com/storytold/craft-anything/pull/761) 覆盖；尝试通过连接器关闭 #768 返回 403，可由你在 GitHub 网页处理。

## 3. 优先推进合并的项目与真实阻塞

### 已有明确成功验证的优先项

| PR | 当前 head | 已有证据 | 仍需完成 |
|---|---|---|---|
| [robotics-toolbox-python #908](https://github.com/petercorke/robotics-toolbox-python/pull/908) | `0265bbd7` | petercorke 的 APPROVED 精确对应当前 head；[CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591101571)；clean | 上游维护者执行合并 |
| [robotics-toolbox-python #909](https://github.com/petercorke/robotics-toolbox-python/pull/909) | `87a68c08` | 当前 head 获 petercorke APPROVED；[CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591117133)；clean | 上游维护者执行合并 |
| [robotics-toolbox-python #910](https://github.com/petercorke/robotics-toolbox-python/pull/910) | `8b0c46a1` | 当前 head 获 petercorke APPROVED；[CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591137417)；clean | 上游维护者执行合并；独立的零混合生成器问题已另列 #924 |
| [tilelang #3718](https://github.com/tile-ai/tilelang/pull/3718) | `0c6eca3a` | [完整 CI 成功](https://github.com/tile-ai/tilelang/actions/runs/41040008322)，含 pre-commit；clean | 尚无正式人工 approval 记录，待维护者审查和合并 |
| [craft-anything #716](https://github.com/storytold/craft-anything/pull/716) | `53a902a7` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693894074)；clean | 维护者审查、合并 |
| [craft-anything #717](https://github.com/storytold/craft-anything/pull/717) | `f6c86482` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693880980)；clean | 维护者审查、合并；不把自动评审的额度提示视为代码失败 |
| [craft-anything #718](https://github.com/storytold/craft-anything/pull/718) | `fb8a7df0` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693868110)；clean | 维护者审查、合并 |

以上 40 个上游仓库的权限证据均为 `pull=true`，`push/maintain/admin/triage=false`。当前连接可以继续维护自己的分支，但无法代替上游维护者合并、批准受限工作流或关闭无权限编辑的 PR。权限证据见各组 JSON 记录；[craft-anything 仓库权限](https://api.github.com/repos/storytold/craft-anything)也在本报告整理时重新读取确认。

### 需要具体处理的阻塞

- **craft-anything #767 最新 head 仍有真实 CI 阻塞。** `1d5bd792` 只更新了路线图，但运行 41767797058 在 `cli_agents::models::tests::discovery_reads_the_selected_program_in_the_project_and_reports_errors` 启动临时程序时收到 `Text file busy (os error 26)`。该 crate 为 388 passed、1 failed、1 ignored，后续 workspace/wasm 门禁没有完成；不能用先前 cd468aca 的成功覆盖最新失败。相同用例在上一轮成功，models.rs 与 process.rs 的 blob 也完全相同；日志仍未识别持有可写句柄的进程。单作业重跑尝试返回 403。[失败作业](https://github.com/storytold/craft-anything/actions/runs/41767797058/job/126370174961)
- **已准备独立的 R2.2 夹具修复。** 分支 `test/r2.2-model-discovery-fixture-20261104` 先发布路线图 claim，再提交一个测试函数的修改。所选路径是已有 shell 的 symlink，写入的 `api` 只作为脚本数据读取；所有原有断言保留，并增加现存不可执行文件的负例。该分支尚未执行自动验证，需创建上游 PR 后运行目标测试和完整 CI。[修复提交](https://github.com/Afloat16/craft-anything/commit/dac9261afa8323176a98c3a14c9d5512e505cc13)
- **LeRobot #5159：审查状态尚未更新。** 请求移除的 `test_small_quaternion_sign_does_not_change_rotation` 已从当前代码删除，Fast Tests 等流程成功；nepyope 的正式 `CHANGES_REQUESTED` 仍存在，需要重新审查。当前 head 未见 Full Tests/GPU Tests 结果，不推断其是否属于仓库必需门槛。[审查记录](https://github.com/huggingface/lerobot/pull/5159#pullrequestreview-5750684606)
- **待批准的工作流要先实际执行。** openpi、Open3D、MuJoCo、COLMAP、Genesis、Anthropic SDK、部分工具项目和仿真项目返回 `action_required`。部分只有静态分析、CLA 或辅助检查成功，不能把这些结果代替测试流程。精确范围见开放清单与原始 run 记录。
- **Genesis 已解决落后 main 的问题。** 当前剩余工作是工作流批准、规则要求的检查与审查线程。已读取的有效规则要求严格更新到基线、23 个检查、至少 1 个 approval、代码所有者审查和所有讨论解决；早期“落后 22 个提交”的结论已被本轮同步覆盖。[规则证据](https://api.github.com/repos/Genesis-Embodied-AI/genesis-world/rulesets/5489548)
- **tilelang #3716/#3717 保留失败证据。** #3716 有两个 checkout 作业因内部 Git CDN 连接被拒绝而退出，另有 FP16 reducer 数值断言失败；新增的 7 个 cache-key 用例已通过，仍不能据此断言完整 CI 失败与 PR 无关。#3717 虽有当前 head 的人工 APPROVED，但失败 CUDA 作业日志返回 BlobNotFound，具体原因未确认。[#3716 CI](https://github.com/tile-ai/tilelang/actions/runs/41039741738) · [#3717 CI](https://github.com/tile-ai/tilelang/actions/runs/41044447045)
- **trimesh #2663 有明确的环境失败阶段。** i386 作业在 apt 安装时无法解析 `deb.debian.org`，尚未运行 i386 测试；s390x 已完成 171 项测试。适合由维护者重试该失败作业，再依据实际结果决定是否需要代码修复。[失败运行](https://github.com/mikedh/trimesh/actions/runs/41646972201)
- **geomstats #2147 的完整后端检查已成功。** `fd6e11e2` 恢复 RNG 状态后，14 个检查全部成功，6 个测试作业中的新增回归累计 27 次执行通过。#2148/#2149 仍分别有 autograd/numpy 测试失败；尚未建立这些失败也发生于基线的证据，因此未将其称为随机失败，也没有放宽容差。[#2147 测试运行](https://github.com/geomstats/geomstats/actions/runs/41762910310)
- **mlxtend 的通用 CI 环境修复已通过 fork 验证。** 上游 #1267/#1268/#1269 的单元测试成功而 lint 失败；独立修复有助于处理共同的工作流问题，但上游 PR 尚未成功创建，更未合并，原三个 PR 的失败状态仍保留。
- **需要维护者设计判断的讨论继续保留。** haystack #13498 还有真实 retriever 重复 document ID 的应用场景问题及 Vercel 部署授权；roma #34 还有实际用途与每次调用计算成本问题；labml #325 的 approval 来自旧 head，过期但未解决的线程需要确认。没有补造应用场景或把旧 approval 当作新 head 的 approval。

## 4. 原 27 项 craft-anything 改进：已通过 #722 整合收录

维护者通过 [#722](https://github.com/storytold/craft-anything/pull/722) 将原批次中的 27 项改进合并，合并时间 **2026-11-03 18:43:07 UTC**。主分支提交为 [`b9ca0999`](https://github.com/storytold/craft-anything/commit/b9ca099903aeef5d4b33a7e83e8e7062ab94c65a)，保留与 Afloat16 对应的共同作者署名。整合前的提交作者记录同样保留 Afloat16；最终上游采用 squash 合并，因此贡献的落点以最终合并提交及 #722 为准。

整合 head `cf860e3566cfc15aeef327687daf05a5e2c728d5` 的 [CI 41693727323](https://github.com/storytold/craft-anything/actions/runs/41693727323) 成功，27 个新增回归目标合计 **103 passed、0 failed、0 ignored**。原 27 个新增回归文件已经过完整性核对；SQLite、ZIP、ext、JSONC 和网络解析的叠加结果也有对应核对记录。[历史完整性与 CI 证据](20261104-merge-readiness.json)

这些原 PR 的 GitHub 状态为 **closed、未直接 merged**；其改进已被 #722 接纳。这 27 项属于“81 项 closed 未直接 merged”中的已确认整合收录部分，单独列示，不在直接 merged 数中重复增加。后续 Java/DEX/AXML/ext 审查修订继续由 #766 跟进，其最新分支状态见前文。

| 原 PR | 已收录改进 | 上游接纳路径 |
|---|---|---|
| [#688](https://github.com/storytold/craft-anything/pull/688) | 拒绝超出 64 位范围的 DEX signed LEB128 | #722 → `b9ca0999` |
| [#690](https://github.com/storytold/craft-anything/pull/690) | 保留损坏 Modified UTF-8 周围的有效文字 | #722 → `b9ca0999` |
| [#691](https://github.com/storytold/craft-anything/pull/691) | 拒绝不合法的 Java wide 指令 | #722 → `b9ca0999` |
| [#692](https://github.com/storytold/craft-anything/pull/692) | 校验 Java 调用指令的保留字节及参数计数 | #722 → `b9ca0999` |
| [#693](https://github.com/storytold/craft-anything/pull/693) | 拒绝越界的 DEX 调用寄存器列表 | #722 → `b9ca0999` |
| [#694](https://github.com/storytold/craft-anything/pull/694) | 限制 DEX 数组载荷元素宽度 | #722 → `b9ca0999` |
| [#695](https://github.com/storytold/craft-anything/pull/695) | 按 SQLite 声明的编码读取文本 | #722 → `b9ca0999` |
| [#696](https://github.com/storytold/craft-anything/pull/696) | 限制 SQLite 单元格读取范围 | #722 → `b9ca0999` |
| [#697](https://github.com/storytold/craft-anything/pull/697) | 阻止 SQLite 溢出页循环伪造数据 | #722 → `b9ca0999` |
| [#698](https://github.com/storytold/craft-anything/pull/698) | 限制 Android 字符串池读取边界 | #722 → `b9ca0999` |
| [#699](https://github.com/storytold/craft-anything/pull/699) | 对 ZIP 未压缩条目执行输出上限 | #722 → `b9ca0999` |
| [#700](https://github.com/storytold/craft-anything/pull/700) | 校验 ZIP 实际解压长度 | #722 → `b9ca0999` |
| [#701](https://github.com/storytold/craft-anything/pull/701) | 限制 ZIP64 扩展字段读取范围 | #722 → `b9ca0999` |
| [#702](https://github.com/storytold/craft-anything/pull/702) | 校验 ASAR 嵌套头部长度 | #722 → `b9ca0999` |
| [#703](https://github.com/storytold/craft-anything/pull/703) | 正确遍历 ROMFS 点名称及空名称根目录 | #722 → `b9ca0999` |
| [#704](https://github.com/storytold/craft-anything/pull/704) | 识别 cramfs 解压块不完整 | #722 → `b9ca0999` |
| [#705](https://github.com/storytold/craft-anything/pull/705) | 保留 ext 间接块中的稀疏文件位置 | #722 → `b9ca0999` |
| [#706](https://github.com/storytold/craft-anything/pull/706) | 区分 ext 缺失数据与合法稀疏空洞 | #722 → `b9ca0999` |
| [#707](https://github.com/storytold/craft-anything/pull/707) | 限制 PyInstaller PYZ 模块读取范围 | #722 → `b9ca0999` |
| [#708](https://github.com/storytold/craft-anything/pull/708) | 保留无法恢复内容的 JFFS2 文件大小 | #722 → `b9ca0999` |
| [#709](https://github.com/storytold/craft-anything/pull/709) | 在仅含注释的 JSON 配置中添加值时保留注释 | #722 → `b9ca0999` |
| [#710](https://github.com/storytold/craft-anything/pull/710) | 移除 JSON 配置中的全部重复键 | #722 → `b9ca0999` |
| [#711](https://github.com/storytold/craft-anything/pull/711) | 按 UDP 声明长度截取载荷 | #722 → `b9ca0999` |
| [#712](https://github.com/storytold/craft-anything/pull/712) | 拒绝无效 TCP 首部长度 | #722 → `b9ca0999` |
| [#713](https://github.com/storytold/craft-anything/pull/713) | 按 DNS over TCP 的帧长度解析消息 | #722 → `b9ca0999` |
| [#714](https://github.com/storytold/craft-anything/pull/714) | 校验 pcapng 块尾长度及最小头部 | #722 → `b9ca0999` |
| [#715](https://github.com/storytold/craft-anything/pull/715) | 限制 TLS 记录、握手及扩展的读取范围 | #722 → `b9ca0999` |

原 30 项中的另外 3 项为 #716、#717、#718，当前仍开放，已经取得各自 head 的成功 CI，列在开放清单及优先推进表中。

## 5. 全部开放上游 PR 清单

下列 **117 项**覆盖最终清单中全部开放上游 PR；另有 3 项 fork 验证单列。新增直接合并与部分测试收录已移至相应记录。SHA 显示前 8 位；完整 SHA、run 与详细证据见 JSON 及本文引用的后续 commit/run 链接。PR 原标题保留便于与 GitHub 逐项核对；合并状态和 CI 是独立列。

### 5.1 craft-anything：7 项开放上游 PR

来源 S5 与后续更新 U；#773 和 #766 已合并，移至直接合并记录。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [storytold/craft-anything #618](https://github.com/storytold/craft-anything/pull/618) — Keep unverified SSH agents out of automatic suggestions (R2.2) | `be0094ad` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41615837578) | `dirty` | 有合并冲突；维护者要求先由 #62 决定 SSH agent 的产品方向，作者已同意在决定前保持该分支 |
| [storytold/craft-anything #681](https://github.com/storytold/craft-anything/pull/681) — Isolate builder discovery in the UI smoke test (R5.2) | `5a115fe6` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41670991884) | `clean` | 仍 open；已合并 #773 覆盖同一修复，可在 GitHub 网页关闭此重复提案 |
| [storytold/craft-anything #716](https://github.com/storytold/craft-anything/pull/716) — Preserve distinct response metadata in replay samples | `53a902a7` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693894074) | `clean` | 回放响应元数据回归通过；等待维护者审查/合并 |
| [storytold/craft-anything #717](https://github.com/storytold/craft-anything/pull/717) — Apply replay body limits after redaction | `f6c86482` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693880980) | `clean` | 脱敏后预算回归通过；等待维护者审查/合并 |
| [storytold/craft-anything #718](https://github.com/storytold/craft-anything/pull/718) — Create atomic-write temporary files exclusively | `fb8a7df0` | [CI 成功](https://github.com/storytold/craft-anything/actions/runs/41693868110) | `clean` | 临时文件排他创建回归通过；等待维护者审查/合并 |
| [storytold/craft-anything #767](https://github.com/storytold/craft-anything/pull/767) — Stabilize loopback request and cancellation fixtures | `1d5bd792` | [cd468aca 完整 CI 成功](https://github.com/storytold/craft-anything/actions/runs/41765850311)；[最新 CI 失败](https://github.com/storytold/craft-anything/actions/runs/41767797058) | `unstable` | 最新失败为未改动模型发现夹具的 ETXTBSY；重跑返回403；独立R2.2修复已发布待创建与验证；原审查线程已resolved |
| [storytold/craft-anything #768](https://github.com/storytold/craft-anything/pull/768) — Keep the pasted-picture cleanup test independent of filename reuse | `fb1e2fde` | 仅自动评审检查成功；未见 gate CI | `dirty` | 被已合并 #761 覆盖；连接器关闭请求返回 403，可在 GitHub 网页关闭 |

### 5.2 机器人与机器学习：25 项

来源 S2。LeRobot 的成功结论限定于已实际运行的 Fast Tests/质量/安全/文档流程；没有把缺失的 Full/GPU 结果算作成功。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [huggingface/lerobot #4850](https://github.com/huggingface/lerobot/pull/4850) — fix(gaussian_actor): preserve diagonal standard deviations | `6e423b0d` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/39982928660)；质量/安全/文档成功 | `blocked` | COMMENTED 中非阻塞文档建议已有回应；待维护者审查 |
| [huggingface/lerobot #5158](https://github.com/huggingface/lerobot/pull/5158) — fix(processor): preserve teleop input data during NumPy conversion | `c3739a3b` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41040402954)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5159](https://github.com/huggingface/lerobot/pull/5159) — fix(utils): preserve small rotation vectors | `b051f64e` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41638210384)；质量/安全/文档成功 | `blocked` | 请求移除的测试已删除；nepyope CHANGES_REQUESTED 仍待更新 |
| [huggingface/lerobot #5160](https://github.com/huggingface/lerobot/pull/5160) — fix(processor): convert NumPy views with negative strides | `69943644` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41041129804)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5226](https://github.com/huggingface/lerobot/pull/5226) — fix(rollout): enforce ring-buffer frame and memory limits | `d002891a` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41648690558)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5227](https://github.com/huggingface/lerobot/pull/5227) — fix(rl): route mixer batches only to selected sources | `85fa899f` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41648704952)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5228](https://github.com/huggingface/lerobot/pull/5228) — fix(rl): preserve truncation flags across replay dataset conversions | `b5a8c8a3` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41648710682)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5229](https://github.com/huggingface/lerobot/pull/5229) — fix(processor): flatten non-contiguous IsaacLab state components | `86e39160` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41648719898)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [huggingface/lerobot #5230](https://github.com/huggingface/lerobot/pull/5230) — fix(features): merge vector specifications loaded from JSON | `60a44f6c` | [Fast Tests 成功](https://github.com/huggingface/lerobot/actions/runs/41648725220)；质量/安全/文档成功 | `blocked` | 无正式 review；待维护者审查，blocked 的完整规则未确定 |
| [leggedrobotics/rsl_rl #263](https://github.com/leggedrobotics/rsl_rl/pull/263) — Fix bucketed reduction of noncontiguous gradients | `f73cb0f2` | [Tests 成功](https://github.com/leggedrobotics/rsl_rl/actions/runs/41648626923)；Lint/Docs 成功 | `blocked` | 无正式 review；测试/文档/lint 成功，具体 blocked 规则未确定 |
| [leggedrobotics/rsl_rl #264](https://github.com/leggedrobotics/rsl_rl/pull/264) — Preserve recurrent hidden-state dtypes in rollout storage | `c41d548c` | [Tests 成功](https://github.com/leggedrobotics/rsl_rl/actions/runs/41648632010)；Lint/Docs 成功 | `blocked` | 无正式 review；测试/文档/lint 成功，具体 blocked 规则未确定 |
| [petercorke/robotics-toolbox-python #908](https://github.com/petercorke/robotics-toolbox-python/pull/908) — fix: reject mismatched operational acceleration trajectories | `0265bbd7` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591101571) | `clean` | petercorke APPROVED 精确对应当前 head；待上游合并 |
| [petercorke/robotics-toolbox-python #909](https://github.com/petercorke/robotics-toolbox-python/pull/909) — fix: stop quintic paths at their terminal time | `87a68c08` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591117133) | `clean` | petercorke APPROVED 精确对应当前 head；待上游合并 |
| [petercorke/robotics-toolbox-python #910](https://github.com/petercorke/robotics-toolbox-python/pull/910) — fix: plot trapezoidal trajectories with empty motion phases | `8b0c46a1` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41591137417) | `clean` | petercorke APPROVED 精确对应当前 head；待上游合并 |
| [petercorke/robotics-toolbox-python #925](https://github.com/petercorke/robotics-toolbox-python/pull/925) — fix(build): allow pure wheels without a compiler | `7e3266ff` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648640688) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #926](https://github.com/petercorke/robotics-toolbox-python/pull/926) — fix(mobile): sample particles from weighted CDF intervals | `15963440` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648648436) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #927](https://github.com/petercorke/robotics-toolbox-python/pull/927) — fix(mobile): resolve landmark indices in sensor Jacobians | `47a16459` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648656460) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #928](https://github.com/petercorke/robotics-toolbox-python/pull/928) — fix(mobile): clip occupancy updates to grid bounds | `84fa2bcb` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648664099) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #929](https://github.com/petercorke/robotics-toolbox-python/pull/929) — fix(mobile): retain reverse motion in vehicle odometry | `c07c5217` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648667419) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #930](https://github.com/petercorke/robotics-toolbox-python/pull/930) — fix(mobile): add opt-in four-connected path queries | `d282e72c` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41664455467) | `clean` | 保留默认 8 连通，4 连通为显式选项；待审查/合并 |
| [petercorke/robotics-toolbox-python #931](https://github.com/petercorke/robotics-toolbox-python/pull/931) — fix(mobile): reject PRM maps with no free cells | `fb7e3728` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648679026) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [petercorke/robotics-toolbox-python #932](https://github.com/petercorke/robotics-toolbox-python/pull/932) — fix(mobile): wrap curvature planner heading residuals | `2253d367` | [CI 成功](https://github.com/petercorke/robotics-toolbox-python/actions/runs/41648684739) | `clean` | 无正式 review；CI 已成功，待维护者审查/合并 |
| [Physical-Intelligence/openpi #1077](https://github.com/Physical-Intelligence/openpi/pull/1077) — Validate action chunks before returning the first action | `3786df6c` | [2 个流程待批准](https://github.com/Physical-Intelligence/openpi/actions/runs/41649836390) | `blocked` | 已有前缀有限值/docstring 修订；待批准工作流与维护者审查 |
| [Physical-Intelligence/openpi #1078](https://github.com/Physical-Intelligence/openpi/pull/1078) — Place policy WebSocket ports in the URL authority | `8ccbaee9` | [2 个流程待批准](https://github.com/Physical-Intelligence/openpi/actions/runs/41196649809) | `blocked` | Test/pre-commit 为 action_required；Test jobs=0，待上游授权 |
| [Physical-Intelligence/openpi #1079](https://github.com/Physical-Intelligence/openpi/pull/1079) — Unnormalize truncated feature vectors using matching statistics | `09c2e038` | [2 个流程待批准](https://github.com/Physical-Intelligence/openpi/actions/runs/41196658404) | `blocked` | Test/pre-commit 为 action_required；Test jobs=0，待上游授权 |

### 5.3 工具与推理基础设施：28 项

来源 S1。没有当前 head CI 的项保留“未见记录”，需要批准的工作流保留 action_required。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [AgriciDaniel/claude-seo #403](https://github.com/AgriciDaniel/claude-seo/pull/403) — fix(drift): preserve IPv6 authorities in normalized URLs | `1c54e6e4` | [1 个流程待批准](https://github.com/AgriciDaniel/claude-seo/actions/runs/41661284580) | `blocked` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [AgriciDaniel/claude-seo #404](https://github.com/AgriciDaniel/claude-seo/pull/404) — fix(drift): recognize none as a noindex directive | `f9201f97` | [1 个流程待批准](https://github.com/AgriciDaniel/claude-seo/actions/runs/41661291406) | `blocked` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [AgriciDaniel/claude-seo #405](https://github.com/AgriciDaniel/claude-seo/pull/405) — fix(metadata): match brand suffixes at word boundaries | `3d1cfef2` | [1 个流程待批准](https://github.com/AgriciDaniel/claude-seo/actions/runs/41661296520) | `blocked` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [deepseek-ai/DeepEP-Ascend #19](https://github.com/deepseek-ai/DeepEP-Ascend/pull/19) — Fix communication stream reuse across NPU devices | `ed203f65` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepEP-Ascend #20](https://github.com/deepseek-ai/DeepEP-Ascend/pull/20) — Resolve remote workspace addresses by registration tag | `a0d8bdde` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepEP-Ascend #21](https://github.com/deepseek-ai/DeepEP-Ascend/pull/21) — Preserve the active Engram layout when configuration validation fails | `b836b0bc` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepEP-Ascend #22](https://github.com/deepseek-ai/DeepEP-Ascend/pull/22) — Preserve NPU format metadata for externally allocated buffer tensors | `e3f02085` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepGEMM-Ascend #22](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/22) — Accept packed UE8M0 scale factors in ACLNN GEMM | `6c5d08c5` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepGEMM-Ascend #23](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/23) — Fix the ACLNN BF16 accumulation destination | `616d5796` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepGEMM-Ascend #24](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/24) — Preserve NPU storage metadata in MegaMoE buffers | `edceb910` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepGEMM-Ascend #25](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/25) — Handle empty GEMMs before operand layout dispatch | `51e9795f` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepInfer #258](https://github.com/deepseek-ai/DeepInfer/pull/258) — Fix repeated scalar paths in metadata broadcasts | `17fc7d20` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepInfer #259](https://github.com/deepseek-ai/DeepInfer/pull/259) — Release BlockManager locks after rejected operations | `bdfb18af` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepInfer #260](https://github.com/deepseek-ai/DeepInfer/pull/260) — Cancel queued grammar compilation during shutdown | `793fd246` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepInfer #261](https://github.com/deepseek-ai/DeepInfer/pull/261) — Use monotonic time for multimodal worker deadlines | `dc5422c4` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepInfer #262](https://github.com/deepseek-ai/DeepInfer/pull/262) — Wait for multimodal workers when shared pooling is disabled | `9b3679f1` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [deepseek-ai/DeepJIT #22](https://github.com/deepseek-ai/DeepJIT/pull/22) — fix(cuda): preserve argv boundaries in compiler shell commands | `b7f78967` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [JetBrains/thinkrail #1184](https://github.com/JetBrains/thinkrail/pull/1184) — fix(fs): identify SVG by its document root | `21857b74` | [1 个流程待批准](https://github.com/JetBrains/thinkrail/actions/runs/41661253543) | `blocked` | 自动评审 APPROVED 匹配当前 head；工作流尚未执行 |
| [JetBrains/thinkrail #1185](https://github.com/JetBrains/thinkrail/pull/1185) — fix(fs): detect AVIF compatible brands within ftyp | `3411e8bc` | [1 个流程待批准](https://github.com/JetBrains/thinkrail/actions/runs/41661261680) | `blocked` | 自动评审 APPROVED 匹配当前 head；工作流尚未执行 |
| [JetBrains/thinkrail #1186](https://github.com/JetBrains/thinkrail/pull/1186) — fix(terminal): preserve Unicode boundaries when truncating output | `843c8f2a` | [1 个流程待批准](https://github.com/JetBrains/thinkrail/actions/runs/41661619767) | `blocked` | 自动评审 APPROVED 匹配当前 head；工作流尚未执行 |
| [OpenSQZ/MiniCPM-V-CookBook #143](https://github.com/OpenSQZ/MiniCPM-V-CookBook/pull/143) — fix(finetune): bound recovery from invalid official training samples | `1406ae32` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [OpenSQZ/MiniCPM-V-CookBook #144](https://github.com/OpenSQZ/MiniCPM-V-CookBook/pull/144) — fix(finetune): align official image features with placeholder order | `55d24b79` | 未见当前 head 的 CI/检查记录 | `clean` | 无可见当前 head CI、无维护者 review；等待审查/验证 |
| [TauricResearch/TradingAgents #1646](https://github.com/TauricResearch/TradingAgents/pull/1646) — fix(polymarket): reject non-array outcome values | `068052c0` | [1 个流程待批准](https://github.com/TauricResearch/TradingAgents/actions/runs/41661307343) | `unstable` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [TauricResearch/TradingAgents #1647](https://github.com/TauricResearch/TradingAgents/pull/1647) — fix(polymarket): reject invalid implied probabilities | `fcae97f0` | [1 个流程待批准](https://github.com/TauricResearch/TradingAgents/actions/runs/41661314997) | `unstable` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [TauricResearch/TradingAgents #1648](https://github.com/TauricResearch/TradingAgents/pull/1648) — fix(memory): isolate temporary files for log settlement | `4949f51c` | [1 个流程待批准](https://github.com/TauricResearch/TradingAgents/actions/runs/41661322258) | `unstable` | 待维护者批准工作流与审查；不据此判定代码失败 |
| [tile-ai/tilelang #3716](https://github.com/tile-ai/tilelang/pull/3716) — [BugFix][Autotuner] Canonicalize target dictionary cache keys | `786d9eee` | [CI 失败](https://github.com/tile-ai/tilelang/actions/runs/41039741738)；checkout 与 FP16 数值问题并存 | `clean` | 7 个新增 cache-key 用例成功；完整失败归因仍需证据 |
| [tile-ai/tilelang #3717](https://github.com/tile-ai/tilelang/pull/3717) — [BugFix][Profiler] Resolve compound symbolic input shapes | `22526a11` | [CI 失败](https://github.com/tile-ai/tilelang/actions/runs/41044447045)；具体失败日志不可取 | `unstable` | KellyFrog APPROVED 匹配当前 head；需补取失败原因 |
| [tile-ai/tilelang #3718](https://github.com/tile-ai/tilelang/pull/3718) — [BugFix][NVCC] Honor target architectures in Tensor Core detection | `0c6eca3a` | [CI 成功](https://github.com/tile-ai/tilelang/actions/runs/41040008322) | `clean` | 完整 CI 与 pre-commit 成功；尚无正式人工 approval |

### 5.4 仿真、SDK、检索与其他项目：17 项

来源 S3；Genesis 的 head 与 behind 状态由最新同步记录 U 覆盖，旧审批/线程记录仍按原读取时间注明。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [anthropics/anthropic-sdk-go #477](https://github.com/anthropics/anthropic-sdk-go/pull/477) — fix(middleware): preserve serving-hop usage iterations | `1b60093b` | [1 个流程待批准](https://github.com/anthropics/anthropic-sdk-go/actions/runs/41591442986) | `blocked` | 待维护者批准工作流，再检查实际执行结果 |
| [anthropics/anthropic-sdk-python #2012](https://github.com/anthropics/anthropic-sdk-python/pull/2012) — fix(middleware): preserve serving-hop usage iterations | `d0b86fff` | [1 个流程待批准](https://github.com/anthropics/anthropic-sdk-python/actions/runs/41591422476) | `blocked` | 待维护者批准工作流，再检查实际执行结果 |
| [anthropics/anthropic-sdk-typescript #1260](https://github.com/anthropics/anthropic-sdk-typescript/pull/1260) — fix(middleware): preserve serving-hop usage iterations | `f7f058da` | [2 个流程待批准](https://github.com/anthropics/anthropic-sdk-typescript/actions/runs/41591415742) | `blocked` | CI 与 Ecosystem tests 均待批准 |
| [ARISE-Initiative/robosuite #833](https://github.com/ARISE-Initiative/robosuite/pull/833) — Fix forward wrench transformation to preserve frame-invariant power | `4eb39dc5` | [2 个流程待批准](https://github.com/ARISE-Initiative/robosuite/actions/runs/37432200695) | `blocked` | 有第三方测试反馈；待批准实际 CI，未见维护者改动要求 |
| [deepset-ai/haystack #13498](https://github.com/deepset-ai/haystack/pull/13498) — fix: count each source once in reciprocal rank fusion | `a2a0bb78` | [Tests 成功](https://github.com/deepset-ai/haystack/actions/runs/41027484060)；慢集成成功；Vercel 授权失败 | `blocked` | 待 Vercel 团队授权；需确认真实 retriever 重复 ID 场景 |
| [Genesis-Embodied-AI/genesis-world #3649](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649) — [BUG FIX] Preserve quaternion interpolation gradients at coincident orientations. | `fcd621b0` | [新 head 的 4 个流程待批准](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649/checks) | `blocked` | behind 已解决；审查快照有 1 条过期未解决线程，需批准 CI/审查 |
| [Genesis-Embodied-AI/genesis-world #3650](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650) — [BUG FIX] Return complete homogeneous transforms with reused output buffers. | `f2d54cb8` | [新 head 的 4 个流程待批准](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650/checks) | `blocked` | behind 已解决；审查快照有 1 条过期未解决线程，需批准 CI/审查 |
| [Genesis-Embodied-AI/genesis-world #3651](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651) — [BUG FIX] Preserve orientation when directions approach negative z. | `0ce2464b` | [新 head 的 4 个流程待批准](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651/checks) | `blocked` | behind 已解决；审查快照有 9 条未解决线程，维护者要求优先 |
| [labmlai/annotated_deep_learning_paper_implementations #325](https://github.com/labmlai/annotated_deep_learning_paper_implementations/pull/325) — Apply GroupNorm before DDPM attention projections | `57a4a6d8` | 未见当前 head 的 CI/检查记录 | `clean` | 旧 head 有人工 APPROVED；1 条过期未解决线程已有作者回复，待维护者确认 |
| [lucidrains/x-transformers #379](https://github.com/lucidrains/x-transformers/pull/379) — Apply custom belief-state pair weights with default direction weights | `dd2a1dec` | 未见当前 head 的 CI/检查记录 | `clean` | 暂无正式 review 或 CI 记录；等待维护者审查 |
| [mikedh/trimesh #2663](https://github.com/mikedh/trimesh/pull/2663) — Preserve voxel run-length operations and value types | `d31f4750` | [CI 失败](https://github.com/mikedh/trimesh/actions/runs/41646972201)；i386 apt DNS 失败 | `unstable` | s390x 的 171 项测试成功；维护者可重试失败 i386 作业 |
| [naver/roma #34](https://github.com/naver/roma/pull/34) — Rescale quaternions before normalization and inverse norms | `10ff28a2` | [Unit tests 成功](https://github.com/naver/roma/actions/runs/38915750940) | `clean` | 4 项检查成功；实际用途和每次调用成本仍待维护者判断 |
| [QwenLM/qwen-code #13086](https://github.com/QwenLM/qwen-code/pull/13086) — fix(acp): add defensive legacy NDJSON cancellation cleanup | `4bf95f2d` | [1 个流程待批准](https://github.com/QwenLM/qwen-code/actions/runs/38737875452) | `blocked` | 此前维护者核验针对旧 head；两文件 blob 相同，仍需当前树实际 CI |
| [Toni-SM/skrl #465](https://github.com/Toni-SM/skrl/pull/465) — Restore the learned SAC entropy coefficient on resume | `4eb3f189` | [2 个流程待批准](https://github.com/Toni-SM/skrl/actions/runs/37563325747) | `unstable` | 有当前 head 的积极测试反馈，正式状态为 COMMENTED；待批准 CI |
| [Toni-SM/skrl #488](https://github.com/Toni-SM/skrl/pull/488) — Handle clipping for unbounded action spaces | `c4ba17b2` | [2 个流程待批准](https://github.com/Toni-SM/skrl/actions/runs/41039943824) | `unstable` | 无正式 review；待批准工作流并审查 |
| [Toni-SM/skrl #489](https://github.com/Toni-SM/skrl/pull/489) — Reduce noncontiguous model gradients | `62832600` | [2 个流程待批准](https://github.com/Toni-SM/skrl/actions/runs/41040084995) | `unstable` | 无正式 review；待批准工作流并审查 |
| [Toni-SM/skrl #490](https://github.com/Toni-SM/skrl/pull/490) — Initialize linear layers in nested containers | `77abc9d7` | [2 个流程待批准](https://github.com/Toni-SM/skrl/actions/runs/41040258834) | `unstable` | 无正式 review；待批准工作流并审查 |

### 5.5 其余视觉、数学与学习库：40 项

head/CI 来自 S5；正式 reviews 与权限来自 S4。40 项正式 review 列表均为空，不能据此推断 approval 必为仓库门槛。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [colmap/colmap #4929](https://github.com/colmap/colmap/pull/4929) — Reuse adjacent-frame motion across overlapping TartanAir windows | `8d7fc97a` | [5 个流程待批准](https://github.com/colmap/colmap/actions/runs/41165567802) | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [Eigenwise/atomic-agents #335](https://github.com/Eigenwise/atomic-agents/pull/335) — Resolve MCP schema references by their complete JSON Pointer | `88683182` | [Code Quality & Tests 成功](https://github.com/Eigenwise/atomic-agents/actions/runs/41029441065) | `blocked` | 质量、低版本依赖和安装导入检查成功；待维护者审查 |
| [Eigenwise/atomic-agents #336](https://github.com/Eigenwise/atomic-agents/pull/336) — Bound PDF page-range iteration by the document length | `8d96fc07` | [Code Quality & Tests 成功](https://github.com/Eigenwise/atomic-agents/actions/runs/41029526523) | `blocked` | 质量、低版本依赖和安装导入检查成功；待维护者审查 |
| [Eigenwise/atomic-agents #337](https://github.com/Eigenwise/atomic-agents/pull/337) — Close streamed PDF responses on download failures | `ba0b578a` | [Code Quality & Tests 成功](https://github.com/Eigenwise/atomic-agents/actions/runs/41029728018) | `blocked` | 质量、低版本依赖和安装导入检查成功；待维护者审查 |
| [geomstats/geomstats #2147](https://github.com/geomstats/geomstats/pull/2147) — Preserve batch axes in Brownian sample paths | `fd6e11e2` | [6 个测试作业成功](https://github.com/geomstats/geomstats/actions/runs/41762910310)；共 14 checks 全成功 | `blocked` | RNG 状态恢复已验证；新增回归 27 次执行成功，待维护者审查/合并 |
| [geomstats/geomstats #2148](https://github.com/geomstats/geomstats/pull/2148) — Assign fitted k-medoids labels from the final centers | `6ad346de` | [Testing 失败](https://github.com/geomstats/geomstats/actions/runs/41030310414) | `blocked` | Lint/Docs 成功；仍有测试失败，未证明属于基线问题 |
| [geomstats/geomstats #2149](https://github.com/geomstats/geomstats/pull/2149) — Center affine-invariant SPD lognormal samples at the supplied mean | `9a40304e` | [Testing 失败](https://github.com/geomstats/geomstats/actions/runs/41036470138) | `blocked` | Lint/Docs 成功；仍有测试失败，未证明属于基线问题 |
| [geoopt/geoopt #278](https://github.com/geoopt/geoopt/pull/278) — Support Euclidean norms with more than two manifold dimensions | `6f54b572` | [Python package 成功](https://github.com/geoopt/geoopt/actions/runs/41030839909) | `clean` | 已执行的 Python package 检查成功；待维护者审查/合并 |
| [geoopt/geoopt #279](https://github.com/geoopt/geoopt/pull/279) — Preserve batch dimensions in UpperHalf origins | `b4ac023c` | [Python package 成功](https://github.com/geoopt/geoopt/actions/runs/41030895395) | `clean` | 已执行的 Python package 检查成功；待维护者审查/合并 |
| [geoopt/geoopt #280](https://github.com/geoopt/geoopt/pull/280) — Preserve tangent vectors in canonical Stiefel projection | `a3ea54f9` | [Python package 成功](https://github.com/geoopt/geoopt/actions/runs/41036782265) | `clean` | 已执行的 Python package 检查成功；待维护者审查/合并 |
| [google-deepmind/mujoco #3745](https://github.com/google-deepmind/mujoco/pull/3745) — Fix axis-aware zero handling and normalization in MJX | `8ee1eecd` | [lint/build 待批准](https://github.com/google-deepmind/mujoco/pull/3745/checks)；CLA/辅助检查成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [google-deepmind/mujoco #3746](https://github.com/google-deepmind/mujoco/pull/3746) — Preserve quaternion integration derivatives at zero velocity | `42fa9154` | [lint/build 待批准](https://github.com/google-deepmind/mujoco/pull/3746/checks)；CLA/辅助检查成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [google-deepmind/mujoco #3747](https://github.com/google-deepmind/mujoco/pull/3747) — Remove scale bias from MJX segment projection | `edafc711` | [lint/build 待批准](https://github.com/google-deepmind/mujoco/pull/3747/checks)；CLA/辅助检查成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [graphdeco-inria/gaussian-splatting #1350](https://github.com/graphdeco-inria/gaussian-splatting/pull/1350) — Preserve exposure parameters and optimizer state in checkpoints | `f179a000` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [graphdeco-inria/gaussian-splatting #1351](https://github.com/graphdeco-inria/gaussian-splatting/pull/1351) — Preserve per-image LPIPS distances for batched inputs | `e2522e30` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [graphdeco-inria/gaussian-splatting #1352](https://github.com/graphdeco-inria/gaussian-splatting/pull/1352) — Accept SIMPLE_PINHOLE cameras in COLMAP text models | `069b35dc` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [isl-org/Open3D #7613](https://github.com/isl-org/Open3D/pull/7613) — Keep scalar material textures linear during Mitsuba conversion | `8b80500e` | [8 个流程待批准](https://github.com/isl-org/Open3D/pull/7613/checks)；Codacy 成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [isl-org/Open3D #7614](https://github.com/isl-org/Open3D/pull/7614) — Finalize Mitsuba mesh parameters before returning the converted mesh | `4be59082` | [8 个流程待批准](https://github.com/isl-org/Open3D/pull/7614/checks)；Codacy 成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [isl-org/Open3D #7615](https://github.com/isl-org/Open3D/pull/7615) — Preserve triangle-corner UV seams when exporting meshes to Mitsuba | `d38a749f` | [8 个流程待批准](https://github.com/isl-org/Open3D/pull/7615/checks)；Codacy 成功 | `blocked` | 无正式 review；维护者需先批准受限工作流 |
| [leggedrobotics/rsl_rl #257](https://github.com/leggedrobotics/rsl_rl/pull/257) — Include action-range scaling in Beta entropy | `59757058` | [Tests 成功](https://github.com/leggedrobotics/rsl_rl/actions/runs/41031338965)；Lint/Docs 成功 | `blocked` | 无正式 review；测试/文档/lint 成功，具体 blocked 规则未确定 |
| [leggedrobotics/rsl_rl #258](https://github.com/leggedrobotics/rsl_rl/pull/258) — Normalize vector rewards without ambiguous tensor conditions | `7a5e7ece` | [Tests 成功](https://github.com/leggedrobotics/rsl_rl/actions/runs/41031450875)；Lint/Docs 成功 | `blocked` | 无正式 review；测试/文档/lint 成功，具体 blocked 规则未确定 |
| [leggedrobotics/rsl_rl #259](https://github.com/leggedrobotics/rsl_rl/pull/259) — Keep computed CNN padding nonnegative for small kernels | `dff6d3f0` | [Tests 成功](https://github.com/leggedrobotics/rsl_rl/actions/runs/41032169269)；Lint/Docs 成功 | `blocked` | 无正式 review；测试/文档/lint 成功，具体 blocked 规则未确定 |
| [lucidrains/denoising-diffusion-pytorch #391](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/391) — Honor the clipping setting in simple diffusion | `66383024` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [lucidrains/denoising-diffusion-pytorch #392](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/392) — Respect the normalization setting for RePaint ground truth | `bce1564a` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [lucidrains/denoising-diffusion-pytorch #393](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/393) — Keep RePaint resampling aligned with the current noise level | `4d7159d1` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [NVlabs/instant-ngp #1616](https://github.com/NVlabs/instant-ngp/pull/1616) — Make antipodal camera alignment deterministic and orthogonal | `7ed00ff8` | [1 个流程待批准](https://github.com/NVlabs/instant-ngp/actions/runs/41165165385) | `unstable` | 无正式 review；维护者需先批准受限工作流 |
| [NVlabs/instant-ngp #1617](https://github.com/NVlabs/instant-ngp/pull/1617) — Preserve empty observations and image names in COLMAP text imports | `20e874e8` | [1 个流程待批准](https://github.com/NVlabs/instant-ngp/actions/runs/41196629924) | `unstable` | 无正式 review；维护者需先批准受限工作流 |
| [NVlabs/instant-ngp #1618](https://github.com/NVlabs/instant-ngp/pull/1618) — Fix scalar MtRSE reduction in image error evaluation | `31ba787f` | [1 个流程待批准](https://github.com/NVlabs/instant-ngp/actions/runs/41196640508) | `unstable` | 无正式 review；维护者需先批准受限工作流 |
| [OpenGVLab/InternVL #1281](https://github.com/OpenGVLab/InternVL/pull/1281) — [Fix] Preserve stratified random video sampling at interval boundaries | `c10c3cc4` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [OpenGVLab/InternVL #1282](https://github.com/OpenGVLab/InternVL/pull/1282) — [Fix] Decode only sampled frames from video folders | `f54d6f8c` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [OpenGVLab/InternVL #1283](https://github.com/OpenGVLab/InternVL/pull/1283) — [Fix] Concatenate NumPy image patches in SFT and DPO collators | `41f9f3a2` | 未见当前 head 的 CI/检查记录 | `clean` | 无正式 review；未见 CI，clean 不代表测试通过 |
| [pypose/pypose #446](https://github.com/pypose/pypose/pull/446) — Keep neighbor indices aligned when radius-filtering point clouds | `b8d364a6` | 未见当前 head 的 CI/检查记录 | `blocked` | 无正式 review、未见当前 head CI；待确认 required gates |
| [pypose/pypose #447](https://github.com/pypose/pypose/pull/447) — Compute L1 distance for reprojection sum reduction | `582412fb` | 未见当前 head 的 CI/检查记录 | `blocked` | 无正式 review、未见当前 head CI；待确认 required gates |
| [pypose/pypose #448](https://github.com/pypose/pypose/pull/448) — Preserve point-cloud axes for empty and single-voxel inputs | `fa9242dd` | 未见当前 head 的 CI/检查记录 | `blocked` | 无正式 review、未见当前 head CI；待确认 required gates |
| [rasbt/mlxtend #1267](https://github.com/rasbt/mlxtend/pull/1267) — Preserve sample and feature axes in MeanCenterer | `cd0489c7` | [单元测试成功](https://github.com/rasbt/mlxtend/actions/runs/41035116167)；[lint 失败](https://github.com/rasbt/mlxtend/actions/runs/41035116094) | `unstable` | 独立 Python 3.11 工作流修复已在 fork 验证；待创建上游修复 PR |
| [rasbt/mlxtend #1268](https://github.com/rasbt/mlxtend/pull/1268) — Fix default ColumnSelector feature selection | `0fb2e078` | [单元测试成功](https://github.com/rasbt/mlxtend/actions/runs/41035417632)；[lint 失败](https://github.com/rasbt/mlxtend/actions/runs/41035417587) | `unstable` | 独立 Python 3.11 工作流修复已在 fork 验证；待创建上游修复 PR |
| [rasbt/mlxtend #1269](https://github.com/rasbt/mlxtend/pull/1269) — Preserve integer precision in paired permutation tests | `fa4d9350` | [单元测试成功](https://github.com/rasbt/mlxtend/actions/runs/41035609602)；[lint 失败](https://github.com/rasbt/mlxtend/actions/runs/41035609504) | `unstable` | 独立 Python 3.11 工作流修复已在 fork 验证；待创建上游修复 PR |
| [recommenders-team/recommenders #2419](https://github.com/recommenders-team/recommenders/pull/2419) — Preserve input columns in stratified splitters | `e6f66e06` | [6 项检查成功（含测试矩阵与 DCO）](https://github.com/recommenders-team/recommenders/pull/2419/checks) | `blocked` | 无正式 review；测试检查成功，具体 blocked 规则未确定 |
| [recommenders-team/recommenders #2420](https://github.com/recommenders-team/recommenders/pull/2420) — Prevent integer overflow in similarity normalization | `3a720f77` | [6 项检查成功（含测试矩阵与 DCO）](https://github.com/recommenders-team/recommenders/pull/2420/checks) | `blocked` | 无正式 review；测试检查成功，具体 blocked 规则未确定 |
| [recommenders-team/recommenders #2421](https://github.com/recommenders-team/recommenders/pull/2421) — Preserve full precision in NumPy split ratios | `6cad9dd6` | [6 项检查成功（含测试矩阵与 DCO）](https://github.com/recommenders-team/recommenders/pull/2421/checks) | `blocked` | 无正式 review；测试检查成功，具体 blocked 规则未确定 |

### 5.6 三项 fork 验证

这三项均为验证草稿。craft-anything #2 与已合并的上游 #766 共用 c044adf1 分支；该 head 在 fork 的工作流列表为空，上游成功 CI 已在 #766 单列。

| PR（原标题） | 当前 head | CI / 验证证据 | 合并状态 | 审查与下一步 |
|---|---|---|---|---|
| [Afloat16/craft-anything #2](https://github.com/Afloat16/craft-anything/pull/2) — Validate the post-merge parser review follow-ups (R3.12) | `c044adf1` | fork 未见 CI 运行 | `unknown` | 验证草稿；对应的上游 #766 已通过完整 CI 并合并 |
| [Afloat16/mlxtend #1](https://github.com/Afloat16/mlxtend/pull/1) — Validate Python 3.11 for the lint workflow | `410abc2b` | [lint 成功](https://github.com/Afloat16/mlxtend/actions/runs/41763206302)；[unit tests 两个 jobs 成功](https://github.com/Afloat16/mlxtend/actions/runs/41763206340) | `clean` | 验证草稿；上游 PR 创建 403，fork 成功不等于上游 merged |
| [Afloat16/craft-anything #3](https://github.com/Afloat16/craft-anything/pull/3) — Validate the model-discovery shell fixture (R2.2) | `dac9261a` | 未见工作流运行，执行验证待完成 | `clean` | 独立夹具修复草稿；基线为上游 main0356c4bc，差异仅2文件 |


## 6. 已关闭 PR：直接合并与其他关闭分别记录

### 6.1 窗口内直接 merged 的 51 项

以下每项均有实际 `mergedAt` 记录。本轮新增 #773、#766、#5341、#5340；其余 47 项在本轮初始清单中已经合并。

| 仓库 | 窗口内直接 merged |
|---|---:|
| storytold/craft-anything | 25 |
| geoopt/geoopt | 6 |
| Denys88/rl_games | 4 |
| KevinMusgrave/pytorch-metric-learning | 3 |
| serengil/deepface | 3 |
| colmap/colmap | 2 |
| huggingface/sentence-transformers | 2 |
| kornia/kornia | 2 |
| mikedh/trimesh | 1 |

| PR | 改进（原标题） | 实际合并时间 UTC |
|---|---|---|
| [storytold/craft-anything #766](https://github.com/storytold/craft-anything/pull/766) **本轮新增** | Address the parser review findings from #722 (R3.12) | 2026-11-04 03:30:58 |
| [thedotmack/claude-mem #5340](https://github.com/thedotmack/claude-mem/pull/5340) **本轮新增** | fix(providers): release capped error-body reader locks | 2026-11-04 03:26:00 |
| [thedotmack/claude-mem #5341](https://github.com/thedotmack/claude-mem/pull/5341) **本轮新增** | fix(worker): reject empty URL request modifiers | 2026-11-04 03:25:45 |
| [storytold/craft-anything #773](https://github.com/storytold/craft-anything/pull/773) **本轮新增** | Keep builder availability fixtures independent of live discovery (R2.9) | 2026-11-04 03:05:53 |
| [storytold/craft-anything #673](https://github.com/storytold/craft-anything/pull/673) | Preserve empty query pairs in older stand-in recordings | 2026-11-03 16:16:53 |
| [storytold/craft-anything #675](https://github.com/storytold/craft-anything/pull/675) | Handle generic GGUF paths and numeric task versions in chat discovery (R5.4) | 2026-11-03 16:14:26 |
| [storytold/craft-anything #677](https://github.com/storytold/craft-anything/pull/677) | Verify stand-in session cleanup by answerer lifetime | 2026-11-03 16:14:19 |
| [storytold/craft-anything #674](https://github.com/storytold/craft-anything/pull/674) | Exercise the unterminated DER OID guard | 2026-11-03 15:54:15 |
| [kornia/kornia #6449](https://github.com/kornia/kornia/pull/6449) | fix(augmentation): register dispatcher sequences as child modules | 2026-11-03 15:11:30 |
| [storytold/craft-anything #557](https://github.com/storytold/craft-anything/pull/557) | Stop unchanged rebuilds after two turns across runs (R5.3) | 2026-11-03 12:31:23 |
| [storytold/craft-anything #552](https://github.com/storytold/craft-anything/pull/552) | Retry builder turns from typed failures, including stateless backends (R5.3) | 2026-11-03 08:51:05 |
| [storytold/craft-anything #546](https://github.com/storytold/craft-anything/pull/546) | Help models correct rejected tool argument types (R5.4) | 2026-11-03 08:48:57 |
| [storytold/craft-anything #554](https://github.com/storytold/craft-anything/pull/554) | Recover JSON tool calls emitted as text (R5.4) | 2026-11-03 08:39:40 |
| [storytold/craft-anything #550](https://github.com/storytold/craft-anything/pull/550) | Keep ACP history replay out of new turns (R2.2) | 2026-11-03 08:01:32 |
| [storytold/craft-anything #433](https://github.com/storytold/craft-anything/pull/433) | Test big-endian nanosecond pcap captures with FCS metadata (R0.5) | 2026-11-03 07:56:02 |
| [storytold/craft-anything #559](https://github.com/storytold/craft-anything/pull/559) | Honor per-backend conversation context budgets (R5.4) | 2026-11-03 07:53:55 |
| [storytold/craft-anything #544](https://github.com/storytold/craft-anything/pull/544) | Respect stop reasons before running native tool calls (R5.4) | 2026-11-03 07:50:11 |
| [storytold/craft-anything #543](https://github.com/storytold/craft-anything/pull/543) | Report unavailable cached bodies in stand-in replay (R3.11) | 2026-11-03 07:50:01 |
| [storytold/craft-anything #541](https://github.com/storytold/craft-anything/pull/541) | Wait for keep-awake cleanup before rebuild completion (R2.9) | 2026-11-03 07:49:56 |
| [storytold/craft-anything #542](https://github.com/storytold/craft-anything/pull/542) | Show upstream provider reasons in API errors (R5.5) | 2026-11-03 07:49:51 |
| [mikedh/trimesh #2618](https://github.com/mikedh/trimesh/pull/2618) | Fix nonuniform weighted Procrustes rotation | 2026-11-02 20:55:10 |
| [storytold/craft-anything #439](https://github.com/storytold/craft-anything/pull/439) | Pinned builds keep runtime models in obj folders (R2.10) | 2026-11-02 17:16:58 |
| [storytold/craft-anything #442](https://github.com/storytold/craft-anything/pull/442) | Build usage includes context-summary calls (R2.1) | 2026-11-02 16:48:58 |
| [storytold/craft-anything #441](https://github.com/storytold/craft-anything/pull/441) | Compare WebAssembly exported tag parameter types (R0.5) | 2026-11-02 16:48:32 |
| [storytold/craft-anything #440](https://github.com/storytold/craft-anything/pull/440) | App comparisons keep every unassigned window (R3.10) | 2026-11-02 16:48:06 |
| [storytold/craft-anything #438](https://github.com/storytold/craft-anything/pull/438) | Minified esbuild modules keep their dependencies (R0.5) | 2026-11-02 16:42:50 |
| [storytold/craft-anything #437](https://github.com/storytold/craft-anything/pull/437) | Find class usage in multidimensional object arrays (R0.5) | 2026-11-02 16:42:23 |
| [storytold/craft-anything #436](https://github.com/storytold/craft-anything/pull/436) | Use ARM32 registers in value-flow graphs (R0.5) | 2026-11-02 16:41:57 |
| [storytold/craft-anything #435](https://github.com/storytold/craft-anything/pull/435) | Bun text assets report their stored UTF-16LE encoding (R3.11) | 2026-11-02 16:41:30 |
| [storytold/craft-anything #434](https://github.com/storytold/craft-anything/pull/434) | Files inside extended SquashFS directories are recovered (R0.5) | 2026-11-02 16:38:00 |
| [huggingface/sentence-transformers #4114](https://github.com/huggingface/sentence-transformers/pull/4114) | Exclude unretrieved positives from observed RRF ranking metrics | 2026-11-02 10:54:28 |
| [colmap/colmap #4931](https://github.com/colmap/colmap/pull/4931) | Resolve benchmark paths before changing subprocess directories | 2026-11-02 10:40:17 |
| [colmap/colmap #4932](https://github.com/colmap/colmap/pull/4932) | Make benchmark translation-angle errors invariant to reconstruction scale | 2026-11-02 08:40:02 |
| [huggingface/sentence-transformers #4301](https://github.com/huggingface/sentence-transformers/pull/4301) | fix: preserve padded sequence lengths in LSTM outputs | 2026-11-02 08:32:38 |
| [Denys88/rl_games #394](https://github.com/Denys88/rl_games/pull/394) | Reject recurrent minibatches that split sequences | 2026-11-02 02:38:53 |
| [Denys88/rl_games #393](https://github.com/Denys88/rl_games/pull/393) | Include the final occupied slot in prioritized replay sampling | 2026-11-02 02:38:43 |
| [Denys88/rl_games #392](https://github.com/Denys88/rl_games/pull/392) | Support NumPy 2 when sampling scalar replay values | 2026-11-02 02:38:33 |
| [serengil/deepface #1677](https://github.com/serengil/deepface/pull/1677) | Keep resized image dimensions positive | 2026-10-31 17:31:40 |
| [serengil/deepface #1676](https://github.com/serengil/deepface/pull/1676) | Correct skipped-detector representation dimensions | 2026-10-31 17:31:11 |
| [serengil/deepface #1675](https://github.com/serengil/deepface/pull/1675) | Preserve representation batch result structure | 2026-10-31 17:25:58 |
| [KevinMusgrave/pytorch-metric-learning #756](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/756) | Cover HistogramLoss endpoints and interpolation in v3 | 2026-10-30 21:53:05 |
| [KevinMusgrave/pytorch-metric-learning #759](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/759) | Add independent PNP value and gradient regressions | 2026-10-30 21:53:00 |
| [KevinMusgrave/pytorch-metric-learning #758](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/758) | Preserve NT-Xent values and gradients under common score shifts | 2026-10-30 21:44:19 |
| [kornia/kornia #6451](https://github.com/kornia/kornia/pull/6451) | fix(feature): avoid NaN orientation weight gradients on flat patches | 2026-10-30 19:09:03 |
| [geoopt/geoopt #254](https://github.com/geoopt/geoopt/pull/254) | Initialize Riemannian SGD momentum from the first tangent gradient | 2026-10-29 11:55:47 |
| [geoopt/geoopt #253](https://github.com/geoopt/geoopt/pull/253) | Fix signed hyperplane distances and gradients for negative inputs | 2026-10-29 11:54:41 |
| [geoopt/geoopt #255](https://github.com/geoopt/geoopt/pull/255) | Avoid mutating parameter gradients during optimizer regularization | 2026-10-29 11:54:20 |
| [geoopt/geoopt #256](https://github.com/geoopt/geoopt/pull/256) | Keep spherical exponential-map gradients finite at zero steps | 2026-10-29 11:53:57 |
| [geoopt/geoopt #258](https://github.com/geoopt/geoopt/pull/258) | Retain spherical geodesic precision at small arcs and endpoints | 2026-10-29 08:31:46 |
| [geoopt/geoopt #259](https://github.com/geoopt/geoopt/pull/259) | Keep sparse AMSGrad maxima in raw second-moment units | 2026-10-29 08:31:15 |
| [Denys88/rl_games #384](https://github.com/Denys88/rl_games/pull/384) | Preserve the latest transitions when replay batches exceed capacity | 2026-10-29 01:57:39 |

### 6.2 窗口前已经合并、在窗口内更新的 1 项

[huggingface/lerobot #4835](https://github.com/huggingface/lerobot/pull/4835) — fix(vla_jepa): pair rotary frequencies with adjacent coordinates。实际合并时间为 **2026-10-22 14:00:39 UTC**，最近更新时间为 2026-10-29 09:21:59 UTC。它计入已关闭总数，但不计入本窗口的 51 项直接合并。

### 6.3 Closed 且未直接 merged 的 81 项

其中 27 项已在第 4 节逐项确认通过 #722 收录；另 1 项 claude-mem #5339 的部分测试通过 #5336 收录，范围与署名见第 2 节。其余 **53 项**如下，原 PR 已关闭且没有直接合并记录；接纳范围以具体整合证据为准。

| 仓库 | 数量 | 原 PR |
|---|---:|---|
| storytold/craft-anything | 34 | [#591](https://github.com/storytold/craft-anything/pull/591)、[#592](https://github.com/storytold/craft-anything/pull/592)、[#593](https://github.com/storytold/craft-anything/pull/593)、[#594](https://github.com/storytold/craft-anything/pull/594)、[#595](https://github.com/storytold/craft-anything/pull/595)、[#596](https://github.com/storytold/craft-anything/pull/596)、[#597](https://github.com/storytold/craft-anything/pull/597)、[#598](https://github.com/storytold/craft-anything/pull/598)、[#599](https://github.com/storytold/craft-anything/pull/599)、[#600](https://github.com/storytold/craft-anything/pull/600)、[#601](https://github.com/storytold/craft-anything/pull/601)、[#602](https://github.com/storytold/craft-anything/pull/602)、[#603](https://github.com/storytold/craft-anything/pull/603)、[#604](https://github.com/storytold/craft-anything/pull/604)、[#605](https://github.com/storytold/craft-anything/pull/605)、[#606](https://github.com/storytold/craft-anything/pull/606)、[#607](https://github.com/storytold/craft-anything/pull/607)、[#608](https://github.com/storytold/craft-anything/pull/608)、[#609](https://github.com/storytold/craft-anything/pull/609)、[#610](https://github.com/storytold/craft-anything/pull/610)、[#611](https://github.com/storytold/craft-anything/pull/611)、[#612](https://github.com/storytold/craft-anything/pull/612)、[#613](https://github.com/storytold/craft-anything/pull/613)、[#614](https://github.com/storytold/craft-anything/pull/614)、[#615](https://github.com/storytold/craft-anything/pull/615)、[#616](https://github.com/storytold/craft-anything/pull/616)、[#617](https://github.com/storytold/craft-anything/pull/617)、[#619](https://github.com/storytold/craft-anything/pull/619)、[#620](https://github.com/storytold/craft-anything/pull/620)、[#621](https://github.com/storytold/craft-anything/pull/621)、[#640](https://github.com/storytold/craft-anything/pull/640)、[#641](https://github.com/storytold/craft-anything/pull/641)、[#642](https://github.com/storytold/craft-anything/pull/642)、[#676](https://github.com/storytold/craft-anything/pull/676) |
| mikedh/trimesh | 5 | [#2664](https://github.com/mikedh/trimesh/pull/2664)、[#2665](https://github.com/mikedh/trimesh/pull/2665)、[#2666](https://github.com/mikedh/trimesh/pull/2666)、[#2667](https://github.com/mikedh/trimesh/pull/2667)、[#2668](https://github.com/mikedh/trimesh/pull/2668) |
| huggingface/sentence-transformers | 3 | [#4136](https://github.com/huggingface/sentence-transformers/pull/4136)、[#4302](https://github.com/huggingface/sentence-transformers/pull/4302)、[#4303](https://github.com/huggingface/sentence-transformers/pull/4303) |
| OpenBMB/MiniCPM-V | 3 | [#1160](https://github.com/OpenBMB/MiniCPM-V/pull/1160)、[#1161](https://github.com/OpenBMB/MiniCPM-V/pull/1161)、[#1162](https://github.com/OpenBMB/MiniCPM-V/pull/1162) |
| opendatalab/MinerU | 3 | [#5701](https://github.com/opendatalab/MinerU/pull/5701)、[#5702](https://github.com/opendatalab/MinerU/pull/5702)、[#5703](https://github.com/opendatalab/MinerU/pull/5703) |
| KevinMusgrave/pytorch-metric-learning | 2 | [#755](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/755)、[#757](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/757) |
| Afloat16/craft-anything | 1 | [#1](https://github.com/Afloat16/craft-anything/pull/1) |
| kornia/kornia | 1 | [#6450](https://github.com/kornia/kornia/pull/6450) |
| morluto/rea | 1 | [#3940](https://github.com/morluto/rea/pull/3940) |

上表包含 1 项已关闭的 fork 验证 PR（Afloat16/craft-anything #1）；同样没有把它计入上游贡献接纳数。全部 133 条 closed 记录可从最终清单或对应 PR 页面复核。

## 7. 下一步与必要的上游操作

1. **处理 #767 最新 CI 的启动阻塞。** #766 已 merged；#767 的先前源码 head 完整通过，最新文档 head 的 CI 则因模型发现夹具启动时 ETXTBSY 失败。维护者可正常重跑该失败作业；独立 R2.2 修复也已准备完成，创建后应验证并按实际结果整合。geomstats #2147 的 14 个检查已全部成功。
2. **优先合并现有证据最充分的 PR。** robotics-toolbox-python #908/#909/#910 已有与当前 head 精确对应的维护者 APPROVED 和成功 CI；tilelang #3718、craft-anything #716/#717/#718 的已执行 CI 成功，等待上游审查与合并权限操作。
3. **让 action_required 工作流实际执行。** 需要具有上游权限的维护者批准；批准后再根据新 run 的真实结果决定是否还要修代码。Genesis 的 behind 已解决，不再为触发运行而重复同步或制造无效提交。
4. **创建已准备好的 mlxtend 上游 CI 修复 PR。** [已发布分支](https://github.com/Afloat16/mlxtend/tree/chore/pin-lint-python-20261104)和 [fork 验证 #1](https://github.com/Afloat16/mlxtend/pull/1)可供审查；两个工作流均已成功。连接器创建上游 PR 返回 403，需要通过有权限的 GitHub 创建入口提交。
   [点击创建 mlxtend 上游 PR（标题、说明、验证链接均已预填）](https://github.com/rasbt/mlxtend/compare/master...Afloat16:chore/pin-lint-python-20261104?quick_pull=1&title=ci%3A%20pin%20lint%20environment%20to%20Python%203.11&body=%23%23%23%20Description%0A%0AThe%20lint%20workflow%20currently%20selects%20CPython%203.15.1%20through%20%60python-version%3A%20%273.x%27%60.%20Installing%20the%20frozen%20dependencies%20then%20builds%20SciPy%201.16.3%20from%20source%20and%20fails%20because%20OpenBLAS%20is%20unavailable%2C%20before%20the%20formatting%20checks%20run.%0A%0AUse%20Python%203.11%20in%20both%20setup-python%20and%20uv%20sync%2C%20matching%20the%20existing%20unit-test%20workflow.%20This%20changes%20two%20lines%20in%20one%20workflow.%20The%20lock%20file%2C%20dependency%20versions%2C%20and%20isort%2C%20black%20and%20flake8%20commands%20are%20unchanged.%0A%0A%23%23%23%20Related%20issues%20or%20pull%20requests%0A%0AObserved%20on%20%231267%2C%20%231268%20and%20%231269.%20Their%20unit-test%20workflows%20pass%20with%20Python%203.11%3B%20their%20lint%20jobs%20fail%20during%20dependency%20installation%3A%0A-%20%5B%231267%20lint%20job%5D%28https%3A%2F%2Fgithub.com%2Frasbt%2Fmlxtend%2Factions%2Fruns%2F41035116094%2Fjob%2F123993938948%29%0A-%20%5B%231268%20lint%20job%5D%28https%3A%2F%2Fgithub.com%2Frasbt%2Fmlxtend%2Factions%2Fruns%2F41035417587%2Fjob%2F123994929556%29%0A-%20%5B%231269%20lint%20job%5D%28https%3A%2F%2Fgithub.com%2Frasbt%2Fmlxtend%2Factions%2Fruns%2F41035609504%2Fjob%2F123995568470%29%0A%0A%23%23%23%20Verification%0A%0AHosted%20validation%20on%20commit%20%60410abc2b778310c64cab57dd284171a73747181d%60%20in%20the%20fork%20succeeded%3A%0A-%20%5BLint%20workflow%5D%28https%3A%2F%2Fgithub.com%2FAfloat16%2Fmlxtend%2Factions%2Fruns%2F41763206302%29%3A%20dependency%20installation%20and%20all%20existing%20lint%20commands%20passed.%0A-%20%5BUnit-test%20workflow%5D%28https%3A%2F%2Fgithub.com%2FAfloat16%2Fmlxtend%2Factions%2Fruns%2F41763206340%29%3A%20both%20build-linux%20and%20build-linux-latest-sklearn%20passed.%0A%0AThe%20validation%20PR%20is%20based%20on%20the%20same%20upstream%20master%20commit%20%601aee767ef57b84a8191511a08955c583b65de382%60.%20The%20upstream%20PR%20still%20needs%20its%20own%20required%20checks.%0A%0AThis%20is%20a%20CI%20configuration%20change%2C%20so%20product%20documentation%2C%20a%20release%20note%20and%20new%20product%20unit%20tests%20are%20not%20applicable.)
5. **整理已被合并改动覆盖的提案。** #773 已 merged，应整理仍开放的 #681；#768 已被 #761 覆盖，关闭操作返回 403。保留维护者的接纳路径，避免再提交重复生产改动。
6. **创建独立的 R2.2 夹具修复 PR。** [已发布分支](https://github.com/Afloat16/craft-anything/tree/test/r2.2-model-discovery-fixture-20261104) 含 claim 与修复两个提交，当前执行验证待完成；创建上游 PR 的连接器请求返回 403。
   [点击创建 craft-anything 夹具修复 PR（标题、说明、失败证据均已预填）](https://github.com/storytold/craft-anything/compare/main...Afloat16:test/r2.2-model-discovery-fixture-20261104?quick_pull=1&title=Run%20the%20model-discovery%20fixture%20through%20an%20existing%20shell%20%28R2.2%29&body=%23%23%20Problem%0A%0AThe%20full%20CI%20for%20%23767%20failed%20before%20the%20model-discovery%20fixture%20could%20run%3A%20starting%20its%20newly%20written%20%60fake%20agent%60%20script%20returned%20%60Text%20file%20busy%20%28os%20error%2026%29%60.%20The%20same%20fixture%20passed%20in%20the%20previous%20full%20run.%0A%5BFailure%20log%5D%28https%3A%2F%2Fgithub.com%2Fstorytold%2Fcraft-anything%2Factions%2Fruns%2F41767797058%2Fjob%2F126370174961%29%0A%0A%23%23%20Change%0A%0AKeep%20the%20selected%20program%20path%2C%20including%20its%20space%2C%20as%20a%20symlink%20to%20the%20installed%20shell.%20The%20shell%20reads%20the%20fixture%20from%20%60api%60%20in%20the%20project%20directory%2C%20so%20the%20writable%20fixture%20is%20not%20itself%20executed.%0A%0APreserve%20the%20catalogue%2C%20working-directory%2C%20argument%2C%20malformed-output%2C%20exit-error%20and%2050%20ms%20timeout%20checks.%20Add%20a%20control%20that%20selects%20an%20existing%20non-executable%20file%2C%20ensuring%20an%20implementation%20that%20ignores%20the%20selected%20program%20cannot%20pass.%0A%0AThe%20change%20is%20limited%20to%20one%20test%20function%20and%20its%20R2.2%20roadmap%20entry.%20The%20roadmap%20claim%20was%20published%20first.%0A%0A%23%23%20Verification%0A%0AThe%20diff%20and%20call%20path%20have%20been%20checked.%20Targeted%20execution%20and%20full%20Linux%20CI%20are%20pending%20for%20this%20branch%3A%0A%60cargo%20test%20-p%20craft-anything-ai%20--lib%20cli_agents%3A%3Amodels%3A%3Atests%3A%3Adiscovery_reads_the_selected_program_in_the_project_and_reports_errors%20--%20--exact%60%0A%0AThe%20failure%20log%20does%20not%20identify%20which%20process%20held%20the%20original%20executable%20open%20for%20writing.%20This%20fixture%20change%20removes%20that%20execution%20condition%20without%20changing%20production%20process%20handling.)
7. **按失败日志和具体审查继续推进。** tilelang、geomstats、haystack、roma 等保留各自真实问题；需要补证据的失败先补证据，需要维护者设计判断的讨论交由维护者确认，不降低断言标准来换取绿灯。

上游合并和受限工作流批准需要相应仓库权限。#681/#768 的作者可登录 GitHub，在各 PR 页面底部使用 **Close pull request** 整理重复提案；当前连接器对上游 PR 的状态修改返回 403。[GitHub 关闭操作说明](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/closing-a-pull-request)本轮代码、回归测试和分支更新均已发布；mlxtend 与独立 R2.2 修复的两个上游创建入口均已预填。最新失败、尚待执行的验证及权限要求逐项列出。

## 8. 证据来源与状态口径

| 编号 | 数据范围 | 读取或快照时间（UTC） | 采用方式 |
|---|---|---|---|
| S0 | 账号初始扫描、当前开放/关闭清单、已发布动作 | 2026-11-04 03:02:07 | [固定 JSON 快照](https://github.com/Afloat16/craft-anything/blob/b65ee9841d79e0561a5bd806ae5c32eeb7149c79/docs/contributions/20261104-account-merge-audit.json)；后续确认变化另列 |
| S1 | 31 项工具、推理/训练基础设施 PR | 2026-11-04 02:30:58—02:38:15 | `evidence.binary`；含当前 head 的 runs、reviews、失败日志证据及权限 |
| S2 | 25 项 robotics/ML PR | metadata 最后刷新 2026-11-04 02:45:46 | `evidence.network`；CI、review、threads、permissions 在同轮审计读取；最后刷新确认 head/state 不变 |
| S3 | 17 项仿真、SDK、检索与其他 PR | 2026-11-04 02:35:33；规则 02:42:00 | `evidence.archives`；Genesis 旧 head 与 behind 结论已由本轮已发布动作及后续核对覆盖 |
| S4 | 其余 40 项正式 reviews 与 14 仓权限 | 2026-11-04 02:55:33 | `evidence.remainingReviews`；40 项均返回正式 reviews 空列表，14 仓均 pull-only |
| S5 | 其余 40 项、9 项 craft-anything、2 项 fork 的 head/CI | 汇总于 2026-11-04 03:02:07 | `refreshedPullRequests`；同一 PR 以这些刷新记录覆盖较早的 head/CI |
| S6 | 原 30 项 craft-anything 改进及 #722 整合结果 | 2026-11-03 18:50:59 | [原批次证据 JSON](20261104-merge-readiness.json)；仅用于已确认的历史收录与署名，不用旧“待 CI”覆盖新结果 |
| U | 新增合并、#766/#767 完整 CI 与最终失败、独立 R2.2 修复 | #766 merged 03:30:58；账号总数 04:01:42 | #766 c044adf1 完整 CI 成功并 merged；#767 cd468aca 完整 CI 成功，最新1d5bd792的CI41767797058为failure；独立dac9261a尚待执行验证 |

| S7 | 完整 252 项清单及后续 delta，新增 fork 验证后共253项 | 完整清单 03:28:25；delta 04:00:06；总数复核 04:01:42 | `inventorySources`；完整分页结合已核实 merged_at，新增fork#3单列。日期搜索漏列的geoopt#258/#259已逐项核实 |
| S8 | Robotics Toolbox 3 项与 Genesis 3 项最终候选刷新 | 2026-11-04 03:38:07 | `evidence.finalPriorityRefresh`；robotics 三项的人工批准仍对应当前 head，25 success/2 skipped；Genesis 当前 12 runs 仍 action_required |
| S9 | #767最终失败、单作业重跑权限、独立夹具修复 | 失败证据03:49:19；分支与fork检查04:00:06 | `evidence.latest767Failure` 与 `evidence.modelFixtureCorrection`；源码比较和实际执行结果分别记录 |

表内 `clean`、`blocked`、`unstable`、`dirty`、`behind` 是 GitHub 的合并状态字段。**clean 只说明该快照的合并状态，不能证明测试执行、审查批准或实际合并；未见 CI 表示没有读到对应记录，不能写成 passed。** `action_required` 表示工作流仍需要授权动作，不按测试执行失败解读。未能取得完整规则的仓库，仅记录 blocked，不猜测具体必需条件。

成功结论对应表中明确的 head 或注明的先前源码 head。文档提交后即使源码 blob 未变，也不会把先前 run 冒充为新提交的 run。个人测试反馈、自动评审意见、正式人工 APPROVED 以及维护者实际 merged 分别记录。

完整原始证据保存在 [账号审计 JSON](20261104-account-merge-audit.json)，可据仓库、PR 号、完整 SHA、workflow run ID 和来源时间复核。本轮未发送维护者催办消息、review 请求或代替维护者解决审查线程。

