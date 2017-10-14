#!/bin/bash

# Build binaries for 32-bit and 64-bit Windows and Linux automatically

cargo build --release
cargo build --release --target=i686-unknown-linux-gnu
cargo build --release --target=x86_64-pc-windows-gnu
cargo build --release --target=i686-pc-windows-gnu
echo Success: cargo build --release --target=platform

cp ./target/release/primeutils ./bin/primeutils-x86_64-unknown-linux-gnu
cp ./target/i686-unknown-linux-gnu/release/primeutils ./bin/primeutils-i686-unknown-linux-gnu
cp ./target/x86_64-pc-windows-gnu/release/primeutils.exe ./bin/primeutils-x86_64-pc-windows-gnu.exe
cp ./target/i686-pc-windows-gnu/release/primeutils.exe ./bin/primeutils-i686-pc-windows-gnu.exe
echo Success: cp ./target/platform/executable ./bin/executable-platform

strip ./bin/primeutils-x86_64-unknown-linux-gnu
strip ./bin/primeutils-i686-unknown-linux-gnu
strip ./bin/primeutils-x86_64-pc-windows-gnu.exe
strip ./bin/primeutils-i686-pc-windows-gnu.exe
echo Success: strip ./bin/executable-platform
