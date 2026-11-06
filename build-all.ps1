# 双模式打包：online + offline
# 用法：npm run build:all  或  powershell -ExecutionPolicy Bypass -File build-all.ps1

$ErrorActionPreference = 'Continue'
$ProjectDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ProjectDir

$OutDir   = Join-Path $ProjectDir 'src-tauri\target\release\bundle\nsis'
$SetupExe = Join-Path $OutDir 'html-ppt-editor_0.1.0_x64-setup.exe'

# === 1. Online 模式 ===
Write-Host "`n========== 构建 ONLINE 模式 ==========" -ForegroundColor Cyan
$sw1 = [Diagnostics.Stopwatch]::StartNew()
& npm run tauri -- build --config src-tauri/tauri.conf.online.json
if ($LASTEXITCODE -ne 0) { Write-Error "Online 构建失败"; exit $LASTEXITCODE }
Move-Item -LiteralPath $SetupExe -Destination (Join-Path $OutDir 'html-ppt-editor_0.1.0_x64-online-setup.exe') -Force
$sw1.Stop()
$s1 = [math]::Round((Get-Item (Join-Path $OutDir 'html-ppt-editor_0.1.0_x64-online-setup.exe')).Length / 1MB, 2)
Write-Host ("✓ ONLINE: {0} MB  (耗时 {1})" -f $s1, $sw1.Elapsed.ToString('mm\:ss')) -ForegroundColor Green

# === 2. Offline 模式 ===
Write-Host "`n========== 构建 OFFLINE 模式 ==========" -ForegroundColor Cyan
$sw2 = [Diagnostics.Stopwatch]::StartNew()
& npm run tauri -- build
if ($LASTEXITCODE -ne 0) { Write-Error "Offline 构建失败"; exit $LASTEXITCODE }
Move-Item -LiteralPath $SetupExe -Destination (Join-Path $OutDir 'html-ppt-editor_0.1.0_x64-offline-setup.exe') -Force
$sw2.Stop()
$s2 = [math]::Round((Get-Item (Join-Path $OutDir 'html-ppt-editor_0.1.0_x64-offline-setup.exe')).Length / 1MB, 2)
Write-Host ("✓ OFFLINE: {0} MB  (耗时 {1})" -f $s2, $sw2.Elapsed.ToString('mm\:ss')) -ForegroundColor Green

# === 汇总 ===
Write-Host "`n========== 全部完成 ==========" -ForegroundColor Green
Get-ChildItem $OutDir -File | Where-Object { $_.Name -match 'setup' } |
    Select-Object Name,@{n='MB';e={[math]::Round($_.Length/1MB,2)}},LastWriteTime |
    Format-Table -AutoSize
