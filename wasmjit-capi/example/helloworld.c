#include "stdio.h"
#include "wasmjit.h"
#include <string.h>

void func_test(wasmjit_chain_context_t *ctx) {

	wasmjit_chain_context_set_gas(ctx, 999);
	uint64_t gas = wasmjit_chain_context_get_gas(ctx);
	if (gas!= 999) {
		printf("wasmjit_chain_context_get_gas failed\n");
	}

	address_t caller = {1};
	wasmjit_chain_context_push_caller(ctx, caller);
	address_t caller2 = {0};
	wasmjit_chain_context_pop_caller(ctx, &caller2);
	for (int i=0;i<20;i++) {
		if (caller[i] != caller2[i]) {
			printf("num:%d, num2:%d\n",caller[i],caller2[i]);
			break;
		}
	}
	wasmjit_bytes_t buf = wasmjit_bytes_new(20);
	wasmjit_chain_context_set_output(ctx,buf);
	wasmjit_bytes_t buf2 = wasmjit_chain_context_take_output(ctx);
	if (*buf.data != *buf2.data){
		printf("wasmjit_chain_context_set_output failed\n");
	}
    char *file_path = "/Users/sss/dev/dockerData/rust_project/ontio-wasmjit/tests/add.wast";
	wasmjit_slice_t name = {data:(uint8_t *)file_path,len:strlen(file_path)};
	wasmjit_bytes_t wasm = wasmjit_test_read_wasm_file(name);
	if (wasm.len == 0){
	    printf("wasmjit_test_read_wasm_file failed\n");
	    return;
	}
	wasmjit_slice_t wasm_slice = wasmjit_bytes_as_slice(wasm);
	wasmjit_module_t *module;
    wasmjit_result_t res = wasmjit_compile(&module,wasm_slice);
    if (res.kind != 0) {
        printf("wasmjit_compile failed\n");
        return;
    }
    wasmjit_resolver_t *resolver = wasmjit_simple_resolver_create();
    if (resolver == NULL) {
        printf("wasmjit_simple_resolver_create failed\n");
        return;
    }
    wasmjit_instance_t *instance;
    res = wasmjit_module_instantiate(module,resolver,&instance);
    if (res.kind != 0) {
        printf("wasmjit_module_instantiate failed\n");
        return;
    }
//    res = wasmjit_instance_invoke(instance,ctx);
//    if (res.kind != 0) {
//        printf("wasmjit_instance_invoke failed\n");
//        return;
//    }

    printf("wasmjit_module_destroy %d\n", module->_unused);
//    wasmjit_module_destroy(module);
//    wasmjit_resolver_destroy(resolver);
//    wasmjit_instance_destroy(instance);
    printf("wasmjit_module_destroy %d\n", module->_unused);
	printf("success\n");
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
	height, &blockhash, timestamp, &txhash, &addr, callers, witness, input, gas_left, 2);

	printf("ctx :%p\n", ctx);
	func_test(ctx);
	return 0;
}

