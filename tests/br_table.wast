(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32) (result i32)))
  (func $br_table (export "br_table") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            get_local $p0
            i32.const 3
            i32.gt_u
            br_if $B3
            i32.const 1
            set_local $l0
            block $B4
              get_local $p0
              br_table $B0 $B4 $B2 $B1 $B0
            end
            i32.const 3
            set_local $l0
            br $B0
          end
          i32.const 0
          set_local $l0
          br $B0
        end
        get_local $p1
        set_local $l0
        br $B0
      end
      get_local $p1
      i32.const 2
      i32.add
      set_local $l0
    end
    get_local $l0
    i32.const 255
    i32.and)
  (func $call_br_table (export "call_br_table") (type $t1) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 0
    set_local $l0
    i32.const 0
    set_local $l1
    loop $L0
      get_local $l1
      i32.const 255
      i32.and
      get_local $p0
      call $br_table
      get_local $l0
      i32.add
      set_local $l0
      get_local $l1
      i32.const 1
      i32.add
      tee_local $l1
      i32.const 255
      i32.and
      i32.const 255
      i32.ne
      br_if $L0
    end
    get_local $l0
    i32.const 255
    i32.and)
  (table $T0 1 1 anyfunc)
  (memory $memory 17))

