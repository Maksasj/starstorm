@echo off
setlocal EnableExtensions DisableDelayedExpansion
for /F %%a in ('echo prompt $E ^| cmd') do (
  set "ESC=%%a"
)
setlocal EnableDelayedExpansion

echo !ESC![91mDeleting previous exe build!ESC![0m
    rmdir /s /q dist\exe

echo !ESC![93mBuilding exe build!ESC![0m
    cargo build --release

echo !ESC![92mDeploying exe build!ESC![0m
    xcopy .\target\release\starstorm.exe .\dist\exe\
    xcopy /i/s/e/f .\assets\ .\dist\exe\assets

echo !ESC![91mDeleting previous web build!ESC![0m
    rmdir /s/q dist\web

echo !ESC![93mBuilding exe build!ESC![0m
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir ./dist/web --target web ./target/wasm32-unknown-unknown/release/starstorm.wasm

echo !ESC![92mDeploying web build!ESC![0m
    xcopy /i/s/e/f .\web\ .\dist\web\
    xcopy /i/s/e/f .\assets\ .\dist\web\assets
