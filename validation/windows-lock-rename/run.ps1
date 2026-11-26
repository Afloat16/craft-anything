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
    'workspace::tests::windows_retirement_preserves_a_source_replaced_after_its_last_identity_check',
    'workspace::tests::windows_retirement_never_overwrites_a_private_target_created_after_its_source_check',
    'workspace::tests::windows_retirement_cleans_projects_under_long_parent_paths',
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
$sourcePath = Join-Path (Get-Location).Path 'crates/engine/src/workspace.rs'
$originalBytes = [System.IO.File]::ReadAllBytes($sourcePath)
$originalHash = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash
$productionSource = [System.Text.Encoding]::UTF8.GetString($originalBytes).Replace("`r`n", "`n")
$oldMove = @'
        crate::feed::check_coordination_entry(&public, held)?;
        before_move();
        // This is one Windows move without replacement, not link followed by public unlink.
        // A source changed after the check is moved but preserved; an occupied target is
        // never overwritten. The original OS lock remains held through either result.
        atomicwrites::move_atomic(&public, &retained).map_err(|error| ToolError::io(retained.display(), error))?;
        crate::feed::check_coordination_entry(&retained, held)
'@
$unsafeUnlink = @'
        std::fs::hard_link(&public, &retained).map_err(|error| ToolError::io(retained.display(), error))?;
        crate::feed::check_coordination_entry(&public, held)?;
        before_move();
        std::fs::remove_file(&public).map_err(|error| ToolError::io(public.display(), error))?;
        crate::feed::check_coordination_entry(&retained, held)
'@
$controls = @(
    @{
        name = 'negative-overwrite-private-target'
        test = 'workspace::tests::windows_retirement_never_overwrites_a_private_target_created_after_its_source_check'
        match = 'atomicwrites::move_atomic(&public, &retained)'
        replacement = 'atomicwrites::replace_atomic(&public, &retained)'
    },
    @{
        name = 'negative-unlink-replaced-public-source'
        test = 'workspace::tests::windows_retirement_preserves_a_source_replaced_after_its_last_identity_check'
        match = $oldMove.Replace("`r`n", "`n")
        replacement = $unsafeUnlink.Replace("`r`n", "`n")
    }
)
$negativeResults = @()
foreach ($control in $controls) {
    $match = $control.match
    $first = $productionSource.IndexOf($match, [StringComparison]::Ordinal)
    if ($first -lt 0 -or $productionSource.IndexOf($match, $first + $match.Length, [StringComparison]::Ordinal) -ge 0) {
        throw "Negative control must alter exactly one production expression: $($control.name)"
    }
    $log = Join-Path $out "$($control.name).log"
    $started = [DateTime]::UtcNow
    try {
        $modified = $productionSource.Replace($match, $control.replacement)
        [System.IO.File]::WriteAllText($sourcePath, $modified, [System.Text.UTF8Encoding]::new($false))
        $test = $control.test
        & cargo +1.100.0 test --locked -p craft-anything-engine --lib $test -- --exact --nocapture --test-threads=1 2>&1 |
            Tee-Object -FilePath $log
        $code = $LASTEXITCODE
        $executed = [bool](Select-String -LiteralPath $log -Pattern '^running 1 test$' -Quiet)
        $failedOne = [bool](Select-String -LiteralPath $log -Pattern '^test result: FAILED\. 0 passed; 1 failed;' -Quiet)
        $unexpectedSuccess = [bool](Select-String -LiteralPath $log -SimpleMatch 'Result::unwrap_err()' -Quiet)
        $caught = $code -eq 101 -and $executed -and $failedOne -and $unexpectedSuccess
    }
    finally {
        [System.IO.File]::WriteAllBytes($sourcePath, $originalBytes)
    }
    $restored = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash -eq $originalHash
    $negativeResults += [ordered]@{
        name = $control.name
        test = $control.test
        startedUtc = $started.ToString('o')
        finishedUtc = [DateTime]::UtcNow.ToString('o')
        exitCode = $code
        exactlyOneTestStarted = $executed
        exactlyOneBehaviorFailure = $failedOne -and $unexpectedSuccess
        rejectedWrongImplementation = $caught
        originalSourceRestored = $restored
    }
    if (-not $caught -or -not $restored) { $failed = $true }
}
& git diff --exit-code -- crates/engine/src/workspace.rs Cargo.lock
$productionUnchanged = $LASTEXITCODE -eq 0
if (-not $productionUnchanged) { $failed = $true }
& cargo +1.100.0 clippy --locked -p craft-anything-engine --all-targets -- -D warnings 2>&1 |
    Tee-Object -FilePath (Join-Path $out 'engine-clippy.log')
$clippyCode = $LASTEXITCODE
if ($clippyCode -ne 0) { $failed = $true }
& git diff --exit-code -- Cargo.lock
$lockfileUnchanged = $LASTEXITCODE -eq 0
if (-not $lockfileUnchanged) { $failed = $true }
[ordered]@{
    scope = 'Exactly 25 production cleanup and race tests, plus two separately recorded wrong-implementation controls; no full-suite claim'
    clippyExitCode = $clippyCode
    tests = $results
    negativeControls = $negativeResults
    originalSourceSha256 = $originalHash.ToLowerInvariant()
    finalSourceSha256 = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant()
    productionSourceRestoredBeforeClippy = $productionUnchanged
    productionLockfileUnchanged = $lockfileUnchanged
    failed = $failed
} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $out 'result.json')
if ($failed) { exit 1 }
