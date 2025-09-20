#!/bin/bash
export PATH="/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/usr/local/go/bin:/root/.go/bin"
echo "Running cargo command with PATH: $PATH"
cargo run --package demo --example 0_guess_number

