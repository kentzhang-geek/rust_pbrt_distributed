DEL src\libs.rs
FOR %%i IN (fbs\*.*) DO flatc -r -o src/ --filename-suffix "" %%i && ECHO "pub mod %%i" >> src\libs.rs 
