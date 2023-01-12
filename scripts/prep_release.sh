#!/usr/bin/env bash

cd "$(dirname "$0")"
cd ..

PREFIX_X86=/usr/local/share
PREFIX_ARM=/opt/homebrew/share
RESOURCES_DIR_NAME=sjml-paper

PAPER_RESOURCES_DIR=$PREFIX_X86/$RESOURCES_DIR_NAME cargo build --release --target=x86_64-apple-darwin
PAPER_RESOURCES_DIR=$PREFIX_ARM/$RESOURCES_DIR_NAME cargo build --release --target=aarch64-apple-darwin

mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/paper \
    target/x86_64-apple-darwin/release/paper \
    target/aarch64-apple-darwin/release/paper



rm -rf dist

mkdir -p dist/macos/{bin,share,etc}
cp target/universal-apple-darwin/release/paper ./dist/macos/bin/
cp -R resources/project_template ./dist/macos/share/
cp -R resources/scripts ./dist/macos/share/
cp -R resources/completions ./dist/macos/etc

VERSION_TAG="${VERSION_TAG:-$(./scripts/get_version.sh)}"
OUT_NAME=paper-$VERSION_TAG-macos-universal
cd dist
mv macos $OUT_NAME
cd $OUT_NAME
tar cvf ../$OUT_NAME.tar.gz ./*