#!/bin/bash

# the compiled libraries go into the `bin` folder
mkdir -p bin

# 1.) build the version with UMFPACK bindings: libolcar_withumf
cargo clean
export RUSTFLAGS="--cfg umfpack -C target-feature=-crt-static"
cargo build --release
LIB=./target/release/libolcar
DIST=libolcar_withumf
if [ -e "$LIB.so" ]
then
    LIB="$LIB.so"
    DIST="$DIST.so"
else
    LIB="$LIB.dylib"
    DIST="$DIST.dylib"
fi
cp $LIB "./bin/$DIST"


# 2.) build the version without UMFPACK: libolcar_withumf
cargo clean
export RUSTFLAGS="-C target-feature=-crt-static"
cargo build --release
LIB=./target/release/libolcar
DIST=libolcar
if [ -e "$LIB.so" ]
then
    LIB="$LIB.so"
    DIST="$DIST.so"
else
    LIB="$LIB.dylib"
    DIST="$DIST.dylib"
fi
cp $LIB "./bin/$DIST"
