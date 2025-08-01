#!/bin/bash

set -euxo pipefail

mkdir -p target/end-to-end-test-target
export CARGO_TARGET_DIR="$(pwd)/target/end-to-end-test-target"

start_dir="$(pwd)"
tempdir=""
target=""

docker ps -aq | xargs docker rm -f

cleanup_phat_port() {
    set +o pipefail
    lsof -i -P -n \
        | grep LISTEN \
        | grep 8000 \
        | awk '{ print $2 }' \
        | xargs --no-run-if-empty kill
    set -o pipefail
}
cleanup_phat_port

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
    ./scripts/bootstrap.bash > boot_out 2>&1 &
    boot_pid="$!"

    feeling="happy"
    tries=0
    while [ -z "$(grep "listening on 0.0.0.0:8000" boot_out)" ]
    do
        tries=$((tries+1))
        if [[ "$tries" -gt 100 ]]
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
    cleanup_phat_port
    if [ "$feeling" = "sad" ]
    then
        return 1
    fi
}

test_start_stop_db() {
    test_script_meta_before
    ./scripts/start_development_database.bash
    echo 'select 1' > script.sql
    ./scripts/shell_development_database.bash -f script.sql
    ./scripts/stop_development_database.bash
    test_script_meta_after
}

# DEPENDENCY: test_bootstrap
test_build_container() {
    test_script_meta_before
    ./scripts/_build_x86_64_unknown-linux-musl.bash
    mkdir -p target/x86_64-unknown-linux-musl/release
    mv \
        "${CARGO_TARGET_DIR}/x86_64-unknown-linux-musl/release/phat_stack" \
        target/x86_64-unknown-linux-musl/release/phat_stack
    ./scripts/_build_x86_64_container.bash
    test_script_meta_after
}

# DEPENDENCY: test_bootstrap
test_develop() {
    test_script_meta_before
    ./scripts/develop.bash 2>&1 > dev_out &
    dev_pid="$!"

    feeling="happy"
    tries=0
    while [ -z "$(grep "listening on 0.0.0.0:8000" dev_out)" ]
    do
        tries=$((tries+1))
        if [[ "$tries" -gt 100 ]]
        then
            feeling="sad"
            break
        fi
        echo "waiting for dev startup..."
        sleep 1
    done
    kill "$dev_pid"
    set +e
    wait "$dev_pid"
    set -e
    cleanup_phat_port
    test_script_meta_after
    if [ "$feeling" = "sad" ]
    then
        return 1
    fi
}

main() {
    test_clippy
    test_unittest
    test_start_stop_db
    test_bootstrap
    test_build_container
    test_develop
    echo "✅ tests passed"
}

main
