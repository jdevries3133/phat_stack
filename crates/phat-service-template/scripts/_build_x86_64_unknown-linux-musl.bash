#!/bin/bash

# This script is invoked by ./build_container.bash and is not intended to be
# ran directly.

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

rustup target add x86_64-unknown-linux-musl
cargo build \
    --release \
    --target x86_64-unknown-linux-musl \
    --features enable_smtp_email
