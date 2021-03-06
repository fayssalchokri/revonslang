#!/bin/bash
# Modified from https://github.com/nikomatsakis/lalrpop/blob/master/version.sh
#
# A script to bump the version number on all Cargo.toml files etc in
# an atomic fashion.

set -e

if [ "$1" == "" ]; then
    echo "Usage: version.sh <new-version-number>"
    exit 1
fi

VERSION=$(
    ls **/Cargo.toml | \
        xargs grep "# REVONSLANG" | \
        perl -p -e 's/.*version = "([0-9.]+)"[^#]+# REVONSLANG/$1/' |
        sort |
        uniq)

if [ $(echo $VERSION | wc -w) != 1 ]; then
    echo "Error: inconsistent versions detected across Cargo.toml files!"
    echo "$VERSION"
    exit 1
fi

echo "Found consistent version $VERSION"

perl -p -i -e 's/version *= *"[0-9.]+"([^#]+)# REVONSLANG/version = "'$1'"$1# REVONSLANG/' \
     $(ls **/Cargo.toml Cargo.toml)

perl -p -i -e 's/^revonslang *= *"[0-9.]+"/revonslang = "'$1'"/' \
     README.md

perl -p -i -e 's/[0-9][0-9.]+([^#]+)# revonslang/'$1'$1# revonslang/' \
     $(ls **/src/lib.rs src/lib.rs)

# Update Cargo.lock
cargo fetch

git add .
CHANGES=$(git diff  HEAD --unified=0 CHANGELOG.md | tail +6 | sed -e 's/^\+//')
git commit -m "Version ${1}"$'\n\n'"${CHANGES}"
git tag "v${1}"
