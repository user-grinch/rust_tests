@echo off
echo --------------------------------------------------
echo Building Debug
echo --------------------------------------------------
echo[
call ".vscode\init.bat"
cargo build --release
del %OUT_DIR%"\"%DLL_NAME%".asi" /Q
set "from=%TARGET_DIR%%DLL_NAME%.dll"
set "to=%OUT_DIR%%DLL_NAME%.asi"
xcopy /s "%from%" "%to%"* /I /Q /Y /F