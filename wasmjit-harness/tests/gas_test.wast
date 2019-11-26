(module
  (func (export "gas-add") (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y)))
  (func (export "gas-empty-block")
      (block)
      (block $l)
  )
  (func (export "gas-type-i32-value-br") (result i32)
    (block (result i32) (i32.ctz (br 0 (i32.const 1))))
  )
  (func (export "gas-as-br-value") (result i32)
     (block (result i32) (br 0 (br_if 0 (i32.const 1) (i32.const 2))))
  )

  (func (export "gas-type-i32-value-br-table") (result i32)
    (block (result i32) (i32.ctz (br_table 0 0 (i32.const 1) (i32.const 0))))
  )
  (global $x (mut i32) (i32.const -12))
  (func (export "gas-set-x") (param i32) (global.set $x (local.get 0)))
  (func (export "gas-empty-if") (param i32)
   (if (local.get 0) (then))
   (if (local.get 0) (then) (else))
   (if $l (local.get 0) (then))
   (if $l (local.get 0) (then) (else))
  )
  (func $i32-i64 (param i32 i64) (result i64) (local.get 1))
  (func (export "gas-type-second-i64") (result i64)
      (call $i32-i64 (i32.const 32) (i64.const 64))
  )

  (memory 3)
  (func (export "gas-empty-loop")
      (loop)
      (loop $l)
  )
  (func (export "gas-singular-loop") (result i32)
      (loop (nop))
      (loop (result i32) (i32.const 7))
  )
  (func (export "gas-load_at_zero") (result i32) (i32.load (i32.const 0)))
  (func (export "gas-select_i32") (param $lhs i32) (param $rhs i32) (param $cond i32) (result i32)
     (select (local.get $lhs) (local.get $rhs) (local.get $cond)))

  ;;switch
  (func (export "gas-stmt") (param $i i32) (result i32)
      (local $j i32)
      (local.set $j (i32.const 100))
      (block $switch
        (block $7
          (block $default
            (block $6
              (block $5
                (block $4
                  (block $3
                    (block $2
                      (block $1
                        (block $0
                          (br_table $0 $1 $2 $3 $4 $5 $6 $7 $default
                            (local.get $i)
                          )
                        ) ;; 0
                        (return (local.get $i))
                      ) ;; 1
                      (nop)
                      ;; fallthrough
                    ) ;; 2
                    ;; fallthrough
                  ) ;; 3
                  (local.set $j (i32.sub (i32.const 0) (local.get $i)))
                  (br $switch)
                ) ;; 4
                (br $switch)
              ) ;; 5
              (local.set $j (i32.const 101))
              (br $switch)
            ) ;; 6
            (local.set $j (i32.const 101))
            ;; fallthrough
          ) ;; default
          (local.set $j (i32.const 102))
        ) ;; 7
        ;; fallthrough
      )
      (return (local.get $j))
    )
)
(assert_return (invoke "gas-add" (i32.const 1) (i32.const 1)) (i32.const 2))
(assert_return (invoke "gas-empty-block"))
(assert_return (invoke "gas-type-i32-value-br") (i32.const 1))
(assert_return (invoke "gas-as-br-value") (i32.const 1))
(assert_return (invoke "gas-type-i32-value-br-table") (i32.const 1))
(assert_return (invoke "gas-set-x" (i32.const 6)))
(assert_return (invoke "gas-empty-if" (i32.const 0)))
(assert_return (invoke "gas-type-second-i64") (i64.const 64))
(assert_return (invoke "gas-empty-loop"))
(assert_return (invoke "gas-singular-loop") (i32.const 7))
(assert_return (invoke "gas-load_at_zero") (i32.const 0))
(assert_return (invoke "gas-select_i32" (i32.const 1) (i32.const 2) (i32.const 1)) (i32.const 1))
(assert_return (invoke "gas-stmt" (i32.const 0)) (i32.const 0))
