$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$out = $env:CRAFT_VALIDATION_OUTPUT
if (-not $out) { throw 'CRAFT_VALIDATION_OUTPUT is required' }
New-Item -ItemType Directory -Path $out -Force | Out-Null
& rustup toolchain install 1.100.0 --profile minimal 2>&1 | Tee-Object -FilePath (Join-Path $out 'toolchain-install.log')
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& rustc +1.100.0 -Vv 2>&1 | Tee-Object -FilePath (Join-Path $out 'rustc-version.log')
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
if (-not (Select-String -LiteralPath (Join-Path $out 'rustc-version.log') -SimpleMatch 'host: x86_64-pc-windows-msvc' -Quiet)) { throw 'Native Windows MSVC required' }
$started = [DateTime]::UtcNow
& cargo +1.100.0 run --locked --manifest-path validation/windows-lock-rename/Cargo.toml 2>&1 |
    Tee-Object -FilePath (Join-Path $out 'target-probe.log')
$code = $LASTEXITCODE
$completed = [bool](Select-String -LiteralPath (Join-Path $out 'target-probe.log') -SimpleMatch 'PROBE_COMPLETE' -Quiet)
[ordered]@{
    scope = 'Four isolated NTFS target-replacement observations; no Engine-suite claim'
    startedUtc = $started.ToString('o')
    finishedUtc = [DateTime]::UtcNow.ToString('o')
    exitCode = $code
    completedFourCases = $completed
} | ConvertTo-Json | Tee-Object -FilePath (Join-Path $out 'result.json')
if ($code -ne 0) { exit $code }
if (-not $completed) { exit 1 }
