$ErrorActionPreference = "Stop"

$GitHubRepo = "LuisM0112/jman-rs"
$BinaryName = "jman.exe"

$UserHome = [Environment]::GetFolderPath("UserProfile")
$JmanBin  = Join-Path $UserHome ".jman\bin"
$JavaHome = Join-Path $UserHome ".jman\current"

New-Item -ItemType Directory -Force -Path $JmanBin | Out-Null

try {
  $releaseUrl = "https://api.github.com/repos/$GitHubRepo/releases/latest"
  $release = Invoke-RestMethod -Uri $releaseUrl -Headers $headers
}
catch {
  Write-Host "No stable release found, falling back to latest pre-release..."

  $releasesUrl = "https://api.github.com/repos/$GitHubRepo/releases"
  $releases = Invoke-RestMethod -Uri $releasesUrl -Headers $headers

  if (-not $releases -or $releases.Count -eq 0) {
      throw "No releases or pre-releases found in repository"
  }

  $release = $releases | Select-Object -First 1
}

$asset = $release.assets | Where-Object {
  $_.name -match "windows"
} | Select-Object -First 1

if (-not $asset) {
  throw "Windows binary not found in the latest release"
}

$destination = Join-Path $JmanBin $BinaryName
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $destination

function Add-ToUserPath($newPath) {
  $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
  if ($currentPath -notlike "*$newPath*") {
    [Environment]::SetEnvironmentVariable(
      "PATH",
      "$currentPath;$newPath",
      "User"
    )
  }
}

Add-ToUserPath $JmanBin

[Environment]::SetEnvironmentVariable("JAVA_HOME", $JavaHome, "User")

Add-ToUserPath (Join-Path $JavaHome "bin")

Write-Host "Installed jman in: $JmanBin"
Write-Host "JAVA_HOME set: $JavaHome"
Write-Host "PATH updated (requires new session)"
