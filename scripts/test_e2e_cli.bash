#!/bin/bash

set -euxo pipefail

mkdir -p target/end-to-end-test-target
export CARGO_TARGET_DIR="$(pwd)/target/end-to-end-test-target"

start_dir="$(pwd)"
tempdir=""
target=""

docker ps -aq | xargs docker rm -f
set +o pipefail
lsof -i -P -n \
    | grep LISTEN \
    | grep 8000 \
    | awk '{ print $2 }' \
    | xargs --no-run-if-empty kill
set -o pipefail

test_setup() {
    tempdir="$(mktemp -d)"
    target="${tempdir}/new-crate"
    mkdir -p "${target}"
}

test_cleanup() {
    cd "${start_dir}"
}

test_no_overwriting() {
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
    test_cleanup
}

test_clippy() {
    test_setup
    cargo run --bin create-phat-app -- \
        --name test \
        --location "${target}" \
        --base-template moderate

    cd "${target}"
    SQLX_OFFLINE=1 cargo clippy
    test_cleanup
}

test_unittest() {
    test_setup
    cargo run --bin create-phat-app -- \
        --name test \
        --location "${target}" \
        --base-template moderate

    SQLX_OFFLINE=1 cargo test
    test_cleanup
}

test_script_meta_before() {
    test_setup
    cargo run --bin create-phat-app -- \
        --name test \
        --location "${target}" \
        --base-template moderate
    cd "${target}"

}

test_script_meta_after() {
    test_cleanup
}

test_bootstrap() {
    test_script_meta_before
    ./scripts/bootstrap.bash 2>&1 > boot_out &
    boot_pid="$!"

    feeling="happy"
    tries=0
    while [ -z "$(grep "listening on 0.0.0.0:8000" boot_out)" ]
    do
        tries=$((tries+1))
        if [[ "$tries" -gt 10 ]]
        then
            feeling="sad"
            break
        fi
        echo "waiting for bootstrap..."
        sleep 1
    done
    kill "$boot_pid"
    set +e
    wait "$boot_pid"
    set -e
    test_script_meta_after
    if [ "$feeling" = "sad" ]
    then
        return 1
    fi
}

test_start_stop_db() {
    test_script_meta_before
    ./scripts/start_development_database.bash
    ./scripts/stop_development_database.bash
    test_script_meta_after
}

main() {
    test_clippy
    test_unittest
    test_bootstrap
    test_start_stop_db
}

main
