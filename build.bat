@echo off

rem delete the lib files and resources
if not exist bin mkdir bin
del /s /q /f bin\*.lib
del /s /q /f bin\*.exp
del /s /q /f bin\*.def

rem generate the lib files
xcopy /y windefs\libumfpack.def bin
xcopy /y windefs\libopenblas64_.def bin
cd bin
lib /def:libumfpack.def /out:libumfpack.lib /machine:X64
lib /def:libopenblas64_.def /out:libopenblas64_.lib /machine:X64
cd ..

rem 1.) build the version with UMFPACK bindings
cargo clean
set RUSTFLAGS=--cfg umfpack -C target-feature=+crt-static
cargo build --release
copy /y target\release\olcar.dll bin\olcar_withumf.dll

rem 2.) build the version without UMFPACK
cargo clean
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release
copy /y target\release\olcar.dll bin\olcar.dll

rem delete the lib files and resources
del /s /q /f bin\*.lib
del /s /q /f bin\*.exp
del /s /q /f bin\*.def