(module
  (type $t0 (func))
  (type $t1 (func (param i32 i32)))
  (import "env" "ontio_panic" (func $ontio_panic (type $t1)))
  (func $invoke (export "invoke") (type $t0)
    i32.const 0
    i32.const 1
    call $ontio_panic
    unreachable)
  )
