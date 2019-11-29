(module

  (func $depth-check (export "depth-check") (param $depth i64)
    (if (local.get $depth) (then
    local.get $depth
    i64.const 1
    i64.sub
    call $depth-check
    )
    (else
    return )
    )
  )
)

(assert_return (invoke "depth-check" (i64.const 3)))

