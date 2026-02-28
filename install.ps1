$ErrorActionPreference = "Stop"

$Repo = "SAMIR897/rfetch"
$AssetInfo = "rfetch-windows-amd64"

Write-Host "Downloading rfetch for Windows..."

# Get latest release tag
$ReleaseParams = @{
    Uri = "https://api.github.com/repos/$Repo/releases/latest"
    UseBasicParsing = $true
}
$LatestRelease = Invoke-RestMethod @ReleaseParams
$LatestTag = $LatestRelease.tag_name

if (-not $LatestTag) {
    Write-Error "Could not find latest release tag."
    exit 1
}

$DownloadUrl = "https://github.com/$Repo/releases/download/$LatestTag/$AssetInfo.zip"
$ZipPath = "$env:TEMP\rfetch.zip"
$ExtractPath = "$env:TEMP\rfetch_extracted"

Write-Host "Downloading $DownloadUrl..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing

Write-Host "Extracting..."
if (Test-Path $ExtractPath) { Remove-Item -Recurse -Force $ExtractPath }
New-Item -ItemType Directory -Force -Path $ExtractPath | Out-Null
Expand-Archive -Path $ZipPath -DestinationPath $ExtractPath -Force

# Define installation directory
$InstallDir = Join-Path $env:USERPROFILE ".cargo\bin"
if (-not (Test-Path $InstallDir)) {
    $InstallDir = "$env:LOCALAPPDATA\Programs\rfetch"
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    }
}

$ExeFile = Get-ChildItem -Path $ExtractPath -Filter "rfetch.exe" -Recurse | Select-Object -First 1

if (-not $ExeFile) {
    Write-Error "Failed to find rfetch.exe in the downloaded archive."
    exit 1
}

Write-Host "Installing to $InstallDir..."
Copy-Item -Path $ExeFile.FullName -Destination (Join-Path $InstallDir "rfetch.exe") -Force

# Add to user PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    Write-Host "Adding $InstallDir to user PATH..."
    $NewPath = if ($UserPath.EndsWith(";")) { "$UserPath$InstallDir" } else { "$UserPath;$InstallDir" }
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Note: You may need to restart your terminal for the PATH changes to take effect."
}

# Cleanup
Remove-Item $ZipPath -Force
Remove-Item -Recurse -Force $ExtractPath

Write-Host "Done! Run 'rfetch' to test it."
