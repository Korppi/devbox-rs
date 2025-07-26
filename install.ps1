# install.ps1 – Devbox CLI for Windows

Write-Host "Building Devbox with cargo..."
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Ensure Rust and cargo are installed: https://www.rust-lang.org/tools/install"
    exit 1
}

Write-Host "Installing devbox into Cargo bin..."
cargo install --path . --force

# Completions
$completionPath = "$env:USERPROFILE\devbox.ps1"
Write-Host "Generating PowerShell completions to $completionPath..."
.\target\release\devbox.exe completions powershell > $completionPath

# PowerShell profile update
$profilePath = "$PROFILE"
if (-not (Test-Path $profilePath)) {
    New-Item -ItemType File -Path $profilePath -Force | Out-Null
}

if (-not (Get-Content $profilePath | Select-String -Pattern "devbox.ps1")) {
    Write-Host "Adding completion script to PowerShell profile..."
    Add-Content $profilePath "`n. `$env:USERPROFILE\devbox.ps1"
} else {
    Write-Host "Completion script already in PowerShell profile."
}

Write-Host "Done! Restart PowerShell or run '. $env:USERPROFILE\devbox.ps1' to enable completions immediately."
