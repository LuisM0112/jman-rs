$ErrorActionPreference = "Stop"

Write-Host "Installing jman..."

$repo = "LuisM0112/jman-rs"
$binary = "jman.exe"
$installDir = "$env:LOCALAPPDATA\Programs\jman"

if (!(Test-Path $installDir)) {
  New-Item -ItemType Directory -Path $installDir | Out-Null
}

$latest = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"

$asset = $latest.assets | Where-Object { $_.name -like "*windows*" -or $_.name -like "*win*" } | Select-Object -First 1

if (!$asset) {
  Write-Error "No Windows binary found in latest release"
  exit 1
}

$downloadUrl = $asset.browser_download_url
$destination = Join-Path $installDir $binary

Write-Host "Downloading $downloadUrl"
Invoke-WebRequest -Uri $downloadUrl -OutFile $destination

$path = [Environment]::GetEnvironmentVariable("Path", "User")

if ($path -notlike "*$installDir*") {
  [Environment]::SetEnvironmentVariable("Path", "$path;$installDir", "User")
  Write-Host "Added jman to PATH"
}

Write-Host "Installation completed!"
Write-Host "Restart your terminal and run: jman --version"
