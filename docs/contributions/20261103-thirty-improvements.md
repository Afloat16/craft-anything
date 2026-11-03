# Thirty focused correctness improvements

Target repository: [storytold/craft-anything](https://github.com/storytold/craft-anything).

All 30 implementation branches below are published in Afloat16/craft-anything. Each branch contains its focused production change, regression fixture, ROADMAP entry and STATUS coverage description. GitHub links both the author and committer of every published head to Afloat16.

## Creating the upstream pull requests

The links below select upstream main and the corresponding fork branch, and prefill the title and full description. Click Create pull request in GitHub to submit. Upstream PR creation through the connected integration returned HTTP 403 (Resource not accessible by integration); no upstream PR number is claimed here.

## Validation record

- Implementation baseline: `b79fc506e2f119f9188edb791b900df5170e155b`.
- Items 1–27: individual affected-formats tests, strict Clippy and formatting completed successfully.
- Combined source/test tree for items 1–27: `450d06683b305748efc07415a05d0edd7e9ed374`; 563 tests passed, 0 ignored. Doctests, strict Clippy, formatting and whitespace checks also passed. This is a recorded local integration tree, not a published branch.
- Independent shared WASM checks confirmed for items 1–19 and 22. Checks for 20, 21 and 23–29 remain pending. Item 30 changes the native platform crate, outside that shared five-crate WASM check.
- Items 28–30: source, regression logic and published blob identities checked. Native tests, formatting, Clippy and other required execution gates remain pending because the execution environment became unavailable.
- The fork CI probe produced no Actions run or check suite and has been closed without merging. It is not one of the 30 proposed upstream changes.
- Upstream snapshot `87174476d9e8b0395155926f8329bf93a2c60e82`, checked at 2026-11-03 17:32:57 UTC, added two AI/CLI commits after the implementation baseline. Their source files, tests, dependency manifests and document edit locations do not overlap these changes. This was a file/hunk comparison, not an executed merge.
- The independent PRs edit common ROADMAP/STATUS sections, so applying them in sequence may require resolving documentation insertions. Their combined format code and tests were validated together as recorded above.

## Submission links

| # | Change | Commit | Prefilled pull request | Execution status |
|---|---|---|---|---|
| 1 | Reject signed LEB128 values outside the 64-bit range | [c6f25651](https://github.com/Afloat16/craft-anything/commit/c6f256518b2e16c91087211665beab4a78df3e96) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-dex-signed-leb128-20261103b?quick_pull=1&title=Reject+signed+LEB128+values+outside+the+64-bit+range&body=%23%23+Change%0A%0AReject+invalid+sign+extension+in+the+tenth+byte+so+malformed+exception+counts+cannot+become+valid+values.%0A%0A%23%23+Regression+coverage%0A%0ASigned-limit+and+overflowing+tenth-byte+regressions.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.%0A%0AScope%3A+the+current+signed+64-bit+LEB128+helper%3B+broader+DEX+32-bit+operand+rules+are+unchanged.) | Native + shared WASM passed |
| 2 | Preserve valid text around damaged modified UTF-8 | [7defc6c3](https://github.com/Afloat16/craft-anything/commit/7defc6c3583946aa1072ddb209734f2f25817c6b) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-modified-utf8-validation-20261103b?quick_pull=1&title=Preserve+valid+text+around+damaged+modified+UTF-8&body=%23%23+Change%0A%0AValidate+continuation+and+overlong+encodings+in+the+shared+DEX%2FJava+decoder+while+preserving+valid+following+text%2C+encoded+NUL+and+surrogate+pairs.%0A%0A%23%23+Regression+coverage%0A%0AMalformed+encodings+and+valid+surrounding+text+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 3 | Reject invalid widened Java instructions | [206c2804](https://github.com/Afloat16/craft-anything/commit/206c2804e54ad8639965203c6a16cff2732f3c8c) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-java-wide-opcodes-20261103b?quick_pull=1&title=Reject+invalid+widened+Java+instructions&body=%23%23+Change%0A%0AAllow+wide+only+for+the+defined+load%2C+store%2C+ret+and+iinc+instructions+so+malformed+bytecode+cannot+be+decoded+as+a+valid+widened+operation.%0A%0A%23%23+Regression+coverage%0A%0AAllowed+widened+operations+and+invalid+opcode+regressions.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 4 | Validate Java invocation operands before decoding calls | [63e3af8d](https://github.com/Afloat16/craft-anything/commit/63e3af8dbad23f0885590d3b7d7912440acf1d68) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-java-invoke-operands-20261103b?quick_pull=1&title=Validate+Java+invocation+operands+before+decoding+calls&body=%23%23+Change%0A%0ARequire+a+nonzero+invokeinterface+count+and+zero+reserved+operand+bytes+before+accepting+interface+or+dynamic+calls.%0A%0A%23%23+Regression+coverage%0A%0AInvalid+counts%2C+reserved+bytes+and+valid+invocation+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 5 | Reject impossible DEX invocation register lists | [6da3b104](https://github.com/Afloat16/craft-anything/commit/6da3b104817744d3baefad1a01413db6d3df2930) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-dex-invoke-registers-20261103b?quick_pull=1&title=Reject+impossible+DEX+invocation+register+lists&body=%23%23+Change%0A%0AReject+fixed+invocation+lists+larger+than+five+registers+and+range+operands+extending+beyond+the+register+index+space.%0A%0A%23%23+Regression+coverage%0A%0AOversized+fixed+lists%2C+overflowing+ranges+and+valid+boundaries.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 6 | Reject invalid DEX array payload element widths | [e4802ee7](https://github.com/Afloat16/craft-anything/commit/e4802ee7713eff91eb9a6e27a0459c0383f63761) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-dex-array-widths-20261103b?quick_pull=1&title=Reject+invalid+DEX+array+payload+element+widths&body=%23%23+Change%0A%0AAccept+only+1%2C+2%2C+4+or+8+byte+array+elements%2C+rejecting+unsupported+and+zero+widths+before+payload-size+calculations.%0A%0A%23%23+Regression+coverage%0A%0AInvalid+element+widths%2C+large+counts+and+valid+width+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 7 | Read SQLite text using the database encoding | [be78da06](https://github.com/Afloat16/craft-anything/commit/be78da063a6ebac4e8f399d8ef7da793981ec95b) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-sqlite-text-encoding-20261103b?quick_pull=1&title=Read+SQLite+text+using+the+database+encoding&body=%23%23+Change%0A%0AUse+the+database+text+encoding+for+schema+and+row+text+so+UTF-16+databases+are+decoded+correctly.%0A%0A%23%23+Regression+coverage%0A%0AUTF-8%2C+UTF-16LE+and+UTF-16BE+schema+and+row+fixtures.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 8 | Keep SQLite cell reads within their page | [1d2a3b0f](https://github.com/Afloat16/craft-anything/commit/1d2a3b0f812bc0cac6414d278142ad82a13b5bd7) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-sqlite-page-boundaries-20261103b?quick_pull=1&title=Keep+SQLite+cell+reads+within+their+page&body=%23%23+Change%0A%0AConstrain+cell+headers%2C+pointer+arrays+and+payload+reads+to+the+current+usable+page+instead+of+borrowing+bytes+from+a+neighboring+page.%0A%0A%23%23+Regression+coverage%0A%0AMalformed+page+pointers+and+truncated+page-local+data.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 9 | Stop repeated SQLite overflow pages from fabricating data | [bc148211](https://github.com/Afloat16/craft-anything/commit/bc1482115787d4fa0172c2901ac16c1479724dcb) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-sqlite-overflow-cycles-20261103b?quick_pull=1&title=Stop+repeated+SQLite+overflow+pages+from+fabricating+data&body=%23%23+Change%0A%0ATrack+visited+overflow+pages+and+stop+repeated+pages+while+retaining+only+the+actually+recovered+partial+payload.%0A%0A%23%23+Regression+coverage%0A%0ARepeated-page+and+cyclic+overflow-chain+regressions.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 10 | Keep Android string pools within declared chunks | [4c00f0e3](https://github.com/Afloat16/craft-anything/commit/4c00f0e3dca3fac29d95b85f53620ceed323ebe7) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-android-string-pool-bounds-20261103b?quick_pull=1&title=Keep+Android+string+pools+within+declared+chunks&body=%23%23+Change%0A%0AValidate+chunk+framing%2C+offsets%2C+string%2Fstyle+regions+and+terminators+before+decoding+string+pool+entries.%0A%0A%23%23+Regression+coverage%0A%0AOut-of-chunk+offsets%2C+malformed+regions+and+termination+checks.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 11 | Apply ZIP output limits to stored entries | [77e23540](https://github.com/Afloat16/craft-anything/commit/77e235402385fe77f436a107db8bc6123e494571) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-zip-stored-cap-20261103b?quick_pull=1&title=Apply+ZIP+output+limits+to+stored+entries&body=%23%23+Change%0A%0ACheck+the+actual+stored+entry+length+before+allocation+so+stored+members+obey+the+same+extraction+output+cap.%0A%0A%23%23+Regression+coverage%0A%0AOversized+stored+members+and+exact-limit+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 12 | Check ZIP member sizes after decompression | [fc6b69da](https://github.com/Afloat16/craft-anything/commit/fc6b69da397429a80715f8e4b9f6c2d3580301ee) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-zip-decoded-size-20261103b?quick_pull=1&title=Check+ZIP+member+sizes+after+decompression&body=%23%23+Change%0A%0ACompare+the+decoded+member+size+with+the+advertised+uncompressed+size+before+returning+recovered+content.%0A%0A%23%23+Regression+coverage%0A%0ADecoded-size+mismatches+and+valid+member+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 13 | Keep ZIP64 values within their extra field | [9aecbbae](https://github.com/Afloat16/craft-anything/commit/9aecbbae5e80a50a0cc4e4652cf9d42595430ad3) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-zip64-extra-boundaries-20261103b?quick_pull=1&title=Keep+ZIP64+values+within+their+extra+field&body=%23%23+Change%0A%0ARead+ZIP64+metadata+only+inside+its+own+extra+field+and+reject+missing+metadata+for+sentinel+size+or+offset+values.%0A%0A%23%23+Regression+coverage%0A%0ATruncated+ZIP64+fields%2C+adjacent+fields+and+missing+sentinel+metadata.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 14 | Validate ASAR header framing | [e0010694](https://github.com/Afloat16/craft-anything/commit/e0010694cbf37bdd894a9528a309d7a258bcd66a) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-asar-header-bounds-20261103b?quick_pull=1&title=Validate+ASAR+header+framing&body=%23%23+Change%0A%0AValidate+nested+header+lengths+and+ensure+the+complete+declared+header+lies+within+the+archive+before+parsing+it.%0A%0A%23%23+Regression+coverage%0A%0AInconsistent+nested+lengths+and+truncated+archive+headers.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 15 | Traverse ROMFS dot and empty root directories | [d2b4ec71](https://github.com/Afloat16/craft-anything/commit/d2b4ec71bf85888c47930c222963db2c0418d831) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-romfs-root-directory-20261103b?quick_pull=1&title=Traverse+ROMFS+dot+and+empty+root+directories&body=%23%23+Change%0A%0ATraverse+children+for+supported+dot+and+empty+root+names+so+valid+root+layouts+expose+their+contained+files.%0A%0A%23%23+Regression+coverage%0A%0ARoot+layout%2C+recovered+contents+and+entry-limit+fixtures.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.%0A%0AScope%3A+supported+root-directory+layouts%3B+existing+nested-directory+handling+is+preserved.) | Native + shared WASM passed |
| 16 | Report incomplete cramfs block output | [49b87d98](https://github.com/Afloat16/craft-anything/commit/49b87d98ff18f2a01411c083e9d8a379afc9943d) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-cramfs-block-lengths-20261103b?quick_pull=1&title=Report+incomplete+cramfs+block+output&body=%23%23+Change%0A%0ARequire+full-sized+decoded+blocks+and+the+exact+final+remainder+before+claiming+complete+file+recovery.%0A%0A%23%23+Regression+coverage%0A%0AShort+intermediate+blocks%2C+final+block+lengths+and+valid+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 17 | Preserve sparse ext positions across indirect levels | [dc86da3e](https://github.com/Afloat16/craft-anything/commit/dc86da3e15d599217c8662c90d48487ed66e0f77) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-ext-sparse-indirect-20261103b?quick_pull=1&title=Preserve+sparse+ext+positions+across+indirect+levels&body=%23%23+Change%0A%0AAdvance+by+the+logical+span+of+a+missing+indirect+block+so+later+data+stays+at+the+correct+file+offset.%0A%0A%23%23+Regression+coverage%0A%0AMissing+indirect+spans+and+later+mapped+data+regressions.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 18 | Distinguish missing ext data from sparse holes | [59b9f2ea](https://github.com/Afloat16/craft-anything/commit/59b9f2ea006993970fe5a27cab99d94442927577) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-ext-missing-blocks-20261103b?quick_pull=1&title=Distinguish+missing+ext+data+from+sparse+holes&body=%23%23+Change%0A%0AReport+mapped+data+blocks+beyond+the+captured+image+as+unavailable+instead+of+silently+returning+zero-filled+bytes.%0A%0A%23%23+Regression+coverage%0A%0AOut-of-image+mapped+blocks+and+real+sparse-hole+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.%0A%0AScope%3A+missing+mapped+data+blocks.+Indirect+and+extent+metadata+recovery+remains+separate+roadmap+work.) | Native + shared WASM passed |
| 19 | Keep PyInstaller modules within their PYZ archive | [caefe67e](https://github.com/Afloat16/craft-anything/commit/caefe67e5102f34483352944ce160f8a9a25e704) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-pyz-module-bounds-20261103b?quick_pull=1&title=Keep+PyInstaller+modules+within+their+PYZ+archive&body=%23%23+Change%0A%0AReject+listed+module+payloads+outside+the+PYZ+archive+rather+than+reading+bytes+from+neighboring+outer+members.%0A%0A%23%23+Regression+coverage%0A%0AA+module+starting+at+the+PYZ+end+with+a+valid+adjacent+outer+member.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.%0A%0AScope%3A+module+entries+produced+by+the+archive+listing.) | Native + shared WASM passed |
| 20 | Preserve JFFS2 sizes when recovery budgets are exhausted | [07705032](https://github.com/Afloat16/craft-anything/commit/077050320b0dc5a7a5fc0ef9d72655185f5848a0) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-jffs2-unrecovered-size-20261103b?quick_pull=1&title=Preserve+JFFS2+sizes+when+recovery+budgets+are+exhausted&body=%23%23+Change%0A%0AKeep+the+latest+inode+logical+size+when+per-file+or+total+recovery+budgets+prevent+retaining+the+file+contents.%0A%0A%23%23+Regression+coverage%0A%0APer-file+and+total-budget+exhaustion+fixtures.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.%0A%0AScope%3A+recovery-budget+failures%3B+unsupported+compression+handling+is+unchanged.) | Native passed; shared WASM pending |
| 21 | Keep comments when adding the first JSON configuration value | [35da589f](https://github.com/Afloat16/craft-anything/commit/35da589f43453bd1322f823f0f03fdec608510fb) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-jsonc-comment-only-20261103b?quick_pull=1&title=Keep+comments+when+adding+the+first+JSON+configuration+value&body=%23%23+Change%0A%0APreserve+leading+comments%2C+a+byte-order+mark+and+newline+style+when+inserting+the+first+configuration+value.%0A%0A%23%23+Regression+coverage%0A%0AComment-only+inputs%2C+BOM+and+newline-style+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.) | Native passed; shared WASM pending |
| 22 | Remove every duplicate JSON configuration key | [a4ce5ea4](https://github.com/Afloat16/craft-anything/commit/a4ce5ea4cca6c56cee7bf860751ea7a3fd775736) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-jsonc-duplicate-removal-20261103b?quick_pull=1&title=Remove+every+duplicate+JSON+configuration+key&body=%23%23+Change%0A%0ARemove+every+occurrence+of+the+requested+key+while+keeping+valid+separators+and+independent+comments+between+entries.%0A%0A%23%23+Regression+coverage%0A%0AAdjacent+duplicates%2C+separators+and+retained+line%2Fblock+comments.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+Shared+WASM+checks+passed.) | Native + shared WASM passed |
| 23 | Keep UDP payloads within declared datagram lengths | [11f4e9ea](https://github.com/Afloat16/craft-anything/commit/11f4e9ea85375e06204e98fac55bab9261602f61) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-udp-length-bounds-20261103b?quick_pull=1&title=Keep+UDP+payloads+within+declared+datagram+lengths&body=%23%23+Change%0A%0ADiagnose+invalid+or+truncated+datagrams+and+prevent+trailing+capture+bytes+from+being+interpreted+as+UDP+payload+or+DNS+data.%0A%0A%23%23+Regression+coverage%0A%0AInvalid+lengths%2C+truncation%2C+trailing+padding+and+valid+datagrams.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.%0A%0AScope%3A+ordinary+UDP+datagrams%3B+IPv6+jumbograms+are+not+added.) | Native passed; shared WASM pending |
| 24 | Reject invalid TCP header lengths | [d9a65e6f](https://github.com/Afloat16/craft-anything/commit/d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-tcp-header-bounds-20261103b?quick_pull=1&title=Reject+invalid+TCP+header+lengths&body=%23%23+Change%0A%0AReject+data+offsets+below+the+minimum+header+or+beyond+captured+bytes+before+exposing+a+TCP+payload.%0A%0A%23%23+Regression+coverage%0A%0AUndersized+headers%2C+offsets+beyond+capture+and+valid+headers.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.) | Native passed; shared WASM pending |
| 25 | Respect DNS-over-TCP frame lengths | [8cbfa44c](https://github.com/Afloat16/craft-anything/commit/8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-tcp-dns-framing-20261103b?quick_pull=1&title=Respect+DNS-over-TCP+frame+lengths&body=%23%23+Change%0A%0ALimit+decoding+to+the+first+complete+length-prefixed+DNS+message+and+diagnose+truncated+frames.%0A%0A%23%23+Regression+coverage%0A%0AIncomplete+frames%2C+trailing+data+and+valid+first-message+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.%0A%0AScope%3A+one+complete+captured+DNS%2FTCP+message%3B+stream+reassembly+is+not+added.) | Native passed; shared WASM pending |
| 26 | Validate pcapng block framing | [533d404f](https://github.com/Afloat16/craft-anything/commit/533d404ff9234db41e15f11ac63c49c931326f02) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-pcapng-block-framing-20261103b?quick_pull=1&title=Validate+pcapng+block+framing&body=%23%23+Change%0A%0AValidate+repeated+block+lengths+and+each+supported+fixed+header+before+reading+interfaces+or+packets%3B+update+section+byte+order+only+after+validation.%0A%0A%23%23+Regression+coverage%0A%0AMismatched+trailers%2C+short+supported+blocks%2C+both+byte+orders+and+valid+capture+controls.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.%0A%0ARemaining+roadmap+work%3A+checked+32-bit+arithmetic+for+captured+lengths+and+section-relative+interface+ids.) | Native passed; shared WASM pending |
| 27 | Keep TLS metadata within declared lengths | [b0decf6f](https://github.com/Afloat16/craft-anything/commit/b0decf6f3187770147f9f921a2b4f4656a280641) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.12-tls-declared-bounds-20261103b?quick_pull=1&title=Keep+TLS+metadata+within+declared+lengths&body=%23%23+Change%0A%0ABound+nested+record%2C+handshake+and+extension+slices+before+reading+TLS+hello+metadata.%0A%0A%23%23+Regression+coverage%0A%0ATen+fixtures+cover+truncation%2C+injected+trailing+bytes+and+valid+SNI%2FALPN+and+TLS+1.2+hellos.%0A%0A%23%23+Validation%0A%0AAffected+formats+tests%2C+strict+Clippy+and+formatting+passed.+The+combined+27-change+formats+run+passed+563+tests+with+0+ignored.+The+independent+shared+WASM+gate+remains+pending.) | Native passed; shared WASM pending |
| 28 | Preserve distinct response metadata in replay samples | [ec385c38](https://github.com/Afloat16/craft-anything/commit/ec385c38b69e021617bd2b93f226d891cefb0c9a) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.11-standin-sample-metadata-20261103b?quick_pull=1&title=Preserve+distinct+response+metadata+in+replay+samples&body=%23%23+Change%0A%0AConsecutive+samples+compare+all+response+fields%2C+keeping+distinct+headers%2C+content+types+and+omitted-body+metadata.%0A%0A%23%23+Regression+coverage%0A%0AFive+regressions+cover+changing+Location+headers%2C+MIME+types%2C+omitted-body+reasons%2C+cached+304+markers+and+identical-sample+deduplication.%0A%0A%23%23+Validation%0A%0ASource+and+published+files+checked.+Automated+tests%2C+formatting%2C+Clippy+and+remaining+gates+are+pending+because+the+execution+environment+is+unavailable.) | Execution gates pending |
| 29 | Apply replay body limits after redaction | [b9586247](https://github.com/Afloat16/craft-anything/commit/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r3.11-standin-redacted-budget-20261103b?quick_pull=1&title=Apply+replay+body+limits+after+redaction&body=%23%23+Change%0A%0ACheck+the+redacted+text+against+each+body+and+capture+budget+before+retaining+it+or+charging+its+bytes.%0A%0A%23%23+Regression+coverage%0A%0AFour+regressions+cover+expanding+redactions%2C+per-body+and+total+limits%2C+exact+retained+size+and+rejected-body+budget+accounting.%0A%0A%23%23+Validation%0A%0ASource+and+published+files+checked.+Automated+tests%2C+formatting%2C+Clippy+and+remaining+gates+are+pending+because+the+execution+environment+is+unavailable.%0A%0AScope%3A+retained+text+bytes.+Existing+raw-input+prechecks+remain+in+place%3B+temporary+redaction+allocations+are+not+bounded+by+this+change.) | Execution gates pending |
| 30 | Create atomic-write temporary files exclusively | [5f0f1ce2](https://github.com/Afloat16/craft-anything/commit/5f0f1ce228af21a6ccc6494f6dacf392357f56a1) | [Open form](https://github.com/storytold/craft-anything/compare/main...Afloat16:bugfix/r2.9-atomic-temp-creation-20261103b?quick_pull=1&title=Create+atomic-write+temporary+files+exclusively&body=%23%23+Change%0A%0AReserve+temporary+files+exclusively+with+bounded+collision+retries%2C+and+only+clean+up+a+file+reserved+by+the+current+write.%0A%0A%23%23+Regression+coverage%0A%0AOne+filesystem+regression+covers+existing+files%2C+Unix+symlinks%2C+private+and+inherited+permissions%2C+exhausted+retries+and+cleanup+after+rename+failure.%0A%0A%23%23+Validation%0A%0ASource+and+published+files+checked.+Automated+tests%2C+formatting%2C+Clippy+and+remaining+gates+are+pending+because+the+execution+environment+is+unavailable.) | Execution gates pending |

## Change details

### 1. Reject signed LEB128 values outside the 64-bit range

Branch: `bugfix/r3.12-dex-signed-leb128-20261103b`  
Head: `c6f256518b2e16c91087211665beab4a78df3e96`  
Tree: `234f18ee48d22b60a29da7db5da11fb2f885fa39`

## Change

Reject invalid sign extension in the tenth byte so malformed exception counts cannot become valid values.

## Regression coverage

Signed-limit and overflowing tenth-byte regressions.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Scope: the current signed 64-bit LEB128 helper; broader DEX 32-bit operand rules are unchanged.

Files:

- [crates/formats/src/dex.rs](https://github.com/Afloat16/craft-anything/blob/c6f256518b2e16c91087211665beab4a78df3e96/crates/formats/src/dex.rs)
- [crates/formats/tests/dex_signed_leb128.rs](https://github.com/Afloat16/craft-anything/blob/c6f256518b2e16c91087211665beab4a78df3e96/crates/formats/tests/dex_signed_leb128.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/c6f256518b2e16c91087211665beab4a78df3e96/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/c6f256518b2e16c91087211665beab4a78df3e96/docs/STATUS.md)

### 2. Preserve valid text around damaged modified UTF-8

Branch: `bugfix/r3.12-modified-utf8-validation-20261103b`  
Head: `7defc6c3583946aa1072ddb209734f2f25817c6b`  
Tree: `88f3f94d63b7862da67fff9eed48aab26c4cc249`

## Change

Validate continuation and overlong encodings in the shared DEX/Java decoder while preserving valid following text, encoded NUL and surrogate pairs.

## Regression coverage

Malformed encodings and valid surrounding text controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/bytes.rs](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/crates/formats/src/bytes.rs)
- [crates/formats/src/dex.rs](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/crates/formats/src/dex.rs)
- [crates/formats/src/java_class.rs](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/crates/formats/src/java_class.rs)
- [crates/formats/tests/modified_utf8_validation.rs](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/crates/formats/tests/modified_utf8_validation.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/7defc6c3583946aa1072ddb209734f2f25817c6b/docs/STATUS.md)

### 3. Reject invalid widened Java instructions

Branch: `bugfix/r3.12-java-wide-opcodes-20261103b`  
Head: `206c2804e54ad8639965203c6a16cff2732f3c8c`  
Tree: `f6e75080a84518ec24519679f734e6e0bd4322e0`

## Change

Allow wide only for the defined load, store, ret and iinc instructions so malformed bytecode cannot be decoded as a valid widened operation.

## Regression coverage

Allowed widened operations and invalid opcode regressions.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/java_ops.rs](https://github.com/Afloat16/craft-anything/blob/206c2804e54ad8639965203c6a16cff2732f3c8c/crates/formats/src/java_ops.rs)
- [crates/formats/tests/java_wide_opcodes.rs](https://github.com/Afloat16/craft-anything/blob/206c2804e54ad8639965203c6a16cff2732f3c8c/crates/formats/tests/java_wide_opcodes.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/206c2804e54ad8639965203c6a16cff2732f3c8c/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/206c2804e54ad8639965203c6a16cff2732f3c8c/docs/STATUS.md)

### 4. Validate Java invocation operands before decoding calls

Branch: `bugfix/r3.12-java-invoke-operands-20261103b`  
Head: `63e3af8dbad23f0885590d3b7d7912440acf1d68`  
Tree: `64850cad57780d7247edc8ee4707181b9aff15ec`

## Change

Require a nonzero invokeinterface count and zero reserved operand bytes before accepting interface or dynamic calls.

## Regression coverage

Invalid counts, reserved bytes and valid invocation controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/java_ops.rs](https://github.com/Afloat16/craft-anything/blob/63e3af8dbad23f0885590d3b7d7912440acf1d68/crates/formats/src/java_ops.rs)
- [crates/formats/tests/java_invoke_operands.rs](https://github.com/Afloat16/craft-anything/blob/63e3af8dbad23f0885590d3b7d7912440acf1d68/crates/formats/tests/java_invoke_operands.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/63e3af8dbad23f0885590d3b7d7912440acf1d68/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/63e3af8dbad23f0885590d3b7d7912440acf1d68/docs/STATUS.md)

### 5. Reject impossible DEX invocation register lists

Branch: `bugfix/r3.12-dex-invoke-registers-20261103b`  
Head: `6da3b104817744d3baefad1a01413db6d3df2930`  
Tree: `33ecd2b5201577a01da57a6c0e945c6e0997f6f6`

## Change

Reject fixed invocation lists larger than five registers and range operands extending beyond the register index space.

## Regression coverage

Oversized fixed lists, overflowing ranges and valid boundaries.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/dex_ops.rs](https://github.com/Afloat16/craft-anything/blob/6da3b104817744d3baefad1a01413db6d3df2930/crates/formats/src/dex_ops.rs)
- [crates/formats/tests/dex_invoke_registers.rs](https://github.com/Afloat16/craft-anything/blob/6da3b104817744d3baefad1a01413db6d3df2930/crates/formats/tests/dex_invoke_registers.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/6da3b104817744d3baefad1a01413db6d3df2930/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/6da3b104817744d3baefad1a01413db6d3df2930/docs/STATUS.md)

### 6. Reject invalid DEX array payload element widths

Branch: `bugfix/r3.12-dex-array-widths-20261103b`  
Head: `e4802ee7713eff91eb9a6e27a0459c0383f63761`  
Tree: `0a129c8be008308a9839b24d99a04f4b1b0ac71f`

## Change

Accept only 1, 2, 4 or 8 byte array elements, rejecting unsupported and zero widths before payload-size calculations.

## Regression coverage

Invalid element widths, large counts and valid width controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/dex_ops.rs](https://github.com/Afloat16/craft-anything/blob/e4802ee7713eff91eb9a6e27a0459c0383f63761/crates/formats/src/dex_ops.rs)
- [crates/formats/tests/dex_array_widths.rs](https://github.com/Afloat16/craft-anything/blob/e4802ee7713eff91eb9a6e27a0459c0383f63761/crates/formats/tests/dex_array_widths.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/e4802ee7713eff91eb9a6e27a0459c0383f63761/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/e4802ee7713eff91eb9a6e27a0459c0383f63761/docs/STATUS.md)

### 7. Read SQLite text using the database encoding

Branch: `bugfix/r3.12-sqlite-text-encoding-20261103b`  
Head: `be78da063a6ebac4e8f399d8ef7da793981ec95b`  
Tree: `6911a2e3bec92002c7931949085c71e5e29332bf`

## Change

Use the database text encoding for schema and row text so UTF-16 databases are decoded correctly.

## Regression coverage

UTF-8, UTF-16LE and UTF-16BE schema and row fixtures.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/sqlite.rs](https://github.com/Afloat16/craft-anything/blob/be78da063a6ebac4e8f399d8ef7da793981ec95b/crates/formats/src/sqlite.rs)
- [crates/formats/tests/sqlite_text_encoding.rs](https://github.com/Afloat16/craft-anything/blob/be78da063a6ebac4e8f399d8ef7da793981ec95b/crates/formats/tests/sqlite_text_encoding.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/be78da063a6ebac4e8f399d8ef7da793981ec95b/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/be78da063a6ebac4e8f399d8ef7da793981ec95b/docs/STATUS.md)

### 8. Keep SQLite cell reads within their page

Branch: `bugfix/r3.12-sqlite-page-boundaries-20261103b`  
Head: `1d2a3b0f812bc0cac6414d278142ad82a13b5bd7`  
Tree: `d5ad364841a8fc44ce0e9951e0171b06742d527d`

## Change

Constrain cell headers, pointer arrays and payload reads to the current usable page instead of borrowing bytes from a neighboring page.

## Regression coverage

Malformed page pointers and truncated page-local data.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/sqlite.rs](https://github.com/Afloat16/craft-anything/blob/1d2a3b0f812bc0cac6414d278142ad82a13b5bd7/crates/formats/src/sqlite.rs)
- [crates/formats/tests/sqlite_page_boundaries.rs](https://github.com/Afloat16/craft-anything/blob/1d2a3b0f812bc0cac6414d278142ad82a13b5bd7/crates/formats/tests/sqlite_page_boundaries.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/1d2a3b0f812bc0cac6414d278142ad82a13b5bd7/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/1d2a3b0f812bc0cac6414d278142ad82a13b5bd7/docs/STATUS.md)

### 9. Stop repeated SQLite overflow pages from fabricating data

Branch: `bugfix/r3.12-sqlite-overflow-cycles-20261103b`  
Head: `bc1482115787d4fa0172c2901ac16c1479724dcb`  
Tree: `cdea39a5cc6a3e69aee261d2989007e4351a982d`

## Change

Track visited overflow pages and stop repeated pages while retaining only the actually recovered partial payload.

## Regression coverage

Repeated-page and cyclic overflow-chain regressions.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/sqlite.rs](https://github.com/Afloat16/craft-anything/blob/bc1482115787d4fa0172c2901ac16c1479724dcb/crates/formats/src/sqlite.rs)
- [crates/formats/tests/sqlite_overflow_cycles.rs](https://github.com/Afloat16/craft-anything/blob/bc1482115787d4fa0172c2901ac16c1479724dcb/crates/formats/tests/sqlite_overflow_cycles.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/bc1482115787d4fa0172c2901ac16c1479724dcb/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/bc1482115787d4fa0172c2901ac16c1479724dcb/docs/STATUS.md)

### 10. Keep Android string pools within declared chunks

Branch: `bugfix/r3.12-android-string-pool-bounds-20261103b`  
Head: `4c00f0e3dca3fac29d95b85f53620ceed323ebe7`  
Tree: `ca938493e6f8ac602145d69cff8fe3d1517d9c22`

## Change

Validate chunk framing, offsets, string/style regions and terminators before decoding string pool entries.

## Regression coverage

Out-of-chunk offsets, malformed regions and termination checks.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/axml.rs](https://github.com/Afloat16/craft-anything/blob/4c00f0e3dca3fac29d95b85f53620ceed323ebe7/crates/formats/src/axml.rs)
- [crates/formats/tests/android_string_pool_bounds.rs](https://github.com/Afloat16/craft-anything/blob/4c00f0e3dca3fac29d95b85f53620ceed323ebe7/crates/formats/tests/android_string_pool_bounds.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/4c00f0e3dca3fac29d95b85f53620ceed323ebe7/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/4c00f0e3dca3fac29d95b85f53620ceed323ebe7/docs/STATUS.md)

### 11. Apply ZIP output limits to stored entries

Branch: `bugfix/r3.12-zip-stored-cap-20261103b`  
Head: `77e235402385fe77f436a107db8bc6123e494571`  
Tree: `63966315fec68752c498c38a7debe769efe2819d`

## Change

Check the actual stored entry length before allocation so stored members obey the same extraction output cap.

## Regression coverage

Oversized stored members and exact-limit controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/zip.rs](https://github.com/Afloat16/craft-anything/blob/77e235402385fe77f436a107db8bc6123e494571/crates/formats/src/zip.rs)
- [crates/formats/tests/zip_stored_cap.rs](https://github.com/Afloat16/craft-anything/blob/77e235402385fe77f436a107db8bc6123e494571/crates/formats/tests/zip_stored_cap.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/77e235402385fe77f436a107db8bc6123e494571/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/77e235402385fe77f436a107db8bc6123e494571/docs/STATUS.md)

### 12. Check ZIP member sizes after decompression

Branch: `bugfix/r3.12-zip-decoded-size-20261103b`  
Head: `fc6b69da397429a80715f8e4b9f6c2d3580301ee`  
Tree: `770e8ed1d724d13544995fb348439f1692994f35`

## Change

Compare the decoded member size with the advertised uncompressed size before returning recovered content.

## Regression coverage

Decoded-size mismatches and valid member controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/zip.rs](https://github.com/Afloat16/craft-anything/blob/fc6b69da397429a80715f8e4b9f6c2d3580301ee/crates/formats/src/zip.rs)
- [crates/formats/tests/zip_decoded_size.rs](https://github.com/Afloat16/craft-anything/blob/fc6b69da397429a80715f8e4b9f6c2d3580301ee/crates/formats/tests/zip_decoded_size.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/fc6b69da397429a80715f8e4b9f6c2d3580301ee/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/fc6b69da397429a80715f8e4b9f6c2d3580301ee/docs/STATUS.md)

### 13. Keep ZIP64 values within their extra field

Branch: `bugfix/r3.12-zip64-extra-boundaries-20261103b`  
Head: `9aecbbae5e80a50a0cc4e4652cf9d42595430ad3`  
Tree: `e25ed354018e68e36414029164cb2904e1199d6f`

## Change

Read ZIP64 metadata only inside its own extra field and reject missing metadata for sentinel size or offset values.

## Regression coverage

Truncated ZIP64 fields, adjacent fields and missing sentinel metadata.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/zip.rs](https://github.com/Afloat16/craft-anything/blob/9aecbbae5e80a50a0cc4e4652cf9d42595430ad3/crates/formats/src/zip.rs)
- [crates/formats/tests/zip64_extra_boundaries.rs](https://github.com/Afloat16/craft-anything/blob/9aecbbae5e80a50a0cc4e4652cf9d42595430ad3/crates/formats/tests/zip64_extra_boundaries.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/9aecbbae5e80a50a0cc4e4652cf9d42595430ad3/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/9aecbbae5e80a50a0cc4e4652cf9d42595430ad3/docs/STATUS.md)

### 14. Validate ASAR header framing

Branch: `bugfix/r3.12-asar-header-bounds-20261103b`  
Head: `e0010694cbf37bdd894a9528a309d7a258bcd66a`  
Tree: `4b96dcf7be03a254253c3ef01686de35899aedf7`

## Change

Validate nested header lengths and ensure the complete declared header lies within the archive before parsing it.

## Regression coverage

Inconsistent nested lengths and truncated archive headers.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/asar.rs](https://github.com/Afloat16/craft-anything/blob/e0010694cbf37bdd894a9528a309d7a258bcd66a/crates/formats/src/asar.rs)
- [crates/formats/tests/asar_header_bounds.rs](https://github.com/Afloat16/craft-anything/blob/e0010694cbf37bdd894a9528a309d7a258bcd66a/crates/formats/tests/asar_header_bounds.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/e0010694cbf37bdd894a9528a309d7a258bcd66a/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/e0010694cbf37bdd894a9528a309d7a258bcd66a/docs/STATUS.md)

### 15. Traverse ROMFS dot and empty root directories

Branch: `bugfix/r3.12-romfs-root-directory-20261103b`  
Head: `d2b4ec71bf85888c47930c222963db2c0418d831`  
Tree: `b0638143da0c871957937d42142e3c7801c4d7a7`

## Change

Traverse children for supported dot and empty root names so valid root layouts expose their contained files.

## Regression coverage

Root layout, recovered contents and entry-limit fixtures.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Scope: supported root-directory layouts; existing nested-directory handling is preserved.

Files:

- [crates/formats/src/fsread/romfs.rs](https://github.com/Afloat16/craft-anything/blob/d2b4ec71bf85888c47930c222963db2c0418d831/crates/formats/src/fsread/romfs.rs)
- [crates/formats/tests/romfs_root_directory.rs](https://github.com/Afloat16/craft-anything/blob/d2b4ec71bf85888c47930c222963db2c0418d831/crates/formats/tests/romfs_root_directory.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/d2b4ec71bf85888c47930c222963db2c0418d831/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/d2b4ec71bf85888c47930c222963db2c0418d831/docs/STATUS.md)

### 16. Report incomplete cramfs block output

Branch: `bugfix/r3.12-cramfs-block-lengths-20261103b`  
Head: `49b87d98ff18f2a01411c083e9d8a379afc9943d`  
Tree: `0fd9f66d7dfd40d41dc01487ec23ef3d94ea37c7`

## Change

Require full-sized decoded blocks and the exact final remainder before claiming complete file recovery.

## Regression coverage

Short intermediate blocks, final block lengths and valid controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/fsread/cramfs.rs](https://github.com/Afloat16/craft-anything/blob/49b87d98ff18f2a01411c083e9d8a379afc9943d/crates/formats/src/fsread/cramfs.rs)
- [crates/formats/tests/cramfs_block_lengths.rs](https://github.com/Afloat16/craft-anything/blob/49b87d98ff18f2a01411c083e9d8a379afc9943d/crates/formats/tests/cramfs_block_lengths.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/49b87d98ff18f2a01411c083e9d8a379afc9943d/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/49b87d98ff18f2a01411c083e9d8a379afc9943d/docs/STATUS.md)

### 17. Preserve sparse ext positions across indirect levels

Branch: `bugfix/r3.12-ext-sparse-indirect-20261103b`  
Head: `dc86da3e15d599217c8662c90d48487ed66e0f77`  
Tree: `cf1411f9f1426df26d12a29f209796d349a4b55e`

## Change

Advance by the logical span of a missing indirect block so later data stays at the correct file offset.

## Regression coverage

Missing indirect spans and later mapped data regressions.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/fsread/extfs.rs](https://github.com/Afloat16/craft-anything/blob/dc86da3e15d599217c8662c90d48487ed66e0f77/crates/formats/src/fsread/extfs.rs)
- [crates/formats/tests/ext_sparse_indirect.rs](https://github.com/Afloat16/craft-anything/blob/dc86da3e15d599217c8662c90d48487ed66e0f77/crates/formats/tests/ext_sparse_indirect.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/dc86da3e15d599217c8662c90d48487ed66e0f77/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/dc86da3e15d599217c8662c90d48487ed66e0f77/docs/STATUS.md)

### 18. Distinguish missing ext data from sparse holes

Branch: `bugfix/r3.12-ext-missing-blocks-20261103b`  
Head: `59b9f2ea006993970fe5a27cab99d94442927577`  
Tree: `d6da5b41ce8918ea2820092ca9c4e9888b1c4424`

## Change

Report mapped data blocks beyond the captured image as unavailable instead of silently returning zero-filled bytes.

## Regression coverage

Out-of-image mapped blocks and real sparse-hole controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Scope: missing mapped data blocks. Indirect and extent metadata recovery remains separate roadmap work.

Files:

- [crates/formats/src/fsread/extfs.rs](https://github.com/Afloat16/craft-anything/blob/59b9f2ea006993970fe5a27cab99d94442927577/crates/formats/src/fsread/extfs.rs)
- [crates/formats/tests/ext_missing_blocks.rs](https://github.com/Afloat16/craft-anything/blob/59b9f2ea006993970fe5a27cab99d94442927577/crates/formats/tests/ext_missing_blocks.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/59b9f2ea006993970fe5a27cab99d94442927577/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/59b9f2ea006993970fe5a27cab99d94442927577/docs/STATUS.md)

### 19. Keep PyInstaller modules within their PYZ archive

Branch: `bugfix/r3.12-pyz-module-bounds-20261103b`  
Head: `caefe67e5102f34483352944ce160f8a9a25e704`  
Tree: `e8cb9eaa37a18535a8369cc1f6fe3ed7c0610bd3`

## Change

Reject listed module payloads outside the PYZ archive rather than reading bytes from neighboring outer members.

## Regression coverage

A module starting at the PYZ end with a valid adjacent outer member.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Scope: module entries produced by the archive listing.

Files:

- [crates/formats/src/pyinstaller.rs](https://github.com/Afloat16/craft-anything/blob/caefe67e5102f34483352944ce160f8a9a25e704/crates/formats/src/pyinstaller.rs)
- [crates/formats/tests/pyz_module_bounds.rs](https://github.com/Afloat16/craft-anything/blob/caefe67e5102f34483352944ce160f8a9a25e704/crates/formats/tests/pyz_module_bounds.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/caefe67e5102f34483352944ce160f8a9a25e704/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/caefe67e5102f34483352944ce160f8a9a25e704/docs/STATUS.md)

### 20. Preserve JFFS2 sizes when recovery budgets are exhausted

Branch: `bugfix/r3.12-jffs2-unrecovered-size-20261103b`  
Head: `077050320b0dc5a7a5fc0ef9d72655185f5848a0`  
Tree: `94a3919f8c83b38aedc885d544e812d081148c17`

## Change

Keep the latest inode logical size when per-file or total recovery budgets prevent retaining the file contents.

## Regression coverage

Per-file and total-budget exhaustion fixtures.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Scope: recovery-budget failures; unsupported compression handling is unchanged.

Files:

- [crates/formats/src/fsread/jffs2.rs](https://github.com/Afloat16/craft-anything/blob/077050320b0dc5a7a5fc0ef9d72655185f5848a0/crates/formats/src/fsread/jffs2.rs)
- [crates/formats/tests/jffs2_unrecovered_size.rs](https://github.com/Afloat16/craft-anything/blob/077050320b0dc5a7a5fc0ef9d72655185f5848a0/crates/formats/tests/jffs2_unrecovered_size.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/077050320b0dc5a7a5fc0ef9d72655185f5848a0/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/077050320b0dc5a7a5fc0ef9d72655185f5848a0/docs/STATUS.md)

### 21. Keep comments when adding the first JSON configuration value

Branch: `bugfix/r3.12-jsonc-comment-only-20261103b`  
Head: `35da589f43453bd1322f823f0f03fdec608510fb`  
Tree: `d88664c487ec28091e7f0f69001ee95ce8a1475e`

## Change

Preserve leading comments, a byte-order mark and newline style when inserting the first configuration value.

## Regression coverage

Comment-only inputs, BOM and newline-style controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Files:

- [crates/formats/src/jsonc.rs](https://github.com/Afloat16/craft-anything/blob/35da589f43453bd1322f823f0f03fdec608510fb/crates/formats/src/jsonc.rs)
- [crates/formats/tests/regression_jsonc_comment_only.rs](https://github.com/Afloat16/craft-anything/blob/35da589f43453bd1322f823f0f03fdec608510fb/crates/formats/tests/regression_jsonc_comment_only.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/35da589f43453bd1322f823f0f03fdec608510fb/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/35da589f43453bd1322f823f0f03fdec608510fb/docs/STATUS.md)

### 22. Remove every duplicate JSON configuration key

Branch: `bugfix/r3.12-jsonc-duplicate-removal-20261103b`  
Head: `a4ce5ea4cca6c56cee7bf860751ea7a3fd775736`  
Tree: `592542a2f5ce7d3525d698e8b124daa11f685ab2`

## Change

Remove every occurrence of the requested key while keeping valid separators and independent comments between entries.

## Regression coverage

Adjacent duplicates, separators and retained line/block comments.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. Shared WASM checks passed.

Files:

- [crates/formats/src/jsonc.rs](https://github.com/Afloat16/craft-anything/blob/a4ce5ea4cca6c56cee7bf860751ea7a3fd775736/crates/formats/src/jsonc.rs)
- [crates/formats/tests/regression_jsonc_duplicate_removal.rs](https://github.com/Afloat16/craft-anything/blob/a4ce5ea4cca6c56cee7bf860751ea7a3fd775736/crates/formats/tests/regression_jsonc_duplicate_removal.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/a4ce5ea4cca6c56cee7bf860751ea7a3fd775736/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/a4ce5ea4cca6c56cee7bf860751ea7a3fd775736/docs/STATUS.md)

### 23. Keep UDP payloads within declared datagram lengths

Branch: `bugfix/r3.12-udp-length-bounds-20261103b`  
Head: `11f4e9ea85375e06204e98fac55bab9261602f61`  
Tree: `99abd411769e4d29eb313fdedb080397d6d7ce3a`

## Change

Diagnose invalid or truncated datagrams and prevent trailing capture bytes from being interpreted as UDP payload or DNS data.

## Regression coverage

Invalid lengths, truncation, trailing padding and valid datagrams.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Scope: ordinary UDP datagrams; IPv6 jumbograms are not added.

Files:

- [crates/formats/src/pcap.rs](https://github.com/Afloat16/craft-anything/blob/11f4e9ea85375e06204e98fac55bab9261602f61/crates/formats/src/pcap.rs)
- [crates/formats/tests/regression_udp_length.rs](https://github.com/Afloat16/craft-anything/blob/11f4e9ea85375e06204e98fac55bab9261602f61/crates/formats/tests/regression_udp_length.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/11f4e9ea85375e06204e98fac55bab9261602f61/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/11f4e9ea85375e06204e98fac55bab9261602f61/docs/STATUS.md)

### 24. Reject invalid TCP header lengths

Branch: `bugfix/r3.12-tcp-header-bounds-20261103b`  
Head: `d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d`  
Tree: `8e7f449a8af37516de29f556f548b9cab59aa95a`

## Change

Reject data offsets below the minimum header or beyond captured bytes before exposing a TCP payload.

## Regression coverage

Undersized headers, offsets beyond capture and valid headers.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Files:

- [crates/formats/src/pcap.rs](https://github.com/Afloat16/craft-anything/blob/d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d/crates/formats/src/pcap.rs)
- [crates/formats/tests/regression_tcp_header_length.rs](https://github.com/Afloat16/craft-anything/blob/d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d/crates/formats/tests/regression_tcp_header_length.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/d9a65e6f4d2db3dc288bb6cf3b2808bf74323d8d/docs/STATUS.md)

### 25. Respect DNS-over-TCP frame lengths

Branch: `bugfix/r3.12-tcp-dns-framing-20261103b`  
Head: `8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe`  
Tree: `2f819662cf4dd519bc5526e83486cc3b460225fe`

## Change

Limit decoding to the first complete length-prefixed DNS message and diagnose truncated frames.

## Regression coverage

Incomplete frames, trailing data and valid first-message controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Scope: one complete captured DNS/TCP message; stream reassembly is not added.

Files:

- [crates/formats/src/pcap.rs](https://github.com/Afloat16/craft-anything/blob/8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe/crates/formats/src/pcap.rs)
- [crates/formats/tests/regression_tcp_dns_framing.rs](https://github.com/Afloat16/craft-anything/blob/8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe/crates/formats/tests/regression_tcp_dns_framing.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/8cbfa44cd91470b0b1bfb27ac3d1c1e89a145fbe/docs/STATUS.md)

### 26. Validate pcapng block framing

Branch: `bugfix/r3.12-pcapng-block-framing-20261103b`  
Head: `533d404ff9234db41e15f11ac63c49c931326f02`  
Tree: `928ef0e3f08ac9c8c0fb49b8ca57a302bfbfaf56`

## Change

Validate repeated block lengths and each supported fixed header before reading interfaces or packets; update section byte order only after validation.

## Regression coverage

Mismatched trailers, short supported blocks, both byte orders and valid capture controls.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Remaining roadmap work: checked 32-bit arithmetic for captured lengths and section-relative interface ids.

Files:

- [crates/formats/src/pcap.rs](https://github.com/Afloat16/craft-anything/blob/533d404ff9234db41e15f11ac63c49c931326f02/crates/formats/src/pcap.rs)
- [crates/formats/tests/regression_pcapng_framing.rs](https://github.com/Afloat16/craft-anything/blob/533d404ff9234db41e15f11ac63c49c931326f02/crates/formats/tests/regression_pcapng_framing.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/533d404ff9234db41e15f11ac63c49c931326f02/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/533d404ff9234db41e15f11ac63c49c931326f02/docs/STATUS.md)

### 27. Keep TLS metadata within declared lengths

Branch: `bugfix/r3.12-tls-declared-bounds-20261103b`  
Head: `b0decf6f3187770147f9f921a2b4f4656a280641`  
Tree: `9474bd3608cad4d02a128fb1b2064574b9e06dc2`

## Change

Bound nested record, handshake and extension slices before reading TLS hello metadata.

## Regression coverage

Ten fixtures cover truncation, injected trailing bytes and valid SNI/ALPN and TLS 1.2 hellos.

## Validation

Affected formats tests, strict Clippy and formatting passed. The combined 27-change formats run passed 563 tests with 0 ignored. The independent shared WASM gate remains pending.

Files:

- [crates/formats/src/pcap.rs](https://github.com/Afloat16/craft-anything/blob/b0decf6f3187770147f9f921a2b4f4656a280641/crates/formats/src/pcap.rs)
- [crates/formats/tests/regression_tls_bounds.rs](https://github.com/Afloat16/craft-anything/blob/b0decf6f3187770147f9f921a2b4f4656a280641/crates/formats/tests/regression_tls_bounds.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/b0decf6f3187770147f9f921a2b4f4656a280641/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/b0decf6f3187770147f9f921a2b4f4656a280641/docs/STATUS.md)

### 28. Preserve distinct response metadata in replay samples

Branch: `bugfix/r3.11-standin-sample-metadata-20261103b`  
Head: `ec385c38b69e021617bd2b93f226d891cefb0c9a`  
Tree: `1a21e7cdb44bf234b626067b40ca9e64aede7568`

## Change

Consecutive samples compare all response fields, keeping distinct headers, content types and omitted-body metadata.

## Regression coverage

Five regressions cover changing Location headers, MIME types, omitted-body reasons, cached 304 markers and identical-sample deduplication.

## Validation

Source and published files checked. Automated tests, formatting, Clippy and remaining gates are pending because the execution environment is unavailable.

Files:

- [crates/analysis/src/standin.rs](https://github.com/Afloat16/craft-anything/blob/ec385c38b69e021617bd2b93f226d891cefb0c9a/crates/analysis/src/standin.rs)
- [crates/analysis/tests/regression_standin_sample_metadata.rs](https://github.com/Afloat16/craft-anything/blob/ec385c38b69e021617bd2b93f226d891cefb0c9a/crates/analysis/tests/regression_standin_sample_metadata.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/ec385c38b69e021617bd2b93f226d891cefb0c9a/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/ec385c38b69e021617bd2b93f226d891cefb0c9a/docs/STATUS.md)

### 29. Apply replay body limits after redaction

Branch: `bugfix/r3.11-standin-redacted-budget-20261103b`  
Head: `b95862470c5e75af14f5bcd9247fc7a1c2cce9dd`  
Tree: `33fb331d39a7fd659ec7ba2ab7ff7ea2c420ee3e`

## Change

Check the redacted text against each body and capture budget before retaining it or charging its bytes.

## Regression coverage

Four regressions cover expanding redactions, per-body and total limits, exact retained size and rejected-body budget accounting.

## Validation

Source and published files checked. Automated tests, formatting, Clippy and remaining gates are pending because the execution environment is unavailable.

Scope: retained text bytes. Existing raw-input prechecks remain in place; temporary redaction allocations are not bounded by this change.

Files:

- [crates/analysis/src/standin.rs](https://github.com/Afloat16/craft-anything/blob/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd/crates/analysis/src/standin.rs)
- [crates/analysis/tests/regression_standin_redacted_budget.rs](https://github.com/Afloat16/craft-anything/blob/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd/crates/analysis/tests/regression_standin_redacted_budget.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/b95862470c5e75af14f5bcd9247fc7a1c2cce9dd/docs/STATUS.md)

### 30. Create atomic-write temporary files exclusively

Branch: `bugfix/r2.9-atomic-temp-creation-20261103b`  
Head: `5f0f1ce228af21a6ccc6494f6dacf392357f56a1`  
Tree: `2d5696bfc08ab757953679ce1cfedaed2059642a`

## Change

Reserve temporary files exclusively with bounded collision retries, and only clean up a file reserved by the current write.

## Regression coverage

One filesystem regression covers existing files, Unix symlinks, private and inherited permissions, exhausted retries and cleanup after rename failure.

## Validation

Source and published files checked. Automated tests, formatting, Clippy and remaining gates are pending because the execution environment is unavailable.

Files:

- [crates/platform/src/fs.rs](https://github.com/Afloat16/craft-anything/blob/5f0f1ce228af21a6ccc6494f6dacf392357f56a1/crates/platform/src/fs.rs)
- [crates/platform/tests/regression_atomic_temp_creation.rs](https://github.com/Afloat16/craft-anything/blob/5f0f1ce228af21a6ccc6494f6dacf392357f56a1/crates/platform/tests/regression_atomic_temp_creation.rs)
- [docs/ROADMAP.md](https://github.com/Afloat16/craft-anything/blob/5f0f1ce228af21a6ccc6494f6dacf392357f56a1/docs/ROADMAP.md)
- [docs/STATUS.md](https://github.com/Afloat16/craft-anything/blob/5f0f1ce228af21a6ccc6494f6dacf392357f56a1/docs/STATUS.md)

## Link format

Prefilled links use GitHub's documented `quick_pull`, `title` and `body` parameters:

https://docs.github.com/en/pull-requests/reference/using-query-parameters-to-create-a-pull-request
