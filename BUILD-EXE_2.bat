@echo off
title ClippedbyShowdown - Build Real .EXE
echo ============================================================
echo  ClippedbyShowdown - BUILDING REAL .EXE
echo  This runs ONCE on your Windows PC. It needs internet
echo  the first time to install Rust + Tauri.
echo ============================================================
echo.
where cargo >nul 2>nul
if %errorlevel% neq 0 (
  echo [1/4] Installing Rust - this takes a few minutes...
  echo       If this fails, go to https://rustup.rs and install Rust, then run this file again.
  winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements
  if %errorlevel% neq 0 (
    echo.
    echo Rust install failed. Install Rust from https://rustup.rs then run BUILD-EXE.bat again.
    pause
    exit /b 1
  )
  call "%USERPROFILE%\.cargo\bin\rustup.exe" default stable
  set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
)
echo [2/4] Rust OK.
where cargo >nul 2>nul
if %errorlevel% neq 0 set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
echo [3/4] Installing Tauri CLI...
cargo install tauri-cli --version "^2" --locked
if %errorlevel% neq 0 (
  echo Tauri CLI install failed. Run BUILD-EXE.bat again.
  pause
  exit /b 1
)
echo [4/4] Building .EXE - this takes 3-8 minutes first time. Do not close this window.
cd /d "%~dp0src-tauri"
cargo tauri build
if %errorlevel% neq 0 (
  echo.
  echo BUILD FAILED. Copy the red error above and send it to me.
  pause
  exit /b 1
)
echo.
echo ============================================================
echo  DONE - YOUR REAL .EXE IS HERE:
echo  src-tauri\target\release\ClippedbyShowdown.exe
echo  And installer: src-tauri\target\release\bundle\nsis\
echo ============================================================
echo Opening the folder...
explorer "%~dp0src-tauri\target\release"
start "" "%~dp0src-tauri\target\release\ClippedbyShowdown.exe"
pause
