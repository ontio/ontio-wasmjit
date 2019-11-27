(module
  (func (export "gas-as-br-value") (result i32)
     (block (result i32) (br 0 (br_if 0 (i32.const 1) (i32.const 2))))
  )
)

(assert_return (invoke "gas-as-br-value") (i32.const 1))