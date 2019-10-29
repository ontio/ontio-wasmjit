(module
  (type $t0 (func (param i32) (param i32) (result i32)))
  (func $add (export "add") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    get_local $p0
    get_local $p1
    i32.add)
)

