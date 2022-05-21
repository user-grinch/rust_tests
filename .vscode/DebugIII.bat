@echo off
echo --------------------------------------------------
echo Building DebugIII
echo --------------------------------------------------
echo[
call "tools\Setup.bat"
MsBuild CheatMenu.sln /property:Configuration=Debug /t:CheatMenuIII
del %III_DIR%"\CheatMenuIII.asi" /Q
%systemroot%\System32\xcopy /s "bin\CheatMenuIII.asi" %III_DIR% /K /D /H /Y 
