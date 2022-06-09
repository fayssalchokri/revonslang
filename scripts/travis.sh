#!/bin/bash
set -ex

export RUST_BACKTRACE=1

# Split the tests into two on travis so to avoid timing out

declare -a PROJECTS=(
    revonslang_codegen
    revonslang_base
    revonslang_parser
    revonslang_check
    revonslang_completion
    revonslang_vm
    revonslang_format
    revonslang
    revonslang_c-api
    revonslang_doc
    revonslang_repl
)

if [ -z $NO_NORMAL_TEST ]; then
    cargo test --features "test" --all "$@"
    cargo test --features "test" --all --bins "$@"
    cargo test --features "test" --all --examples "$@"
    cargo test --features "test" --all --benches "$@"
    cargo test --features "test" -p revonslang_parser --benches "$@"
    echo "" | cargo run --features "test" --example 24
    cargo run --features "test" --example marshalling

    echo "TRAVIS_RUST_VERSION=$TRAVIS_RUST_VERSION"
    (echo $TRAVIS_RUST_VERSION | grep nightly) && cargo test --features "test nightly" -p revonslang --test compiletest "$@"

    # Check each crate individually so that features from one do not affect another which would break publish
    for PROJECT in "${PROJECTS[@]}"
    do
        cargo check --package "${PROJECT}" --no-default-features "$@"
    done
fi
