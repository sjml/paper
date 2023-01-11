#!/usr/bin/env bash


cd "$(dirname "$0")"

set -e



# assumes you have stylua installed
#   https://github.com/johnnymorganz/stylua
pushd ../resources/project_template/.paper_resources/filters > /dev/null
  echo "Auto-formatting Lua filters..."
  stylua --indent-type=Spaces --indent-width=2 ./*
popd > /dev/null

pushd ../resources/scripts > /dev/null
  echo "Auto-formatting Lua writers..."
  stylua --indent-type=Spaces --indent-width=2 ./*
popd > /dev/null



pushd .. > /dev/null
  echo "Auto-formatting Rust files..."
  cargo fmt
popd > /dev/null
