# Native Windows cleanup candidate validation for #2183

The sole production parent is `609e4ad04deb20ae9df97e298010221072fd53d6`. Its seven changed files cover Engine cleanup, its Windows-only atomicwrites dependency, the minimal lockfile,
required attribution and platform-specific warning fixes relative to feature head `f367fe8fd0f054bbe2320742eac6fd853d6dbf91`.
The validation child adds exactly these nine fork-only files and changes no production source
or lockfile. The feature branch is not advanced by this validation.

The workflow listens only for branch creation in `Afloat16/craft-anything`, with the exact
execution ref `refs/heads/test/r2.9-windows-cleanup-closed-target-20261126`. It uses read-only repository permissions,
disables persisted credentials, and has no release or publishing step. Checkout and artifact
actions retain their reviewed pinned revisions from the earlier native validation.

The single job requires native x86_64 Windows, NTFS for both runner and test temporary paths,
and Rust 1.100.0. It independently executes the 26 exact production regression names below,
requires each command to start exactly one test, and runs two wrong-implementation controls
before restoring the exact source bytes and running Engine all-target Clippy with warnings denied.
Logs, per-test exit codes, OS/filesystem metadata and exact source hashes
are retained even on failure. This does not claim a full Engine or application suite run.

The earlier head was actually reproduced on NTFS in run 45743642250: three selected production
cleanup regressions failed and the occupied/published-manifest control passed. That evidence
belongs to the old head, not this candidate. The independent rename probe is retained here
for provenance but is not rerun by this workflow.

The earlier candidate `7202e344` is abandoned: independent review found its public-path unlink
could delete an entry replaced after the identity check. Its run 45747427145 was cancelled
before native test results completed. It is not part of this candidate's ancestry and provides
no passing repair evidence. This candidate uses an atomic no-replace move, with separate
controls for late source replacement, an occupied private target, and a long parent path.

The two negative controls leave the regression tests unchanged. One temporarily uses
`replace_atomic` to demonstrate that the occupied-target assertion rejects overwriting;
the other temporarily uses the former hard-link/check/public-unlink sequence to demonstrate
that the source-replacement assertion rejects deleting a late entry. Each must actually run
one test, exit 101, and report the expected behavior assertion failure. A compilation error or
test setup failure does not count. The original source bytes are restored in a `finally` block
after each control, and matching hashes plus `git diff` are required before Clippy. These
expected failures are recorded separately and are never counted as passing production tests.

The earlier atomic candidate `527b254e` ran in 45749993882: 23 normal tests passed;
two full-path assertions failed after file contents/identities were checked. Its source-unlink
negative control rejected the incorrect implementation, but the occupied-target control was
masked by its open target handle. Source hashes were restored exactly; Windows Clippy then
reported a pre-existing-in-this-PR boolean guard expression in `same_copy_entry`. That run is
retained as a failed attempt and is not reported as a passing candidate. The intermediate
canonical-assertion run 45752030424 was cancelled once the additional failures were known.

Native probe 45753230848 separately confirmed all four held/closed-target cases. With no
open target handle, `replace_atomic` actually replaced the destination while the source lock
remained held; `move_atomic` rejected it and preserved both identities and bytes. With a held
target identity handle, replacement failed with PermissionDenied (OS 5), explaining the
masked control. The closed case uses a hard-link identity witness without keeping either
name open until after the attempted move. The original held-target regression remains.

This candidate checks complete canonical inspection paths after identity verification,
retains all content/identity/lock assertions, adds the independently witnessed closed-target
test, and fixes the boolean expression without changing its conditions or Unix identity
checks. The replacement negative control runs only the new closed-target test. All 26 normal
tests and both negative controls must complete again on this exact source before publication.

## Selected tests

- `workspace::tests::creation_reservation_is_released_after_definite_picked_folder_publication_failure`
- `workspace::tests::deletion_reuses_reserved_cleanup_space_after_removing_the_manifest`
- `workspace::tests::creation_reservation_is_released_after_definite_managed_publication_failure`
- `workspace::tests::managed_publication_failure_keeps_published_or_occupied_reservations_and_the_original_error`
- `workspace::tests::initial_creators_hold_the_coordination_lock_through_publication_and_cleanup`
- `workspace::tests::failed_creation_releases_only_same_file_coordination_spellings`
- `workspace::tests::deletion_keeps_lock_file_aliases_until_the_manifest_is_removed`
- `workspace::tests::deletion_never_returns_to_the_public_inbox_after_claiming_it`
- `workspace::tests::deletion_preserves_inbox_entries_arriving_after_payload_cleanup`
- `workspace::tests::reservation_release_preserves_a_replacement_after_its_last_public_path_check`
- `workspace::tests::failed_initialization_releases_only_its_own_empty_coordination_entries`
- `workspace::tests::failed_initialization_preserves_foreign_or_changed_lock_files`
- `workspace::tests::an_unfinished_copy_destination_refuses_another_project_creator`
- `workspace::tests::copy_cleanup_preserves_files_added_to_its_unpublished_root`
- `workspace::tests::copy_cleanup_preserves_a_replacement_after_identity_validation`
- `workspace::tests::a_failed_initializer_cannot_release_a_deleters_active_inbox`
- `workspace::tests::project_deletion_waits_for_an_active_feed_transaction`
- `workspace::tests::windows_retirement_preserves_a_source_replaced_after_its_last_identity_check`
- `workspace::tests::windows_retirement_never_overwrites_a_private_target_created_after_its_source_check`
- `workspace::tests::windows_retirement_never_overwrites_a_closed_private_target_created_after_its_source_check`
- `workspace::tests::windows_retirement_cleans_projects_under_long_parent_paths`
- `workspace::tests::windows_retirement_preserves_a_late_initializers_new_lock_and_the_old_held_inode`
- `workspace::tests::windows_retirement_keeps_both_locks_when_an_initializer_arrives_after_the_empty_check`
- `workspace::tests::windows_retirement_closes_the_captured_directory_before_removing_its_old_lock`
- `workspace::tests::windows_retirement_preserves_new_manifests_and_replacement_inboxes`
- `workspace::tests::windows_copy_cleanup_preserves_a_new_project_after_releasing_its_old_reservation`

## Provenance

Runtime fixtures are original empty directories/files generated inside job-owned temporary
locations. The copied helper/probe source and locked dependencies retain their existing
provenance in `ATTRIBUTION.md`. No personal or third-party fixture is loaded. Production tests
use the actual repository `same-file` identities, locks and filesystem helpers.

Only actual completed job results establish a pass. The candidate remains subject to native
validation and independent protocol review before the feature branch can advance.
