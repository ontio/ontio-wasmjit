package gotest

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -lwasmjit_go_test -ldl -lc -lm
#include "wasmjit_runtime.h"
extern uint32_t addtest(void *context, uint32_t x, uint32_t y);
extern uint32_t panictest(void *context, uint32_t x, uint32_t y);
extern uint32_t subtest(void *context, uint32_t x, uint32_t y);
*/
import "C"
import (
	"errors"
	"fmt"
	"github.com/stretchr/testify/assert"
	"sync"
	"testing"
	"unsafe"
)

const (
	wasmjit_result_success  uint = 0
	wasmjit_result_err_trap uint = 4
)

type Imports struct {
	imports []C.wasmjit_import_func_t
}

func jitErr(err error) C.wasmjit_result_t {
	s := err.Error()
	ptr := []byte(s)
	errslice := C.wasmjit_slice_t{
		data: ((*C.uint8_t)((unsafe.Pointer)(&ptr[0]))),
		len:  C.uint32_t(len(ptr)),
	}

	msg := C.wasmjit_bytes_from_slice(errslice)
	return C.wasmjit_result_t{
		kind: C.wasmjit_result_kind(wasmjit_result_err_trap),
		msg:  msg,
	}
}

func (imports *Imports) Append(importfunc C.wasmjit_import_func_t) {
	imports.imports = append(imports.imports, importfunc)
}

func (imports *Imports) Destroy() {
	for _, imp := range imports.imports {
		C.wasmjit_bytes_destroy(imp.name)
	}
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

func WasmJitInvoke(name string, resolver *C.wasmjit_resolver_t) {
	ptr := []byte(name)
	s := C.wasmjit_slice_t{
		data: (*C.uint8_t)((unsafe.Pointer)(&ptr[0])),
		len:  C.uint32_t(len(name)),
	}

	C.wasmjit_invoke(s, resolver)
}

func WasmJitInvokeArgs(name string, n uint32, resolver *C.wasmjit_resolver_t) (uint32, uint32, error) {
	ptr := []byte(name)
	s := C.wasmjit_slice_t{
		data: (*C.uint8_t)((unsafe.Pointer)(&ptr[0])),
		len:  C.uint32_t(len(name)),
	}

	ret := C.wasmjit_invoke_args(s, C.uint32_t(n), resolver)
	if ret.res.kind != C.wasmjit_result_kind(wasmjit_result_success) {
		if ret.res.kind == C.wasmjit_result_kind(wasmjit_result_err_trap) {
			err := errors.New(C.GoStringN((*C.char)((unsafe.Pointer)(ret.res.msg.data)), C.int(ret.res.msg.len)))
			C.wasmjit_bytes_destroy(ret.res.msg)
			return 0, 0, err
		} else {
			panic("inner happend")
		}
	} else {
		return uint32(ret.v), uint32(ret.isnone), nil
	}
}

//export addtest
func addtest(context unsafe.Pointer, x C.uint32_t, y C.uint32_t) C.uint32_t {
	fmt.Printf("enter add for check\n")
	return x + y
}

//export subtest
func subtest(context unsafe.Pointer, x C.uint32_t, y C.uint32_t) C.uint32_t {
	fmt.Printf("enter sub for check\n")
	return x - y
}

func testImportAdd(t *testing.T) {
	var imports Imports
	defer imports.Destroy()
	importfunc, err := NewWasmJitImportFunc("addtest", C.addtest)
	assert.Nil(t, err)
	imports.Append(importfunc)

	importfunc, err = NewWasmJitImportFunc("subtest", C.subtest)
	assert.Nil(t, err)
	imports.Append(importfunc)

	wasmImports := make([]C.wasmjit_import_func_t, imports.Num())
	for index, imp := range imports.imports {
		wasmImports[index] = imp
	}

	resolver := C.wasmjit_go_resolver_create(((*C.wasmjit_import_func_t)((unsafe.Pointer)(&wasmImports[0]))), C.uint32_t(imports.Num()))
	v, _, err := WasmJitInvokeArgs("./test/test0.wat", 3, resolver)
	assert.Equal(t, v, uint32(7))
	//fmt.Printf("testImportAdd done\n")
}

func testDiv(t *testing.T) {
	var imports Imports
	defer imports.Destroy()
	importfunc, err := NewWasmJitImportFunc("addtest", C.addtest)
	assert.Nil(t, err)
	imports.Append(importfunc)

	wasmImports := make([]C.wasmjit_import_func_t, imports.Num())
	for index, imp := range imports.imports {
		wasmImports[index] = imp
	}

	resolver := C.wasmjit_go_resolver_create(((*C.wasmjit_import_func_t)((unsafe.Pointer)(&wasmImports[0]))), C.uint32_t(imports.Num()))
	v, _, err := WasmJitInvokeArgs("./test/test1.wat", 2, resolver)
	assert.Equal(t, v, uint32(10))
	//fmt.Printf("testDiv done\n")
}

func testPanic(t *testing.T) {
	var imports Imports
	defer imports.Destroy()
	importfunc, err := NewWasmJitImportFunc("addtest", C.addtest)
	assert.Nil(t, err)
	imports.Append(importfunc)

	wasmImports := make([]C.wasmjit_import_func_t, imports.Num())
	for index, imp := range imports.imports {
		wasmImports[index] = imp
	}

	resolver := C.wasmjit_go_resolver_create(((*C.wasmjit_import_func_t)((unsafe.Pointer)(&wasmImports[0]))), C.uint32_t(imports.Num()))
	v, _, err := WasmJitInvokeArgs("./test/test1.wat", 2, resolver)
	assert.Equal(t, v, uint32(10))

	resolver0 := C.wasmjit_go_resolver_create(((*C.wasmjit_import_func_t)((unsafe.Pointer)(&wasmImports[0]))), C.uint32_t(imports.Num()))
	_, _, err = WasmJitInvokeArgs("./test/test1.wat", 0, resolver0)
	assert.NotNil(t, err)
	assert.EqualError(t, err, "wasm trap: integer divide by zero, source location: @0029")
	//fmt.Printf("testPanic done\n")
}

func testMultiThread(t *testing.T) {
	var wg sync.WaitGroup
	for i := 1; i < 10000; i++ {
		wg.Add(1)
		switch i % 3 {
		case 0:
			go func(t *testing.T) {
				defer wg.Done()
				testPanic(t)
			}(t)
		case 1:
			go func(t *testing.T) {
				defer wg.Done()
				testDiv(t)
			}(t)
		case 2:
			go func(t *testing.T) {
				defer wg.Done()
				testImportAdd(t)
			}(t)
		}
	}

	wg.Wait()

	fmt.Printf("all thread done\n")
}
