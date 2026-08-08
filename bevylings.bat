@echo off
rem bevylings launcher (Windows)
rem usage: bevylings list / next / run 0301 / test 0301 / verify 0301 / hint 0301 / status / reset
setlocal
cd /d "%~dp0"
if not exist target\debug\bevylings.exe (
    echo First build: compiling CLI and Bevy dependencies, please wait...
    cargo build -p bevylings || exit /b 1
)
target\debug\bevylings.exe %*
