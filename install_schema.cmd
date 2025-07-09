@echo off
setlocal

set SCHEMA_FILE=org.my_gtk_app.MyGtkApp.gschema.xml
set SCHEMA_DIR=%ProgramData%\glib-2.0\schemas
set COMPILED_FILE=%SCHEMA_DIR%\gschemas.compiled

echo Installing GSettings schema...

if not exist "%SCHEMA_FILE%" (
    echo [ERROR] Schema file not found: %SCHEMA_FILE%
    exit /b 1
)

if not exist "%SCHEMA_DIR%" (
    mkdir "%SCHEMA_DIR%"
)

copy /Y "%SCHEMA_FILE%" "%SCHEMA_DIR%" >nul
if errorlevel 1 (
    echo [ERROR] Failed to copy schema file.
    exit /b 1
)

where glib-compile-schemas >nul 2>&1
if errorlevel 1 (
    echo [ERROR] glib-compile-schemas not found in PATH.
    exit /b 1
)

glib-compile-schemas "%SCHEMA_DIR%"
if errorlevel 1 (
    echo [ERROR] Failed to compile schemas.
    exit /b 1
)

if exist "%COMPILED_FILE%" (
    echo Schema installed and compiled successfully.
    exit /b 0
) else (
    echo [ERROR] gschemas.compiled not found.
    exit /b 1
)
