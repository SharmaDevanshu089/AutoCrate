@echo off
setlocal enabledelayedexpansion

:: Define target folder in user profile
set "TARGET_DIR=%USERPROFILE%\.AutoCrate"
set "SOURCE_FILE=USE_INSTALLER_FIRST"
set "DEST_FILE=autocrate.exe"

:: Step 1: Create folder
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%" 2>nul
    if errorlevel 1 (
        echo [ERROR] Failed to create folder %TARGET_DIR%
        exit /b 1
    )
)

:: Step 2: Copy file
copy /Y "%~dp0%SOURCE_FILE%" "%TARGET_DIR%\%DEST_FILE%" >nul
if errorlevel 1 (
    echo [ERROR] Failed to copy %SOURCE_FILE% to %TARGET_DIR%
    exit /b 2
)

:: Step 3: Add folder to PATH (user environment only)
:: First check if already in PATH
echo %PATH% | find /I "%TARGET_DIR%" >nul
if errorlevel 1 (
    setx PATH "%PATH%;%TARGET_DIR%" >nul
    if errorlevel 1 (
        echo [ERROR] Failed to add %TARGET_DIR% to PATH
        exit /b 3
    ) else (
        echo [INFO] Added %TARGET_DIR% to PATH
    )
) else (
    echo [INFO] %TARGET_DIR% already in PATH
)

echo [SUCCESS] Installation complete. You can now run "autocrate" from terminal.
exit /b 0