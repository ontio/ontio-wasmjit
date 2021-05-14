#ifndef WASMJIT_RUNTIME_H
#define WASMJIT_RUNTIME_H
#include"wasmjit.h"

typedef struct {
  uint8_t _unused[0];
} wasmjit_body_t;

typedef struct {
  wasmjit_bytes_t name;
  wasmjit_body_t* body;
} wasmjit_import_func_t;

typedef struct {
	uint32_t v;
	uint32_t isnone;
	wasmjit_result_t res;
} wasmjit_return;

uint32_t wasmjit_invoke(wasmjit_slice_t name, wasmjit_resolver_t *resolver);
wasmjit_return wasmjit_invoke_args(wasmjit_slice_t name, uint32_t n, wasmjit_resolver_t *resolver);
wasmjit_resolver_t* wasmjit_go_resolver_create(wasmjit_import_func_t* imports, uint32_t num);
wasmjit_bytes_t wasmjit_bytes_from_slice(wasmjit_slice_t s);
#endif
