#!/usr/bin/env bash

set -e

cd "$(dirname "$0")"
cd ..

PORCELAIN=$(git status --porcelain)
if [[ ${#PORCELAIN} -ne 0 ]]; then
  echo "ERROR: cannot release from dirty directory"
  # exit 1
fi

CARGO_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
export VERSION_TAG=v$CARGO_VERSION

git fetch --tags origin
if [ $(git rev-parse --verify "$VERSION_TAG^{tag}" >/dev/null) ]; then
  echo "ERROR: tag $VERSION_TAG already exists!"
  exit 1
fi

git tag -m "tagging ${VERSION_TAG%%*( )}" $VERSION_TAG main
git push origin $VERSION_TAG

COMMIT=$(git rev-parse --verify "$TAG_VERSION^{tag}" >/dev/null)

./scripts/prep_release.sh

gh release create \
  $VERSION_TAG \
  ./dist/*.tar.gz \
  --draft
  --target "$COMMIT"

