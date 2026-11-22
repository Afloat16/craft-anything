# File formats

Every file Craft Anything saves carries a version number. Reading a file runs the same three steps:
parse it into a raw document (a JSON value or a TOML table), run the migration steps from the file's
version up to the current one, then turn the document into Rust types. The step runner is
`crates/engine/src/versioned.rs`.

| File | Owner | Current version | Older files | Newer files |
|---|---|---|---|---|
| `<project>/craft.json` | `engine::workspace` (`Manifest`) | `MANIFEST_VERSION` = 7 (`version`; missing = 0; 3 adds `approvals`, 4 adds the saved builder session's mode `agent_session_deep`, 5 adds the current capture's mode `capture_deep`, 6 adds the person's saved `website_language` choice, 7 saves `website_language_request_id` with that choice; a version-3 session and a version-4 capture migrate as Deep) | brought forward, saved as current | opened read-only: saving is refused; close older processes that already opened the project before upgrading it |
| `<project>/inbox/build-progress.json` | `engine::workspace::progress` (`BuildProgress`; written only by the rebuild run) | `BUILD_PROGRESS_VERSION` = 2 (`version`; missing = 0; 2 adds `checked`, the round kept while nothing it looked at changes) | brought forward; an absent file starts with no counted turns | the build stops and the record is left unchanged |
| `<project>/inbox/notes/<id>*.json` | `ai::rebuild::notes` (`NoteFile`, JSON Schema derived with `JsonSchema`) | 1 (`{version, id, text, left}`; `left` is the person's `{text, shown, pictures}`) | unversioned waiting notes keep their original words; legacy `.read.json` files remain excluded from replay | not claimed; restart preserves the notes directory beside the inbox instead of clearing it |
| `config.toml` (`settings::config_dir()`) | `engine::settings` (`Settings`) | `SETTINGS_VERSION` = 2 (`version`; missing = 0; 2 moved `[engines.ghidra] enabled` to `deep_mode`) | brought forward | loaded, never saved over |
| `<name>.cany/project.toml` | `engine::project` (`ProjectMeta`) | `FORMAT_VERSION` = 1 (`format_version`, required) | brought forward | refused, with "update Craft Anything" |
| `<project>/transcript.json` | `ai::context` (the API builder's conversation) | `TRANSCRIPT_VERSION` = 2 (`{version, deep, history}`; a version-1 transcript counts as Deep) | read as before; one without `deep` is resumed only in Deep | not resumed; a fresh build replaces it |
| `<project>/app/mechanics.json` | `engine::mechanics` (written by the builder, read by `MechanicsCheck`) | `MECHANICS_VERSION` = 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/checks/round-N/self-test.json` | `engine::self_test` (written by the built game's `--self-test`, read by `SelfTestCheck`) | 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/usage.json` | `engine::workspace` (`ProjectUsage`; written by builds and questions) | `USAGE_VERSION` = 1 (`version`; missing = 0) | read as before, saved as current; absent `tokens_incomplete` means no missing token counts were recorded | read as far as its fields are known, never written over |
| `<project>/questions.json` | `engine::questions` (`AskedQuestion`, written by `rebuild_question_ask`) | `QUESTIONS_VERSION` = 1 (`{version, items}`) | read as before | read as far as its fields are known |
| `<project>/pinned/<n>/pin.json` | `engine::pins` (`Pin`) | `PIN_VERSION` = 2 (`version`; missing = 1; 2 records the selected program for a copied direct launch) | brought forward | read as far as its fields are known |
| `<folder>/standin.json` | `engine::tools::standin_tools` (`analysis::standin::StandIn`; written only by `web_standin_build`) | `STANDIN_VERSION` = 1 (`version`, required) | brought forward | refused, with "update Craft Anything", and never written over |
| `<project>/feed.json` | `ai::rebuild::feed` (written by the run) | `FEED_VERSION` = 5 (`{version, items}`; a bare list = 0; 4 adds `PlanDismissed`, a folded plan card; 5 marks open steps stopped at a run's end) | read as before, saved as current | never written over; the conversation is rebuilt from the project for display |

A saved `User` message with pictures is `{text, pictures: [paths]}`, where the pictures are the
copies in the project's `attachments/`; one without pictures stays the plain string earlier
versions wrote.

Other JSON in a project (`assets.json`, `inbox/` except `inbox/build-progress.json` and `inbox/notes/`, `checks/`,
evidence records) has no version yet; add one the first time its shape changes.

## Waiting builder notes

A version-1 note is first saved as `<id>.json`. The builder's hook claims it with a no-replace
link to `<id>.read.json`; that proves hand-over, while recording request state such as the website
language can still fail. A failed claim-directory sync keeps both names. Once request recording
succeeds, the worker syncs the read link, removes its matching waiting name and syncs that removal
before renaming the read file to `<id>.recorded.json`. A failure before that rename keeps the claim
recoverable; completion never leaves a waiting alias beside its marker. The completed marker is
synced before cleanup, and cleanup errors are reported instead of discarded. A completed record
left after a cleanup failure is excluded from replay.

Restart validates the full waiting/claimed/completed set before changing any record. Version-1
waiting and claimed notes with kept original words are recovered in id order. An active and a
completed record sharing an id stop startup even if the active record has no recovery words;
the live hook also refuses to claim that collision. A lone completed record does not replay.
Unreadable entries or newer formats stop startup with the directory unchanged. Legacy claimed
notes carry no evidence that request recording was unfinished, so they remain excluded.

The initial message carries each note's id and original request boundaries. Recovered notes are
merged between those parts in id order, before an unrelated new message. The language choice
and the id of its latest explicit request are saved together in `craft.json`. A delayed older
note can still reach the builder after a restart, but it cannot replace a newer saved language
choice. New request ids advance beyond that saved id and all occupied note filenames; exhaustion
returns an error. Version-6 language choices have no ordering evidence and migrate without an
invented id; the next explicit request establishes their ordering record.

Recovery remains at-least-once: a builder can receive a previously claimed note again after an
interrupted recording. Carried initial notes and notes prepared for a follow-up prompt remain
claimed until their builder turn succeeds; a failed intervening progress save leaves them
recoverable. Claiming uses a no-replace link so an existing read record is preserved for inspection.
In a live run, only a continuous read prefix of the id-ordered queue is recorded, so a claim
arriving during a scan cannot apply a newer language before an earlier request.

## Stand-in cached responses

Version 1 stand-in files retain a recorded 304 when no usable cached body was saved. Those samples
remain in the file for diagnostics, but replay skips them within the best matching path, query and
operation group. If that group has no usable answer, it reports `cached_body_unavailable` instead
of sending a bare 304 to a fresh client. A usable cached body is still stored and replayed as a 200
with `was_not_modified: true`. This interpretation also applies to existing v1 files; the persisted
shape and version are unchanged.

## Newer files

A file from a newer Craft Anything is never downgraded or overwritten, because the fields we don't
know would be lost. `craft.json` and `config.toml` are still read as far as their fields are known,
so the person can see their project or keep their settings; every save returns a `ToolError` that
says to update Craft Anything. A newer `craft.json` whose fields can't be read at all is refused the
same way, and a newer `config.toml` that can't be read gives defaults without touching the file.
A newer `.cany` is refused outright: its evidence layout may have changed in ways we can't
see.

A file that can't be parsed is not "newer": `config.toml` is set aside as `config.toml.broken` and
defaults are used; an unreadable `craft.json` gives a plain-English error when opened and is left
out of the project list.

## Adding a migration step

1. Bump the version constant (`MANIFEST_VERSION`, `SETTINGS_VERSION` or `FORMAT_VERSION`).
2. Append one function to the matching steps list (`MANIFEST_STEPS`, `SETTINGS_STEPS`,
   `FORMAT_STEPS`). It takes the raw document from version n to n + 1: rename, move or fill keys.
   Return `Err` with a plain-English reason when it can't. A compile-time assertion fails if the
   list and the constant disagree.
3. Add a test that reads a literal sample of the old file (keep the existing samples; old files
   must keep loading forever).
4. For `craft.json`, `inbox/build-progress.json`, `mechanics.json` and `self-test.json`, run `cargo xtask schemas` and commit `docs/schemas/`.
   `cargo xtask schemas --check` (and an engine test) fail when the published schema is stale.
