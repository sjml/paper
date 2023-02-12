#!/usr/bin/env bash

cd "$(dirname "$0")"
cd ../examples

set -e

cargo build
PAPER=$(pwd)/../target/debug/paper

for d in *; do
  if [ -d $d ]; then
    pushd $d
      rm -rf output
      echo "   Formatting..."
      $PAPER fmt
      echo "   Building docx..."
      $PAPER build --output-format docx --docx-revision 2
      echo "   Building latex+pdf..."
      $PAPER build --output-format latex+pdf

      diffs=$(git diff output)
      if [[ ${#diffs} -le 0 ]]; then
        cd .paper_data
          git checkout . 2> /dev/null
        cd ..
      fi
    popd
  fi
done
