(module
  (global $a i32 (i32.const 1))
  (type $t0 (func (param i32)(result i32)))
  (func (export "get-global") (type $t0) (param $p0 i32) (result i32)
    global.get $a
    get_local $p0
    i32.add
    global.set $a
    global.get $a)
)