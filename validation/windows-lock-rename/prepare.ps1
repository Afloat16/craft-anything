$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$baseline = 'f3715a5539b28ae24ba244c7dc5f6f865257f3cf'
$out = $env:CRAFT_VALIDATION_OUTPUT
if (-not $out) { throw 'CRAFT_VALIDATION_OUTPUT is required' }
New-Item -ItemType Directory -Path $out -Force | Out-Null

$driveRoot = [System.IO.Path]::GetPathRoot([System.IO.Path]::GetFullPath($env:RUNNER_TEMP))
$drive = [System.IO.DriveInfo]::new($driveRoot)
$runtimeTemp = [System.IO.Path]::GetTempPath()
$tempDrive = [System.IO.DriveInfo]::new([System.IO.Path]::GetPathRoot($runtimeTemp))
$os = Get-CimInstance Win32_OperatingSystem
$revision = (& git rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Cannot read checked-out commit' }
$parent = (& git rev-parse 'HEAD^').Trim()
if ($LASTEXITCODE -ne 0 -or $parent -ne $baseline) { throw 'Validation must be one commit directly on the reviewed baseline' }
$changed = @(& git diff --name-only $baseline HEAD)
if ($LASTEXITCODE -ne 0) { throw 'Cannot compare the validation tree' }
foreach ($path in $changed) {
    if ($path -ne '.github/workflows/validation-2183-windows.yml' -and -not $path.StartsWith('validation/windows-lock-rename/')) {
        throw "Production content changed in the validation branch: $path"
    }
}

$paths = @(
    '.github/workflows/validation-2183-windows.yml',
    'Cargo.toml', 'Cargo.lock',
    'crates/platform/Cargo.toml', 'crates/platform/src/fs.rs',
    'crates/engine/Cargo.toml', 'crates/engine/src/workspace.rs',
    'crates/engine/src/feed/transaction.rs'
)
$paths += @(Get-ChildItem -LiteralPath 'validation/windows-lock-rename' -File -Recurse | ForEach-Object {
    [System.IO.Path]::GetRelativePath((Get-Location).Path, $_.FullName).Replace('\', '/')
})
$hashes = @($paths | Sort-Object -Unique | ForEach-Object {
    [ordered]@{ path = $_; sha256 = (Get-FileHash -LiteralPath $_ -Algorithm SHA256).Hash.ToLowerInvariant() }
})
[ordered]@{
    observedUtc = [DateTime]::UtcNow.ToString('o')
    baseline = $baseline
    commit = $revision
    parent = $parent
    productionUnchanged = $true
    filesystem = $drive.DriveFormat
    driveRoot = $driveRoot
    runtimeTemp = $runtimeTemp
    runtimeTempFilesystem = $tempDrive.DriveFormat
    os = $os.Caption
    osVersion = $os.Version
    osBuild = $os.BuildNumber
    processArchitecture = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString()
    sourceHashes = $hashes
} | ConvertTo-Json -Depth 8 | Tee-Object -FilePath (Join-Path $out 'environment-and-source.json')
if ($drive.DriveFormat -ne 'NTFS') { throw "The probe needs native NTFS; found $($drive.DriveFormat)" }
if ($tempDrive.DriveFormat -ne 'NTFS') { throw "Production test temp files need native NTFS; found $($tempDrive.DriveFormat)" }
if (-not $IsWindows) { throw 'This validation requires native Windows' }
