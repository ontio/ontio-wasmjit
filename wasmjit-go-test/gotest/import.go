package gotest

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -lwasmjit_go_test -ldl -lc -lm
#include "wasmjit_runtime.h"
extern uint32_t add(void *context, uint32_t x, uint32_t y);
*/
import "C"
import (
	"errors"
	"github.com/stretchr/testify/assert"
	"testing"
	"unsafe"
)

type Imports struct {
	imports []C.wasmjit_import_func_t
}

func (imports *Imports) Append(importfunc C.wasmjit_import_func_t) {
	imports.imports = append(imports.imports, importfunc)
}

func (imports *Imports) Num() int {
	return len(imports.imports)
}

func NewWasmJitImportFunc(importName string, cgoPointer unsafe.Pointer) (C.wasmjit_import_func_t, error) {
	ptr := []byte(importName)
	if len(ptr) == 0 {
		return C.wasmjit_import_func_t{}, errors.New("import name error")
	}

	nameslice := C.wasmjit_slice_t{
		data: ((*C.uint8_t)((unsafe.Pointer)(&ptr[0]))),
		len:  C.uint32_t(len(ptr)),
	}

	return C.wasmjit_import_func_t{
		name: C.wasmjit_bytes_from_slice(nameslice),
		body: (*C.wasmjit_body_t)(cgoPointer),
	}, nil
}

//export add
func add(context unsafe.Pointer, x C.uint32_t, y C.uint32_t) C.uint32_t {
	return x + y
}

func testImportAdd(t *testing.T) {
	var imports Imports
	importfunc, err := NewWasmJitImportFunc("add", C.add)
	assert.Nil(t, err)
	imports.Append(importfunc)
	wasmImports := make([]C.wasmjit_import_func_t, imports.Num())
	C.wasmjit_go_resolver_create(((*C.wasmjit_import_func_t)((unsafe.Pointer)(&wasmImports[0]))), C.uint32_t(imports.Num()))
}
