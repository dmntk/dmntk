#!/usr/bin/env bash

export RUSTFLAGS="-Zinstrument-coverage"

export LLVM_PROFILE_FILE="./target/debug/coverage/profraw/%p-%m.profraw"

cargo clean

cargo test

grcov ./target/debug/coverage/profraw/ --binary-path ./target/debug/ --source-dir . --output-type html --branch --ignore-not-existing --ignore "*build*" --ignore "*cargo*" --ignore "*dmntk*" --output-path ./target/debug/coverage/html/

grcov ./target/debug/coverage/profraw/ --binary-path ./target/debug/ --source-dir . --output-type lcov --branch --ignore-not-existing --ignore "*build*" --ignore "*cargo*" --ignore "*dmntk*" --output-path ./target/debug/coverage/coverage.lcov

lcov --summary ./target/debug/coverage/coverage.lcov