(module
 (type $t0 (func (param i32) (result i32)))
 (type $t1 (func (param i32 i32) (result i32)))
 (import "env" "addtest" (func $add (type $t1)))
 (import "env" "subtest" (func $sub (type $t1)))
 (func $invoke (export "invoke") (type $t0) (param $p0 i32) (result i32)
  get_local $p0
  i32.const 4
  call $add
  ))
