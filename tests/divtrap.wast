(module
  (type $t0 (func (result i32)))
  (func $div (export "divtrap") (type $t0) (result i32)
    i32.const 1
    i32.const 0
    i32.div_s)
)

