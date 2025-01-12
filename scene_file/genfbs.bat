%% git submodule update --recursive --remote
cd scene_flat && python gen.py -rust && cd ..
copy /Y scene_flat\generated\rust\* src\
pause
