#!/bin/bash

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

# This is broken into two sub-scripts for convenience of end-to-end testing
# that happens upstream in the PHAT stack repo. You can feel free to combine
# these into one script if you wish.
"${self}/_build_x86_64_unknown-linux-musl.bash"
"${self}/_build_x86_64_container.bash"
