#!/bin/bash
set -e
set -x

rustc --version

if rustup component add clippy;
then
	cargo clippy --all ;
else
	echo 'Skipping clippy';
fi

cargo fmt --all -- --check
cargo build --release 
cargo test

cd wasmjit-harness
cargo run -- -q

