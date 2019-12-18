#!/bin/bash
set -e
set -x

rustc --version

if rustup component add clippy;
then
	cargo clippy --all -- -D clippy::all;
else
	echo 'Skipping clippy';
fi

cargo fmt --all -- --check
cargo build --all
cargo test --all

cd wasmjit-harness
cargo run -- -q

