@echo off

qemu-system-x86_64 -drive file=target\x86-64\debug\osn.bin,format=raw,index=0,media=disk

rem -S -gdb tcp::9000
