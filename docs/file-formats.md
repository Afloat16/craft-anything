# File formats

Every file Craft Anything saves carries a version number. Reading a file runs the same three steps:
parse it into a raw document (a JSON value or a TOML table), run the migration steps from the file's
version up to the current one, then turn the document into Rust types. The step runner is
`crates/engine/src/versioned.rs`.

| File | Owner | Current version | Older files | Newer files |
|---|---|---|---|---|
| `<project>/craft.json` | `engine::workspace` (`Manifest`) | `MANIFEST_VERSION` = 5 (`version`; missing = 0; 3 adds `approvals`, 4 adds the saved builder session's mode `agent_session_deep`, 5 adds the current capture's mode `capture_deep`; a version-3 session and a version-4 capture migrate as Deep) | brought forward, saved as current | opened read-only: saving is refused |
| `<project>/inbox/build-progress.json` | `engine::workspace::progress` (`BuildProgress`; written only by the rebuild run) | `BUILD_PROGRESS_VERSION` = 2 (`version`; missing = 0; 2 adds `checked`, the round kept while nothing it looked at changes) | brought forward; an absent file starts with no counted turns | the build stops and the record is left unchanged |
| `config.toml` (`settings::config_dir()`) | `engine::settings` (`Settings`) | `SETTINGS_VERSION` = 2 (`version`; missing = 0; 2 moved `[engines.ghidra] enabled` to `deep_mode`) | brought forward | loaded, never saved over |
| `<name>.cany/project.toml` | `engine::project` (`ProjectMeta`) | `FORMAT_VERSION` = 1 (`format_version`, required) | brought forward | refused, with "update Craft Anything" |
| `<project>/transcript.json` | `ai::context` (the API builder's conversation) | `TRANSCRIPT_VERSION` = 2 (`{version, deep, history}`; a version-1 transcript counts as Deep) | read as before; one without `deep` is resumed only in Deep | not resumed; a fresh build replaces it |
| `<project>/app/mechanics.json` | `engine::mechanics` (written by the builder, read by `MechanicsCheck`) | `MECHANICS_VERSION` = 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/checks/round-N/self-test.json` | `engine::self_test` (written by the built game's `--self-test`, read by `SelfTestCheck`) | 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/usage.json` | `engine::workspace` (`ProjectUsage`; written by builds and questions) | `USAGE_VERSION` = 1 (`version`; missing = 0) | read as before, saved as current; absent `tokens_incomplete` means no missing token counts were recorded | read as far as its fields are known, never written over |
| `<project>/questions.json` | `engine::questions` (`AskedQuestion`, written by `rebuild_question_ask`) | `QUESTIONS_VERSION` = 1 (`{version, items}`) | read as before | read as far as its fields are known |
| `<project>/pinned/<n>/pin.json` | `engine::pins` (`Pin`) | `PIN_VERSION` = 2 (`version`; missing = 1; 2 records the selected program for a copied direct launch) | brought forward | read as far as its fields are known |
| `<folder>/standin.json` | `engine::tools::standin_tools` (`analysis::standin::StandIn`; written only by `web_standin_build`) | `STANDIN_VERSION` = 1 (`version`, required) | brought forward | refused, with "update Craft Anything", and never written over |
| `<project>/feed.json` | `ai::rebuild::feed` (written by the run) | `FEED_VERSION` = 6 (`{version, items, recovery?}`; a bare list = 0; 4 adds `PlanDismissed`; 5 marks open steps stopped at a run's end; 6 retains the manifest projection used during recovery) | read as before, saved as current | never written over; the conversation is rebuilt from the project for display |
| `<project>/inbox/<id>.delivery.json` | `engine::inbox::delivery` | 1 (`format`, `version`, `id`, `state`; staged or delivered) | absent records leave existing feed answers compatible | unrecognized records are preserved and block a new recorded submission |
| `<project>/inbox/feed-backups/<backup-name>.json` | `engine::feed::backups` | 1 (`format`, `version`, `backup`, `length`, `sha256`) | no receipt means no ownership of a backup-shaped path | unrecognized records are preserved during recovery and never authorize backup deletion; deleting a project clears its inbox records but keeps unverified backup-shaped entries |

A saved `User` message with pictures is `{text, pictures: [paths]}`, where the pictures are the
copies in the project's `attachments/`; one without pictures stays the plain string earlier
versions wrote.

Other JSON in a project (`assets.json`, `inbox/` except the versioned records listed above, `checks/`,
evidence records) has no version yet; add one the first time its shape changes.

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

Before saving `feed.json`, a filesystem read failure also stops the save and keeps the existing path.
Only a path confirmed absent without following symlinks can be created without inspecting an old
version. A fallback remembers that the original wasn't loaded, even if the file becomes readable
later. A live writer may load an actually decoded replacement and replay its unsaved events; a missing, unreadable, damaged or newer replacement cannot clear the original guard. The recovery save compares that decoded source's digest with its existing pre-save disk read. A mismatch retains the old writer and unsaved events for a fresh load; this check is not a filesystem lock. Version 6 records the damaged source's SHA-256 and the exact manifest request, checks, reply and failure note that produced a recovery prefix. Only a writer that observed that same damaged source can consume its projection; a temporary read failure cannot authorize removing old history. Later manifest updates cannot change the recorded origin. Older feeds have no recovery provenance; matching text alone never establishes it. Readable bytes that cannot be parsed are copied
exactly to an unused `feed.json.broken` (then `.broken.1` through `.broken.15`) before replacement.
Existing backups are never replaced. A private ownership receipt records each generated copy's name, length and digest. When all backup slots are occupied, move a backup and its associated receipt together before retrying. Project deletion first moves each selected backup into a fresh `.craft-backup-quarantine-*` folder beside the project records, then verifies the claimed regular file against its receipt before unlinking it. A changed entry is restored without replacing another path, or retained in that folder with an error explaining where it is. That folder is outside the records deleted with the project and is never cleaned recursively. Legacy personal files or directories with backup-shaped names survive deletion and are included when duplicating an in-place project; verified generated copies are excluded. A failed backup or receipt stops the save and leaves the source intact;
the recovered conversation names the backup. A newer format is left in place without recovery.

Answers are saved in the conversation before they are delivered through the inbox. A save failure
returns an error and leaves the question pending. A delivery failure also leaves it available for
retry; display views show the pending question and saved reply while writers retain the recorded answer. A versioned delivery receipt is staged before persistence and marked delivered before the waiter removes an answer. If receipt persistence fails at consumption, the answer file remains as durable delivery evidence. An existing answer's I/O error stops another submission before its persistence callback. A confirmed staged reply whose request expires is shown as undelivered, with its text available for a new request. If delivery evidence cannot be read, the display keeps the words and says delivery is unknown, without suggesting another submission. Ordinary temporary inbox answers without feed persistence do not create receipts.

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
