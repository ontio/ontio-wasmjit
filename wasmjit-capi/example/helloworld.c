#include "stdio.h"
#include "wasmjit.h"

int main() {
	u8x6 val = {0};
    abi_test(1,2,3,4,5,6, (const u8x6 *)&val);

	wasmjit_bytes_t buf = wasmjit_bytes_new(100);
	printf("buf len:%d", buf.len);
	wasmjit_bytes_destroy(buf);

    uint32_t height = 10;
	h256_t blockhash = {0};
	uint64_t timestamp = 23;
	h256_t txhash = {2};
	address_t addr  = {1};
	wasmjit_slice_t  callers = {data:NULL, len:0};
	wasmjit_slice_t  witness = {data:NULL, len:0};
	wasmjit_slice_t  input = {data:NULL, len:0};
	uint64_t gas_left = 456;
	wasmjit_chain_context_t * ctx = wasmjit_chain_context_create(
	height, &blockhash, timestamp, &txhash, &addr, callers, witness, input, gas_left, 2);

	printf("ctx :%p", ctx);
	return 0;
}
