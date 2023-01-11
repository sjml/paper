#!/usr/bin/env bash

cd "$(dirname "$0")"
cd ..

PREFIX_X86=/usr/local/share
PREFIX_ARM=/opt/homebrew/share
RESOURCES_DIR_NAME=sjml-paper

PAPER_RESOURCES_DIR=$PREFIX_X86/$RESOURCES_DIR_NAME cargo build --release --target=x86_64-apple-darwin
PAPER_RESOURCES_DIR=$PREFIX_ARM/$RESOURCES_DIR_NAME cargo build --release --target=aarch64-apple-darwin

mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/rust-paper \
    target/x86_64-apple-darwin/release/rust-paper \
    target/aarch64-apple-darwin/release/rust-paper



rm -rf dist

mkdir -p dist/macos
cp target/universal-apple-darwin/release/rust-paper ./dist/macos/
