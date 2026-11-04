# Afloat16 开源贡献与合并推进报告

**核对截止：2026-11-04 16:08:39（北京时间，UTC+8）。** 统计范围继续沿用 2026-10-28 08:00:00（北京时间）起有更新的 PR；本轮完整 open/closed 分页形成当前清单，各项 CI 以对应提交及运行记录为准，具体观察时间分别保存。

本轮已创建并验证 mlxtend #1274，完成 craft-anything #767 与 #716–#718 的主线同步、必要修复和源码回归验证，解决 Genesis 的 7 条已完成要求的审查，并关闭 5 个已完成使命或被替代的提案。完整清单仍为 254 项，116 个开放项全部属于上游；截至北京时间 16:05 的账号增量核对，自本轮 12:01:42 基线起尚无新增直接 merged。#716、#717 和 #767 的最终提交通过完整 CI；#718 的源码通过，但最终 CI 在未改动的 Cutter 测试处失败，需要维护者重跑，或先合并已验证的 #767 后同步检查。

## 1. 当前计数

| 范围 | 总数 | Open | Closed | 窗口内直接 merged | 窗口前 merged | 关闭未 merged |
|---|---|---|---|---|---|---|
| 全部近期清单 | 254 | 116 | 138 | 51 | 1 | 86 |
| 上游 PR | 250 | 116 | 134 | 51 | 1 | 82 |
| Fork 验证 PR | 4 | 0 | 4 | 0 | 0 | 4 |

直接 merged 以 GitHub 的 mergedAt 为准。整合收录、部分测试收录、关闭重复提案及 fork 验证均分别记录。该窗口清单不代表账号全部历史贡献。

## 2. 本轮已完成的推进

| 项目 | 当前提交 | 实际改动或操作 | 验证与状态 |
|---|---|---|---|
| [storytold/craft-anything #767](https://github.com/storytold/craft-anything/pull/767) | `30248fdb` | 正常同步主线，保留 loopback/取消测试修复并纳入上游 #787 的进程启动修复。 PR 描述已替换过期验证说明，链接当前 head 的实际成功运行。 | 当前 30248fdb 的完整 CI 41785798578 成功；Gate、锁文件检查及相关回归均通过。旧 ETXTBSY 失败不再是当前提交结果。 当前机器人复审暂停，CodeRabbit 检查 skipped；不等同于本 head 已重新获得机器人审查。 |
| [storytold/craft-anything #716](https://github.com/storytold/craft-anything/pull/716) | `ae1e2237` | 完成主线同步和两项有效评审修复：按 HTTP 语义比较头字段，并在容量已满时允许可用响应替换同一请求的不可用 304；保留原五项回归，新增九项边界回归。仅采用 CI 给出的四处格式调整，保留断言及测试语义。 | 最终 ae1e2237 的完整 CI 41795319675 成功；原 5 项和新 9 项回归、完整门禁均通过，checkout tree 与分支一致。原 inline 线程已自动解决；未将机器人复审写成人工批准。 |
| [storytold/craft-anything #717](https://github.com/storytold/craft-anything/pull/717) | `bfe15dd9` | 完成主线同步；随后修复聚合正文预算在去重前计费的问题，保留拒绝样本和 304 语义，新增六项针对性回归。 完成ROADMAP/STATUS及PR描述，链接最终提交的实跑结果。 | 最终 bfe15dd9 的完整 CI41790800135 成功；原4项和新6项回归、完整门禁均通过，checkout tree与分支一致；原线程已自动解决。 |
| [storytold/craft-anything #718](https://github.com/storytold/craft-anything/pull/718) | `c9360fd6` | 完成主线同步及临时文件修复：短固定前缀、排除临时名与目标同名、私有权限在创建时设置，保留公开文件的既有权限语义；补齐回归与完成状态文档。 | 最终 c9360fd6 的CI41793931423失败：未改动的Cutter测试出现BrokenPipe和unwrap(None)，本轮未运行到platform/atomic目标。源码27e5d631的完整CI41792023953成功，两个版本的证据分别保存。 |
| [rasbt/mlxtend #1274](https://github.com/rasbt/mlxtend/pull/1274) | `410abc2b` | 已创建上游 PR #1274，将 lint 环境固定到 Python 3.11；正文补充上游实跑结果，原预填入口已撤出待办。 | 当前 410abc2b 的上游 lint 41785648936 与 unit tests 41785648934 均成功；待维护者审阅和合并。 |

### 已完成修改的审查线程

| 项目 | 本轮 resolved | 剩余未解决 | 依据 |
|---|---|---|---|
| [Genesis-Embodied-AI/genesis-world #3649](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649) | 1 | 0 | 当前实现已按维护者要求移除单测试 precision 参数，使用会话 dtype 和相应容差；核对代码及原回复后由 Afloat16 resolved。 |
| [Genesis-Embodied-AI/genesis-world #3650](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650) | 1 | 0 | 当前补充已按维护者要求并入现有 test_geom_numpy_vs_torch_consistency；原有测试定义保留，未另设重复测试。核对当前差异和原线程后由 Afloat16 resolved。 |
| [Genesis-Embodied-AI/genesis-world #3651](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651) | 5 | 4 | 五条已完成要求逐项核对：近似归一化、倒数复用、测试归入现有用例、会话精度等；四条仍涉及基准/复制及与维护者意见相交的测试建议，保留未解决。 |

### 已完成使命或被主线修复替代的提案

| 提案 | 关闭原因 | 对应上游记录 |
|---|---|---|
| [storytold/craft-anything #681](https://github.com/storytold/craft-anything/pull/681) | 对应发现线程夹具问题已由 #773 合并修复，关闭重复提案。 | [上游 #773（上一轮已合并）](https://github.com/storytold/craft-anything/pull/773) |
| [storytold/craft-anything #768](https://github.com/storytold/craft-anything/pull/768) | 相同清理夹具问题已由主线 #761 修复，关闭重复提案。 | [上游 #761](https://github.com/storytold/craft-anything/pull/761) |
| [Afloat16/craft-anything #2](https://github.com/Afloat16/craft-anything/pull/2) | 上游解析器跟进 #766 已合并，本 fork 验证完成使命。 | [上游 #766（上一轮已合并）](https://github.com/storytold/craft-anything/pull/766) |
| [Afloat16/craft-anything #3](https://github.com/Afloat16/craft-anything/pull/3) | 独立模型发现夹具提案已被维护者的上游进程启动修复 #787 替代，不再创建相同上游提案。 | [上游 #787；不计为 Afloat16 的直接合并](https://github.com/storytold/craft-anything/pull/787) |
| [Afloat16/mlxtend #1](https://github.com/Afloat16/mlxtend/pull/1) | 上游 #1274 已创建并通过两项工作流，关闭 fork 验证。 | [上游 #1274](https://github.com/rasbt/mlxtend/pull/1274) |

这些关闭操作保留分支和提交历史，不增加直接 merged 计数。

## 3. 合并优先项与剩余阻塞

| 项目 | 当前情况 | 下一步 |
|---|---|---|
| [petercorke/robotics-toolbox-python #908](https://github.com/petercorke/robotics-toolbox-python/pull/908) | 当前head的27checks中25success、2skipped，完整CI成功；15:06（北京时间）核验的petercorke正式批准及0未解决线程记录保留，head和PR更新时间未变。 | 维护者执行上游合并；当前 GitHub 连接显示 push/maintain/admin 均为 false。 |
| [petercorke/robotics-toolbox-python #909](https://github.com/petercorke/robotics-toolbox-python/pull/909) | 当前head的27checks中25success、2skipped，完整CI成功；15:06（北京时间）核验的petercorke正式批准及0未解决线程记录保留，head和PR更新时间未变。 | 维护者执行上游合并；当前 GitHub 连接显示 push/maintain/admin 均为 false。 |
| [petercorke/robotics-toolbox-python #910](https://github.com/petercorke/robotics-toolbox-python/pull/910) | 当前head的27checks中25success、2skipped，完整CI成功；15:06（北京时间）核验的petercorke正式批准及0未解决线程记录保留，head和PR更新时间未变。 | 维护者执行上游合并；当前 GitHub 连接显示 push/maintain/admin 均为 false。 |
| [storytold/craft-anything #767](https://github.com/storytold/craft-anything/pull/767) | 30248fdb 完整 CI 成功，原 loopback 审查线程已解决。 | 维护者优先合并此已验证夹具修复，再同步#718；仍须在实际组合版本上验证。 |
| [rasbt/mlxtend #1274](https://github.com/rasbt/mlxtend/pull/1274) | 410abc2b上游lint与unit工作流成功，3checks成功；原审查记录无正式批准或未解决线程。 | 维护者审阅并合并环境固定修复，再处理原 #1267–#1269 的环境相关 lint 验证。 |
| [geomstats/geomstats #2147](https://github.com/geomstats/geomstats/pull/2147) | fd6e11e2全部14checks成功，mergeable=true、blocked；原0未解决线程，尚无正式批准。 | 维护者完成审查并核对剩余上游合并门槛；不把 blocked 的具体原因自行推断为某一规则。 |
| [tile-ai/tilelang #3718](https://github.com/tile-ai/tilelang/pull/3718) | 当前0c6eca3a mergeable=true、clean；最新CI41040008322成功，原0未解决线程记录保留。同head的两次早期取消运行单独记录。 | 维护者审阅并在仓库条件满足后合并。 |
| [storytold/craft-anything #716](https://github.com/storytold/craft-anything/pull/716) | 最终 ae1e2237 的 CI 41795319675 成功；原 5 项、新 9 项回归及完整门禁通过，0 个未解决 inline 线程。 | 维护者先合并 #717，再同步 #716 并验证实际组合；独立 CI 成功不能替代组合验证。 |
| [storytold/craft-anything #717](https://github.com/storytold/craft-anything/pull/717) | bfe15dd9最终CI41790800135成功；原4项、新6项回归及完整门禁通过，原线程自动resolved。 | 维护者审阅后先合并#717，再处理#716的实际整合。 |
| [storytold/craft-anything #718](https://github.com/storytold/craft-anything/pull/718) | 最终 c9360fd6 的CI41793931423失败于未改动Cutter测试；atomic目标本轮未执行。源码27e5d631全门禁成功、4个inline线程均已解决。 | 维护者优先合并已绿#767再正常同步主线，或重跑失败job126453460296；之后核验最终head完整门禁。 |
| [Genesis-Embodied-AI/genesis-world #3649](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649) | 当前head的4个工作流仍action_required；已完成修改的线程已解决，原记录剩余0条未解决，尚无正式批准。 | 维护者批准工作流并完成 required review；审批前不能称验证通过。 |
| [Genesis-Embodied-AI/genesis-world #3650](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650) | 当前head的4个工作流仍action_required；已完成修改的线程已解决，原记录剩余0条未解决，尚无正式批准。 | 维护者批准工作流并完成 required review。 |
| [Genesis-Embodied-AI/genesis-world #3651](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651) | 当前head的4个工作流仍action_required；已完成修改的线程已解决，原记录剩余4条未解决，尚无正式批准。 | 维护者评估现有基准、复制依据和测试意见，处理剩余讨论并批准工作流。 |
| [rasbt/mlxtend #1267](https://github.com/rasbt/mlxtend/pull/1267) | 原功能 PR 的 unit tests 成功、lint 失败；日志定位到浮动 Python 版本引入的 SciPy/OpenBLAS 安装环境问题。 | 先由维护者审阅 #1274；该修复尚未合并，原 PR 不标记为全部通过。 |
| [rasbt/mlxtend #1268](https://github.com/rasbt/mlxtend/pull/1268) | 原功能 PR 的 unit tests 成功、lint 失败；日志定位到浮动 Python 版本引入的 SciPy/OpenBLAS 安装环境问题。 | 先由维护者审阅 #1274；该修复尚未合并，原 PR 不标记为全部通过。 |
| [rasbt/mlxtend #1269](https://github.com/rasbt/mlxtend/pull/1269) | 原功能 PR 的 unit tests 成功、lint 失败；日志定位到浮动 Python 版本引入的 SciPy/OpenBLAS 安装环境问题。 | 先由维护者审阅 #1274；该修复尚未合并，原 PR 不标记为全部通过。 |

### 两项回放修复的合并顺序

建议先合并 [#717](https://github.com/storytold/craft-anything/pull/717)，再将 [#716](https://github.com/storytold/craft-anything/pull/716) 同步到实际接受的主线。两者改动重叠于样本比较、正文计费及容量处理；整合时应保留完整元数据比较，并确保重复样本只计费一次，预算拒绝后仍保留不可用 304 的证据。各分支单独通过 CI 不能代替组合后的验证。实际组合版本应运行四个回归目标中的 24 项现有测试及完整门禁。精确冲突位置与计费约束保存在 JSON 的 integrationAudit 中。

后续已调整文档条目的位置：#716 最终 ae1e 的 ROADMAP/STATUS 与 #717 的实际三方合并检查通过；#718 最终 c936 的 STATUS 与 #717 的三方检查也通过，双方条目各保留一次。原审计中的文档插入冲突已消除；源码尚未组合，以上计费与容量约束仍需在实际合并时验证。

### 本轮新确认的直接合并

本轮截止时尚未检出新的上游直接 merged。上一轮确认的四项合并保留在下表，不重复归入本轮新增。

### 上一轮已确认的四项合并

| PR | 合并时间（北京时间） | 主线提交 |
|---|---|---|
| [storytold/craft-anything #773](https://github.com/storytold/craft-anything/pull/773) | 2026-11-04 11:05:53 | `686f3429` |
| [thedotmack/claude-mem #5341](https://github.com/thedotmack/claude-mem/pull/5341) | 2026-11-04 11:25:45 | `6ed60fbc` |
| [thedotmack/claude-mem #5340](https://github.com/thedotmack/claude-mem/pull/5340) | 2026-11-04 11:26:00 | `4f6ea065` |
| [storytold/craft-anything #766](https://github.com/storytold/craft-anything/pull/766) | 2026-11-04 11:30:58 | `eab6a0f0` |

## 4. 原始 30 项改进的接纳情况

其中 27 项已由维护者通过 [#722](https://github.com/storytold/craft-anything/pull/722) 整合进入主线，提交 [`b9ca0999`](https://github.com/storytold/craft-anything/commit/b9ca099903aeef5d4b33a7e83e8e7062ab94c65a) 保留 Afloat16 对应的共同作者署名。整合 CI 中新增回归目标合计 103 passed、0 failed、0 ignored。完整逐项映射见[原始合并证据](20261104-merge-readiness.json)。

另外三项 #716、#717、#718 的最新提交、CI 和审查进展已列在本报告第二节。原 27 个 PR 自身为 closed 未直接 merged；整合收录与直接 merged 分开计数。

claude-mem #5339 的五类拒绝用例另由 [#5336](https://github.com/thedotmack/claude-mem/pull/5336) 部分收录，提交 [`de134d3b`](https://github.com/thedotmack/claude-mem/commit/de134d3be64d2b835b4f8fe933b53fa5fb983198) 保留共同作者署名。范围仅为这五类用例，[维护者确认](https://github.com/thedotmack/claude-mem/pull/5339#issuecomment-6621608877)不能解释为原 PR 全部代码已合并。

## 5. 全部开放清单

下表给出完整窗口清单的当前 PR 状态。已刷新检查的精确 head、运行和审查证据在 JSON 中；其余项沿用基线记录及本轮变化查询，不将未运行或需批准的工作流记为通过。

### AgriciDaniel/claude-seo：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#403](https://github.com/AgriciDaniel/claude-seo/pull/403) | fix(drift): preserve IPv6 authorities in normalized URLs | 2026-11-03 23:11:26 | 见 PR |
| [#404](https://github.com/AgriciDaniel/claude-seo/pull/404) | fix(drift): recognize none as a noindex directive | 2026-11-03 23:11:29 | 见 PR |
| [#405](https://github.com/AgriciDaniel/claude-seo/pull/405) | fix(metadata): match brand suffixes at word boundaries | 2026-11-03 23:11:31 | 见 PR |

### anthropics/anthropic-sdk-go：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#477](https://github.com/anthropics/anthropic-sdk-go/pull/477) | fix(middleware): preserve serving-hop usage iterations | 2026-11-03 14:41:04 | 见 PR |

### anthropics/anthropic-sdk-python：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#2012](https://github.com/anthropics/anthropic-sdk-python/pull/2012) | fix(middleware): preserve serving-hop usage iterations | 2026-11-03 14:40:51 | 见 PR |

### anthropics/anthropic-sdk-typescript：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1260](https://github.com/anthropics/anthropic-sdk-typescript/pull/1260) | fix(middleware): preserve serving-hop usage iterations | 2026-11-03 14:40:47 | 见 PR |

### ARISE-Initiative/robosuite：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#833](https://github.com/ARISE-Initiative/robosuite/pull/833) | Fix forward wrench transformation to preserve frame-invariant power | 2026-11-02 21:19:33 | 见 PR |

### colmap/colmap：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#4929](https://github.com/colmap/colmap/pull/4929) | Reuse adjacent-frame motion across overlapping TartanAir windows | 2026-10-31 11:43:05 | `8d7fc97a` |

### deepseek-ai/DeepEP-Ascend：4 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#19](https://github.com/deepseek-ai/DeepEP-Ascend/pull/19) | Fix communication stream reuse across NPU devices | 2026-11-02 20:43:29 | 见 PR |
| [#20](https://github.com/deepseek-ai/DeepEP-Ascend/pull/20) | Resolve remote workspace addresses by registration tag | 2026-11-02 20:43:32 | 见 PR |
| [#21](https://github.com/deepseek-ai/DeepEP-Ascend/pull/21) | Preserve the active Engram layout when configuration validation fails | 2026-11-02 20:43:36 | 见 PR |
| [#22](https://github.com/deepseek-ai/DeepEP-Ascend/pull/22) | Preserve NPU format metadata for externally allocated buffer tensors | 2026-11-02 20:43:40 | 见 PR |

### deepseek-ai/DeepGEMM-Ascend：4 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#22](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/22) | Accept packed UE8M0 scale factors in ACLNN GEMM | 2026-11-02 20:42:23 | 见 PR |
| [#23](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/23) | Fix the ACLNN BF16 accumulation destination | 2026-11-02 20:42:14 | 见 PR |
| [#24](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/24) | Preserve NPU storage metadata in MegaMoE buffers | 2026-11-02 20:42:29 | 见 PR |
| [#25](https://github.com/deepseek-ai/DeepGEMM-Ascend/pull/25) | Handle empty GEMMs before operand layout dispatch | 2026-11-02 20:42:33 | 见 PR |

### deepseek-ai/DeepInfer：5 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#258](https://github.com/deepseek-ai/DeepInfer/pull/258) | Fix repeated scalar paths in metadata broadcasts | 2026-11-02 20:43:44 | 见 PR |
| [#259](https://github.com/deepseek-ai/DeepInfer/pull/259) | Release BlockManager locks after rejected operations | 2026-11-02 20:43:48 | 见 PR |
| [#260](https://github.com/deepseek-ai/DeepInfer/pull/260) | Cancel queued grammar compilation during shutdown | 2026-11-02 20:43:52 | 见 PR |
| [#261](https://github.com/deepseek-ai/DeepInfer/pull/261) | Use monotonic time for multimodal worker deadlines | 2026-11-02 20:43:56 | 见 PR |
| [#262](https://github.com/deepseek-ai/DeepInfer/pull/262) | Wait for multimodal workers when shared pooling is disabled | 2026-11-02 20:44:04 | 见 PR |

### deepseek-ai/DeepJIT：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#22](https://github.com/deepseek-ai/DeepJIT/pull/22) | fix(cuda): preserve argv boundaries in compiler shell commands | 2026-11-03 14:35:59 | 见 PR |

### deepset-ai/haystack：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#13498](https://github.com/deepset-ai/haystack/pull/13498) | fix: count each source once in reciprocal rank fusion | 2026-11-03 15:48:23 | 见 PR |

### Eigenwise/atomic-agents：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#335](https://github.com/Eigenwise/atomic-agents/pull/335) | Resolve MCP schema references by their complete JSON Pointer | 2026-10-30 16:52:53 | `88683182` |
| [#336](https://github.com/Eigenwise/atomic-agents/pull/336) | Bound PDF page-range iteration by the document length | 2026-10-30 16:53:42 | `8d96fc07` |
| [#337](https://github.com/Eigenwise/atomic-agents/pull/337) | Close streamed PDF responses on download failures | 2026-10-30 16:55:30 | `ba0b578a` |

### Genesis-Embodied-AI/genesis-world：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#3649](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649) | [BUG FIX] Preserve quaternion interpolation gradients at coincident orientations. | 2026-11-04 10:52:11 | `fcd621b0` |
| [#3650](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650) | [BUG FIX] Return complete homogeneous transforms with reused output buffers. | 2026-11-04 10:52:15 | `f2d54cb8` |
| [#3651](https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651) | [BUG FIX] Preserve orientation when directions approach negative z. | 2026-11-04 10:52:18 | `0ce2464b` |

### geomstats/geomstats：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#2147](https://github.com/geomstats/geomstats/pull/2147) | Preserve batch axes in Brownian sample paths | 2026-11-04 11:05:54 | `fd6e11e2` |
| [#2148](https://github.com/geomstats/geomstats/pull/2148) | Assign fitted k-medoids labels from the final centers | 2026-10-30 17:11:58 | `6ad346de` |
| [#2149](https://github.com/geomstats/geomstats/pull/2149) | Center affine-invariant SPD lognormal samples at the supplied mean | 2026-10-30 18:27:51 | `9a40304e` |

### geoopt/geoopt：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#278](https://github.com/geoopt/geoopt/pull/278) | Support Euclidean norms with more than two manifold dimensions | 2026-10-30 17:05:02 | `6f54b572` |
| [#279](https://github.com/geoopt/geoopt/pull/279) | Preserve batch dimensions in UpperHalf origins | 2026-10-30 17:05:51 | `b4ac023c` |
| [#280](https://github.com/geoopt/geoopt/pull/280) | Preserve tangent vectors in canonical Stiefel projection | 2026-10-30 18:00:00 | `a3ea54f9` |

### google-deepmind/mujoco：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#3745](https://github.com/google-deepmind/mujoco/pull/3745) | Fix axis-aware zero handling and normalization in MJX | 2026-10-31 11:25:31 | `8ee1eecd` |
| [#3746](https://github.com/google-deepmind/mujoco/pull/3746) | Preserve quaternion integration derivatives at zero velocity | 2026-10-31 19:12:52 | `42fa9154` |
| [#3747](https://github.com/google-deepmind/mujoco/pull/3747) | Remove scale bias from MJX segment projection | 2026-10-31 19:13:06 | `edafc711` |

### graphdeco-inria/gaussian-splatting：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1350](https://github.com/graphdeco-inria/gaussian-splatting/pull/1350) | Preserve exposure parameters and optimizer state in checkpoints | 2026-10-31 11:15:21 | `f179a000` |
| [#1351](https://github.com/graphdeco-inria/gaussian-splatting/pull/1351) | Preserve per-image LPIPS distances for batched inputs | 2026-10-31 11:45:25 | `e2522e30` |
| [#1352](https://github.com/graphdeco-inria/gaussian-splatting/pull/1352) | Accept SIMPLE_PINHOLE cameras in COLMAP text models | 2026-10-31 19:20:51 | `069b35dc` |

### huggingface/lerobot：9 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#4850](https://github.com/huggingface/lerobot/pull/4850) | fix(gaussian_actor): preserve diagonal standard deviations | 2026-11-03 10:49:44 | 见 PR |
| [#5158](https://github.com/huggingface/lerobot/pull/5158) | fix(processor): preserve teleop input data during NumPy conversion | 2026-11-03 10:45:32 | 见 PR |
| [#5159](https://github.com/huggingface/lerobot/pull/5159) | fix(utils): preserve small rotation vectors | 2026-11-04 00:16:20 | 见 PR |
| [#5160](https://github.com/huggingface/lerobot/pull/5160) | fix(processor): convert NumPy views with negative strides | 2026-11-03 10:47:22 | 见 PR |
| [#5226](https://github.com/huggingface/lerobot/pull/5226) | fix(rollout): enforce ring-buffer frame and memory limits | 2026-11-03 21:59:05 | 见 PR |
| [#5227](https://github.com/huggingface/lerobot/pull/5227) | fix(rl): route mixer batches only to selected sources | 2026-11-03 21:58:15 | 见 PR |
| [#5228](https://github.com/huggingface/lerobot/pull/5228) | fix(rl): preserve truncation flags across replay dataset conversions | 2026-11-03 21:57:55 | 见 PR |
| [#5229](https://github.com/huggingface/lerobot/pull/5229) | fix(processor): flatten non-contiguous IsaacLab state components | 2026-11-03 21:58:29 | 见 PR |
| [#5230](https://github.com/huggingface/lerobot/pull/5230) | fix(features): merge vector specifications loaded from JSON | 2026-11-03 21:58:29 | 见 PR |

### isl-org/Open3D：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#7613](https://github.com/isl-org/Open3D/pull/7613) | Keep scalar material textures linear during Mitsuba conversion | 2026-10-31 19:52:22 | `8b80500e` |
| [#7614](https://github.com/isl-org/Open3D/pull/7614) | Finalize Mitsuba mesh parameters before returning the converted mesh | 2026-10-31 19:52:39 | `4be59082` |
| [#7615](https://github.com/isl-org/Open3D/pull/7615) | Preserve triangle-corner UV seams when exporting meshes to Mitsuba | 2026-10-31 19:52:51 | `d38a749f` |

### JetBrains/thinkrail：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1184](https://github.com/JetBrains/thinkrail/pull/1184) | fix(fs): identify SVG by its document root | 2026-11-03 23:15:18 | 见 PR |
| [#1185](https://github.com/JetBrains/thinkrail/pull/1185) | fix(fs): detect AVIF compatible brands within ftyp | 2026-11-03 23:14:28 | 见 PR |
| [#1186](https://github.com/JetBrains/thinkrail/pull/1186) | fix(terminal): preserve Unicode boundaries when truncating output | 2026-11-03 23:14:50 | 见 PR |

### labmlai/annotated_deep_learning_paper_implementations：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#325](https://github.com/labmlai/annotated_deep_learning_paper_implementations/pull/325) | Apply GroupNorm before DDPM attention projections | 2026-11-03 23:57:59 | 见 PR |

### leggedrobotics/rsl_rl：5 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#257](https://github.com/leggedrobotics/rsl_rl/pull/257) | Include action-range scaling in Beta entropy | 2026-10-30 17:09:09 | `59757058` |
| [#258](https://github.com/leggedrobotics/rsl_rl/pull/258) | Normalize vector rewards without ambiguous tensor conditions | 2026-10-30 17:15:36 | `7a5e7ece` |
| [#259](https://github.com/leggedrobotics/rsl_rl/pull/259) | Keep computed CNN padding nonnegative for small kernels | 2026-10-30 17:15:55 | `dff6d3f0` |
| [#263](https://github.com/leggedrobotics/rsl_rl/pull/263) | Fix bucketed reduction of noncontiguous gradients | 2026-11-03 21:54:49 | 见 PR |
| [#264](https://github.com/leggedrobotics/rsl_rl/pull/264) | Preserve recurrent hidden-state dtypes in rollout storage | 2026-11-03 21:54:51 | 见 PR |

### lucidrains/denoising-diffusion-pytorch：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#391](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/391) | Honor the clipping setting in simple diffusion | 2026-10-30 17:17:52 | `66383024` |
| [#392](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/392) | Respect the normalization setting for RePaint ground truth | 2026-10-30 17:20:03 | `bce1564a` |
| [#393](https://github.com/lucidrains/denoising-diffusion-pytorch/pull/393) | Keep RePaint resampling aligned with the current noise level | 2026-10-30 17:21:34 | `4d7159d1` |

### lucidrains/x-transformers：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#379](https://github.com/lucidrains/x-transformers/pull/379) | Apply custom belief-state pair weights with default direction weights | 2026-11-03 01:02:02 | 见 PR |

### mikedh/trimesh：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#2663](https://github.com/mikedh/trimesh/pull/2663) | Preserve voxel run-length operations and value types | 2026-11-03 21:43:54 | 见 PR |

### naver/roma：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#34](https://github.com/naver/roma/pull/34) | Rescale quaternions before normalization and inverse norms | 2026-11-03 10:50:31 | 见 PR |

### NVlabs/instant-ngp：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1616](https://github.com/NVlabs/instant-ngp/pull/1616) | Make antipodal camera alignment deterministic and orthogonal | 2026-10-31 11:37:11 | `7ed00ff8` |
| [#1617](https://github.com/NVlabs/instant-ngp/pull/1617) | Preserve empty observations and image names in COLMAP text imports | 2026-10-31 19:22:56 | `20e874e8` |
| [#1618](https://github.com/NVlabs/instant-ngp/pull/1618) | Fix scalar MtRSE reduction in image error evaluation | 2026-10-31 19:23:04 | `31ba787f` |

### OpenGVLab/InternVL：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1281](https://github.com/OpenGVLab/InternVL/pull/1281) | [Fix] Preserve stratified random video sampling at interval boundaries | 2026-10-31 11:17:16 | `c10c3cc4` |
| [#1282](https://github.com/OpenGVLab/InternVL/pull/1282) | [Fix] Decode only sampled frames from video folders | 2026-10-31 19:50:51 | `f54d6f8c` |
| [#1283](https://github.com/OpenGVLab/InternVL/pull/1283) | [Fix] Concatenate NumPy image patches in SFT and DPO collators | 2026-10-31 19:12:19 | `41f9f3a2` |

### OpenSQZ/MiniCPM-V-CookBook：2 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#143](https://github.com/OpenSQZ/MiniCPM-V-CookBook/pull/143) | fix(finetune): bound recovery from invalid official training samples | 2026-11-02 20:45:19 | 见 PR |
| [#144](https://github.com/OpenSQZ/MiniCPM-V-CookBook/pull/144) | fix(finetune): align official image features with placeholder order | 2026-11-02 20:45:19 | 见 PR |

### petercorke/robotics-toolbox-python：11 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#908](https://github.com/petercorke/robotics-toolbox-python/pull/908) | fix: reject mismatched operational acceleration trajectories | 2026-11-03 23:51:41 | `0265bbd7` |
| [#909](https://github.com/petercorke/robotics-toolbox-python/pull/909) | fix: stop quintic paths at their terminal time | 2026-11-03 23:55:16 | `87a68c08` |
| [#910](https://github.com/petercorke/robotics-toolbox-python/pull/910) | fix: plot trapezoidal trajectories with empty motion phases | 2026-11-03 23:56:43 | `8b0c46a1` |
| [#925](https://github.com/petercorke/robotics-toolbox-python/pull/925) | fix(build): allow pure wheels without a compiler | 2026-11-03 21:56:41 | 见 PR |
| [#926](https://github.com/petercorke/robotics-toolbox-python/pull/926) | fix(mobile): sample particles from weighted CDF intervals | 2026-11-03 21:57:01 | 见 PR |
| [#927](https://github.com/petercorke/robotics-toolbox-python/pull/927) | fix(mobile): resolve landmark indices in sensor Jacobians | 2026-11-03 21:56:51 | 见 PR |
| [#928](https://github.com/petercorke/robotics-toolbox-python/pull/928) | fix(mobile): clip occupancy updates to grid bounds | 2026-11-03 21:58:44 | 见 PR |
| [#929](https://github.com/petercorke/robotics-toolbox-python/pull/929) | fix(mobile): retain reverse motion in vehicle odometry | 2026-11-03 21:57:27 | 见 PR |
| [#930](https://github.com/petercorke/robotics-toolbox-python/pull/930) | fix(mobile): add opt-in four-connected path queries | 2026-11-04 00:10:07 | 见 PR |
| [#931](https://github.com/petercorke/robotics-toolbox-python/pull/931) | fix(mobile): reject PRM maps with no free cells | 2026-11-03 21:58:24 | 见 PR |
| [#932](https://github.com/petercorke/robotics-toolbox-python/pull/932) | fix(mobile): wrap curvature planner heading residuals | 2026-11-03 21:59:40 | 见 PR |

### Physical-Intelligence/openpi：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1077](https://github.com/Physical-Intelligence/openpi/pull/1077) | Validate action chunks before returning the first action | 2026-11-03 23:31:48 | 见 PR |
| [#1078](https://github.com/Physical-Intelligence/openpi/pull/1078) | Place policy WebSocket ports in the URL authority | 2026-11-01 07:35:21 | 见 PR |
| [#1079](https://github.com/Physical-Intelligence/openpi/pull/1079) | Unnormalize truncated feature vectors using matching statistics | 2026-11-01 07:35:14 | 见 PR |

### pypose/pypose：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#446](https://github.com/pypose/pypose/pull/446) | Keep neighbor indices aligned when radius-filtering point clouds | 2026-10-30 17:23:54 | `b8d364a6` |
| [#447](https://github.com/pypose/pypose/pull/447) | Compute L1 distance for reprojection sum reduction | 2026-10-30 17:26:28 | `582412fb` |
| [#448](https://github.com/pypose/pypose/pull/448) | Preserve point-cloud axes for empty and single-voxel inputs | 2026-10-30 17:31:10 | `fa9242dd` |

### QwenLM/qwen-code：1 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#13086](https://github.com/QwenLM/qwen-code/pull/13086) | fix(acp): add defensive legacy NDJSON cancellation cleanup | 2026-11-03 10:50:10 | 见 PR |

### rasbt/mlxtend：4 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1267](https://github.com/rasbt/mlxtend/pull/1267) | Preserve sample and feature axes in MeanCenterer | 2026-10-30 17:40:06 | `cd0489c7` |
| [#1268](https://github.com/rasbt/mlxtend/pull/1268) | Fix default ColumnSelector feature selection | 2026-10-30 17:42:40 | `0fb2e078` |
| [#1269](https://github.com/rasbt/mlxtend/pull/1269) | Preserve integer precision in paired permutation tests | 2026-10-30 17:57:30 | `fa4d9350` |
| [#1274](https://github.com/rasbt/mlxtend/pull/1274) | ci: pin lint environment to Python 3.11 | 2026-11-04 14:40:51 | `410abc2b` |

### recommenders-team/recommenders：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#2419](https://github.com/recommenders-team/recommenders/pull/2419) | Preserve input columns in stratified splitters | 2026-10-30 18:11:28 | `e6f66e06` |
| [#2420](https://github.com/recommenders-team/recommenders/pull/2420) | Prevent integer overflow in similarity normalization | 2026-10-30 18:13:07 | `3a720f77` |
| [#2421](https://github.com/recommenders-team/recommenders/pull/2421) | Preserve full precision in NumPy split ratios | 2026-10-30 18:14:15 | `6cad9dd6` |

### storytold/craft-anything：5 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#618](https://github.com/storytold/craft-anything/pull/618) | Keep unverified SSH agents out of automatic suggestions (R2.2) | 2026-11-04 00:00:44 | `be0094ad` |
| [#716](https://github.com/storytold/craft-anything/pull/716) | Preserve distinct response metadata in replay samples (R3.11) | 2026-11-04 16:08:36 | `ae1e2237` |
| [#717](https://github.com/storytold/craft-anything/pull/717) | Apply replay body limits after redaction (R3.11) | 2026-11-04 16:01:49 | `bfe15dd9` |
| [#718](https://github.com/storytold/craft-anything/pull/718) | Create atomic-write temporary files exclusively (R2.9) | 2026-11-04 16:02:44 | `c9360fd6` |
| [#767](https://github.com/storytold/craft-anything/pull/767) | Stabilize loopback request and cancellation fixtures (R3.11) | 2026-11-04 16:03:32 | `30248fdb` |

### TauricResearch/TradingAgents：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#1646](https://github.com/TauricResearch/TradingAgents/pull/1646) | fix(polymarket): reject non-array outcome values | 2026-11-03 23:11:32 | 见 PR |
| [#1647](https://github.com/TauricResearch/TradingAgents/pull/1647) | fix(polymarket): reject invalid implied probabilities | 2026-11-03 23:11:34 | 见 PR |
| [#1648](https://github.com/TauricResearch/TradingAgents/pull/1648) | fix(memory): isolate temporary files for log settlement | 2026-11-03 23:11:38 | 见 PR |

### tile-ai/tilelang：3 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#3716](https://github.com/tile-ai/tilelang/pull/3716) | [BugFix][Autotuner] Canonicalize target dictionary cache keys | 2026-10-30 18:21:56 | 见 PR |
| [#3717](https://github.com/tile-ai/tilelang/pull/3717) | [BugFix][Profiler] Resolve compound symbolic input shapes | 2026-11-02 15:02:02 | 见 PR |
| [#3718](https://github.com/tile-ai/tilelang/pull/3718) | [BugFix][NVCC] Honor target architectures in Tensor Core detection | 2026-10-30 18:26:41 | `0c6eca3a` |

### Toni-SM/skrl：4 项

| PR | 标题 | 最近 PR 更新（北京时间） | 已记录 head |
|---|---|---|---|
| [#465](https://github.com/Toni-SM/skrl/pull/465) | Restore the learned SAC entropy coefficient on resume | 2026-11-03 05:23:30 | 见 PR |
| [#488](https://github.com/Toni-SM/skrl/pull/488) | Handle clipping for unbounded action spaces | 2026-10-30 18:19:46 | 见 PR |
| [#489](https://github.com/Toni-SM/skrl/pull/489) | Reduce noncontiguous model gradients | 2026-10-30 18:20:55 | 见 PR |
| [#490](https://github.com/Toni-SM/skrl/pull/490) | Initialize linear layers in nested containers | 2026-10-30 18:22:20 | 见 PR |

## 6. 已关闭清单

### 窗口内直接 merged：51 项

| PR | 标题 | 合并或关闭时间（北京时间） | 范围 |
|---|---|---|---|
| [storytold/craft-anything #766](https://github.com/storytold/craft-anything/pull/766) | Address the parser review findings from #722 (R3.12) | 2026-11-04 11:30:58 | 上游 |
| [thedotmack/claude-mem #5340](https://github.com/thedotmack/claude-mem/pull/5340) | fix(providers): release capped error-body reader locks | 2026-11-04 11:26:00 | 上游 |
| [thedotmack/claude-mem #5341](https://github.com/thedotmack/claude-mem/pull/5341) | fix(worker): reject empty URL request modifiers | 2026-11-04 11:25:45 | 上游 |
| [storytold/craft-anything #773](https://github.com/storytold/craft-anything/pull/773) | Keep builder availability fixtures independent of live discovery (R2.9) | 2026-11-04 11:05:53 | 上游 |
| [storytold/craft-anything #673](https://github.com/storytold/craft-anything/pull/673) | Preserve empty query pairs in older stand-in recordings | 2026-11-04 00:16:53 | 上游 |
| [storytold/craft-anything #675](https://github.com/storytold/craft-anything/pull/675) | Handle generic GGUF paths and numeric task versions in chat discovery (R5.4) | 2026-11-04 00:14:26 | 上游 |
| [storytold/craft-anything #677](https://github.com/storytold/craft-anything/pull/677) | Verify stand-in session cleanup by answerer lifetime | 2026-11-04 00:14:19 | 上游 |
| [storytold/craft-anything #674](https://github.com/storytold/craft-anything/pull/674) | Exercise the unterminated DER OID guard | 2026-11-03 23:54:15 | 上游 |
| [kornia/kornia #6449](https://github.com/kornia/kornia/pull/6449) | fix(augmentation): register dispatcher sequences as child modules | 2026-11-03 23:11:30 | 上游 |
| [storytold/craft-anything #557](https://github.com/storytold/craft-anything/pull/557) | Stop unchanged rebuilds after two turns across runs (R5.3) | 2026-11-03 20:31:23 | 上游 |
| [storytold/craft-anything #552](https://github.com/storytold/craft-anything/pull/552) | Retry builder turns from typed failures, including stateless backends (R5.3) | 2026-11-03 16:51:05 | 上游 |
| [storytold/craft-anything #546](https://github.com/storytold/craft-anything/pull/546) | Help models correct rejected tool argument types (R5.4) | 2026-11-03 16:48:57 | 上游 |
| [storytold/craft-anything #554](https://github.com/storytold/craft-anything/pull/554) | Recover JSON tool calls emitted as text (R5.4) | 2026-11-03 16:39:40 | 上游 |
| [storytold/craft-anything #550](https://github.com/storytold/craft-anything/pull/550) | Keep ACP history replay out of new turns (R2.2) | 2026-11-03 16:01:32 | 上游 |
| [storytold/craft-anything #433](https://github.com/storytold/craft-anything/pull/433) | Test big-endian nanosecond pcap captures with FCS metadata (R0.5) | 2026-11-03 15:56:02 | 上游 |
| [storytold/craft-anything #559](https://github.com/storytold/craft-anything/pull/559) | Honor per-backend conversation context budgets (R5.4) | 2026-11-03 15:53:55 | 上游 |
| [storytold/craft-anything #544](https://github.com/storytold/craft-anything/pull/544) | Respect stop reasons before running native tool calls (R5.4) | 2026-11-03 15:50:11 | 上游 |
| [storytold/craft-anything #543](https://github.com/storytold/craft-anything/pull/543) | Report unavailable cached bodies in stand-in replay (R3.11) | 2026-11-03 15:50:01 | 上游 |
| [storytold/craft-anything #541](https://github.com/storytold/craft-anything/pull/541) | Wait for keep-awake cleanup before rebuild completion (R2.9) | 2026-11-03 15:49:56 | 上游 |
| [storytold/craft-anything #542](https://github.com/storytold/craft-anything/pull/542) | Show upstream provider reasons in API errors (R5.5) | 2026-11-03 15:49:51 | 上游 |
| [mikedh/trimesh #2618](https://github.com/mikedh/trimesh/pull/2618) | Fix nonuniform weighted Procrustes rotation | 2026-11-03 04:55:10 | 上游 |
| [storytold/craft-anything #439](https://github.com/storytold/craft-anything/pull/439) | Pinned builds keep runtime models in obj folders (R2.10) | 2026-11-03 01:16:58 | 上游 |
| [storytold/craft-anything #442](https://github.com/storytold/craft-anything/pull/442) | Build usage includes context-summary calls (R2.1) | 2026-11-03 00:48:58 | 上游 |
| [storytold/craft-anything #441](https://github.com/storytold/craft-anything/pull/441) | Compare WebAssembly exported tag parameter types (R0.5) | 2026-11-03 00:48:32 | 上游 |
| [storytold/craft-anything #440](https://github.com/storytold/craft-anything/pull/440) | App comparisons keep every unassigned window (R3.10) | 2026-11-03 00:48:06 | 上游 |
| [storytold/craft-anything #438](https://github.com/storytold/craft-anything/pull/438) | Minified esbuild modules keep their dependencies (R0.5) | 2026-11-03 00:42:50 | 上游 |
| [storytold/craft-anything #437](https://github.com/storytold/craft-anything/pull/437) | Find class usage in multidimensional object arrays (R0.5) | 2026-11-03 00:42:23 | 上游 |
| [storytold/craft-anything #436](https://github.com/storytold/craft-anything/pull/436) | Use ARM32 registers in value-flow graphs (R0.5) | 2026-11-03 00:41:57 | 上游 |
| [storytold/craft-anything #435](https://github.com/storytold/craft-anything/pull/435) | Bun text assets report their stored UTF-16LE encoding (R3.11) | 2026-11-03 00:41:30 | 上游 |
| [storytold/craft-anything #434](https://github.com/storytold/craft-anything/pull/434) | Files inside extended SquashFS directories are recovered (R0.5) | 2026-11-03 00:38:00 | 上游 |
| [huggingface/sentence-transformers #4114](https://github.com/huggingface/sentence-transformers/pull/4114) | Exclude unretrieved positives from observed RRF ranking metrics | 2026-11-02 18:54:28 | 上游 |
| [colmap/colmap #4931](https://github.com/colmap/colmap/pull/4931) | Resolve benchmark paths before changing subprocess directories | 2026-11-02 18:40:17 | 上游 |
| [colmap/colmap #4932](https://github.com/colmap/colmap/pull/4932) | Make benchmark translation-angle errors invariant to reconstruction scale | 2026-11-02 16:40:02 | 上游 |
| [huggingface/sentence-transformers #4301](https://github.com/huggingface/sentence-transformers/pull/4301) | fix: preserve padded sequence lengths in LSTM outputs | 2026-11-02 16:32:38 | 上游 |
| [Denys88/rl_games #394](https://github.com/Denys88/rl_games/pull/394) | Reject recurrent minibatches that split sequences | 2026-11-02 10:38:53 | 上游 |
| [Denys88/rl_games #393](https://github.com/Denys88/rl_games/pull/393) | Include the final occupied slot in prioritized replay sampling | 2026-11-02 10:38:43 | 上游 |
| [Denys88/rl_games #392](https://github.com/Denys88/rl_games/pull/392) | Support NumPy 2 when sampling scalar replay values | 2026-11-02 10:38:33 | 上游 |
| [serengil/deepface #1677](https://github.com/serengil/deepface/pull/1677) | Keep resized image dimensions positive | 2026-11-01 01:31:40 | 上游 |
| [serengil/deepface #1676](https://github.com/serengil/deepface/pull/1676) | Correct skipped-detector representation dimensions | 2026-11-01 01:31:11 | 上游 |
| [serengil/deepface #1675](https://github.com/serengil/deepface/pull/1675) | Preserve representation batch result structure | 2026-11-01 01:25:58 | 上游 |
| [KevinMusgrave/pytorch-metric-learning #756](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/756) | Cover HistogramLoss endpoints and interpolation in v3 | 2026-10-31 05:53:05 | 上游 |
| [KevinMusgrave/pytorch-metric-learning #759](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/759) | Add independent PNP value and gradient regressions | 2026-10-31 05:53:00 | 上游 |
| [KevinMusgrave/pytorch-metric-learning #758](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/758) | Preserve NT-Xent values and gradients under common score shifts | 2026-10-31 05:44:19 | 上游 |
| [kornia/kornia #6451](https://github.com/kornia/kornia/pull/6451) | fix(feature): avoid NaN orientation weight gradients on flat patches | 2026-10-31 03:09:03 | 上游 |
| [geoopt/geoopt #254](https://github.com/geoopt/geoopt/pull/254) | Initialize Riemannian SGD momentum from the first tangent gradient | 2026-10-29 19:55:47 | 上游 |
| [geoopt/geoopt #253](https://github.com/geoopt/geoopt/pull/253) | Fix signed hyperplane distances and gradients for negative inputs | 2026-10-29 19:54:41 | 上游 |
| [geoopt/geoopt #255](https://github.com/geoopt/geoopt/pull/255) | Avoid mutating parameter gradients during optimizer regularization | 2026-10-29 19:54:20 | 上游 |
| [geoopt/geoopt #256](https://github.com/geoopt/geoopt/pull/256) | Keep spherical exponential-map gradients finite at zero steps | 2026-10-29 19:53:57 | 上游 |
| [geoopt/geoopt #258](https://github.com/geoopt/geoopt/pull/258) | Retain spherical geodesic precision at small arcs and endpoints | 2026-10-29 16:31:46 | 上游 |
| [geoopt/geoopt #259](https://github.com/geoopt/geoopt/pull/259) | Keep sparse AMSGrad maxima in raw second-moment units | 2026-10-29 16:31:15 | 上游 |
| [Denys88/rl_games #384](https://github.com/Denys88/rl_games/pull/384) | Preserve the latest transitions when replay batches exceed capacity | 2026-10-29 09:57:39 | 上游 |

### 窗口前已 merged：1 项

| PR | 标题 | 合并或关闭时间（北京时间） | 范围 |
|---|---|---|---|
| [huggingface/lerobot #4835](https://github.com/huggingface/lerobot/pull/4835) | fix(vla_jepa): pair rotary frequencies with adjacent coordinates | 2026-10-22 22:00:39 | 上游 |

### Closed 未直接 merged：86 项

| PR | 标题 | 合并或关闭时间（北京时间） | 范围 |
|---|---|---|---|
| [storytold/craft-anything #768](https://github.com/storytold/craft-anything/pull/768) | Keep the pasted-picture cleanup test independent of filename reuse | 2026-11-04 14:35:06 | 上游 |
| [Afloat16/mlxtend #1](https://github.com/Afloat16/mlxtend/pull/1) | Validate Python 3.11 for the lint workflow | 2026-11-04 14:34:53 | Fork 验证 |
| [Afloat16/craft-anything #2](https://github.com/Afloat16/craft-anything/pull/2) | Validate the post-merge parser review follow-ups (R3.12) | 2026-11-04 14:34:28 | Fork 验证 |
| [storytold/craft-anything #681](https://github.com/storytold/craft-anything/pull/681) | Isolate builder discovery in the UI smoke test (R5.2) | 2026-11-04 14:32:01 | 上游 |
| [Afloat16/craft-anything #3](https://github.com/Afloat16/craft-anything/pull/3) | Validate the model-discovery shell fixture (R2.2) | 2026-11-04 14:21:40 | Fork 验证 |
| [thedotmack/claude-mem #5339](https://github.com/thedotmack/claude-mem/pull/5339) | fix(worker): accept decimal-string observation ids | 2026-11-04 11:19:06 | 上游 |
| [storytold/craft-anything #715](https://github.com/storytold/craft-anything/pull/715) | Keep TLS metadata within declared lengths | 2026-11-04 02:44:40 | 上游 |
| [storytold/craft-anything #714](https://github.com/storytold/craft-anything/pull/714) | Validate pcapng block framing | 2026-11-04 02:44:37 | 上游 |
| [storytold/craft-anything #713](https://github.com/storytold/craft-anything/pull/713) | Respect DNS-over-TCP frame lengths | 2026-11-04 02:44:34 | 上游 |
| [storytold/craft-anything #712](https://github.com/storytold/craft-anything/pull/712) | Reject invalid TCP header lengths | 2026-11-04 02:44:31 | 上游 |
| [storytold/craft-anything #711](https://github.com/storytold/craft-anything/pull/711) | Keep UDP payloads within declared datagram lengths | 2026-11-04 02:44:28 | 上游 |
| [storytold/craft-anything #710](https://github.com/storytold/craft-anything/pull/710) | Remove every duplicate JSON configuration key | 2026-11-04 02:44:25 | 上游 |
| [storytold/craft-anything #709](https://github.com/storytold/craft-anything/pull/709) | Keep comments when adding the first JSON configuration value | 2026-11-04 02:44:22 | 上游 |
| [storytold/craft-anything #708](https://github.com/storytold/craft-anything/pull/708) | Preserve JFFS2 sizes when recovery budgets are exhausted | 2026-11-04 02:44:19 | 上游 |
| [storytold/craft-anything #707](https://github.com/storytold/craft-anything/pull/707) | Keep PyInstaller modules within their PYZ archive | 2026-11-04 02:44:15 | 上游 |
| [storytold/craft-anything #706](https://github.com/storytold/craft-anything/pull/706) | Distinguish missing ext data from sparse holes | 2026-11-04 02:44:11 | 上游 |
| [storytold/craft-anything #705](https://github.com/storytold/craft-anything/pull/705) | Preserve sparse ext positions across indirect levels | 2026-11-04 02:44:08 | 上游 |
| [storytold/craft-anything #704](https://github.com/storytold/craft-anything/pull/704) | Report incomplete cramfs block output | 2026-11-04 02:44:04 | 上游 |
| [storytold/craft-anything #703](https://github.com/storytold/craft-anything/pull/703) | Traverse ROMFS dot and empty root directories | 2026-11-04 02:44:00 | 上游 |
| [storytold/craft-anything #702](https://github.com/storytold/craft-anything/pull/702) | Validate ASAR header framing | 2026-11-04 02:43:57 | 上游 |
| [storytold/craft-anything #701](https://github.com/storytold/craft-anything/pull/701) | Keep ZIP64 values within their extra field | 2026-11-04 02:43:53 | 上游 |
| [storytold/craft-anything #700](https://github.com/storytold/craft-anything/pull/700) | Check ZIP member sizes after decompression | 2026-11-04 02:43:50 | 上游 |
| [storytold/craft-anything #699](https://github.com/storytold/craft-anything/pull/699) | Apply ZIP output limits to stored entries | 2026-11-04 02:43:46 | 上游 |
| [storytold/craft-anything #698](https://github.com/storytold/craft-anything/pull/698) | Keep Android string pools within declared chunks | 2026-11-04 02:43:43 | 上游 |
| [storytold/craft-anything #697](https://github.com/storytold/craft-anything/pull/697) | Stop repeated SQLite overflow pages from fabricating data | 2026-11-04 02:43:39 | 上游 |
| [storytold/craft-anything #696](https://github.com/storytold/craft-anything/pull/696) | Keep SQLite cell reads within their page | 2026-11-04 02:43:36 | 上游 |
| [storytold/craft-anything #695](https://github.com/storytold/craft-anything/pull/695) | Read SQLite text using the database encoding | 2026-11-04 02:43:32 | 上游 |
| [storytold/craft-anything #694](https://github.com/storytold/craft-anything/pull/694) | Reject invalid DEX array payload element widths | 2026-11-04 02:43:29 | 上游 |
| [storytold/craft-anything #693](https://github.com/storytold/craft-anything/pull/693) | Reject impossible DEX invocation register lists | 2026-11-04 02:43:26 | 上游 |
| [storytold/craft-anything #692](https://github.com/storytold/craft-anything/pull/692) | Validate Java invocation operands before decoding calls | 2026-11-04 02:43:23 | 上游 |
| [storytold/craft-anything #691](https://github.com/storytold/craft-anything/pull/691) | Reject invalid widened Java instructions | 2026-11-04 02:43:20 | 上游 |
| [storytold/craft-anything #690](https://github.com/storytold/craft-anything/pull/690) | Preserve valid text around damaged modified UTF-8 | 2026-11-04 02:43:16 | 上游 |
| [storytold/craft-anything #688](https://github.com/storytold/craft-anything/pull/688) | Reject signed LEB128 values outside the 64-bit range | 2026-11-04 02:43:12 | 上游 |
| [storytold/craft-anything #676](https://github.com/storytold/craft-anything/pull/676) | Bound CPIO checksum work across firmware probes (R3.12) | 2026-11-04 01:33:13 | 上游 |
| [Afloat16/craft-anything #1](https://github.com/Afloat16/craft-anything/pull/1) | Run the existing quality gates for the signed LEB128 correction | 2026-11-04 01:32:14 | Fork 验证 |
| [mikedh/trimesh #2668](https://github.com/mikedh/trimesh/pull/2668) | Fix integer overflow when unitizing vectors | 2026-11-03 21:54:56 | 上游 |
| [mikedh/trimesh #2667](https://github.com/mikedh/trimesh/pull/2667) | Fix point-cloud major axis using centered principal direction | 2026-11-03 21:54:53 | 上游 |
| [mikedh/trimesh #2666](https://github.com/mikedh/trimesh/pull/2666) | Support 2D triangles in cross-product barycentric coordinates | 2026-11-03 21:54:51 | 上游 |
| [mikedh/trimesh #2665](https://github.com/mikedh/trimesh/pull/2665) | Bound normal-estimation neighborhoods by the point count | 2026-11-03 21:54:47 | 上游 |
| [mikedh/trimesh #2664](https://github.com/mikedh/trimesh/pull/2664) | Broadcast interval operands before selecting their overlap | 2026-11-03 21:54:37 | 上游 |
| [storytold/craft-anything #642](https://github.com/storytold/craft-anything/pull/642) | Keep window comparison labels distinct without renumbering occurrences | 2026-11-03 21:49:24 | 上游 |
| [storytold/craft-anything #641](https://github.com/storytold/craft-anything/pull/641) | Keep esbuild dependencies after unrelated calls | 2026-11-03 21:49:21 | 上游 |
| [storytold/craft-anything #640](https://github.com/storytold/craft-anything/pull/640) | Preserve value flow across conditional ARM32 calls | 2026-11-03 21:49:16 | 上游 |
| [storytold/craft-anything #621](https://github.com/storytold/craft-anything/pull/621) | Preserve interrupted requests when agent sessions cannot resume | 2026-11-03 21:41:28 | 上游 |
| [storytold/craft-anything #620](https://github.com/storytold/craft-anything/pull/620) | Bound multiline server-sent event data (R5.5) | 2026-11-03 21:41:25 | 上游 |
| [storytold/craft-anything #619](https://github.com/storytold/craft-anything/pull/619) | Avoid specialised-only models in automatic chat setup (R5.4) | 2026-11-03 21:41:22 | 上游 |
| [storytold/craft-anything #617](https://github.com/storytold/craft-anything/pull/617) | Find Antigravity under redirected Windows local app data (R2.2) | 2026-11-03 21:41:18 | 上游 |
| [storytold/craft-anything #616](https://github.com/storytold/craft-anything/pull/616) | Stop recommending Aider without a headless adapter (R2.2) | 2026-11-03 21:41:14 | 上游 |
| [storytold/craft-anything #615](https://github.com/storytold/craft-anything/pull/615) | Report damaged CRC-format cpio member contents (R3.12) | 2026-11-03 21:29:56 | 上游 |
| [storytold/craft-anything #613](https://github.com/storytold/craft-anything/pull/613) | Keep GNU tar timestamps out of recovered member names (R3.12) | 2026-11-03 21:29:53 | 上游 |
| [storytold/craft-anything #614](https://github.com/storytold/craft-anything/pull/614) | Keep PAX member sizes across intervening tar metadata records (R3.12) | 2026-11-03 21:29:52 | 上游 |
| [storytold/craft-anything #612](https://github.com/storytold/craft-anything/pull/612) | Reject malformed base64 tails instead of accepting a decoded prefix (R3.12) | 2026-11-03 21:29:46 | 上游 |
| [storytold/craft-anything #611](https://github.com/storytold/craft-anything/pull/611) | Report indexed source map columns in generated-file coordinates (R3.12) | 2026-11-03 21:29:44 | 上游 |
| [storytold/craft-anything #610](https://github.com/storytold/craft-anything/pull/610) | Decode percent escapes in inline source map data URLs (R3.12) | 2026-11-03 21:29:40 | 上游 |
| [storytold/craft-anything #609](https://github.com/storytold/craft-anything/pull/609) | Do not identify malformed object identifiers as valid algorithms (R3.12) | 2026-11-03 21:29:36 | 上游 |
| [storytold/craft-anything #608](https://github.com/storytold/craft-anything/pull/608) | Reject binary plist values whose widths cannot be represented (R3.12) | 2026-11-03 21:29:33 | 上游 |
| [storytold/craft-anything #607](https://github.com/storytold/craft-anything/pull/607) | Keep binary plist objects inside their declared data area (R3.12) | 2026-11-03 21:29:28 | 上游 |
| [storytold/craft-anything #606](https://github.com/storytold/craft-anything/pull/606) | Keep extracted string offsets inside the address space (R3.12) | 2026-11-03 21:29:24 | 上游 |
| [storytold/craft-anything #605](https://github.com/storytold/craft-anything/pull/605) | Keep non-date paths distinct in HAR API inventories (R3.13) | 2026-11-03 21:26:09 | 上游 |
| [storytold/craft-anything #604](https://github.com/storytold/craft-anything/pull/604) | Count decoded HAR body bytes before text conversion (R3.13) | 2026-11-03 21:26:06 | 上游 |
| [storytold/craft-anything #603](https://github.com/storytold/craft-anything/pull/603) | Redact declared secrets from retained HAR metadata (R3.13) | 2026-11-03 21:26:03 | 上游 |
| [storytold/craft-anything #602](https://github.com/storytold/craft-anything/pull/602) | Redact credentials with encoded query parameter names in HAR reports (R3.13) | 2026-11-03 21:25:59 | 上游 |
| [storytold/craft-anything #601](https://github.com/storytold/craft-anything/pull/601) | Bound preview connections and release its listening port on drop (R2.9) | 2026-11-03 21:07:50 | 上游 |
| [storytold/craft-anything #600](https://github.com/storytold/craft-anything/pull/600) | Serve preview files only to requests addressed to loopback (R2.9) | 2026-11-03 21:07:47 | 上游 |
| [storytold/craft-anything #598](https://github.com/storytold/craft-anything/pull/598) | Stream large preview files without loading them into memory (R2.9) | 2026-11-03 21:07:44 | 上游 |
| [storytold/craft-anything #597](https://github.com/storytold/craft-anything/pull/597) | Return headers only for missing preview files requested with HEAD (R2.9) | 2026-11-03 21:07:39 | 上游 |
| [storytold/craft-anything #596](https://github.com/storytold/craft-anything/pull/596) | Find script preloads in HTML relationship token lists (R3.13) | 2026-11-03 20:58:29 | 上游 |
| [storytold/craft-anything #595](https://github.com/storytold/craft-anything/pull/595) | Decode HTML asset URL references once (R3.13) | 2026-11-03 20:58:25 | 上游 |
| [storytold/craft-anything #594](https://github.com/storytold/craft-anything/pull/594) | Keep quoted HTML attribute values out of the script inventory (R3.13) | 2026-11-03 20:58:22 | 上游 |
| [storytold/craft-anything #593](https://github.com/storytold/craft-anything/pull/593) | Ignore script-shaped text inside HTML text elements (R3.13) | 2026-11-03 20:58:19 | 上游 |
| [storytold/craft-anything #592](https://github.com/storytold/craft-anything/pull/592) | Keep script text through lookalike closing tags (R3.13) | 2026-11-03 20:58:16 | 上游 |
| [storytold/craft-anything #591](https://github.com/storytold/craft-anything/pull/591) | Keep custom HTML tag names out of the script inventory (R3.13) | 2026-11-03 20:58:13 | 上游 |
| [storytold/craft-anything #599](https://github.com/storytold/craft-anything/pull/599) | Reject incomplete and oversized preview HTTP requests (R2.9) | 2026-11-03 20:57:04 | 上游 |
| [huggingface/sentence-transformers #4303](https://github.com/huggingface/sentence-transformers/pull/4303) | fix: register custom weighted layer pooling tensors | 2026-11-02 16:25:04 | 上游 |
| [huggingface/sentence-transformers #4136](https://github.com/huggingface/sentence-transformers/pull/4136) | Fix TSDAE reconstruction with left padding and shared EOS/PAD | 2026-11-02 16:14:50 | 上游 |
| [huggingface/sentence-transformers #4302](https://github.com/huggingface/sentence-transformers/pull/4302) | fix: honor empty document token allowlists | 2026-11-02 16:14:28 | 上游 |
| [OpenBMB/MiniCPM-V #1162](https://github.com/OpenBMB/MiniCPM-V/pull/1162) | fix(finetune): scope Qwen thinking masks to assistant headers | 2026-11-02 13:47:13 | 上游 |
| [OpenBMB/MiniCPM-V #1161](https://github.com/OpenBMB/MiniCPM-V/pull/1161) | fix(finetune): align visual features with image placeholder order | 2026-11-02 13:47:00 | 上游 |
| [OpenBMB/MiniCPM-V #1160](https://github.com/OpenBMB/MiniCPM-V/pull/1160) | fix(finetune): bound recovery from invalid training samples | 2026-11-02 13:46:43 | 上游 |
| [opendatalab/MinerU #5701](https://github.com/opendatalab/MinerU/pull/5701) | fix(ocr): preserve text crops extending outside image bounds | 2026-11-01 00:15:27 | 上游 |
| [opendatalab/MinerU #5702](https://github.com/opendatalab/MinerU/pull/5702) | fix(ocr): composite 16-bit transparent inputs without color overflow | 2026-11-01 00:15:12 | 上游 |
| [opendatalab/MinerU #5703](https://github.com/opendatalab/MinerU/pull/5703) | fix(vlm): preserve explicit server boolean flag overrides | 2026-11-01 00:15:02 | 上游 |
| [morluto/rea #3940](https://github.com/morluto/rea/pull/3940) | fix(javascript): preserve proven WebSocket endpoint identities | 2026-10-31 10:07:41 | 上游 |
| [KevinMusgrave/pytorch-metric-learning #755](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/755) | Detach persistent histogram updates from autograd | 2026-10-31 04:11:22 | 上游 |
| [KevinMusgrave/pytorch-metric-learning #757](https://github.com/KevinMusgrave/pytorch-metric-learning/pull/757) | Keep histogram momentum independent of batch size | 2026-10-31 04:11:17 | 上游 |
| [kornia/kornia #6450](https://github.com/kornia/kornia/pull/6450) | fix(losses): promote integer inputs before robust residual arithmetic | 2026-10-30 23:25:08 | 上游 |

## 7. 后续操作

本轮没有待用户提交的预填 PR 表单。mlxtend 的提案已创建；被上游 #787 替代的模型发现夹具不再列为待创建项。

| 上游操作 | 对应入口 |
|---|---|
| 合并已获当前 head 正式批准、CI 成功的 robotics-toolbox-python #908。 | https://github.com/petercorke/robotics-toolbox-python/pull/908 |
| 合并已获当前 head 正式批准、CI 成功的 robotics-toolbox-python #909。 | https://github.com/petercorke/robotics-toolbox-python/pull/909 |
| 合并已获当前 head 正式批准、CI 成功的 robotics-toolbox-python #910。 | https://github.com/petercorke/robotics-toolbox-python/pull/910 |
| 优先审阅并合并#767：30248fdb完整CI成功，包含对应Cutter夹具的读取修复；之后同步#718并验证实际组合，尚不保证该次失败的确切触发原因。 | https://github.com/storytold/craft-anything/pull/767 |
| 审阅并合并 #1274 的 Python 3.11 lint 环境固定；上游两项工作流均成功。 | https://github.com/rasbt/mlxtend/pull/1274 |
| 完成 #2147 正式审查并处理上游 remaining gate；当前 14 checks 已成功。 | https://github.com/geomstats/geomstats/pull/2147 |
| 审阅 #3718 并确认合并资格；最新完整 CI 已成功。 | https://github.com/tile-ai/tilelang/pull/3718 |
| 批准 #3649 的 4 项待批准 CI，并完成所需正式审查。 | https://github.com/Genesis-Embodied-AI/genesis-world/pull/3649 |
| 批准 #3650 的 4 项待批准 CI，并完成所需正式审查。 | https://github.com/Genesis-Embodied-AI/genesis-world/pull/3650 |
| 核对 #3651 剩余 4 个讨论线程、批准 4 项待批准 CI，并完成所需正式审查。 | https://github.com/Genesis-Embodied-AI/genesis-world/pull/3651 |
| 由维护者重跑#718失败job126453460296，或先合并#767后再正常同步和验证；当前账户没有上游重跑权限。 | https://github.com/storytold/craft-anything/actions/runs/41793931423 |
| 审阅并先合并 #717：最终 bfe15dd9 已通过完整 CI。 | https://github.com/storytold/craft-anything/pull/717 |
| 待 #717 实际合并后同步 #716，按已记录的源码整合约束运行 24 项现有回归及完整门禁，再完成合并。 | https://github.com/storytold/craft-anything/pull/716 |

## 8. 证据来源与范围

完整基线：[上一个固定版本](https://github.com/Afloat16/craft-anything/blob/ed7744df107eefe67ba343f8947fab81caef89d2/docs/contributions/20261104-account-merge-audit.json)；当前机器可读清单：[20261104-account-merge-audit.json](20261104-account-merge-audit.json)。本轮搜索截止、Git head、CI checkout/tree、线程状态及回读证据均在 JSON 中记录。

- 当前清单覆盖2026-10-28起有更新的254项PR，不代表账号全部历史贡献。15:40（北京时间）完成open/closed全四页读取，254个ID唯一，各页incomplete_results=false。
- Afloat16/mlxtend#1已额外直接GET确认closed且unmerged，本次完整closed分页亦包含该项；此前增量搜索遗漏不影响当前计数。
- 北京时间12:01:42之后没有新增直接merged。上一轮四项合并、五个本轮关闭操作、27项经#722整合收录及#5339部分测试收录分别保存，未重复计数。
- 上游#787为维护者的既有修复，不属于本轮Afloat16直接合并；被替代的独立夹具及全部旧预填操作已撤出待办。
- 本轮未重新执行全部254项CI。9项重点PR的head与检查数据重新读取；未变化的审查详情复用15:01–15:06（北京时间）快照并保留观察时间。
- #716 最终 ae1e2237 通过 41795319675，原 5 项和新 9 项回归全部通过；#717 最终 bfe15dd9 通过 41790800135，原 4 项和新 6 项全部通过。#718 源码 27e5d631 全门禁成功，但最终 c9360fd6 的 41793931423 失败，不能标记最终成功。
- #718最终失败位于未改动的cutter::tests::probe_reads_what_is_open，出现BrokenPipe和unwrap(None)，本轮atomic/platform目标及后续门禁未执行。主线、源码成功提交、最终提交的Cutter blob相同；日志未证明该次失败的确切触发时序，也不能据此认定atomic源码回归。
- Genesis共7条已满足要求的线程经代码核对后resolved；#3651仍保留4条讨论。每项PR的4个action_required工作流尚待维护者批准，不记为通过。
- robotics-toolbox-python三项保留正式批准记录及当前headCI成功；上游权限证据为push/maintain/admin均false，需要维护者完成合并。
- tilelang#3718已核对mergeable=true、clean；最新CI成功，同head的两次早期cancelled单独保留，不改写成success。
- clean/mergeable不代表CI通过或已合并；blocked不能在缺少规则证据时归因于某个具体门槛。#716与#717分别验证不代表组合实现已经验证。
- 机器人PR正文模板预检、DocstringCoverage提示、inline线程和CI是不同记录；#717/#718/#767观察到的coverage提示分别为14.29%/25%/0%。不将CI成功写成所有机器人警告清零。
- #767CodeRabbit复审暂停，检查skipped；当前head上游CI成功，不代表机器人已完成该head复审。
- #718的Actions菜单只有Create status badge，当前连接上游权限为pull-only；需维护者重跑。没有通过空提交、放宽断言或重复夹具修复触发新CI。
- 北京时间16:05的最后增量核对仅返回4个本轮编辑且仍open的craft PR；同期merged查询为0，两个查询incomplete_results均为false。
- 四项 craft PR 的标题已包含路线图 ID，正文模板已补齐并经 API 回读；相关说明审计的两项发现均已解决。这不等于额外的 Docstring Coverage 警告已消失，也不代表人工 reviewer 已批准。

## 9. 页面操作结果

已创建的上游 PR 及重复提案清理截图：

- [craft-anything #681：被主线修复替代后的关闭记录](evidence/craft-681-closed-20261104.jpg)
- [craft-anything #768：被主线修复替代后的关闭记录](evidence/craft-768-closed-20261104.jpg)
- [mlxtend #1274：上游 PR 创建及 CI 成功](evidence/mlxtend-1274-ci-passed-20261104.jpg)
- [craft-anything #767：模板、CI 与依赖说明已更新](evidence/craft-767-validation-updated-20261104.jpg)
- [craft-anything #717：最终成功验证及模板说明已更新](evidence/craft-717-validation-updated-20261104.jpg)
- [craft-anything #718：最终失败与源码成功范围已明确](evidence/craft-718-validation-updated-20261104.jpg)
- [craft-anything #718：失败运行及当前账户缺少重跑入口](evidence/craft-718-ci-blocked-20261104.jpg)
- [craft-anything #716：最终 CI 成功及模板说明已更新](evidence/craft-716-validation-updated-20261104.jpg)
