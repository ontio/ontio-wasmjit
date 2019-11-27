
cargo build

cp ../../target/debug/libontio_wasmjit.so .

gcc -g helloworld.c -L . -lontio_wasmjit -o test

LD_LIBRARY_PATH="$LD_LIBRARY_PATH:." ./test

