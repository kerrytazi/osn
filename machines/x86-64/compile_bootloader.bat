@echo off

mkdir target\x86-64\bootloader\

yasm machines\x86-64\bootloader.asm ^
	--arch=x86 ^
	--machine=amd64 ^
	--oformat=elf ^
	--objfile=target\x86-64\bootloader\bootloader.o

rem yasm boot.asm ^
rem 	--arch=x86 ^
rem 	--machine=amd64 ^
rem 	--oformat=bin ^
rem 	--list=bootloader.list ^
rem 	--objfile=bootloader.bin
