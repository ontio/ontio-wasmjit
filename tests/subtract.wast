(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $invoke (export "sub") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    get_local $p0
    get_local $p1
    i32.sub))