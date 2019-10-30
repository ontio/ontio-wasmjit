(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (import "env" "ontio_current_blockhash" (func $ontio_current_blockhash (type 0)))
  (func $invoke (type 1) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 0
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.tee 2
    i64.const 0
    i64.store
    local.get 0
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.tee 3
    i64.const 0
    i64.store
    local.get 0
    i64.const 0
    i64.store offset=32
    local.get 0
    i32.const 32
    i32.add
    call $ontio_current_blockhash
    drop
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 3
    i64.load
    i64.store
    local.get 0
    local.get 0
    i64.load offset=32
    i64.store
    local.get 1
    i64.const 72340172838076673
    i64.store
    local.get 2
    i64.const 72340172838076673
    i64.store
    local.get 3
    i64.const 72340172838076673
    i64.store
    local.get 0
    i64.const 72340172838076673
    i64.store offset=32
    local.get 0
    local.get 0
    i32.const 32
    i32.add
    i32.const 32
    call $memcmp
    local.set 1
    local.get 0
    i32.const 64
    i32.add
    global.set 0
    local.get 1
    i32.eqz)
  (func $memcmp (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 32768))
  (global (;1;) i32 (i32.const 32768))
  (global (;2;) i32 (i32.const 32768))
  (export "get_block_hash" (func $invoke)))
