FOR %%i IN (fbs\*.*) DO flatc -r -o src/ --gen-object-api --filename-suffix "" %%i
