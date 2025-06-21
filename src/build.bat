@echo off
if "%1"=="" (
    echo Usage: build.bat ^<filename_without_extension^> [libraries]
    echo Example: build.bat hello_world
    echo Example: build.bat hello_world kernel32.lib
    echo This will build hello_world.asm into hello_world.exe
    goto :end
)

set filename=%1

echo Assembling %filename%.asm...
nasm -f win64 -o %filename%.obj %filename%.asm
if errorlevel 1 (
    echo Assembly failed!
    goto :end
)

echo Linking %filename%.obj...
if "%2"=="" (
    link %filename%.obj /subsystem:console /out:%filename%.exe
) else (
    link %filename%.obj %2 /subsystem:console /out:%filename%.exe
)
if errorlevel 1 (
    echo Linking failed!
    goto :end
)

echo Build successful! Running %filename%.exe...
echo.
%filename%.exe
echo.
echo Exit code: %errorlevel%

:end
pause