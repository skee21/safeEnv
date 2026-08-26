$InstallDir = Join-Path $HOME ".safeEnv\bin"
$ExePath = Join-Path $InstallDir "safeEnv.exe"
$RepoUrl = "https://github.com/skee21/safeEnv/releases/latest/download/safeEnv-windows-amd64.exe"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host "Downloading safeEnv..."
Invoke-WebRequest -Uri $RepoUrl -OutFile $ExePath

Write-Host "Installing to $InstallDir"

$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notmatch [regex]::Escape($InstallDir)) {
    try {
        $NewPath = "$InstallDir;$CurrentPath"
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        Write-Host "Successfully added $InstallDir to your PATH."
    } catch {
        Write-Host "WARNING: Failed to automatically add to PATH." -ForegroundColor Yellow
        Write-Host "Please manually add this directory to your system PATH: $InstallDir" -ForegroundColor Yellow
    }
} else {
    Write-Host "Directory $InstallDir is already in your PATH."
}
Write-Host "Installation complete! Restart your terminal to use safeEnv."