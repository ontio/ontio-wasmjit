#include "stdio.h"
#include "wasmjit.h"
#include <string.h>

int func_test(wasmjit_chain_context_t *ctx) {
	wasmjit_chain_context_set_gas(ctx, 99900000);
	uint64_t gas = wasmjit_chain_context_get_gas(ctx);
	if (gas!= 99900000) {
		printf("wasmjit_chain_context_get_gas failed\n");
		return -1;
	}

	address_t caller = {1};
	wasmjit_chain_context_push_caller(ctx, caller);
	address_t caller2 = {0};
	wasmjit_chain_context_pop_caller(ctx, &caller2);
	for (int i=0;i<20;i++) {
		if (caller[i] != caller2[i]) {
			printf("num:%d, num2:%d\n",caller[i],caller2[i]);
            return -1;
		}
	}
	wasmjit_bytes_t buf = wasmjit_bytes_new(20);
	wasmjit_chain_context_set_output(ctx,buf);
	wasmjit_bytes_t buf2 = wasmjit_chain_context_take_output(ctx);
	if (*buf.data != *buf2.data){
		printf("wasmjit_chain_context_set_output failed\n");
        return -1;
	}
	wasmjit_bytes_destroy(buf2);

    char *file_path = "./wast/helloworld.wast";
	wasmjit_slice_t name = {data:(uint8_t *)file_path,len:strlen(file_path)};
	wasmjit_bytes_t wasm = wasmjit_test_read_wasm_file(name);
	if (wasm.len == 0){
	    printf("wasmjit_test_read_wasm_file failed\n");
        return -1;
	}
	wasmjit_slice_t wasm_slice = wasmjit_bytes_as_slice(wasm);
    wasmjit_result_t res = wasmjit_validate(wasm_slice);
    if (res.kind != 0) {
        printf("wasmjit_validate failed");
        return -1;
    }
	wasmjit_module_t *module;
    res = wasmjit_compile(&module,wasm_slice);
    if (res.kind != 0) {
        printf("wasmjit_compile failed\n");
        return -1;
    }
    wasmjit_resolver_t *resolver = wasmjit_simple_resolver_create();
    if (resolver == NULL) {
        printf("wasmjit_simple_resolver_create failed\n");
        return -1;
    }
    wasmjit_instance_t *instance;
    res = wasmjit_module_instantiate(module,resolver,&instance);
    if (res.kind != 0) {
        printf("wasmjit_module_instantiate failed\n");
        return -1;
    }

    res = wasmjit_instance_invoke(instance,ctx);
    if (res.kind != 0) {
        printf("wasmjit_instance_invoke failed:%d\n", res.kind);
        wasmjit_bytes_destroy(res.msg);
        return -1;
    }
    wasmjit_bytes_destroy(wasm);
    wasmjit_module_destroy(module);
    wasmjit_instance_destroy(instance);
	printf("success\n");
    return 0;
}

int main() {
	u8x6 val = {0};
    abi_test(1,2,3,4,5,6, (const u8x6 *)&val);

	wasmjit_bytes_t buf = wasmjit_bytes_new(100);
	if (buf.len != 100){
	    printf("wasmjit_bytes_new failed\n");
	}
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
	height, &blockhash, timestamp, &txhash, callers, witness, input, gas_left, 2);

	printf("ctx :%p\n", ctx);
	return func_test(ctx);
}
