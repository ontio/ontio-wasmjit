(module
 (type $t0 (func (param i32) (result i32)))
 (func $invoke (export "invoke") (type $t0) (param $p0 i32) (result i32)
  i32.const 20
  get_local $p0
  i32.div_s
  ))
