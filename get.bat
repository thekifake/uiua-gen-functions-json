@echo off
if not exist "save_dir.txt" (
  set /p saveDir=Where should I send the JSON? 
) else (
  set saveDir=<save_dir.txt
) 
echo %saveDir%> save_dir.txt
echo  Checking for updates...
cargo update
echo  Building binaries and running...
cargo run
echo  Copying uiua_functions.json to the proper directory...
copy uiua_functions.json %saveDir%
