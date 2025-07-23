#!/bin/bash

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

docker buildx build \
    --load \
    --platform linux/amd64 \
    --tag "$(cat "${self}/../PROJECT_NAME.txt")-dev" .
