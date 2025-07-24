#!/bin/bash

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

container_name="$(cat "${self}/../PROJECT_NAME.txt")-db"

set +eo pipefail
if docker ps --format {{.Names}} | grep -q "$container_name"
then
    set -eo pipefail
    docker rm --force "$container_name"
else
    set -eo pipefail
fi
docker run \
        --name "$container_name" \
        --env POSTGRES_DB="$POSTGRES_DB" \
        --env POSTGRES_USER="$POSTGRES_USER" \
        --env POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        --publish 5432:5432 \
        --detach \
        postgres:15

set +eo pipefail

retries=0
echo "Waiting for DB startup..."
while ! docker logs "$container_name" | grep -q "database system is ready to accept connections"
do
    echo -n "."
    sleep 1

    retries=$((retries+1))
    if [[ "$retries" -gt 10 ]]
    then
        echo "Too many retries; check \`docker logs $container_name\`"
        exit 1
    fi
done

echo START DB DONE
