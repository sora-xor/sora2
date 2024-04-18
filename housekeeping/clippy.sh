#!/bin/bash
set -e

if [ "$pr" = true ]; then
    printf "👷‍♂️ starting clippy \n"
    SKIP_WASM_BUILD=1 cargo clippy --features private-net,runtime-benchmarks,try-runtime -- -D warnings
    SKIP_WASM_BUILD=1 cargo clippy --features private-net,ready-to-test,runtime-benchmarks,try-runtime -- -D warnings
    SKIP_WASM_BUILD=1 cargo clippy --features private-net,ready-to-test,wip,runtime-benchmarks,try-runtime --message-format=json -- -D warnings > clippy_report.json
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy --message-format=json > clippy_report.json || exit 0
fi
