# PowerShell script to build and package the release

Write-Host "Building release..." -ForegroundColor Green
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Create distribution directory
$distDir = "AnagramApp-release"
if (Test-Path $distDir) {
    Remove-Item -Recurse -Force $distDir
}
New-Item -ItemType Directory -Path $distDir | Out-Null

# Copy files
Write-Host "Copying files..." -ForegroundColor Green
Copy-Item "target\release\AnagramApp.exe" -Destination $distDir
Copy-Item -Recurse "assets" -Destination $distDir

# Create ZIP
$zipName = "AnagramApp-windows-x64.zip"
if (Test-Path $zipName) {
    Remove-Item $zipName
}

Write-Host "Creating ZIP archive..." -ForegroundColor Green
Compress-Archive -Path $distDir -DestinationPath $zipName

Write-Host "Done! Created $zipName" -ForegroundColor Green
Write-Host "Contents:" -ForegroundColor Cyan
Write-Host "  AnagramApp.exe"
Write-Host "  assets/"
Write-Host "    japanese_dictionary.csv"
Write-Host "    NotoSansJP-Regular.ttf"
