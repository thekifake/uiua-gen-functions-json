@echo off
if not exist "save_dir.txt" (
  set /p saveDir=Where should I send the JSON? 
) else (
  set /p saveDir=<save_dir.txt
) 
echo %saveDir%> save_dir.txt
echo  Checking for updates...
cargo update
echo  Building binaries and running...
cargo run
echo  Converting the JSON to JS...
echo const glyphs= >glyphs.js
type uiua_functions.json>>glyphs.js
echo  Copying...
copy glyphs.js %saveDir%
del glyphs.js
echo  Done
