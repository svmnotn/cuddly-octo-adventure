@echo off
echo Setting variables
set TEMPDIR=C:\COATempBuild
set LOCAL=%cd%
echo Running build sequence
msbuild /nologo /v:m /t:Clean /p:Configuration=Release cuddly-octo-adventure.sln
msbuild /nologo /v:m /t:Build /p:Configuration=Release cuddly-octo-adventure.sln
echo Project Built
echo Creating temporary location
rd /s /q %TEMPDIR%
md %TEMPDIR%
echo Moving files
copy /B "LICENSE" %TEMPDIR%
copy /B "README.md" %TEMPDIR%
copy /B "Core\bin\Release\COA Core.dll" %TEMPDIR%
copy /B "Core\bin\Release\Newtonsoft.Json.dll" %TEMPDIR%
copy /B "ContentCreator\bin\Release\COA Content Creator.exe" %TEMPDIR%
copy /B "Game\bin\Release\Cuddly Octo Adventure.exe" %TEMPDIR%
echo Files moved
echo Making Zip
7z a -tzip cuddly-octo-adventure.zip %TEMPDIR%\*
copy /B %TEMPDIR%\cuddly-octo-adventure.zip %LOCAL%
echo Done making zip
echo Removing temporary location
rd /s /q %TEMPDIR%
echo Done
