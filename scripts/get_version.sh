#!/bin/bash

cd "$(dirname "$0")"
cd ..

HEAD_TAG=$(git tag -l --points-at HEAD)

tag=""

if [[ ${#HEAD_TAG} -ne 0 ]]; then
  echo $HEAD_TAG
  exit 0
fi

HEAD_REV=$(git rev-parse --short HEAD)
PORCELAIN=$(git status --porcelain)

tag="$tag$HEAD_REV"

if [[ ${#PORCELAIN} -ne 0 ]]; then
  tag="$tag+dev"
fi

# tag=$tag-$(date -u +"%Y-%m-%dT%H:%M:%SZ")

echo $tag
