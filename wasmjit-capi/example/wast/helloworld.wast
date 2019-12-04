(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (result i32)))
  (import "env" "ontio_return" (func (;0;) (type 2)))
  (func (;1;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.load
    local.tee 3
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.const -4
    i32.and
    local.tee 2
    i32.const 16
    i32.ge_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 1073741808
      i32.lt_u
    else
      i32.const 0
    end
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
      local.set 2
      i32.const 0
    else
      local.get 2
      i32.const 31
      local.get 2
      i32.clz
      i32.sub
      local.tee 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
      local.set 2
      local.get 3
      i32.const 7
      i32.sub
    end
    local.tee 3
    i32.const 23
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 16
      i32.lt_u
    else
      i32.const 0
    end
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.load offset=20
    local.set 4
    local.get 1
    i32.load offset=16
    local.tee 5
    if  ;; label = @1
      local.get 5
      local.get 4
      i32.store offset=20
    end
    local.get 4
    if  ;; label = @1
      local.get 4
      local.get 5
      i32.store offset=16
    end
    local.get 3
    i32.const 4
    i32.shl
    local.get 2
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=96
    local.get 1
    i32.eq
    if  ;; label = @1
      local.get 3
      i32.const 4
      i32.shl
      local.get 2
      i32.add
      i32.const 2
      i32.shl
      local.get 0
      i32.add
      local.get 4
      i32.store offset=96
      local.get 4
      i32.eqz
      if  ;; label = @2
        local.get 3
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        local.get 3
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        i32.load offset=4
        i32.const 1
        local.get 2
        i32.shl
        i32.const -1
        i32.xor
        i32.and
        local.tee 1
        i32.store offset=4
        local.get 1
        i32.eqz
        if  ;; label = @3
          local.get 0
          local.get 0
          i32.load
          i32.const 1
          local.get 3
          i32.shl
          i32.const -1
          i32.xor
          i32.and
          i32.store
        end
      end
    end)
  (func (;2;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.load
    local.tee 3
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    local.get 1
    i32.load
    i32.const -4
    i32.and
    i32.add
    local.tee 4
    i32.load
    local.tee 5
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 3
      i32.const -4
      i32.and
      i32.const 16
      i32.add
      local.get 5
      i32.const -4
      i32.and
      i32.add
      local.tee 2
      i32.const 1073741808
      i32.lt_u
      if  ;; label = @2
        local.get 0
        local.get 4
        call 1
        local.get 1
        local.get 3
        i32.const 3
        i32.and
        local.get 2
        i32.or
        local.tee 3
        i32.store
        local.get 1
        i32.const 16
        i32.add
        local.get 1
        i32.load
        i32.const -4
        i32.and
        i32.add
        local.tee 4
        i32.load
        local.set 5
      end
    end
    local.get 3
    i32.const 2
    i32.and
    if  ;; label = @1
      local.get 1
      i32.const 4
      i32.sub
      i32.load
      local.tee 2
      i32.load
      local.tee 6
      i32.const 1
      i32.and
      i32.eqz
      if  ;; label = @2
        unreachable
      end
      local.get 6
      i32.const -4
      i32.and
      i32.const 16
      i32.add
      local.get 3
      i32.const -4
      i32.and
      i32.add
      local.tee 7
      i32.const 1073741808
      i32.lt_u
      if (result i32)  ;; label = @2
        local.get 0
        local.get 2
        call 1
        local.get 2
        local.get 6
        i32.const 3
        i32.and
        local.get 7
        i32.or
        local.tee 3
        i32.store
        local.get 2
      else
        local.get 1
      end
      local.set 1
    end
    local.get 4
    local.get 5
    i32.const 2
    i32.or
    i32.store
    local.get 3
    i32.const -4
    i32.and
    local.tee 2
    i32.const 16
    i32.ge_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 1073741808
      i32.lt_u
    else
      i32.const 0
    end
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 4
    local.get 1
    i32.const 16
    i32.add
    local.get 2
    i32.add
    i32.ne
    if  ;; label = @1
      unreachable
    end
    local.get 4
    i32.const 4
    i32.sub
    local.get 1
    i32.store
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
      local.set 4
      i32.const 0
    else
      local.get 2
      i32.const 31
      local.get 2
      i32.clz
      i32.sub
      local.tee 2
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
      local.set 4
      local.get 2
      i32.const 7
      i32.sub
    end
    local.tee 3
    i32.const 23
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 4
      i32.const 16
      i32.lt_u
    else
      i32.const 0
    end
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.const 4
    i32.shl
    local.get 4
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=96
    local.set 2
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    local.get 2
    i32.store offset=20
    local.get 2
    if  ;; label = @1
      local.get 2
      local.get 1
      i32.store offset=16
    end
    local.get 3
    i32.const 4
    i32.shl
    local.get 4
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    local.get 1
    i32.store offset=96
    local.get 0
    local.get 0
    i32.load
    i32.const 1
    local.get 3
    i32.shl
    i32.or
    i32.store
    local.get 3
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    local.get 3
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=4
    i32.const 1
    local.get 4
    i32.shl
    i32.or
    i32.store offset=4)
  (func (;3;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    local.get 2
    i32.const 15
    i32.and
    i32.eqz
    i32.const 0
    local.get 1
    i32.const 15
    i32.and
    i32.eqz
    i32.const 0
    local.get 1
    local.get 2
    i32.le_u
    select
    select
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 0
    i32.load offset=1568
    local.tee 3
    if  ;; label = @1
      local.get 1
      local.get 3
      i32.const 16
      i32.add
      i32.lt_u
      if  ;; label = @2
        unreachable
      end
      local.get 1
      i32.const 16
      i32.sub
      local.get 3
      i32.eq
      if  ;; label = @2
        local.get 3
        i32.load
        local.set 4
        local.get 1
        i32.const 16
        i32.sub
        local.set 1
      end
    else
      local.get 1
      local.get 0
      i32.const 1572
      i32.add
      i32.lt_u
      if  ;; label = @2
        unreachable
      end
    end
    local.get 2
    local.get 1
    i32.sub
    local.tee 2
    i32.const 48
    i32.lt_u
    if  ;; label = @1
      return
    end
    local.get 1
    local.get 4
    i32.const 2
    i32.and
    local.get 2
    i32.const 32
    i32.sub
    i32.const 1
    i32.or
    i32.or
    i32.store
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 0
    i32.store offset=20
    local.get 1
    local.get 2
    i32.add
    i32.const 16
    i32.sub
    local.tee 2
    i32.const 2
    i32.store
    local.get 0
    local.get 2
    i32.store offset=1568
    local.get 0
    local.get 1
    call 2)
  (func (;4;) (type 1)
    (local i32 i32)
    i32.const 1
    memory.size
    local.tee 0
    i32.gt_s
    if (result i32)  ;; label = @1
      i32.const 1
      local.get 0
      i32.sub
      memory.grow
      i32.const 0
      i32.lt_s
    else
      i32.const 0
    end
    if  ;; label = @1
      unreachable
    end
    i32.const 640
    i32.const 0
    i32.store
    i32.const 2208
    i32.const 0
    i32.store
    i32.const 0
    local.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 23
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 2
        i32.shl
        i32.const 640
        i32.add
        i32.const 0
        i32.store offset=4
        i32.const 0
        local.set 1
        loop  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 4
            i32.shl
            local.get 1
            i32.add
            i32.const 2
            i32.shl
            i32.const 640
            i32.add
            i32.const 0
            i32.store offset=96
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            br 1 (;@3;)
          end
        end
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end
    i32.const 640
    i32.const 2224
    memory.size
    i32.const 16
    i32.shl
    call 3
    i32.const 640
    global.set 0)
  (func (;5;) (type 3) (param i32) (result i32)
    local.get 0
    i32.const 1073741808
    i32.ge_u
    if  ;; label = @1
      unreachable
    end
    local.get 0
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    local.tee 0
    i32.const 16
    local.get 0
    i32.const 16
    i32.gt_u
    select)
  (func (;6;) (type 0) (param i32 i32) (result i32)
    (local i32)
    local.get 1
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 1
      i32.const 4
      i32.shr_u
      local.set 1
      i32.const 0
    else
      local.get 1
      i32.const 536870904
      i32.lt_u
      if  ;; label = @2
        i32.const 1
        i32.const 27
        local.get 1
        i32.clz
        i32.sub
        i32.shl
        local.get 1
        i32.add
        i32.const 1
        i32.sub
        local.set 1
      end
      local.get 1
      i32.const 31
      local.get 1
      i32.clz
      i32.sub
      local.tee 2
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
      local.set 1
      local.get 2
      i32.const 7
      i32.sub
    end
    local.tee 2
    i32.const 23
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 1
      i32.const 16
      i32.lt_u
    else
      i32.const 0
    end
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 2
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=4
    i32.const -1
    local.get 1
    i32.shl
    i32.and
    local.tee 1
    if (result i32)  ;; label = @1
      local.get 1
      i32.ctz
      local.get 2
      i32.const 4
      i32.shl
      i32.add
      i32.const 2
      i32.shl
      local.get 0
      i32.add
      i32.load offset=96
    else
      local.get 0
      i32.load
      i32.const -1
      local.get 2
      i32.const 1
      i32.add
      i32.shl
      i32.and
      local.tee 1
      if (result i32)  ;; label = @2
        local.get 1
        i32.ctz
        local.tee 1
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        i32.load offset=4
        local.tee 2
        i32.eqz
        if  ;; label = @3
          unreachable
        end
        local.get 2
        i32.ctz
        local.get 1
        i32.const 4
        i32.shl
        i32.add
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        i32.load offset=96
      else
        i32.const 0
      end
    end)
  (func (;7;) (type 4) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    i32.const 1879048192
    i32.and
    i32.const 268435456
    i32.ne
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.const -1879048193
      i32.and
      i32.const 268435456
      i32.or
      i32.store offset=4
      local.get 0
      i32.const 16
      i32.add
      i32.const 2
      call 33
    end)
  (func (;8;) (type 2) (param i32 i32)
    (local i32)
    local.get 1
    i32.load
    local.tee 2
    i32.const 1
    i32.and
    if  ;; label = @1
      unreachable
    end
    local.get 1
    local.get 2
    i32.const 1
    i32.or
    i32.store
    local.get 0
    local.get 1
    call 2)
  (func (;9;) (type 4) (param i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const -1879048193
    i32.and
    i32.store offset=4
    local.get 0
    i32.const 16
    i32.add
    i32.const 4
    call 33)
  (func (;10;) (type 4) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    i32.const 1879048192
    i32.and
    i32.const 268435456
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.const 268435455
      i32.and
      i32.const 0
      i32.gt_u
      if  ;; label = @2
        local.get 0
        call 9
      else
        local.get 0
        local.get 1
        i32.const -1879048193
        i32.and
        i32.const 536870912
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 16
        i32.add
        i32.const 3
        call 33
      end
    end)
  (func (;11;) (type 4) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    i32.const 1879048192
    i32.and
    i32.const 536870912
    i32.eq
    if (result i32)  ;; label = @1
      local.get 1
      i32.const -2147483648
      i32.and
      i32.eqz
    else
      i32.const 0
    end
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.const -1879048193
      i32.and
      i32.store offset=4
      local.get 0
      i32.const 16
      i32.add
      i32.const 5
      call 33
      global.get 0
      local.get 0
      call 8
    end)
  (func (;12;) (type 1)
    (local i32 i32 i32 i32 i32 i32)
    global.get 2
    local.tee 5
    local.tee 2
    local.set 3
    global.get 3
    local.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 3
        local.get 0
        i32.ge_u
        br_if 0 (;@2;)
        local.get 3
        i32.load
        local.tee 4
        i32.load offset=4
        local.tee 1
        i32.const 1879048192
        i32.and
        i32.const 805306368
        i32.eq
        if (result i32)  ;; label = @3
          local.get 1
          i32.const 268435455
          i32.and
          i32.const 0
          i32.gt_u
        else
          i32.const 0
        end
        if  ;; label = @3
          local.get 4
          call 7
          local.get 2
          local.get 4
          i32.store
          local.get 2
          i32.const 4
          i32.add
          local.set 2
        else
          i32.const 0
          local.get 1
          i32.const 268435455
          i32.and
          i32.eqz
          local.get 1
          i32.const 1879048192
          i32.and
          select
          if  ;; label = @4
            global.get 0
            local.get 4
            call 8
          else
            local.get 4
            local.get 1
            i32.const 2147483647
            i32.and
            i32.store offset=4
          end
        end
        local.get 3
        i32.const 4
        i32.add
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 2
    global.set 3
    local.get 5
    local.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        i32.load
        call 10
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 5
    local.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 1
        local.get 1
        i32.load offset=4
        i32.const 2147483647
        i32.and
        i32.store offset=4
        local.get 1
        call 11
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 5
    global.set 3)
  (func (;13;) (type 2) (param i32 i32)
    (local i32)
    memory.size
    local.tee 2
    i32.const 16
    local.get 0
    i32.load offset=1568
    local.get 2
    i32.const 16
    i32.shl
    i32.const 16
    i32.sub
    i32.ne
    i32.shl
    i32.const 1
    i32.const 27
    local.get 1
    i32.clz
    i32.sub
    i32.shl
    i32.const 1
    i32.sub
    local.get 1
    i32.add
    local.get 1
    local.get 1
    i32.const 536870904
    i32.lt_u
    select
    i32.add
    i32.const 65535
    i32.add
    i32.const -65536
    i32.and
    i32.const 16
    i32.shr_u
    local.tee 1
    local.get 2
    local.get 1
    i32.gt_s
    select
    memory.grow
    i32.const 0
    i32.lt_s
    if  ;; label = @1
      local.get 1
      memory.grow
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        unreachable
      end
    end
    local.get 0
    local.get 2
    i32.const 16
    i32.shl
    memory.size
    i32.const 16
    i32.shl
    call 3)
  (func (;14;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    i32.load
    local.set 3
    local.get 2
    i32.const 15
    i32.and
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.const -4
    i32.and
    local.get 2
    i32.sub
    local.tee 4
    i32.const 32
    i32.ge_u
    if  ;; label = @1
      local.get 1
      local.get 3
      i32.const 2
      i32.and
      local.get 2
      i32.or
      i32.store
      local.get 1
      i32.const 16
      i32.add
      local.get 2
      i32.add
      local.tee 1
      local.get 4
      i32.const 16
      i32.sub
      i32.const 1
      i32.or
      i32.store
      local.get 0
      local.get 1
      call 2
    else
      local.get 1
      local.get 3
      i32.const -2
      i32.and
      i32.store
      local.get 1
      i32.const 16
      i32.add
      local.get 1
      i32.load
      i32.const -4
      i32.and
      i32.add
      local.get 1
      i32.const 16
      i32.add
      local.get 1
      i32.load
      i32.const -4
      i32.and
      i32.add
      i32.load
      i32.const -3
      i32.and
      i32.store
    end)
  (func (;15;) (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 1
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 1
    call 5
    local.tee 3
    call 6
    local.tee 2
    i32.eqz
    if  ;; label = @1
      i32.const 1
      global.set 1
      call 12
      i32.const 0
      global.set 1
      local.get 0
      local.get 3
      call 6
      local.tee 2
      i32.eqz
      if  ;; label = @2
        local.get 0
        local.get 3
        call 13
        local.get 0
        local.get 3
        call 6
        local.tee 2
        i32.eqz
        if  ;; label = @3
          unreachable
        end
      end
    end
    local.get 2
    i32.load
    i32.const -4
    i32.and
    local.get 3
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 2
    i32.const 0
    i32.store offset=4
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 0
    local.get 2
    call 1
    local.get 0
    local.get 2
    local.get 3
    call 14
    local.get 2)
  (func (;16;) (type 0) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    local.tee 2
    if (result i32)  ;; label = @1
      local.get 2
    else
      call 4
      global.get 0
    end
    local.get 0
    call 15
    local.tee 0
    local.get 1
    i32.store offset=8
    local.get 0
    i32.const 16
    i32.add)
  (func (;17;) (type 4) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    i32.const -268435456
    i32.and
    local.get 1
    i32.const 1
    i32.add
    i32.const -268435456
    i32.and
    i32.ne
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 0
    i32.load
    i32.const 1
    i32.and
    if  ;; label = @1
      unreachable
    end)
  (func (;18;) (type 3) (param i32) (result i32)
    local.get 0
    i32.const 636
    i32.gt_u
    if  ;; label = @1
      local.get 0
      i32.const 16
      i32.sub
      call 17
    end
    local.get 0)
  (func (;19;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      local.set 3
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.lt_u
      if  ;; label = @2
        local.get 1
        i32.const 7
        i32.and
        local.get 0
        i32.const 7
        i32.and
        i32.eq
        if  ;; label = @3
          loop  ;; label = @4
            local.get 0
            i32.const 7
            i32.and
            if  ;; label = @5
              local.get 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 3
              i32.const 1
              i32.sub
              local.set 3
              local.get 0
              local.tee 2
              i32.const 1
              i32.add
              local.set 0
              local.get 1
              local.tee 4
              i32.const 1
              i32.add
              local.set 1
              local.get 2
              local.get 4
              i32.load8_u
              i32.store8
              br 1 (;@4;)
            end
          end
          loop  ;; label = @4
            local.get 3
            i32.const 8
            i32.lt_u
            i32.eqz
            if  ;; label = @5
              local.get 0
              local.get 1
              i64.load
              i64.store
              local.get 3
              i32.const 8
              i32.sub
              local.set 3
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              br 1 (;@4;)
            end
          end
        end
        loop  ;; label = @3
          local.get 3
          if  ;; label = @4
            local.get 0
            local.tee 2
            i32.const 1
            i32.add
            local.set 0
            local.get 1
            local.tee 4
            i32.const 1
            i32.add
            local.set 1
            local.get 2
            local.get 4
            i32.load8_u
            i32.store8
            local.get 3
            i32.const 1
            i32.sub
            local.set 3
            br 1 (;@3;)
          end
        end
      else
        local.get 1
        i32.const 7
        i32.and
        local.get 0
        i32.const 7
        i32.and
        i32.eq
        if  ;; label = @3
          loop  ;; label = @4
            local.get 0
            local.get 3
            i32.add
            i32.const 7
            i32.and
            if  ;; label = @5
              local.get 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 3
              i32.const 1
              i32.sub
              local.tee 3
              i32.add
              local.get 1
              local.get 3
              i32.add
              i32.load8_u
              i32.store8
              br 1 (;@4;)
            end
          end
          loop  ;; label = @4
            local.get 3
            i32.const 8
            i32.lt_u
            i32.eqz
            if  ;; label = @5
              local.get 0
              local.get 3
              i32.const 8
              i32.sub
              local.tee 3
              i32.add
              local.get 1
              local.get 3
              i32.add
              i64.load
              i64.store
              br 1 (;@4;)
            end
          end
        end
        loop  ;; label = @3
          local.get 3
          if  ;; label = @4
            local.get 0
            local.get 3
            i32.const 1
            i32.sub
            local.tee 3
            i32.add
            local.get 1
            local.get 3
            i32.add
            i32.load8_u
            i32.store8
            br 1 (;@3;)
          end
        end
      end
    end)
  (func (;20;) (type 4) (param i32)
    global.get 0
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    local.get 0
    i32.const 15
    i32.and
    i32.eqz
    i32.const 0
    local.get 0
    select
    i32.eqz
    if  ;; label = @1
      unreachable
    end
    global.get 0
    local.get 0
    i32.const 16
    i32.sub
    call 8)
  (func (;21;) (type 1)
    (local i32 i32 i32 i32)
    global.get 3
    global.get 2
    local.tee 1
    i32.sub
    local.tee 2
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 256
    local.get 0
    i32.const 256
    i32.gt_u
    select
    local.tee 3
    i32.const 0
    call 16
    local.tee 0
    local.get 1
    local.get 2
    call 19
    local.get 1
    if  ;; label = @1
      local.get 1
      call 20
    end
    local.get 0
    global.set 2
    local.get 0
    local.get 2
    i32.add
    global.set 3
    local.get 0
    local.get 3
    i32.add
    global.set 4)
  (func (;22;) (type 4) (param i32)
    (local i32)
    global.get 3
    local.tee 1
    global.get 4
    i32.ge_u
    if  ;; label = @1
      call 21
      global.get 3
      local.set 1
    end
    local.get 1
    local.get 0
    i32.store
    local.get 1
    i32.const 4
    i32.add
    global.set 3)
  (func (;23;) (type 4) (param i32)
    (local i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.tee 2
    i32.const 268435455
    i32.and
    local.set 1
    local.get 0
    i32.load
    i32.const 1
    i32.and
    if  ;; label = @1
      unreachable
    end
    local.get 1
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 0
      i32.const 16
      i32.add
      i32.const 1
      call 33
      local.get 2
      i32.const -2147483648
      i32.and
      if  ;; label = @2
        local.get 0
        i32.const -2147483648
        i32.store offset=4
      else
        global.get 0
        local.get 0
        call 8
      end
    else
      local.get 1
      i32.const 0
      i32.le_u
      if  ;; label = @2
        unreachable
      end
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.const 592
      i32.load
      i32.gt_u
      if  ;; label = @2
        unreachable
      end
      local.get 3
      i32.const 3
      i32.shl
      i32.const 596
      i32.add
      i32.load
      i32.const 16
      i32.and
      if  ;; label = @2
        local.get 0
        local.get 1
        i32.const 1
        i32.sub
        local.get 2
        i32.const -268435456
        i32.and
        i32.or
        i32.store offset=4
      else
        local.get 0
        local.get 1
        i32.const 1
        i32.sub
        i32.const -1342177280
        i32.or
        i32.store offset=4
        local.get 2
        i32.const -2147483648
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 0
          call 22
        end
      end
    end)
  (func (;24;) (type 4) (param i32)
    local.get 0
    i32.const 636
    i32.gt_u
    if  ;; label = @1
      local.get 0
      i32.const 16
      i32.sub
      call 23
    end)
  (func (;25;) (type 6) (result i32)
    (local i32 i32 i32 i32)
    i32.const 568
    call 18
    drop
    i32.const 568
    local.set 0
    i32.const 564
    i32.load
    i32.const 568
    i32.add
    local.set 2
    loop  ;; label = @1
      local.get 0
      local.get 2
      i32.lt_u
      if  ;; label = @2
        local.get 0
        i32.load16_u
        local.tee 3
        i32.const 128
        i32.lt_u
        if (result i32)  ;; label = @3
          local.get 1
          i32.const 1
          i32.add
        else
          local.get 3
          i32.const 2048
          i32.lt_u
          if (result i32)  ;; label = @4
            local.get 1
            i32.const 2
            i32.add
          else
            local.get 0
            i32.const 2
            i32.add
            local.get 2
            i32.lt_u
            i32.const 0
            local.get 3
            i32.const 64512
            i32.and
            i32.const 55296
            i32.eq
            select
            if  ;; label = @5
              local.get 0
              i32.load16_u offset=2
              i32.const 64512
              i32.and
              i32.const 56320
              i32.eq
              if  ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                local.set 0
                local.get 1
                i32.const 4
                i32.add
                local.set 1
                br 5 (;@1;)
              end
            end
            local.get 1
            i32.const 3
            i32.add
          end
        end
        local.set 1
        local.get 0
        i32.const 2
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end
    i32.const 568
    call 24
    local.get 1)
  (func (;26;) (type 6) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    i32.const 568
    call 18
    drop
    i32.const 568
    local.set 2
    i32.const 564
    i32.load
    i32.const 568
    i32.add
    local.set 3
    call 25
    i32.const 0
    call 16
    local.tee 4
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.lt_u
      if  ;; label = @2
        local.get 2
        i32.load16_u
        local.tee 1
        i32.const 128
        i32.lt_u
        if (result i32)  ;; label = @3
          local.get 0
          local.get 1
          i32.store8
          local.get 0
          i32.const 1
          i32.add
        else
          local.get 1
          i32.const 2048
          i32.lt_u
          if (result i32)  ;; label = @4
            local.get 0
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            i32.store8
            local.get 0
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=1
            local.get 0
            i32.const 2
            i32.add
          else
            local.get 2
            i32.const 2
            i32.add
            local.get 3
            i32.lt_u
            i32.const 0
            local.get 1
            i32.const 64512
            i32.and
            i32.const 55296
            i32.eq
            select
            if  ;; label = @5
              local.get 2
              i32.load16_u offset=2
              local.tee 5
              i32.const 64512
              i32.and
              i32.const 56320
              i32.eq
              if  ;; label = @6
                local.get 0
                local.get 1
                i32.const 1023
                i32.and
                i32.const 10
                i32.shl
                i32.const 65536
                i32.add
                local.get 5
                i32.const 1023
                i32.and
                i32.add
                local.tee 1
                i32.const 18
                i32.shr_u
                i32.const 240
                i32.or
                i32.store8
                local.get 0
                local.get 1
                i32.const 12
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=1
                local.get 0
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=2
                local.get 0
                local.get 1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=3
                local.get 2
                i32.const 4
                i32.add
                local.set 2
                local.get 0
                i32.const 4
                i32.add
                local.set 0
                br 5 (;@1;)
              end
            end
            local.get 0
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8
            local.get 0
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=1
            local.get 0
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=2
            local.get 0
            i32.const 3
            i32.add
          end
        end
        local.set 0
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 2
    local.get 3
    i32.ne
    if  ;; label = @1
      unreachable
    end
    local.get 4
    call 18
    i32.const 568
    call 24)
  (func (;27;) (type 3) (param i32) (result i32)
    local.get 0
    i32.const 16
    i32.sub
    i32.load offset=12)
  (func (;28;) (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    call 18
    drop
    block  ;; label = @1
      i32.const 0
      local.get 0
      call 18
      local.tee 2
      call 27
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        local.get 1
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
        local.get 2
        call 27
        i32.const -2147483648
        i32.and
        br_if 1 (;@1;)
        local.get 2
        call 27
        local.set 1
      end
      local.get 1
      local.get 2
      call 27
      i32.gt_s
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call 16
      local.tee 3
      local.get 2
      call 18
      i32.store
      local.get 3
      local.get 1
      i32.store offset=8
      local.get 3
      local.get 2
      i32.store offset=4
      local.get 3
      call 18
      local.get 2
      call 24
      local.get 0
      call 24
      return
    end
    local.get 2
    call 24
    unreachable)
  (func (;29;) (type 6) (result i32)
    (local i32 i32)
    i32.const 568
    call 18
    drop
    call 26
    local.set 0
    i32.const 1
    global.set 5
    local.get 0
    i32.const -1
    call 28
    local.get 0
    call 24
    i32.const 568
    call 24)
  (func (;30;) (type 1)
    (local i32)
    call 29
    local.tee 0
    call 18
    drop
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    call 0
    local.get 0
    call 24
    local.get 0
    call 24)
  (func (;31;) (type 2) (param i32 i32)
    local.get 0
    i32.const 636
    i32.lt_u
    if  ;; label = @1
      return
    end
    local.get 0
    i32.const 16
    i32.sub
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 1
                i32.ne
                if  ;; label = @7
                  local.get 1
                  i32.const 2
                  i32.eq
                  br_if 1 (;@6;)
                  block  ;; label = @8
                    local.get 1
                    i32.const 3
                    i32.sub
                    br_table 3 (;@5;) 4 (;@4;) 5 (;@3;) 0 (;@8;)
                  end
                  br 5 (;@2;)
                end
                local.get 0
                call 23
                br 5 (;@1;)
              end
              local.get 0
              i32.load offset=4
              i32.const 268435455
              i32.and
              i32.const 0
              i32.le_u
              if  ;; label = @6
                unreachable
              end
              local.get 0
              local.get 0
              i32.load offset=4
              i32.const 1
              i32.sub
              i32.store offset=4
              local.get 0
              call 7
              br 4 (;@1;)
            end
            local.get 0
            call 10
            br 3 (;@1;)
          end
          local.get 0
          i32.load offset=4
          local.tee 1
          i32.const -268435456
          i32.and
          local.get 1
          i32.const 1
          i32.add
          i32.const -268435456
          i32.and
          i32.ne
          if  ;; label = @4
            unreachable
          end
          local.get 0
          local.get 1
          i32.const 1
          i32.add
          i32.store offset=4
          local.get 1
          i32.const 1879048192
          i32.and
          if  ;; label = @4
            local.get 0
            call 9
          end
          br 2 (;@1;)
        end
        local.get 0
        call 11
        br 1 (;@1;)
      end
      unreachable
    end)
  (func (;32;) (type 2) (param i32 i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.tee 2
    local.get 0
    i32.load offset=12
    i32.const 2
    i32.shl
    i32.add
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.lt_u
      if  ;; label = @2
        local.get 2
        i32.load
        local.tee 3
        if  ;; label = @3
          local.get 3
          local.get 1
          call 31
        end
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        br 1 (;@1;)
      end
    end)
  (func (;33;) (type 2) (param i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 8
            i32.sub
            i32.load
            br_table 0 (;@4;) 0 (;@4;) 3 (;@1;) 1 (;@3;) 3 (;@1;) 2 (;@2;)
          end
          return
        end
        local.get 0
        local.get 1
        call 32
        br 1 (;@1;)
      end
      unreachable
    end
    local.get 0
    i32.load
    local.tee 0
    if  ;; label = @1
      local.get 0
      local.get 1
      call 31
    end)
  (memory (;0;) 1 80)
  (global (;0;) (mut i32) (i32.const 0))
  (global (;1;) (mut i32) (i32.const 0))
  (global (;2;) (mut i32) (i32.const 0))
  (global (;3;) (mut i32) (i32.const 0))
  (global (;4;) (mut i32) (i32.const 0))
  (global (;5;) (mut i32) (i32.const 0))
  (export "invoke" (func 30))
  (data (;0;) (i32.const 8) " \00\00\00\01\00\00\00\01\00\00\00 \00\00\000\001\002\003\004\005\006\007\008\009\00a\00b\00c\00d\00e\00f")
  (data (;1;) (i32.const 56) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\000")
  (data (;2;) (i32.const 80) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\001")
  (data (;3;) (i32.const 104) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\002")
  (data (;4;) (i32.const 128) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\003")
  (data (;5;) (i32.const 152) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\004")
  (data (;6;) (i32.const 176) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\005")
  (data (;7;) (i32.const 200) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\006")
  (data (;8;) (i32.const 224) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\007")
  (data (;9;) (i32.const 248) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\008")
  (data (;10;) (i32.const 272) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\009")
  (data (;11;) (i32.const 296) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00a")
  (data (;12;) (i32.const 320) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00b")
  (data (;13;) (i32.const 344) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00c")
  (data (;14;) (i32.const 368) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00d")
  (data (;15;) (i32.const 392) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00e")
  (data (;16;) (i32.const 416) "\02\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00f")
  (data (;17;) (i32.const 440) "@\00\00\00\01\00\00\00\00\00\00\00@\00\00\00H\00\00\00`\00\00\00x\00\00\00\90\00\00\00\a8\00\00\00\c0\00\00\00\d8\00\00\00\f0\00\00\00\08\01\00\00 \01\00\008\01\00\00P\01\00\00h\01\00\00\80\01\00\00\98\01\00\00\b0\01")
  (data (;18;) (i32.const 520) "\10\00\00\00\01\00\00\00\03\00\00\00\10\00\00\00\c8\01\00\00\c8\01\00\00@\00\00\00\10")
  (data (;19;) (i32.const 552) "\16\00\00\00\01\00\00\00\01\00\00\00\16\00\00\00h\00e\00l\00l\00o\00 \00w\00o\00r\00l\00d")
  (data (;20;) (i32.const 592) "\05\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\93 \00\00\02\00\00\001\00\00\00\02"))
