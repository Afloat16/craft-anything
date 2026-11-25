# File formats

The table below lists the files that carry a Craft Anything format version and how older and newer
versions are handled. When a file has migration steps, Craft Anything parses it into raw JSON or
TOML, runs the steps up to the current version, then decodes it into Rust types. The step runner
is `crates/engine/src/versioned.rs`.

| File | Owner | Current version | Older files | Newer files |
|---|---|---|---|---|
| `<project>/craft.json` | `engine::workspace` (`Manifest`) | `MANIFEST_VERSION` = 7 (`version`; missing = 0; 3 adds `approvals`, 4 adds the saved builder session's mode `agent_session_deep`, 5 adds the current capture's mode `capture_deep`, 6 adds the toolchain a move to another language was confirmed with, `toolchain`, 7 adds project `generation`; a version-3 session and a version-4 capture migrate as Deep; older generations stay absent) | brought forward, saved as current | opened read-only: saving is refused, also by a build that read the file before a newer one brought it forward (builds before version 6 don't check this, so close them before running a newer one on the same project) |
| `<project>/inbox/build-progress.json` | `engine::workspace::progress` (`BuildProgress`; written only by the rebuild run) | `BUILD_PROGRESS_VERSION` = 2 (`version`; missing = 0; 2 adds `checked`, the round kept while nothing it looked at changes) | brought forward; an absent file starts with no counted turns | the build stops and the record is left unchanged |
| `config.toml` (`settings::config_dir()`) | `engine::settings` (`Settings`) | `SETTINGS_VERSION` = 2 (`version`; missing = 0; 2 moved `[engines.ghidra] enabled` to `deep_mode`) | brought forward | loaded, never saved over |
| `<name>.cany/project.toml` | `engine::project` (`ProjectMeta`) | `FORMAT_VERSION` = 1 (`format_version`, required) | brought forward | refused, with "update Craft Anything" |
| `<project>/transcript.json` | `ai::context` (the API builder's conversation) | `TRANSCRIPT_VERSION` = 2 (`{version, deep, history}`; a version-1 transcript counts as Deep) | read as before; one without `deep` is resumed only in Deep | skipped when resuming (see [Newer files](#newer-files)) |
| `<project>/app/mechanics.json` | `engine::mechanics` (written by the builder, read by `MechanicsCheck`) | `MECHANICS_VERSION` = 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/checks/round-N/self-test.json` | `engine::self_test` (written by the built game's `--self-test`, read by `SelfTestCheck`) | 1 (`version`; missing = 1) | — | the check fails and says so |
| `<project>/usage.json` | `engine::workspace` (`ProjectUsage`; written by builds and questions) | `USAGE_VERSION` = 1 (`version`; missing = 0) | read as before, saved as current; absent `tokens_incomplete` means no missing token counts were recorded | read as far as its fields are known, never written over |
| `<project>/questions.json` | `engine::questions` (`AskedQuestion`, written by `rebuild_question_ask`) | `QUESTIONS_VERSION` = 1 (`{version, items}`) | read as before | read as far as its fields are known |
| `<project>/pinned/<n>/pin.json` | `engine::pins` (`Pin`) | `PIN_VERSION` = 2 (`version`; missing = 1, any other value that is not a whole number is refused; 2 records the selected program for a copied direct launch) | brought forward | read as far as its fields are known, never written over (renaming is refused) |
| `<folder>/standin.json` | `engine::tools::standin_tools` (`analysis::standin::StandIn`; written only by `web_standin_build`) | `STANDIN_VERSION` = 2 (`version`, required; 2 adds each sample's place in the recording, `order`, and `write` for an answer to a write; a version-1 file has neither: its answers replay in turn, and a write gets each one once) | brought forward | refused, with "update Craft Anything", and never written over |
| `<project>/feed.json` | `ai::rebuild::feed` (written by the run) | `FEED_VERSION` = 6 (`{version, items, recovery?}`; a bare list = 0; 4 adds `PlanDismissed`; 5 marks open steps stopped at a run's end; 6 retains the manifest projection used during recovery) | read as before, saved as current | never written over; the conversation is rebuilt from the project for display |
| `<project>/inbox/<id>.delivery.json` | `engine::inbox::delivery` | 1 (`format`, `version`, `id`, `state`; staged or delivered) | absent records leave existing feed answers compatible | unrecognized records are preserved and block a new recorded submission |
| `<project>/inbox/feed-backups/<backup-name>.json` | `engine::feed::backups` | 2 (`format`, `version`, `backup`, `length`, `sha256`; requires retained private file identity) | version-1 final receipts retain historical length/hash proof; absent or unrecognized records establish no ownership | unrecognized records are preserved during recovery and never authorize backup deletion; deleting a project clears its inbox records but keeps unverified backup-shaped entries |
| `<project>/inbox/feed-backups/pending/<backup-name>` and `<backup-name>.json` | `engine::feed::backups` | raw source bytes and a version-2 ownership receipt; the raw identity link remains after publication | version-1 interrupted records retain their same-file check, including a truncated old final receipt | interrupted and completed version-2 publication require the receipt and the same opened file as the private copy; equal bytes in a separate file remain personal |
| `<project>/inbox/feed-recovery/conversation-<millis>-<pid>-<counter>.recovery.json` | `engine::feed::recovery`, requested by the run worker | 1 (`version`, folded `items`, `metadata` with format, source identity and recovery details) | no earlier format | unknown or invalid records are retained without automatic import or removal |

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

Craft Anything does not migrate newer files backward when reading them. What happens when saving
depends on the format. `craft.json` and `config.toml` are still read as far as their fields are known,
so the person can see their project or keep their settings; every save returns a `ToolError` that
says to update Craft Anything. A newer `craft.json` whose fields can't be read at all is refused the
same way, and a newer `config.toml` that can't be read gives defaults without touching the file.
A newer `.cany` is refused outright: its evidence layout may have changed in ways we can't
see.

Before saving `feed.json`, a filesystem read failure also stops the save and keeps the existing path.
Only a path confirmed absent without following symlinks can be created without inspecting an old
version. A fallback remembers that the original wasn't loaded, even if the file becomes readable
later. A live writer may load an actually decoded replacement and replay its unsaved events; a missing, unreadable, damaged or newer replacement cannot clear the original guard. Every application feed save compares its loaded source while holding a project-scoped operating-system lock through replacement. An originally absent path must still be absent. A mismatch retains the old writer and unsaved events for a fresh load; ordinary answer transactions load and apply their reply under the same lock. Version 6 records the damaged source's SHA-256 and the exact manifest request, checks, reply and failure note that produced a recovery prefix. Only a writer that observed that same damaged source can consume its projection; a temporary read failure cannot authorize removing old history. Later manifest updates cannot change the recorded origin. Older feeds have no recovery provenance; matching text alone never establishes it. Readable bytes that cannot be parsed are copied
exactly to an unused `feed.json.broken` (then `.broken.1` through `.broken.15`) before replacement.
Existing backups are never replaced. Recovery completes and syncs raw bytes and a version-2 receipt inside private `inbox/feed-backups/pending/` before publishing a root-level hard link. It publishes the already-synced receipt by an exclusive hard link, then retains the private raw link as file-identity proof. The directory name `pending` therefore also holds completed backups' identity anchors. Version-2 ownership requires that identity as well as matching length and digest, even after final publication. Version-1 final receipts keep their historical length/hash behavior; past identity evidence cannot be reconstructed. A complete version-1 pending receipt can still prove an interrupted old publication whose final JSON was truncated, while an unknown complete final version never falls back to pending evidence. Before public publication, a failed attempt cleans only its own unchanged staging entries and retains the original error if cleanup also fails. Replacements and already-published proof are preserved. If feed replacement fails after a complete backup, a retry reuses that copy only when its ownership proof and current bytes both match the damaged source. When freeing a slot, move the backup and all associated records, including its retained raw link and private receipt, together. Project deletion first moves each selected backup into a fresh `.craft-backup-quarantine-*` folder beside the project records, then verifies the claimed regular file against its ownership proof before unlinking it. A changed entry is restored without replacing another path, or retained in that folder with an error explaining where it is. That folder is outside the records deleted with the project and is never cleaned recursively. Legacy personal files or directories with backup-shaped names survive deletion, are included when duplicating an in-place project and contribute to its app fingerprint; verified generated copies are excluded. Ownership scans skip absent candidates before reading unrelated receipts, keep I/O errors for present uncertain candidates, and check cancellation between 64 KiB hash chunks. A failed backup or receipt stops the save and leaves the source intact;
the recovered conversation names the backup. A newer format is left in place without recovery.

The empty `inbox/feed.lock` coordination file remains in place after each transaction. Lock waits
are bounded to five seconds; request locks precede this feed lock, never the other way around.
Deletion holds the same lock, removes `craft.json` before removing `inbox/`, and retains its guard
through final directory cleanup. Writers check that the project exists before creating coordination
entries and recheck the root directory identity, manifest creation time and generation after acquiring
the lock. New projects and copies receive an operating-system-random 128-bit generation, saved as
32 hexadecimal characters in `craft.json`. Ordinary manifest updates acquire the project lock,
validate the caller's loaded identity, merge the current person-owned fields and replace the manifest
without releasing that guard. They cannot adopt another project's generation or recreate a deleted
manifest. Only initial creation and successful duplication publish a new manifest without an existing
project guard.
Version-5 and older manifests keep an absent generation when read or saved; opening one does not
write it or invent a per-open identity. A later project created in the same directory receives a new
generation even if its creation timestamp repeats. Loaded writers retain that identity; replay and separate
recovery archives reject a different project later created at the same path. Saved-answer merging
checks that identity before adopting any replacement rows. Deletion also compares
its loaded workspace with the project under the lock before removing anything. They do not write
conversation, backup, archive or recorded-answer payloads after deletion. A lock acquisition that starts just before
deletion can leave an empty coordination directory or file; it still fails the check under the lock.
In-place duplication holds the same lock from backup discovery through manifest publication, so
application recovery cannot change ownership during the copy. A readable legacy source with no
inbox can be copied without creating one when the attempted lock creation is denied; owner write
bits do not establish the current caller's access. This fallback checks the same directory and project
identity, and an absent inbox, both before copying and before publishing the destination manifest.
Application publishers retain their inbox, so an inbox appearing during the copy requires a retry
even after that publisher releases its lock. Manifest and backup classification checks still apply. Each backup-shaped file
is classified through the opened handle that will be copied. Equivalent case spellings count as
the reserved slot only when they identify the same file; a distinct personal file is retained.
A new or changed unverified entry requires a retry. Losing a receipt also cannot turn a previously
verified recovery copy into personal app content during the same copy. The destination manifest
is published after copying succeeds. A failed attempt first claims its destination into a fresh private
`.craft-copy-cleanup-*` folder beside it, then checks the held directory identity before recursive
removal. An unexpected replacement is preserved at the reported claimed path. This coordination does not make a directory copy an atomic filesystem snapshot
against external programs that do not take the lock. App fingerprinting also uses the same opened
regular file for ownership classification and content hashing, so recovery cannot substitute a newly
generated backup between those two reads.

If a run's final feed save fails, its worker attempts a separate recovery copy before sending the
`Finished` event. Version 1 keeps the complete folded conversation, the loaded source's state and
SHA-256 where available, and details containing the pending-event count, original manifest history
and save error. Raw tool-event parameters and results are not serialized. Each copy is exclusively
created in private `inbox/feed-recovery/pending/`, uses mode `0600` on Unix, and is completely serialized
and synced before an exclusive hard link publishes its final name. Partial staging files remain
inspectable but do not enter completed-archive discovery or its file/byte budgets. Unix also syncs its
directory and each parent entry, including on a retry after a failed parent sync. A record is limited to 64 MiB and name reservation to 64 attempts. An error keeping the
copy is reported without claiming the complete unsaved conversation reached disk. Retained partial
paths are reported for inspection before moving them aside, and changed entries are marked unverified.
Repeating the same over-limit snapshot can leave another partial, so retry alone is not a cleanup
strategy. A durable final archive remains a usable recovery copy if private staging cleanup fails;
the report keeps its path and a separate cleanup warning. Polling and dropping a run do not retry
persistence or archive writes.

Display loading discovers at most 256 recovery-directory entries, reads at most 64 MiB in total
and adds at most 16 historical notices for recognized records. A live run caches these notices
when it opens. Writer loads, saved conversations and later recovery snapshots exclude them;
legacy saved notices are removed only when their full generated Error shape and archive path match.
Each notice names its exact path and is deduplicated against that path. These notices do not claim the current feed is unsaved and
do not explain a new run's failure. Unknown versions, invalid or oversized files and links are
left alone. Recovery copies are never automatically replayed, overwritten or cleaned up; inspect
their conversation before deciding what to restore, since it may overlap the current feed.

Within a live run, replay also retains the position and saved context of its active streamed message.
An intervening saved answer and the display-only clearing of streaming flags do not split its next
text delta into another message. A replaced or separately appended external message does not become
that stream merely because its text is equal. This context stays in memory and is committed with the
replayed feed when replacement becomes visible; it adds no persisted feed fields. A newly introduced
question remains at its original event boundary during replay, even if loading already appended
its pending row or another client answered it. Repeated Ask events do not move the existing row
or end an active stream when that row exists. If repaired history no longer contains the question,
replay restores it at the event boundary even if its inbox request has expired; the request is not
reopened. The desktop uses the final display snapshot to show persistence warnings independently of
the build outcome, and refreshes only the answered card after a successful retry without consuming
queued workflow events.

Answers are saved in the conversation before they are delivered through the inbox. On Unix, the
feed's containing directory is synced after atomic replacement and before delivery; non-Unix
platforms retain the existing directory-sync no-op. If replacement is visible but directory sync
fails, the loaded source, replayed state and cleared journal adopt those visible bytes, while the
operation still reports the durability error and withholds answer delivery. A later pre-write failure
cannot inherit the earlier visible-write flag. A final run with unconfirmed durability still attempts
a separate recovery archive, including when its event journal is already empty. A save failure
leaves the question pending. A delivery failure also leaves it available for
retry; display views show the pending question and saved reply while writers retain the recorded answer. Submission reserves the request first, acquires and validates the loaded project's guard, rechecks the request file's identity, then stages a versioned receipt, persists the reply and publishes the answer while retaining that guard. Deletion cannot complete between these steps, and a submission prepared after deletion cannot recreate an old delivery record. The waiter marks the receipt delivered before removing an answer. If receipt persistence fails at consumption, the answer file remains as durable delivery evidence. An existing answer's I/O error stops another submission before its persistence callback. A confirmed staged reply whose request expires is shown as undelivered, with its text available for a new request. If delivery evidence cannot be read, the display keeps the words and says delivery is unknown, without suggesting another submission. Display snapshots retain the opened feed identity, length and modification time and recheck them after reading receipt state. A changed snapshot shows its earlier reply as unconfirmed rather than attributing a later retry's delivery to it; refreshing loads the latest reply. This check covers application atomic replacement, not arbitrary external in-place writes that preserve metadata. Ordinary temporary inbox answers without feed persistence do not create receipts.

A confirmed pending reply keeps its retry card through unrelated feed writes in the same project;
it never gains delivery confirmation from the stale snapshot. A different project at the same path
cannot reuse that card. The display recheck opens only regular files without blocking on a FIFO.
After consumption, Unix directory sync confirms the delivered receipt before either request or
answer is unlinked. A failed receipt write or sync retains both files while returning the actual
accepted answer; discovery does not offer that answer for resubmission.

A complete version-2 raw/receipt pair left in private backup staging can resume publication when
both records match the damaged source and their opened identities remain unchanged. This can use
the last available backup slot without creating another private copy. Unverifiable pairs remain in
place with both exact paths in the error, and version-1 ownership compatibility is unchanged.

Timeout and cancellation withdraw under the same request-file lock as submission. If unlinking
the request fails, the waiter truncates and syncs its locked file as an empty withdrawal marker.
Pending discovery ignores that marker and submission rejects it even after directory access is
restored. A checked wait reports an I/O error if withdrawal cannot be confirmed; it does not report
that failure as a person's refusal or a completed timeout. The compatibility `wait` API logs such
an error and returns no answer; production callers use the checked path.

The API builder skips newer transcripts when resuming and leaves the file in place. If a fresh
build later saves a transcript to the same path, it replaces the existing file.

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
