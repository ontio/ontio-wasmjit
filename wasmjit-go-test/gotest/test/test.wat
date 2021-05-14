(module
 (type $t0 (func))
 (type $t1 (func (param i32 i32) (result i32)))
 (import "env" "addtest" (func $add (type $t1)))
 (import "env" "subtest" (func $sub (type $t1)))
 (func $invoke (export "invoke") (type $t0)
  i32.const 1
  i32.const 4
  call $add
  drop
  i32.const 7
  i32.const 4
  call $sub
  drop
  ))
