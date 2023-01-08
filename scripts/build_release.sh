#!/usr/bin/env bash

cd "$(dirname "$0")"
cd ..

cargo build --release --target=x86_64-apple-darwin
cargo build --release --target=aarch64-apple-darwin

mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/rust-paper \
    target/x86_64-apple-darwin/release/rust-paper \
    target/aarch64-apple-darwin/release/rust-paper



rm -rf dist

mkdir -p dist/macos
cp target/universal-apple-darwin/release/rust-paper ./dist/macos/
