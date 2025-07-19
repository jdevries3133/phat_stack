#!/bin/bash

set -euxo pipefail

tempdir=""
target=""

test_setup() {
    tempdir="$(mktemp -d)"
    target="${tempdir}/new-crate"
    mkdir -p "${target}"
}

test_setup

# Test no-overwriting
echo "item" > "${target}/test"
echo "item" > "${target}/test2"

set +e
output="$(
    cargo run --bin create-phat-app -- \
        --name test \
        --location "${target}" \
        --base-template moderate 2>&1
)"
set -e
if [[ "$output" != *"is not an empty directory"* ]]
then
    echo "FAIL :: did not find expected output"
    exit 1
fi

test_setup
cargo run --bin create-phat-app -- \
    --name test \
    --location "${target}" \
    --base-template moderate

cd "${target}"
cargo test
cargo clippy
