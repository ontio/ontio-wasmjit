(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $sub (export "sub") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    get_local $p0
    i32.load
    get_local $p1
    i32.load
    i32.sub)
  (memory $memory 17))