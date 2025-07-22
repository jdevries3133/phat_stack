#!/bin/bash

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

set +eo pipefail
if docker ps --format {{.Names}} | grep -q "$(cat "${self}/../PROJECT_NAME.txt")-db"
then
    set -eo pipefail
    docker rm --force "$(cat "${self}/../PROJECT_NAME.txt")-db"
else
    set -eo pipefail
fi
docker run \
        --name "$(cat "${self}/../PROJECT_NAME.txt")-db" \
        --env POSTGRES_DB="$POSTGRES_DB" \
        --env POSTGRES_USER="$POSTGRES_USER" \
        --env POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        --publish 5432:5432 \
        --detach \
        postgres:15
