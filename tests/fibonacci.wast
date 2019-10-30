(module
  (type $t0 (func (param i32) (result i32)))
  (func $recursion (export "fib") (type $t0) (param $p0 i32) (result i32)
    get_local $p0
    call $rustc_h_brg0awvvvms::fib::h0c299e912db4f1c8)
  (func $rustc_h_brg0awvvvms::fib::h0c299e912db4f1c8 (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 0
    set_local $l0
    block $B0
      block $B1
        get_local $p0
        i32.const 0
        i32.lt_s
        br_if $B1
        i32.const 1
        set_local $l0
        get_local $p0
        i32.const -1
        i32.add
        tee_local $l1
        i32.const 1
        i32.gt_u
        br_if $B0
      end
      get_local $l0
      return
    end
    get_local $l1
    call $rustc_h_brg0awvvvms::fib::h0c299e912db4f1c8
    get_local $p0
    i32.const -2
    i32.add
    call $rustc_h_brg0awvvvms::fib::h0c299e912db4f1c8
    i32.add)
  (memory $memory 17))