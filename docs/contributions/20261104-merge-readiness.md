# craft-anything：30 项改进的合并结果与后续修复

核对时间：**2026-11-03 18:50:59 UTC**。本文使用精确提交与 CI run 记录；后续状态可通过对应 PR 查看。

## 已实际进入主线的贡献

**原 30 项中的 27 项已进入 main。** 维护者于 **2026-11-03 18:43:07 UTC** 合并 [#722](https://github.com/storytold/craft-anything/pull/722)，主线提交为 [b9ca099](https://github.com/storytold/craft-anything/commit/b9ca099903aeef5d4b33a7e83e8e7062ab94c65a)。

维护者采用 squash 合并。#722 内的 27 个重放提交均由 GitHub 识别为 Afloat16 作者；主线最终提交的正文保留：

```text
Co-authored-by: Afloat <132809511+Afloat16@users.noreply.github.com>
```

因此，主线接纳的是 27 项实质改进，主线记录集中在一个带 Afloat16 共同作者署名的提交。原 #688、#690–#715 均已按维护者计划关闭，原 PR 的 merged 字段为 false；接纳路径是 #722，不能把它们统计成 27 次单独 PR 合并。27 个重放提交也不是 squash 后 main 的祖先。

## 尚待维护者合并的三项

三项当前均 open、mergeable=true、mergeable_state=clean，匹配当前 head 的完整 CI 全部成功。没有新的 inline review 或修改请求。

| PR | 改进 | 当前 head | 完整 CI |
|---|---|---|---|
| [#716](https://github.com/storytold/craft-anything/pull/716) | 保留响应元数据不同的回放样本 | [53a902a](https://github.com/Afloat16/craft-anything/commit/53a902a772d30e78951e2b8af5e301f4e1738052) | [通过](https://github.com/storytold/craft-anything/actions/runs/41693894074) |
| [#717](https://github.com/storytold/craft-anything/pull/717) | 按脱敏后的实际正文执行容量限制 | [f6c8648](https://github.com/Afloat16/craft-anything/commit/f6c86482cab1d888aacc8f932591585a7fd80a61) | [通过](https://github.com/storytold/craft-anything/actions/runs/41693880980) |
| [#718](https://github.com/storytold/craft-anything/pull/718) | 独占创建原子写入的临时文件 | [fb8a7df](https://github.com/Afloat16/craft-anything/commit/fb8a7df04af3ba1b84414a997b0b3bde00d3b5e3) | [通过](https://github.com/storytold/craft-anything/actions/runs/41693868110) |

当前连接没有上游合并权限。用户已授权的 fork 提交和分支操作已完成；剩余三项需要上游维护者执行合并。

## 原 30 项逐项记录

本轮原分支当前 head 的 CI 为 **29 通过、1 失败**。唯一失败是 #705 的共享 UI fixture，随后包含该 ext 改进的 #722 整合 CI 全部通过，故该改进已被接纳。

| 序号 | 原 PR | 改进 | 接纳状态 | 原分支当前 head 的 CI | #722 内作者提交 |
|---:|---|---|---|---|---|
| 1 | [#688](https://github.com/storytold/craft-anything/pull/688) | 拒绝超出 64 位范围的 DEX signed LEB128 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692721275) | [4a1fd2e](https://github.com/storytold/craft-anything/commit/4a1fd2e6577fd6faa12a0deb5daa458f291496d4) |
| 2 | [#690](https://github.com/storytold/craft-anything/pull/690) | 保留损坏 Modified UTF-8 周围的有效文字 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692739762) | [5509a4e](https://github.com/storytold/craft-anything/commit/5509a4ed3beca1ffdfb326c257931707d68cdf21) |
| 3 | [#691](https://github.com/storytold/craft-anything/pull/691) | 拒绝不合法的 Java wide 指令 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692753004) | [d7e508b](https://github.com/storytold/craft-anything/commit/d7e508b51f42ac51ae52c31453c7431134e0e050) |
| 4 | [#692](https://github.com/storytold/craft-anything/pull/692) | 校验 Java 调用指令的保留字节及参数计数 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692767850) | [be4c6dc](https://github.com/storytold/craft-anything/commit/be4c6dc527f029a6eb6bba60f0ba3646d78248c4) |
| 5 | [#693](https://github.com/storytold/craft-anything/pull/693) | 拒绝越界的 DEX 调用寄存器列表 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692778074) | [0872a25](https://github.com/storytold/craft-anything/commit/0872a25f352c1fdeb8489a198c63005579575c54) |
| 6 | [#694](https://github.com/storytold/craft-anything/pull/694) | 限制 DEX 数组载荷元素宽度 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692792746) | [a230f5a](https://github.com/storytold/craft-anything/commit/a230f5ac1a932eebab0df21f36ed4a8f44f7f765) |
| 7 | [#695](https://github.com/storytold/craft-anything/pull/695) | 按 SQLite 声明的编码读取文本 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692801601) | [6085a1c](https://github.com/storytold/craft-anything/commit/6085a1ce984868b627af8c087a3fece24c2b3acc) |
| 8 | [#696](https://github.com/storytold/craft-anything/pull/696) | 限制 SQLite 单元格读取范围 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692814058) | [fd4cb3f](https://github.com/storytold/craft-anything/commit/fd4cb3f71f23297a97e4b1183aa6f56d0706fd68) |
| 9 | [#697](https://github.com/storytold/craft-anything/pull/697) | 阻止 SQLite 溢出页循环伪造数据 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692830030) | [dedc0cd](https://github.com/storytold/craft-anything/commit/dedc0cdb497b0322b1306a48d3e3946c07dec493) |
| 10 | [#698](https://github.com/storytold/craft-anything/pull/698) | 限制 Android 字符串池读取边界 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692842389) | [2713f5d](https://github.com/storytold/craft-anything/commit/2713f5d7149e5cf536b0f28b695e5ce653d44ab8) |
| 11 | [#699](https://github.com/storytold/craft-anything/pull/699) | 对 ZIP 未压缩条目执行输出上限 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692159741) | [74ca075](https://github.com/storytold/craft-anything/commit/74ca0753c019b2e1edf7153306650f2d24e8f0b7) |
| 12 | [#700](https://github.com/storytold/craft-anything/pull/700) | 校验 ZIP 实际解压长度 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692181984) | [422e658](https://github.com/storytold/craft-anything/commit/422e65820a0adfdb6e4c39adaada811509880231) |
| 13 | [#701](https://github.com/storytold/craft-anything/pull/701) | 限制 ZIP64 扩展字段读取范围 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692195763) | [e178bb4](https://github.com/storytold/craft-anything/commit/e178bb447bb7e930556cfa0a7ad6928dc78c7c2d) |
| 14 | [#702](https://github.com/storytold/craft-anything/pull/702) | 校验 ASAR 嵌套头部长度 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692289599) | [e66c805](https://github.com/storytold/craft-anything/commit/e66c8051de95344e8236f4fa039c74283a47c1e2) |
| 15 | [#703](https://github.com/storytold/craft-anything/pull/703) | 正确遍历 ROMFS 点名称及空名称根目录 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692309447) | [9a3f12e](https://github.com/storytold/craft-anything/commit/9a3f12e0620b91b809fefba8af1d6a471c367d79) |
| 16 | [#704](https://github.com/storytold/craft-anything/pull/704) | 识别 cramfs 解压块不完整 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692327401) | [009313c](https://github.com/storytold/craft-anything/commit/009313c429931119d506229952c1f9995e087768) |
| 17 | [#705](https://github.com/storytold/craft-anything/pull/705) | 保留 ext 间接块中的稀疏文件位置 | 已进入 main（#722） | [失败：共享 UI fixture](https://github.com/storytold/craft-anything/actions/runs/41692378272) | [cacaa2a](https://github.com/storytold/craft-anything/commit/cacaa2aadee5aabcf09bcddc942f68ad70d7c94d) |
| 18 | [#706](https://github.com/storytold/craft-anything/pull/706) | 区分 ext 缺失数据与合法稀疏空洞 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692394230) | [03a2d7a](https://github.com/storytold/craft-anything/commit/03a2d7a3043dbfb9e6a006c1b6b44a2f5c46ed33) |
| 19 | [#707](https://github.com/storytold/craft-anything/pull/707) | 限制 PyInstaller PYZ 模块读取范围 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692411966) | [6cedc88](https://github.com/storytold/craft-anything/commit/6cedc883e9b7aa2a726f951bae84441bfd2797af) |
| 20 | [#708](https://github.com/storytold/craft-anything/pull/708) | 保留无法恢复内容的 JFFS2 文件大小 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41692427030) | [ddfd031](https://github.com/storytold/craft-anything/commit/ddfd031795e5cac30f221255565886e98eb8a430) |
| 21 | [#709](https://github.com/storytold/craft-anything/pull/709) | 在仅含注释的 JSON 配置中添加值时保留注释 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693676646) | [aa03055](https://github.com/storytold/craft-anything/commit/aa030550da0140635c54b993ea17975d6f132b07) |
| 22 | [#710](https://github.com/storytold/craft-anything/pull/710) | 移除 JSON 配置中的全部重复键 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693809704) | [9ff8ad7](https://github.com/storytold/craft-anything/commit/9ff8ad7bae13b6b16ec967df72068ed84a149343) |
| 23 | [#711](https://github.com/storytold/craft-anything/pull/711) | 按 UDP 声明长度截取载荷 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693826718) | [22c7434](https://github.com/storytold/craft-anything/commit/22c74345a2e591eabeea5362dbe552afdfc9fda1) |
| 24 | [#712](https://github.com/storytold/craft-anything/pull/712) | 拒绝无效 TCP 首部长度 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693841558) | [16c6e36](https://github.com/storytold/craft-anything/commit/16c6e364edf9dcff0d15e2e2eaaec8ba326eedc8) |
| 25 | [#713](https://github.com/storytold/craft-anything/pull/713) | 按 DNS over TCP 的帧长度解析消息 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693932774) | [9adb0cd](https://github.com/storytold/craft-anything/commit/9adb0cde94fc32da3f593aa9911c2a94c6973bcd) |
| 26 | [#714](https://github.com/storytold/craft-anything/pull/714) | 校验 pcapng 块尾长度及最小头部 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693917149) | [a955c77](https://github.com/storytold/craft-anything/commit/a955c7716ff4b893401222ffc29b5d046c9303a9) |
| 27 | [#715](https://github.com/storytold/craft-anything/pull/715) | 限制 TLS 记录、握手及扩展的读取范围 | 已进入 main（#722） | [通过](https://github.com/storytold/craft-anything/actions/runs/41693905229) | [1935667](https://github.com/storytold/craft-anything/commit/193566726b4516eb099b2e4e5c1b8dec0d3b0c14) |
| 28 | [#716](https://github.com/storytold/craft-anything/pull/716) | 保留响应元数据不同的回放样本 | 待维护者合并 | [通过](https://github.com/storytold/craft-anything/actions/runs/41693894074) | 独立 PR |
| 29 | [#717](https://github.com/storytold/craft-anything/pull/717) | 按脱敏后的实际正文执行容量限制 | 待维护者合并 | [通过](https://github.com/storytold/craft-anything/actions/runs/41693880980) | 独立 PR |
| 30 | [#718](https://github.com/storytold/craft-anything/pull/718) | 独占创建原子写入的临时文件 | 待维护者合并 | [通过](https://github.com/storytold/craft-anything/actions/runs/41693868110) | 独立 PR |

## 已完成的合并清障与验证

- 全部 30 个分支曾同步到固定主线 f5d1f27，并处理 ROADMAP / STATUS 文本冲突，保留原源码和回归测试。
- [Java 后续修复 e7cd6da](https://github.com/Afloat16/craft-anything/commit/e7cd6da59d2858dc9ab5e9fba1fcd2ceaaea434f) 使不完整解码产生 Partial 覆盖率，保留较早引用并继续其他方法。其[完整 CI](https://github.com/storytold/craft-anything/actions/runs/41688827573)通过，原评审线程已由评审者解决；这一后续改动未被原 #722 重放带入，因此现在纳入下面的补充提交。
- [SQLite 格式修复 8357dce](https://github.com/Afloat16/craft-anything/commit/8357dced53cb153764d503c8cee312499d457919) 解决 as_chunks 改动后的 rustfmt 失败，[完整 CI](https://github.com/storytold/craft-anything/actions/runs/41689288428)通过；该修复已在 #722 中保留。
- 对 SQLite 三项、ZIP 三项、ext 两项、JSON 两项、网络五项逐个重放原 patch，与 #722 组合文件全文比较一致；27 个原始回归文件也按 blob SHA 核对。
- [#722 的完整 CI](https://github.com/storytold/craft-anything/actions/runs/41693727323) 中，新增的 27 个测试目标实际运行 **103 个用例，全部通过，0 失败、0 忽略**。完整 workspace、fmt、严格 Clippy、wasm-plugins、五个共享 crate 的 wasm32 检查，以及 layers / locks / locales / secrets / tools / schemas 门禁通过。
- #716–#718 的实施版本和当前版本均有成功的完整 CI；实施版本的新增回归分别为 5、4、1 个，ROADMAP / STATUS 已记录实际验证范围。

## 已发布的四项评审后续修复

基线：已合并的 main [b9ca099](https://github.com/storytold/craft-anything/commit/b9ca099903aeef5d4b33a7e83e8e7062ab94c65a)。分支：`bugfix/r3.12-parser-review-followups-main-20261104`。最终 head：[fc5c886](https://github.com/Afloat16/craft-anything/commit/fc5c886aa1626b2e7b130964c3b56bccb7b9aaac)。

该分支只有 **5 个新提交：1 个任务认领、4 个独立修复**，GitHub 作者均为 Afloat16。相对 main 只改变 **9 个源码／测试文件和 2 个状态文档**，没有重复携带那 27 项已合并提交的历史。

| 修复 | 行为与回归范围 | 提交 |
|---|---|---|
| Java 引用扫描完整性 | 解码中断时保留已有结果、继续其他方法，并将覆盖率标记为部分；5 个相关测试，其中 2 个 class 级后续回归 | [63b8e16](https://github.com/Afloat16/craft-anything/commit/63b8e1668e1080f0d5d3de1150c37b4898c1731a) |
| DEX signed LEB128 | 按 DEX 规范限制为 signed 32 位、最多 5 字节；保留合法冗余符号扩展，7 个回归覆盖上下界、非法第五字节和实际 handler 路径 | [0872d3d](https://github.com/Afloat16/craft-anything/commit/0872d3d0811b5dbcbd45b8264debe7f71b2a5c9e) |
| 旧 AAPT UTF-8 字符串 | 在字符串区间内按低 15 位匹配长度寻找结束位置，保留截取上限；保留原 7 个回归并新增 7 个 | [c81e91a](https://github.com/Afloat16/craft-anything/commit/c81e91ab17fabe2e77ff7cfb26ad5aefb484005a) |
| ext 间接索引元数据 | 不可读的非零根或子节点产生未恢复文件；真正的零指针稀疏范围与 EOF 边界保留；新增 8 个回归 | [fc5c886](https://github.com/Afloat16/craft-anything/commit/fc5c886aa1626b2e7b130964c3b56bccb7b9aaac) |

DEX 范围依据：[Android DEX 格式规范](https://source.android.com/docs/core/runtime/dex-format#leb128)。AAPT 兼容规则依据：[AOSP ResourceTypes.cpp](https://android.googlesource.com/platform/frameworks/base/+/android-9.0.0_r12/libs/androidfw/ResourceTypes.cpp)。ext 修复只覆盖 classic indirect metadata；extent-tree 元数据范围保持不变，三层非零间接寻址仍明确标记未支持。

这四个目标合计 **34 个测试**，已经过独立源码与夹具复核。**新组合版本尚未执行 Rust 测试、fmt 或 Clippy。** 已成功创建 [fork 验证草稿 PR #2](https://github.com/Afloat16/craft-anything/pull/2)，截至 18:51:56 UTC 没有 Actions run 记录，未断言具体原因。

### 上游 PR 创建入口

上游创建／修改 PR 接口返回 403；fork 验证草稿已实际创建。使用以下已填写标题、正文和 main 目标的链接提交完整补充：

[创建四项评审后续修复的上游 PR](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-parser-review-followups-main-20261104?quick_pull=1&title=Address+the+parser+review+findings+from+%23722+%28R3.12%29&body=%2A%2ARoadmap+item%3A%2A%2A+R3.12+%E2%80%94+follow-ups+to+merged+%23722.%0A%0A%2A%2AWhat+changed%3A%2A%2A%0A%0A-+Java+reference+searches+keep+earlier+hits%2C+scan+other+methods+and+report+partial+coverage+after+decoder+rejection.%0A-+DEX+signed+LEB128+accepts+signed+32-bit+values+in+at+most+five+bytes%2C+including+valid+padding.%0A-+Legacy+AAPT+UTF-8+lengths+recover+within+the+string+region%2C+with+the+existing+output+cap.%0A-+Missing+classic+ext+indirect+metadata+produces+an+unrecovered+file%3B+true+sparse+ranges+and+EOF+boundaries+remain+intact.%0A%0A%2A%2AVerified%3A%2A%2A%0A%0AIndependent+source+and+fixture+review%3B+the+diff+against+main+b9ca099+contains+nine+source%2Ftest+files+and+ROADMAP%2FSTATUS.+The+four+regression+targets+contain+34+tests.+The+original+Java+fix+passed+full+CI%3A+https%3A%2F%2Fgithub.com%2Fstorytold%2Fcraft-anything%2Factions%2Fruns%2F41688827573.%0A%0A%2A%2ANot+verified%3A%2A%2A%0A%0AThis+new+combined+revision+still+awaits+formatting%2C+Clippy%2C+tests+and+repository+gates.+Fork+validation+draft%3A+https%3A%2F%2Fgithub.com%2FAfloat16%2Fcraft-anything%2Fpull%2F2.%0A%0A-+%5B+%5D+Formatting%2C+Clippy%2C+tests%2C+layers%2C+locks%2C+secrets+and+shared+WASM+checks.%0A-+%5B+%5D+Complete+ROADMAP+validation+after+CI+passes%3B+STATUS+records+the+pending+result.%0A-+%5Bx%5D+Original+implementation%3B+no+other+project%27s+code%2C+scripts+or+wording+copied.)

此链接已包含较早准备的 Java 单项补丁。它替代之前指向 #722 工作分支的 Java 单项预填链接；#722 已合并，后续应以 main 为目标。

## 另备的两项共享测试稳定性修复

它们是独立的测试夹具修复，不计入原 30 项。两项均已提交 fork；尚未创建上游 PR，尚未运行新版本测试。

### Keep paste cleanup tests independent of clock resolution (R2.9)

把模拟旧图片同时改为历史文件名，避免同一毫秒内新保存合法复用已删除文件名而触发错误断言。保留过期清理、附件保留和新保存断言。

[已提交代码](https://github.com/Afloat16/craft-anything/commit/fb1e2fde6b9eabc67b68c6c4e3ad4ba7a666b82c) · [填写完成的 PR 创建页](https://github.com/storytold/craft-anything/compare/main...Afloat16:test/r2.9-paste-cleanup-fixture-20261104?quick_pull=1&title=Keep+paste+cleanup+tests+independent+of+clock+resolution+%28R2.9%29&body=%2A%2ARoadmap+item%3A%2A%2A+R2.9+%E2%80%94+make+the+pasted-picture+cleanup+regression+independent+of+same-millisecond+filename+reuse.%0A%0A%2A%2AWhat+changed%3A%2A%2A%0A%0AThe+fixture+makes+a+paste+old+by+changing+its+modification+time%2C+while+leaving+a+current-millisecond+filename.+Cleanup+can+remove+that+file+and+a+new+save+can+legitimately+reuse+its+path%2C+causing+the+removal+assertion+to+fail.+Give+the+old+paste+a+historical+filename+and+keep+the+age-based+removal%2C+attached-file+retention+and+new-save+assertions.%0A%0A%2A%2AVerified%3A%2A%2A%0A%0AThe+failure+in+%23702+was+inspected%3A+https%3A%2F%2Fgithub.com%2Fstorytold%2Fcraft-anything%2Factions%2Fruns%2F41687774538.+The+patch+changes+the+test+fixture+and+its+ROADMAP%2FSTATUS+records%3B+production+code+is+unchanged.%0A%0A%2A%2ANot+verified%3A%2A%2A%0A%0AThis+new+revision+has+not+run+tests%2C+formatting+or+Clippy+because+the+execution+environment+is+unavailable.+Repository+CI+is+still+required.%0A%0A-+%5B+%5D+Formatting%2C+Clippy%2C+affected-crate+tests%2C+layers%2C+locks+and+secrets.%0A-+%5B+%5D+Complete+the+ROADMAP+validation+item+after+CI+passes%3B+STATUS+records+the+pending+check.%0A-+%5Bx%5D+Original+implementation%3B+no+other+project%27s+code%2C+scripts+or+wording+copied.)

### Keep builder availability fixtures independent of live discovery (R2.9)

由测试显式控制 agent discovery 完成时机，避免真实后台发现结果覆盖模拟结果。保留提示、对话框、控件和构建可用性断言。

[已提交代码](https://github.com/Afloat16/craft-anything/commit/13a0bf438cc740a9db144cc3513eb44551e21aaf) · [填写完成的 PR 创建页](https://github.com/storytold/craft-anything/compare/main...Afloat16:test/r2.9-builder-discovery-fixture-20261104?quick_pull=1&title=Keep+builder+availability+fixtures+independent+of+live+discovery+%28R2.9%29&body=%2A%2ARoadmap+item%3A%2A%2A+R2.9+%E2%80%94+keep+the+builder-availability+smoke+regression+independent+of+live+agent+discovery.%0A%0A%2A%2AWhat+changed%3A%2A%2A%0A%0AOpening+the+builder+dialog+starts+real+discovery.+A+late+completion+can+replace+the+synthetic+results+before+the+test+checks+Start+availability.+Mark+discovery+as+already+in+progress+before+opening+the+dialog%2C+then+complete+it+with+the+existing+fixture.+Keep+the+warning%2C+dialog%2C+controls+and+build-availability+assertions.%0A%0A%2A%2AVerified%3A%2A%2A%0A%0AThe+failure+in+%23711+was+inspected%3A+https%3A%2F%2Fgithub.com%2Fstorytold%2Fcraft-anything%2Factions%2Fruns%2F41687928153.+Source+tracing+confirms+that+headless+setup+has+no+prior+discovery+worker+and+the+guard+prevents+starting+one.+The+patch+changes+only+this+test+fixture+and+ROADMAP%2FSTATUS+records.%0A%0A%2A%2ANot+verified%3A%2A%2A%0A%0AThis+new+revision+has+not+run+tests%2C+formatting+or+Clippy+because+the+execution+environment+is+unavailable.+Repository+CI+is+still+required.%0A%0A-+%5B+%5D+Formatting%2C+Clippy%2C+affected-crate+tests%2C+layers%2C+locks+and+secrets.%0A-+%5B+%5D+Complete+the+ROADMAP+validation+item+after+CI+passes%3B+STATUS+records+the+pending+check.%0A-+%5Bx%5D+Original+implementation%3B+no+other+project%27s+code%2C+scripts+or+wording+copied.)

## 权限与验证范围

- fork 的分支、commit 和验证草稿 PR 已实际创建。
- 上游 PR 创建／元数据修改、Actions 手动重跑返回 403；当前账户在上游只有 pull 权限，没有执行合并权限。
- 本地执行环境不可用。所有“通过”都对应上面的实际 CI head；新后续代码没有借用旧版本的成功状态。
- 未代替维护者关闭评审线程、批准或合并 PR。#722 的合并和原 27 PR 的关闭均由维护者完成。

机器可读完整记录：[20261104-merge-readiness.json](./20261104-merge-readiness.json)。
