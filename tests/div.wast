(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)))
  (type $t3 (func (param i32 i32)))
  (func $invoke (export "div") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0
      block $B1
        get_local $p1
        i32.eqz
        br_if $B1
        block $B2
          get_local $p0
          i32.const -2147483648
          i32.ne
          br_if $B2
          get_local $p1
          i32.const -1
          i32.eq
          br_if $B0
        end
        get_local $p0
        get_local $p1
        i32.div_s
        return
      end
      i32.const 0
      return
    end
    i32.const 1092
    call $core::panicking::panic::haf7d7779169c0743
    unreachable)
  (func $core::panicking::panic::haf7d7779169c0743 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i64) (local $l2 i64) (local $l3 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p0
    i64.load offset=16 align=4
    set_local $l1
    get_local $p0
    i64.load offset=8 align=4
    set_local $l2
    get_local $p0
    i64.load align=4
    set_local $l3
    get_local $l0
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    get_local $l3
    i64.store offset=24
    get_local $l0
    i64.const 1
    i64.store offset=4 align=4
    get_local $l0
    i32.const 1088
    i32.store offset=16
    get_local $l0
    get_local $l0
    i32.const 24
    i32.add
    i32.store
    get_local $l0
    get_local $l2
    i64.store offset=32
    get_local $l0
    get_local $l1
    i64.store offset=40
    get_local $l0
    get_local $l0
    i32.const 32
    i32.add
    call $core::panicking::panic_fmt::h29e5105b4d53bc05
    unreachable)
  (func $std::panicking::rust_panic_with_hook::h9b1c029d1ceaded2 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 1
    set_local $l0
    block $B0
      block $B1
        block $B2
          i32.const 0
          i32.load offset=1120
          i32.const 1
          i32.ne
          br_if $B2
          i32.const 0
          i32.const 0
          i32.load offset=1124
          i32.const 1
          i32.add
          tee_local $l0
          i32.store offset=1124
          get_local $l0
          i32.const 3
          i32.lt_u
          br_if $B1
          br $B0
        end
        i32.const 0
        i64.const 4294967297
        i64.store offset=1120
      end
      i32.const 0
      i32.load offset=1132
      tee_local $l1
      i32.const -1
      i32.le_s
      br_if $B0
      i32.const 0
      get_local $l1
      i32.store offset=1132
      get_local $l0
      i32.const 2
      i32.lt_u
      drop
    end
    unreachable
    unreachable)
  (func $rust_begin_unwind (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (param $p6 i32) (param $p7 i32) (param $p8 i32) (param $p9 i32)
    (local $l0 i32)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 20
    i32.add
    get_local $p3
    i32.store
    get_local $l0
    i32.const 28
    i32.add
    get_local $p5
    i32.store
    get_local $l0
    get_local $p1
    i32.store offset=12
    get_local $l0
    get_local $p0
    i32.store offset=8
    get_local $l0
    get_local $p2
    i32.store offset=16
    get_local $l0
    get_local $p4
    i32.store offset=24
    get_local $l0
    get_local $p7
    i32.store offset=36
    get_local $l0
    get_local $p6
    i32.store offset=32
    get_local $l0
    get_local $p8
    i32.store offset=40
    get_local $l0
    get_local $p9
    i32.store offset=44
    get_local $l0
    i32.const 8
    i32.add
    get_local $l0
    i32.const 32
    i32.add
    call $std::panicking::begin_panic_fmt::h29d4906ca23d78a0
    unreachable)
  (func $std::panicking::begin_panic_fmt::h29d4906ca23d78a0 (type $t3) (param $p0 i32) (param $p1 i32)
    get_local $p1
    call $std::panicking::rust_panic_with_hook::h9b1c029d1ceaded2
    unreachable)
  (func $core::panicking::panic_fmt::h29e5105b4d53bc05 (type $t3) (param $p0 i32) (param $p1 i32)
    get_local $p0
    i32.load
    get_local $p0
    i32.load offset=4
    get_local $p0
    i32.load offset=8
    get_local $p0
    i32.const 12
    i32.add
    i32.load
    get_local $p0
    i32.load offset=16
    get_local $p0
    i32.const 20
    i32.add
    i32.load
    get_local $p1
    i32.load
    get_local $p1
    i32.load offset=4
    get_local $p1
    i32.load offset=8
    get_local $p1
    i32.load offset=12
    call $rust_begin_unwind
    unreachable)
  (global $g0 (mut i32) (i32.const 1049728))
  (memory $memory 17)
  (data (i32.const 1024) "/tmp/rustc_h_wgp0p1j5v7.rs\00\00\00\00\00\00attempt to divide with overflow\00\00")
  (data (i32.const 1092) " \04\00\00\1f\00\00\00\00\04\00\00\1a\00\00\00\06\00\00\00\07\00\00\00")
  (data (i32.const 1120) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
