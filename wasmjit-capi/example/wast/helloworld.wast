(module
  (type $t0 (func))
  (type (;1;) (func (param i32 i32)))
  (import "env" "ontio_return" (func $ontio_return (type 1)))
  (func $add (export "invoke") (type $t0)
  (i32.store  (i32.const 0) (i32.const 0x00112233) )
  i32.const 0
  i32.const 4
  call $ontio_return
  )
  (memory (;0;) 1 80)
)