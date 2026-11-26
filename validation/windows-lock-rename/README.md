# Native Windows review follow-up validation for #2183

The sole production parent is `3dc2c917d748c9f0f2cbcc54a8cf169ca9ee4031`, a source-only child of
`609e4ad04deb20ae9df97e298010221072fd53d6`. The validation child adds exactly nine
fork-only files, changes no production source or lockfile, and does not advance the PR branch.

The workflow runs only on creation of `test/r2.9-windows-review-followup-20261126` in `Afloat16/craft-anything`.
It uses read-only repository permissions, disables persisted credentials, retains the reviewed
checkout and artifact action pins, and has no release step. The job requires native Windows
x86_64, NTFS for runner and test temporary directories, and Rust 1.100.0.

All 26 earlier cleanup/race tests remain selected. Twelve additional or related cases cover
exact-name cleanup refusal, late and nested recovery directories, ordinary personal entries,
legacy inbox copies without writing locks, live and finished publishers, identity changes,
shared-lock compatibility, cleanup warning semantics, and detached case-variant lock links.
Every one of the 38 normal commands must execute exactly one unchanged production test.

The case-variant test models two detached spellings with real hard links to the held Windows
lock inode. It does not claim that the runner enabled the NTFS case-sensitive directory flag.
The denied-write source tests inject the permission result, then perform real filesystem
reads and identity/lock checks; they do not claim a read-only volume was mounted.

Three separate negative controls keep the assertions unchanged. The two existing controls
substitute an overwriting move and the unsafe public-unlink sequence. A third temporarily
restores the old case-folded-first lookup. Each must execute one test, exit 101, and produce
its specific behavior assertion failure, rather than fail to compile or set up a fixture.
The exact original source is restored in `finally` after every mutation. Matching source
hashes and a clean production source/lockfile diff are required before Engine all-target
Clippy with warnings denied. Negative-control failures are recorded separately from normal
production test results.

The previous native result for `609e4ad` is run 45753694271 (26 normal tests, two negative
controls and Clippy passed). That result is historical and is not evidence for this new
candidate. Only this job's completed logs establish the new result; no full-suite claim is
made. The earlier filesystem probe remains for provenance but is not rerun here.

## Selected production tests

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
- `workspace::tests::duplication_reads_a_legacy_inbox_without_creating_a_coordination_file`
- `workspace::tests::an_uncoordinated_legacy_inbox_copy_rejects_publishers_and_identity_changes`
- `workspace::tests::cleanup_names_block_ambiguous_directories_without_omitting_personal_files`
- `workspace::tests::in_place_copy_rejects_cleanup_directories_arriving_after_discovery`
- `workspace::tests::creation_rechecks_cleanup_state_before_publishing_its_manifest`
- `workspace::tests::windows_rollback_distinguishes_detached_coordination_spellings`
- `workspace::tests::duplication_reads_a_legacy_source_on_a_read_only_filesystem`
- `workspace::tests::duplication_reads_an_existing_coordination_file_without_write_access`
- `workspace::tests::an_unlocked_copy_rejects_a_publisher_that_starts_after_discovery`
- `workspace::tests::published_creation_and_copy_return_success_with_a_cleanup_warning`
- `workspace::tests::published_projects_report_cleanup_failures_without_rejecting_the_project`
- `workspace::tests::a_published_copy_returns_its_warning_without_entering_failure_cleanup`

## Provenance

Fixtures are original files and directories generated only in job-owned temporary locations.
No personal data or third-party fixture is loaded. Existing helper/probe provenance and
locked dependencies remain recorded in `ATTRIBUTION.md`. Exact source hashes, source-parent
checks, OS/filesystem metadata, per-test logs, mutation outcomes and restoration checks are
retained even when a job fails.
