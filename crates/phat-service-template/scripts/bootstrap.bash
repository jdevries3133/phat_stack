#!/bin/bash

# Bootstrap the development environment.

set -euxo pipefail

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

if [ ! -f .env ]
then
    cp env-template .env
fi

set +x
source .env
set -x

"${self}/start_development_database.bash"

echo "===================================================================="
echo "Bootstrap complete! The app is running now, but you need to stop it"
echo "and run ./develop.bash to get live-reloading started."
echo "===================================================================="
SQLX_OFFLINE=true cargo run
