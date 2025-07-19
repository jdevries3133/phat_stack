#!/bin/bash

set -euxo pipefail

tempdir="$(mktemp -d)"
target="${tempdir}/new-create"
mkdir -p "${target}"
cargo run --bin cli -- \
    --name test \
    --location "${target}" \
    --base-template moderate

cd "${target}"
cargo test
cargo clippy
