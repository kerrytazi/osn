
# Prepare

## Install binutils

Required to compose bootloader with rust code.

```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

# Build

## Debug x86-64

```
$ cargo +nightly build --target machines/x86-64/x86-64.json -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem
```

## Release x86-64

Same as `Debug` but with `--release` at the end.

## Compose

```
$ rust-objcopy --output-target binary target/x86-64/debug/osn/target/x86-64/debug/osn.bin
```

# Debug

## Install QEMU

[QEMU Binaries for Windows (64 bit)](https://qemu.weilnetz.de/w64/) or [Direct link](https://qemu.weilnetz.de/w64/2022/qemu-w64-setup-20221123.exe)

## Run QEMU x86-64

Tested on qemu 2022-11-23 \
QEMU emulator version 7.1.92 (v7.2.0-rc2-11944-g151fae8180-dirty)

```
$ qemu-system-x86_64 -drive file=target/x86-64/debug/osn.bin,format=raw,index=0,media=disk
```

# VS Code development

## Settings

For debugging you might need to enable breakpoints in any file: \
`"debug.allowBreakpointsEverywhere": true`

## Plugins

[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for codding. It is just better.

[CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging.

## Compile, Compose, Debug

`Ctrl+Shift+B` \
`> QEMU Debug x86-64`

By default it will wait on the very beginning of BIOS. Just set breakpoint anywhere in your `rust` code and press `Continue`.
