(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $invoke (export "mul") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    get_local $p1
    get_local $p0
    i32.mul)
  )