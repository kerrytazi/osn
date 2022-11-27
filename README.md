
# Prepare

## Install yasm

[Yasm Downloads](https://yasm.tortall.net/Download.html) or [Direct link](http://www.tortall.net/projects/yasm/releases/yasm-1.3.0-win64.exe)

## Install binutils

```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

## Install QEMU (Optional)

[QEMU Binaries for Windows (64 bit)](https://qemu.weilnetz.de/w64/) or [Direct link](https://qemu.weilnetz.de/w64/2022/qemu-w64-setup-20221123.exe)

# Quick debug build
```
$ machines\x86-64\compile_bootloader.bat
$ cargo build
$ rust-objcopy --output-target binary target\x86-64\debug\osn target\x86-64\debug\osn.bin
```

# Test in QEMU

Tested on qemu 2022-11-23 \
QEMU emulator version 7.1.92 (v7.2.0-rc2-11944-g151fae8180-dirty)

```
$ machines\x86-64\debug_boot.bat
```
