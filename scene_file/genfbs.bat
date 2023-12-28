FOR %%i IN (scene_flat\fbs\*.*) DO flatc -r -o src/ --gen-object-api --filename-suffix "" %%i
pause
