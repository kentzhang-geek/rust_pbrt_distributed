FOR %%i IN (fbs\*.*) DO flatc -r -o src/ --filename-suffix "" %%i
