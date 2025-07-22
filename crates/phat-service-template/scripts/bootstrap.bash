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

retries=0
set +eo pipefail
while docker logs "$(cat "${self}/../PROJECT_NAME.txt")-db" \
    | grep -q "database system is ready to accept connections"
do
    if [[ "$retries" -gt 10 ]]
    then
        echo ""
        echo "Max retries exceeded, exiting."
        exit 1
    fi
    echo "Waiting for database..."
    retries=$((retries+1))
    sleep 1
done

set +x
echo "===================================================================="
echo "Bootstrap complete! The app is running now, but you need to stop it"
echo "and run ./develop.bash to get live-reloading started."
echo "===================================================================="
SQLX_OFFLINE=true cargo run
