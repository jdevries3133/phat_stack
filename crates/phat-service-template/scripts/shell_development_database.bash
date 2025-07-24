#!/bin/bash

# Use `psql` to connect to the development database.

which psql
if [ $? != 0 ]
then
    echo "Fatal: command \`psql\` not found. Install the PostgreSQL command-line on your system."
    exit 1
fi

set -euo pipefail
source .env
set -x

# https://stackoverflow.com/a/246128
self="$(dirname "$(readlink -f "$0")")"

PGPASSWORD="$POSTGRES_PASSWORD" \
    psql -U "$POSTGRES_USER" -h 0.0.0.0 "$POSTGRES_DB" $@
