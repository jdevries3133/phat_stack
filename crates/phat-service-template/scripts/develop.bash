#!/bin/bash

set -x

cargo watch -x 'run --features live_reload localhost_base_url'
