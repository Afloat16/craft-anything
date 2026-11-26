param([Parameter(Mandatory = $true)][ValidateSet('Probe', 'Engine')][string]$Mode)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$out = $env:CRAFT_VALIDATION_OUTPUT
if (-not $out) { throw 'CRAFT_VALIDATION_OUTPUT is required' }
New-Item -ItemType Directory -Path $out -Force | Out-Null
& rustup toolchain install 1.100.0 --profile minimal --component clippy 2>&1 | Tee-Object -FilePath (Join-Path $out 'toolchain-install.log')
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& rustc +1.100.0 -Vv 2>&1 | Tee-Object -FilePath (Join-Path $out 'rustc-version.log')
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
if (-not (Select-String -LiteralPath (Join-Path $out 'rustc-version.log') -SimpleMatch 'host: x86_64-pc-windows-msvc' -Quiet)) {
    throw 'Expected native x86_64 Windows MSVC toolchain'
}

if ($Mode -eq 'Probe') {
    $started = [DateTime]::UtcNow
    & cargo +1.100.0 run --locked --manifest-path validation/windows-lock-rename/Cargo.toml 2>&1 |
        Tee-Object -FilePath (Join-Path $out 'probe.log')
    $code = $LASTEXITCODE
    [ordered]@{
        scope = 'Native NTFS filesystem observations; not the full application test suite'
        startedUtc = $started.ToString('o')
        finishedUtc = [DateTime]::UtcNow.ToString('o')
        exitCode = $code
        meaning = 'Operation failures are logged observations. Nonzero means setup, baseline or exclusive-lock premise failed.'
    } | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $out 'result.json')
    exit $code
}

$tests = @(
    'workspace::tests::creation_reservation_is_released_after_definite_picked_folder_publication_failure',
    'workspace::tests::deletion_reuses_reserved_cleanup_space_after_removing_the_manifest',
    'workspace::tests::creation_reservation_is_released_after_definite_managed_publication_failure',
    'workspace::tests::managed_publication_failure_keeps_published_or_occupied_reservations_and_the_original_error',
    'workspace::tests::initial_creators_hold_the_coordination_lock_through_publication_and_cleanup',
    'workspace::tests::failed_creation_releases_only_same_file_coordination_spellings',
    'workspace::tests::deletion_keeps_lock_file_aliases_until_the_manifest_is_removed',
    'workspace::tests::deletion_never_returns_to_the_public_inbox_after_claiming_it',
    'workspace::tests::deletion_preserves_inbox_entries_arriving_after_payload_cleanup',
    'workspace::tests::reservation_release_preserves_a_replacement_after_its_last_public_path_check',
    'workspace::tests::failed_initialization_releases_only_its_own_empty_coordination_entries',
    'workspace::tests::failed_initialization_preserves_foreign_or_changed_lock_files',
    'workspace::tests::an_unfinished_copy_destination_refuses_another_project_creator',
    'workspace::tests::copy_cleanup_preserves_files_added_to_its_unpublished_root',
    'workspace::tests::copy_cleanup_preserves_a_replacement_after_identity_validation',
    'workspace::tests::a_failed_initializer_cannot_release_a_deleters_active_inbox',
    'workspace::tests::project_deletion_waits_for_an_active_feed_transaction',
    'workspace::tests::windows_retirement_preserves_a_late_initializers_new_lock_and_the_old_held_inode',
    'workspace::tests::windows_retirement_keeps_both_locks_when_an_initializer_arrives_after_the_empty_check',
    'workspace::tests::windows_retirement_closes_the_captured_directory_before_removing_its_old_lock',
    'workspace::tests::windows_retirement_preserves_new_manifests_and_replacement_inboxes',
    'workspace::tests::windows_copy_cleanup_preserves_a_new_project_after_releasing_its_old_reservation'
)
$results = @()
$failed = $false
foreach ($test in $tests) {
    $name = ($test -split '::')[-1]
    $log = Join-Path $out "$name.log"
    $started = [DateTime]::UtcNow
    & cargo +1.100.0 test --locked -p craft-anything-engine --lib $test -- --exact --nocapture --test-threads=1 2>&1 |
        Tee-Object -FilePath $log
    $code = $LASTEXITCODE
    $executed = [bool](Select-String -LiteralPath $log -Pattern '^running 1 test$' -Quiet)
    $results += [ordered]@{
        test = $test
        startedUtc = $started.ToString('o')
        finishedUtc = [DateTime]::UtcNow.ToString('o')
        exitCode = $code
        exactlyOneTestStarted = $executed
    }
    if ($code -ne 0 -or -not $executed) { $failed = $true }
}
& cargo +1.100.0 clippy --locked -p craft-anything-engine --all-targets -- -D warnings 2>&1 |
    Tee-Object -FilePath (Join-Path $out 'engine-clippy.log')
$clippyCode = $LASTEXITCODE
if ($clippyCode -ne 0) { $failed = $true }
& git diff --exit-code -- Cargo.lock
$lockfileUnchanged = $LASTEXITCODE -eq 0
if (-not $lockfileUnchanged) { $failed = $true }
[ordered]@{
    scope = 'Exactly 22 production cleanup and race tests; no full-suite claim'
    clippyExitCode = $clippyCode
    tests = $results
    productionLockfileUnchanged = $lockfileUnchanged
    failed = $failed
} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $out 'result.json')
if ($failed) { exit 1 }
