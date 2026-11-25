param([Parameter(Mandatory = $true)][ValidateSet('Probe', 'Engine')][string]$Mode)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$out = $env:CRAFT_VALIDATION_OUTPUT
if (-not $out) { throw 'CRAFT_VALIDATION_OUTPUT is required' }
New-Item -ItemType Directory -Path $out -Force | Out-Null
& rustup toolchain install 1.100.0 --profile minimal 2>&1 | Tee-Object -FilePath (Join-Path $out 'toolchain-install.log')
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
    'workspace::tests::managed_publication_failure_keeps_published_or_occupied_reservations_and_the_original_error'
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
& git diff --exit-code -- Cargo.lock
$lockfileUnchanged = $LASTEXITCODE -eq 0
if (-not $lockfileUnchanged) { $failed = $true }
[ordered]@{
    scope = 'Exactly four existing production engine tests; no full-suite claim'
    tests = $results
    productionLockfileUnchanged = $lockfileUnchanged
    failed = $failed
} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $out 'result.json')
if ($failed) { exit 1 }
