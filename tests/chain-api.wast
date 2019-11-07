(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (result i64)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i32 i32 i32)))
  (type (;10;) (func (param i64 i32) (result i32)))
  (type (;11;) (func (param i32) (result i64)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32) (result i32)))
  (import "env" "ontio_input_length" (func $ontio_input_length (type 2)))
  (import "env" "ontio_get_input" (func $ontio_get_input (type 3)))
  (import "env" "ontio_current_blockhash" (func $ontio_current_blockhash (type 4)))
  (import "env" "ontio_current_txhash" (func $ontio_current_txhash (type 4)))
  (import "env" "ontio_timestamp" (func $ontio_timestamp (type 5)))
  (import "env" "ontio_block_height" (func $ontio_block_height (type 2)))
  (import "env" "ontio_self_address" (func $ontio_self_address (type 3)))
  (import "env" "ontio_caller_address" (func $ontio_caller_address (type 3)))
  (import "env" "ontio_check_witness" (func $ontio_check_witness (type 4)))
  (import "env" "ontio_sha256" (func $ontio_sha256 (type 6)))
  (import "env" "ontio_entry_address" (func $ontio_entry_address (type 3)))
  (import "env" "ontio_panic" (func $ontio_panic (type 7)))
  (func $invoke (type 2) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    i32.const 0
    local.set 1
    block  ;; label = @1
      call $ontio_input_length
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const -1
              i32.le_s
              br_if 0 (;@5;)
              local.get 2
              call $__rust_alloc_zeroed
              local.tee 3
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              call $ontio_get_input
              local.get 2
              i32.const -6
              i32.add
              local.tee 4
              i32.const 16
              i32.gt_u
              br_if 4 (;@1;)
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              br_table 7 (;@6;) 12 (;@1;) 12 (;@1;) 12 (;@1;) 12 (;@1;) 12 (;@1;) 4 (;@9;) 2 (;@11;) 5 (;@8;) 12 (;@1;) 3 (;@10;) 12 (;@1;) 12 (;@1;) 1 (;@12;) 12 (;@1;) 12 (;@1;) 0 (;@13;) 7 (;@6;)
                            end
                            block  ;; label = @13
                              local.get 3
                              i32.const 32768
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 3
                              i32.const 32768
                              i32.const 22
                              call $memcmp
                              br_if 12 (;@1;)
                            end
                            local.get 0
                            i32.const 32
                            i32.add
                            i32.const 24
                            i32.add
                            local.tee 2
                            i64.const 0
                            i64.store
                            local.get 0
                            i32.const 32
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 1
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
                            local.get 2
                            i64.load
                            i64.store
                            local.get 0
                            i32.const 16
                            i32.add
                            local.get 1
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
                            local.get 2
                            i64.const 72340172838076673
                            i64.store
                            local.get 1
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
                            i32.eqz
                            local.set 1
                            br 11 (;@1;)
                          end
                          block  ;; label = @12
                            local.get 3
                            i32.const 32790
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 32790
                            i32.const 19
                            call $memcmp
                            br_if 11 (;@1;)
                          end
                          local.get 0
                          i32.const 32
                          i32.add
                          i32.const 24
                          i32.add
                          local.tee 2
                          i64.const 0
                          i64.store
                          local.get 0
                          i32.const 32
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 1
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
                          call $ontio_current_txhash
                          drop
                          local.get 0
                          i32.const 24
                          i32.add
                          local.get 2
                          i64.load
                          i64.store
                          local.get 0
                          i32.const 16
                          i32.add
                          local.get 1
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
                          local.get 2
                          i64.const 72340172838076673
                          i64.store
                          local.get 1
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
                          i32.eqz
                          local.set 1
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          local.get 3
                          i32.const 32809
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const 32809
                          i32.const 13
                          call $memcmp
                          br_if 4 (;@7;)
                        end
                        call $ontio_timestamp
                        i64.const 1
                        i64.eq
                        local.set 1
                        br 9 (;@1;)
                      end
                      block  ;; label = @10
                        local.get 3
                        i32.const 32822
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 32822
                        i32.const 16
                        call $memcmp
                        br_if 9 (;@1;)
                      end
                      call $ontio_block_height
                      i32.const 1
                      i32.eq
                      local.set 1
                      br 8 (;@1;)
                    end
                    block  ;; label = @9
                      local.get 3
                      i32.const 32838
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 32838
                      i32.const 12
                      call $memcmp
                      br_if 8 (;@1;)
                    end
                    local.get 0
                    i32.const 32
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 2
                    i32.const 0
                    i32.store
                    local.get 0
                    i32.const 32
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 1
                    i64.const 0
                    i64.store
                    local.get 0
                    i64.const 0
                    i64.store offset=32
                    local.get 0
                    i32.const 32
                    i32.add
                    call $ontio_self_address
                    local.get 0
                    i32.const 16
                    i32.add
                    local.get 2
                    i32.load
                    i32.store
                    local.get 0
                    i32.const 8
                    i32.add
                    local.get 1
                    i64.load
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=32
                    i64.store
                    local.get 2
                    i32.const 16843009
                    i32.store
                    local.get 1
                    i64.const 72340172838076673
                    i64.store
                    local.get 0
                    i64.const 72340172838076673
                    i64.store offset=32
                    local.get 0
                    local.get 0
                    i32.const 32
                    i32.add
                    i32.const 20
                    call $memcmp
                    i32.eqz
                    local.set 1
                    br 7 (;@1;)
                  end
                  block  ;; label = @8
                    local.get 3
                    i32.const 32850
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 32850
                    i32.const 14
                    call $memcmp
                    br_if 7 (;@1;)
                  end
                  local.get 0
                  i32.const 32
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 2
                  i32.const 0
                  i32.store
                  local.get 0
                  i32.const 32
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 1
                  i64.const 0
                  i64.store
                  local.get 0
                  i64.const 0
                  i64.store offset=32
                  local.get 0
                  i32.const 32
                  i32.add
                  call $ontio_caller_address
                  local.get 0
                  i32.const 16
                  i32.add
                  local.get 2
                  i32.load
                  i32.store
                  local.get 0
                  i32.const 8
                  i32.add
                  local.get 1
                  i64.load
                  i64.store
                  local.get 0
                  local.get 0
                  i64.load offset=32
                  i64.store
                  local.get 2
                  i32.const 16843009
                  i32.store
                  local.get 1
                  i64.const 72340172838076673
                  i64.store
                  local.get 0
                  i64.const 72340172838076673
                  i64.store offset=32
                  local.get 0
                  local.get 0
                  i32.const 32
                  i32.add
                  i32.const 20
                  call $memcmp
                  i32.eqz
                  local.set 1
                  br 6 (;@1;)
                end
                local.get 3
                i32.const 32864
                i32.eq
                br_if 4 (;@2;)
                local.get 3
                i32.const 32864
                local.get 2
                call $memcmp
                i32.eqz
                br_if 4 (;@2;)
                block  ;; label = @7
                  local.get 3
                  i32.const 32877
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 32877
                  local.get 2
                  call $memcmp
                  br_if 6 (;@1;)
                end
                local.get 0
                i32.const 32
                i32.add
                i32.const 16
                i32.add
                local.tee 2
                i32.const 0
                i32.store
                local.get 0
                i32.const 32
                i32.add
                i32.const 8
                i32.add
                local.tee 1
                i64.const 0
                i64.store
                local.get 0
                i64.const 0
                i64.store offset=32
                local.get 0
                i32.const 32
                i32.add
                call $ontio_caller_address
                local.get 0
                i32.const 16
                i32.add
                local.get 2
                i32.load
                i32.store
                local.get 0
                i32.const 8
                i32.add
                local.get 1
                i64.load
                i64.store
                local.get 0
                local.get 0
                i64.load offset=32
                i64.store
                local.get 0
                call $ontio_check_witness
                i32.const 0
                i32.ne
                local.set 1
                br 5 (;@1;)
              end
              local.get 3
              i32.const 32890
              i32.eq
              br_if 2 (;@3;)
              local.get 3
              i32.const 32890
              i32.const 6
              call $memcmp
              i32.eqz
              br_if 2 (;@3;)
              br 4 (;@1;)
            end
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in28_$u7b$$u7b$closure$u7d$$u7d$17h5b44c16e5b32f737E.llvm.8078137103478694412
            unreachable
          end
          local.get 2
          i32.const 1
          call $rust_oom
          unreachable
        end
        local.get 0
        i32.const 32
        i32.add
        i32.const 24
        i32.add
        local.tee 2
        i64.const 0
        i64.store
        local.get 0
        i32.const 32
        i32.add
        i32.const 16
        i32.add
        local.tee 1
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
        i32.const 32896
        i32.const 4
        local.get 0
        i32.const 32
        i32.add
        call $ontio_sha256
        local.get 0
        i32.const 24
        i32.add
        local.get 2
        i64.load
        i64.store
        local.get 0
        i32.const 16
        i32.add
        local.get 1
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
        block  ;; label = @3
          block  ;; label = @4
            i32.const 64
            call $__rust_alloc
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i64.const 64
            i64.store offset=68 align=4
            local.get 0
            local.get 2
            i32.store offset=64
            i32.const 31
            local.set 2
            loop  ;; label = @5
              local.get 0
              i32.const 1
              i32.store offset=84
              local.get 0
              local.get 0
              local.get 2
              i32.add
              i32.store offset=80
              local.get 0
              local.get 0
              i32.const 64
              i32.add
              i32.store offset=92
              local.get 0
              i32.const 1
              i32.store offset=52
              local.get 0
              i32.const 1
              i32.store offset=44
              local.get 0
              i32.const 33812
              i32.store offset=40
              local.get 0
              i32.const 1
              i32.store offset=36
              local.get 0
              i32.const 33932
              i32.store offset=32
              local.get 0
              local.get 0
              i32.const 80
              i32.add
              i32.store offset=48
              local.get 0
              i32.const 92
              i32.add
              i32.const 33972
              local.get 0
              i32.const 32
              i32.add
              call $_ZN4core3fmt5write17hb9bf49115b8795e8E
              br_if 2 (;@3;)
              local.get 2
              i32.const -1
              i32.add
              local.tee 2
              i32.const -1
              i32.ne
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 1
            local.get 0
            i32.load offset=72
            i32.const 64
            i32.ne
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 0
              i32.load offset=64
              local.tee 2
              i32.const 32900
              i32.ne
              br_if 0 (;@5;)
              i32.const 1
              local.set 1
              br 4 (;@1;)
            end
            local.get 2
            i32.const 32900
            i32.const 64
            call $memcmp
            i32.eqz
            local.set 1
            br 3 (;@1;)
          end
          i32.const 64
          i32.const 1
          call $rust_oom
          unreachable
        end
        i32.const 33848
        i32.const 43
        local.get 0
        i32.const 32
        i32.add
        i32.const 33892
        call $_ZN4core6result13unwrap_failed17h3987edb2614d0bc9E
        unreachable
      end
      local.get 0
      i32.const 32
      i32.add
      i32.const 16
      i32.add
      local.tee 2
      i32.const 0
      i32.store
      local.get 0
      i32.const 32
      i32.add
      i32.const 8
      i32.add
      local.tee 1
      i64.const 0
      i64.store
      local.get 0
      i64.const 0
      i64.store offset=32
      local.get 0
      i32.const 32
      i32.add
      call $ontio_entry_address
      local.get 0
      i32.const 16
      i32.add
      local.get 2
      i32.load
      i32.store
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      i64.load
      i64.store
      local.get 0
      local.get 0
      i64.load offset=32
      i64.store
      local.get 2
      i32.const 16843009
      i32.store
      local.get 1
      i64.const 72340172838076673
      i64.store
      local.get 0
      i64.const 72340172838076673
      i64.store offset=32
      local.get 0
      local.get 0
      i32.const 32
      i32.add
      i32.const 20
      call $memcmp
      i32.eqz
      local.set 1
    end
    local.get 0
    i32.const 96
    i32.add
    global.set 0
    local.get 1)
  (func $__rust_alloc_zeroed (type 4) (param i32) (result i32)
    local.get 0
    call $__rg_alloc_zeroed)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in28_$u7b$$u7b$closure$u7d$$u7d$17h5b44c16e5b32f737E.llvm.8078137103478694412 (type 8)
    call $_ZN5alloc7raw_vec17capacity_overflow17h64ecb6f7cf049a7dE
    unreachable)
  (func $rust_oom (type 7) (param i32 i32)
    unreachable
    unreachable)
  (func $__rust_alloc (type 4) (param i32) (result i32)
    local.get 0
    call $__rg_alloc)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hbe5ab8d7970b6d6bE (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 87
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      call $_ZN4core5slice22slice_index_order_fail17h10af61aef1934e6aE
      unreachable
    end
    local.get 1
    i32.const 33543
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h489353efccf46bb0E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3fmt5write17hb9bf49115b8795e8E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 52
    i32.add
    local.get 2
    i32.const 20
    i32.add
    i32.load
    local.tee 4
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=56
    local.get 3
    i32.const 44
    i32.add
    local.get 2
    i32.load offset=16
    local.tee 5
    local.get 4
    i32.const 3
    i32.shl
    i32.add
    i32.store
    local.get 3
    i64.const 137438953472
    i64.store offset=8
    local.get 3
    local.get 0
    i32.store offset=32
    i32.const 0
    local.set 6
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    local.get 5
    i32.store offset=48
    local.get 3
    local.get 5
    i32.store offset=40
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=8
              local.tee 7
              br_if 0 (;@5;)
              local.get 2
              i32.load
              local.set 8
              local.get 2
              i32.load offset=4
              local.tee 9
              local.get 4
              local.get 4
              local.get 9
              i32.gt_u
              select
              local.tee 10
              i32.eqz
              br_if 1 (;@4;)
              i32.const 1
              local.set 4
              local.get 0
              local.get 8
              i32.load
              local.get 8
              i32.load offset=4
              local.get 1
              i32.load offset=12
              call_indirect (type 0)
              br_if 4 (;@1;)
              local.get 8
              i32.const 8
              i32.add
              local.set 2
              i32.const 1
              local.set 6
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load
                  local.get 3
                  i32.const 8
                  i32.add
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  call_indirect (type 1)
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 1
                  local.set 4
                  br 6 (;@1;)
                end
                local.get 6
                local.get 10
                i32.ge_u
                br_if 2 (;@4;)
                local.get 2
                i32.const 4
                i32.add
                local.set 0
                local.get 2
                i32.load
                local.set 1
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 2
                i32.const 8
                i32.add
                local.set 2
                i32.const 1
                local.set 4
                local.get 6
                i32.const 1
                i32.add
                local.set 6
                local.get 3
                i32.load offset=32
                local.get 1
                local.get 0
                i32.load
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 0)
                i32.eqz
                br_if 0 (;@6;)
                br 5 (;@1;)
              end
            end
            local.get 2
            i32.load
            local.set 8
            local.get 2
            i32.load offset=4
            local.tee 9
            local.get 2
            i32.const 12
            i32.add
            i32.load
            local.tee 5
            local.get 5
            local.get 9
            i32.gt_u
            select
            local.tee 10
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            local.get 0
            local.get 8
            i32.load
            local.get 8
            i32.load offset=4
            local.get 1
            i32.load offset=12
            call_indirect (type 0)
            br_if 3 (;@1;)
            local.get 7
            i32.const 16
            i32.add
            local.set 5
            local.get 8
            i32.const 8
            i32.add
            local.set 2
            i32.const 1
            local.set 6
            loop  ;; label = @5
              local.get 3
              local.get 5
              i32.const -8
              i32.add
              i32.load
              i32.store offset=12
              local.get 3
              local.get 5
              i32.const 16
              i32.add
              i32.load8_u
              i32.store8 offset=56
              local.get 3
              local.get 5
              i32.const -4
              i32.add
              i32.load
              i32.store offset=8
              i32.const 0
              local.set 1
              i32.const 0
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 5
                      i32.const 8
                      i32.add
                      i32.load
                      br_table 0 (;@9;) 1 (;@8;) 2 (;@7;) 3 (;@6;) 0 (;@9;)
                    end
                    local.get 5
                    i32.const 12
                    i32.add
                    i32.load
                    local.set 0
                    i32.const 1
                    local.set 4
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    local.get 5
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee 7
                    local.get 3
                    i32.load offset=52
                    local.tee 4
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 4
                    local.get 3
                    i32.load offset=48
                    local.get 7
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.load offset=4
                    i32.const 2
                    i32.ne
                    br_if 2 (;@6;)
                    local.get 7
                    i32.load
                    i32.load
                    local.set 0
                    i32.const 1
                    local.set 4
                    br 2 (;@6;)
                  end
                  i32.const 33572
                  local.get 7
                  local.get 4
                  call $_ZN4core9panicking18panic_bounds_check17h654e88f8d4c6e8d6E
                  unreachable
                end
                i32.const 0
                local.set 4
                local.get 3
                i32.load offset=40
                local.tee 7
                local.get 3
                i32.load offset=44
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                local.get 7
                i32.const 8
                i32.add
                i32.store offset=40
                i32.const 0
                local.set 4
                local.get 7
                i32.load offset=4
                i32.const 2
                i32.ne
                br_if 0 (;@6;)
                local.get 7
                i32.load
                i32.load
                local.set 0
                i32.const 1
                local.set 4
              end
              local.get 3
              local.get 0
              i32.store offset=20
              local.get 3
              local.get 4
              i32.store offset=16
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            i32.load
                            br_table 4 (;@8;) 1 (;@11;) 0 (;@12;) 6 (;@6;) 4 (;@8;)
                          end
                          local.get 3
                          i32.load offset=40
                          local.tee 0
                          local.get 3
                          i32.load offset=44
                          i32.ne
                          br_if 1 (;@10;)
                          br 5 (;@6;)
                        end
                        local.get 5
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 0
                        local.get 3
                        i32.load offset=52
                        local.tee 4
                        i32.ge_u
                        br_if 1 (;@9;)
                        local.get 3
                        i32.load offset=48
                        local.get 0
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 0
                        i32.load offset=4
                        i32.const 2
                        i32.ne
                        br_if 4 (;@6;)
                        local.get 0
                        i32.load
                        i32.load
                        local.set 4
                        br 3 (;@7;)
                      end
                      local.get 3
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.store offset=40
                      local.get 0
                      i32.load offset=4
                      i32.const 2
                      i32.ne
                      br_if 3 (;@6;)
                      local.get 0
                      i32.load
                      i32.load
                      local.set 4
                      br 2 (;@7;)
                    end
                    i32.const 33572
                    local.get 0
                    local.get 4
                    call $_ZN4core9panicking18panic_bounds_check17h654e88f8d4c6e8d6E
                    unreachable
                  end
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  local.set 4
                end
                i32.const 1
                local.set 1
              end
              local.get 3
              local.get 4
              i32.store offset=28
              local.get 3
              local.get 1
              i32.store offset=24
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.const -16
                  i32.add
                  i32.load
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=40
                  local.tee 4
                  local.get 3
                  i32.load offset=44
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 3
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.store offset=40
                  br 1 (;@6;)
                end
                local.get 5
                i32.const -12
                i32.add
                i32.load
                local.tee 4
                local.get 3
                i32.load offset=52
                local.tee 0
                i32.ge_u
                br_if 4 (;@2;)
                local.get 3
                i32.load offset=48
                local.get 4
                i32.const 3
                i32.shl
                i32.add
                local.set 4
              end
              block  ;; label = @6
                local.get 4
                i32.load
                local.get 3
                i32.const 8
                i32.add
                local.get 4
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 1)
                i32.eqz
                br_if 0 (;@6;)
                i32.const 1
                local.set 4
                br 5 (;@1;)
              end
              local.get 6
              local.get 10
              i32.ge_u
              br_if 1 (;@4;)
              local.get 2
              i32.const 4
              i32.add
              local.set 0
              local.get 2
              i32.load
              local.set 1
              local.get 5
              i32.const 36
              i32.add
              local.set 5
              local.get 2
              i32.const 8
              i32.add
              local.set 2
              i32.const 1
              local.set 4
              local.get 6
              i32.const 1
              i32.add
              local.set 6
              local.get 3
              i32.load offset=32
              local.get 1
              local.get 0
              i32.load
              local.get 3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type 0)
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          block  ;; label = @4
            local.get 9
            local.get 6
            i32.le_u
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            local.get 3
            i32.load offset=32
            local.get 8
            local.get 6
            i32.const 3
            i32.shl
            i32.add
            local.tee 5
            i32.load
            local.get 5
            i32.load offset=4
            local.get 3
            i32.load offset=36
            i32.load offset=12
            call_indirect (type 0)
            br_if 3 (;@1;)
          end
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        i32.const 33548
        call $_ZN4core9panicking5panic17h28e0be805f9ef6d5E
        unreachable
      end
      i32.const 33588
      local.get 4
      local.get 0
      call $_ZN4core9panicking18panic_bounds_check17h654e88f8d4c6e8d6E
      unreachable
    end
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN4core6result13unwrap_failed17h3987edb2614d0bc9E (type 9) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 1
    i32.store offset=12
    local.get 4
    local.get 0
    i32.store offset=8
    local.get 4
    local.get 3
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 4
    i32.const 60
    i32.add
    i32.const 3
    i32.store
    local.get 4
    i64.const 2
    i64.store offset=28 align=4
    local.get 4
    i32.const 33716
    i32.store offset=24
    local.get 4
    i32.const 4
    i32.store offset=52
    local.get 4
    local.get 4
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 4
    local.get 4
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 4
    local.get 4
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 4
    i32.const 24
    i32.add
    i32.const 33736
    call $_ZN4core9panicking9panic_fmt17h3d02c4752a84b829E
    unreachable)
  (func $__rg_alloc (type 4) (param i32) (result i32)
    local.get 0
    call $_ZN72_$LT$ontio_bump_alloc..BumpAlloc$u20$as$u20$core..alloc..GlobalAlloc$GT$5alloc17h9ddc05dbd1c5c3afE)
  (func $__rust_realloc (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rg_realloc)
  (func $__rg_realloc (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      call $_ZN72_$LT$ontio_bump_alloc..BumpAlloc$u20$as$u20$core..alloc..GlobalAlloc$GT$5alloc17h9ddc05dbd1c5c3afE
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      local.get 2
      local.get 1
      local.get 1
      local.get 2
      i32.gt_u
      select
      call $memcpy
      drop
    end
    local.get 3)
  (func $__rg_alloc_zeroed (type 4) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      call $_ZN72_$LT$ontio_bump_alloc..BumpAlloc$u20$as$u20$core..alloc..GlobalAlloc$GT$5alloc17h9ddc05dbd1c5c3afE
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      local.get 0
      call $memset
      drop
    end
    local.get 1)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in28_$u7b$$u7b$closure$u7d$$u7d$17h3d03372a024c568eE (type 8)
    call $_ZN5alloc7raw_vec17capacity_overflow17h64ecb6f7cf049a7dE
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17h64ecb6f7cf049a7dE (type 8)
    i32.const 32964
    call $_ZN4core9panicking5panic17h28e0be805f9ef6d5E
    unreachable)
  (func $_ZN4core9panicking5panic17h28e0be805f9ef6d5E (type 3) (param i32)
    (local i32 i64 i64 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i64.load offset=8 align=4
    local.set 2
    local.get 0
    i64.load offset=16 align=4
    local.set 3
    local.get 0
    i64.load align=4
    local.set 4
    local.get 1
    i64.const 4
    i64.store offset=16
    local.get 1
    i64.const 1
    i64.store offset=4 align=4
    local.get 1
    local.get 4
    i64.store offset=24
    local.get 1
    local.get 1
    i32.const 24
    i32.add
    i32.store
    local.get 1
    local.get 3
    i64.store offset=40
    local.get 1
    local.get 2
    i64.store offset=32
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    call $_ZN4core9panicking9panic_fmt17h3d02c4752a84b829E
    unreachable)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h8a705a0d7d1f18d9E (type 7) (param i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.add
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 2
          i32.const 1
          i32.shl
          local.tee 3
          local.get 1
          local.get 3
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 1
              call $__rust_alloc
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            i32.load
            local.get 2
            local.get 1
            call $__rust_realloc
            local.set 2
          end
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store
        end
        return
      end
      local.get 1
      i32.const 1
      call $rust_oom
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h64ecb6f7cf049a7dE
    unreachable)
  (func $_ZN5alloc3fmt6format17h52854d2b33b7a4cfE (type 7) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 4
        i32.const 3
        i32.shl
        local.tee 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 6
        br 1 (;@1;)
      end
      local.get 3
      i32.const 4
      i32.add
      local.set 7
      i32.const 0
      local.set 6
      loop  ;; label = @2
        local.get 7
        i32.load
        local.get 6
        i32.add
        local.set 6
        local.get 7
        i32.const 8
        i32.add
        local.set 7
        local.get 5
        i32.const -8
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 20
                i32.add
                i32.load
                br_if 0 (;@6;)
                local.get 6
                local.set 7
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 4
                br_if 0 (;@6;)
                i32.const 33008
                i32.const 0
                i32.const 0
                call $_ZN4core9panicking18panic_bounds_check17h654e88f8d4c6e8d6E
                unreachable
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  i32.const 15
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=4
                  i32.eqz
                  br_if 1 (;@6;)
                end
                local.get 6
                local.get 6
                i32.add
                local.tee 7
                local.get 6
                i32.ge_u
                br_if 1 (;@5;)
              end
              i32.const 1
              local.set 5
              i32.const 0
              local.set 7
              local.get 2
              i32.const 8
              i32.add
              local.set 6
              br 1 (;@4;)
            end
            local.get 7
            i32.const -1
            i32.le_s
            br_if 1 (;@3;)
            local.get 2
            i32.const 8
            i32.add
            local.set 6
            block  ;; label = @5
              local.get 7
              br_if 0 (;@5;)
              i32.const 1
              local.set 5
              i32.const 0
              local.set 7
              br 1 (;@4;)
            end
            local.get 7
            call $__rust_alloc
            local.tee 5
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 2
          i32.const 0
          i32.store offset=16
          local.get 2
          local.get 7
          i32.store offset=12
          local.get 2
          local.get 5
          i32.store offset=8
          local.get 2
          local.get 2
          i32.const 8
          i32.add
          i32.store offset=20
          local.get 2
          i32.const 24
          i32.add
          i32.const 16
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i64.load align=4
          i64.store
          local.get 2
          i32.const 24
          i32.add
          i32.const 8
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i64.load align=4
          i64.store
          local.get 2
          local.get 1
          i64.load align=4
          i64.store offset=24
          local.get 2
          i32.const 20
          i32.add
          i32.const 33024
          local.get 2
          i32.const 24
          i32.add
          call $_ZN4core3fmt5write17hb9bf49115b8795e8E
          br_if 2 (;@1;)
          local.get 0
          local.get 6
          i64.load align=4
          i64.store align=4
          local.get 0
          i32.const 8
          i32.add
          local.get 6
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 48
          i32.add
          global.set 0
          return
        end
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in28_$u7b$$u7b$closure$u7d$$u7d$17h3d03372a024c568eE
        unreachable
      end
      local.get 7
      i32.const 1
      call $rust_oom
      unreachable
    end
    i32.const 33048
    i32.const 51
    local.get 2
    i32.const 24
    i32.add
    i32.const 33100
    call $_ZN4core6result13unwrap_failed17h3987edb2614d0bc9E
    unreachable)
  (func $_ZN4core9panicking18panic_bounds_check17h654e88f8d4c6e8d6E (type 6) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    local.get 1
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 5
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 33192
    i32.store offset=8
    local.get 3
    i32.const 5
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    call $_ZN4core9panicking9panic_fmt17h3d02c4752a84b829E
    unreachable)
  (func $_ZN4core3ptr18real_drop_in_place17heb402bda5ba4a707E (type 3) (param i32))
  (func $_ZN4core3ptr18real_drop_in_place17h0999c24ec24a2194E (type 3) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h3e50052967fc4094E (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 2
    call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h8a705a0d7d1f18d9E
    local.get 0
    local.get 0
    i32.load offset=8
    local.tee 3
    local.get 2
    i32.add
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h8e310dcc26e63065E (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 224
              i32.or
              i32.store8 offset=12
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load offset=4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h8a705a0d7d1f18d9E
            local.get 0
            i32.load offset=8
            local.set 3
          end
          local.get 0
          i32.load
          local.get 3
          i32.add
          local.get 1
          i32.store8
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 31
        i32.and
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      local.get 0
      local.get 1
      call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h8a705a0d7d1f18d9E
      local.get 0
      local.get 0
      i32.load offset=8
      local.tee 3
      local.get 1
      i32.add
      i32.store offset=8
      local.get 3
      local.get 0
      i32.load
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hfab4a9c8f73cb395E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 33024
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb9bf49115b8795e8E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h3e4eb74648f24612E (type 1) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h2d8ddd8455dd63c2E)
  (func $_ZN4core9panicking9panic_fmt17h3d02c4752a84b829E (type 7) (param i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i64.load align=4
    local.set 3
    local.get 2
    i32.const 20
    i32.add
    local.get 1
    i64.load offset=8 align=4
    i64.store align=4
    local.get 2
    local.get 3
    i64.store offset=12 align=4
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 33208
    i32.store offset=4
    local.get 2
    i32.const 1
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h2d8ddd8455dd63c2E (type 10) (param i64 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    i32.const 39
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 4
        br 1 (;@1;)
      end
      i32.const 39
      local.set 3
      loop  ;; label = @2
        local.get 2
        i32.const 9
        i32.add
        local.get 3
        i32.add
        local.tee 5
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 4
        i64.const -10000
        i64.mul
        i64.add
        i32.wrap_i64
        local.tee 6
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 7
        i32.const 1
        i32.shl
        i32.const 33274
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 5
        i32.const -2
        i32.add
        local.get 7
        i32.const -100
        i32.mul
        local.get 6
        i32.add
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 33274
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 5
        local.get 4
        local.set 0
        local.get 5
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 4
      i32.wrap_i64
      local.tee 5
      i32.const 99
      i32.le_s
      br_if 0 (;@1;)
      local.get 2
      i32.const 9
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 4
      i32.wrap_i64
      local.tee 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 5
      i32.const -100
      i32.mul
      local.get 6
      i32.add
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 33274
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.const 10
        i32.lt_s
        br_if 0 (;@2;)
        local.get 2
        i32.const 9
        i32.add
        local.get 3
        i32.const -2
        i32.add
        local.tee 3
        i32.add
        local.get 5
        i32.const 1
        i32.shl
        i32.const 33274
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 2
      i32.const 9
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 5
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 1
    i32.const 33940
    i32.const 0
    local.get 2
    i32.const 9
    i32.add
    local.get 3
    i32.add
    i32.const 39
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h489353efccf46bb0E
    local.set 3
    local.get 2
    i32.const 48
    i32.add
    global.set 0
    local.get 3)
  (func $rust_begin_unwind (type 3) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 2
            br_if 0 (;@4;)
            local.get 1
            i32.const 0
            i32.store offset=48
            br 1 (;@3;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 1
          i32.const 92
          i32.add
          i32.const 1
          i32.store
          local.get 1
          i64.const 1
          i64.store offset=76 align=4
          local.get 1
          i32.const 33932
          i32.store offset=72
          local.get 1
          i32.const 6
          i32.store offset=36
          local.get 1
          local.get 1
          i32.const 32
          i32.add
          i32.store offset=88
          local.get 1
          local.get 1
          i32.store offset=32
          local.get 1
          i32.const 48
          i32.add
          local.get 1
          i32.const 72
          i32.add
          call $_ZN5alloc3fmt6format17h52854d2b33b7a4cfE
          local.get 1
          i32.load offset=48
          br_if 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.store offset=8
        local.get 1
        i64.const 1
        i64.store
        br 1 (;@1;)
      end
      local.get 1
      i32.const 8
      i32.add
      local.get 1
      i32.const 48
      i32.add
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=48
      i64.store
    end
    local.get 0
    i32.const 20
    i32.add
    i32.load
    local.set 2
    local.get 1
    local.get 0
    i64.load offset=12 align=4
    i64.store offset=16
    local.get 1
    local.get 2
    i32.store offset=28
    local.get 1
    i32.const 48
    i32.add
    i32.const 20
    i32.add
    i32.const 3
    i32.store
    local.get 1
    i32.const 72
    i32.add
    i32.const 20
    i32.add
    i32.const 5
    i32.store
    local.get 1
    i32.const 84
    i32.add
    i32.const 7
    i32.store
    local.get 1
    i64.const 3
    i64.store offset=52 align=4
    local.get 1
    i32.const 33940
    i32.store offset=48
    local.get 1
    i32.const 8
    i32.store offset=76
    local.get 1
    local.get 1
    i32.const 72
    i32.add
    i32.store offset=64
    local.get 1
    local.get 1
    i32.const 28
    i32.add
    i32.store offset=88
    local.get 1
    local.get 1
    i32.const 16
    i32.add
    i32.store offset=80
    local.get 1
    local.get 1
    i32.store offset=72
    local.get 1
    i32.const 32
    i32.add
    local.get 1
    i32.const 48
    i32.add
    call $_ZN5alloc3fmt6format17h52854d2b33b7a4cfE
    local.get 1
    i32.load offset=32
    local.get 1
    i32.load offset=40
    call $_ZN9ontio_std7runtime5panic17h009a1911cc88a84bE
    unreachable)
  (func $_ZN4core3ptr18real_drop_in_place17h940a829d26b18402E (type 3) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h80c760f782d10510E (type 11) (param i32) (result i64)
    i64.const -2773332969905326582)
  (func $_ZN4core3fmt9Formatter12pad_integral17h489353efccf46bb0E (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load
    local.tee 5
    i32.const 1
    i32.and
    local.tee 6
    local.get 4
    i32.add
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      i32.const 0
      local.set 8
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.set 9
        local.get 1
        local.set 10
        loop  ;; label = @3
          local.get 8
          local.get 10
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 8
          local.get 10
          i32.const 1
          i32.add
          local.set 10
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
        end
      end
      local.get 7
      local.get 2
      i32.add
      local.get 8
      i32.sub
      local.set 7
    end
    i32.const 43
    i32.const 1114112
    local.get 6
    select
    local.set 8
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        i32.const 1
        local.set 10
        local.get 0
        local.get 8
        local.get 1
        local.get 2
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha248e7d03275de78E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 3
        local.get 4
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        return
      end
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 9
        local.get 7
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 10
        local.get 0
        local.get 8
        local.get 1
        local.get 2
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha248e7d03275de78E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 3
        local.get 4
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.const 8
          i32.and
          br_if 0 (;@3;)
          local.get 9
          local.get 7
          i32.sub
          local.set 9
          i32.const 0
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 1
                local.get 0
                i32.load8_u offset=48
                local.tee 7
                local.get 7
                i32.const 3
                i32.eq
                select
                br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 0 (;@6;) 2 (;@4;)
              end
              local.get 9
              local.set 10
              i32.const 0
              local.set 9
              br 1 (;@4;)
            end
            local.get 9
            i32.const 1
            i32.shr_u
            local.set 10
            local.get 9
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 9
          end
          local.get 10
          i32.const 1
          i32.add
          local.set 10
          loop  ;; label = @4
            local.get 10
            i32.const -1
            i32.add
            local.tee 10
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 10
        local.get 0
        i32.const 1
        i32.store8 offset=48
        local.get 0
        i32.const 48
        i32.store offset=4
        local.get 0
        local.get 8
        local.get 1
        local.get 2
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha248e7d03275de78E
        br_if 1 (;@1;)
        local.get 9
        local.get 7
        i32.sub
        local.set 8
        i32.const 0
        local.set 10
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 0
              i32.load8_u offset=48
              local.tee 9
              local.get 9
              i32.const 3
              i32.eq
              select
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 0 (;@5;) 2 (;@3;)
            end
            local.get 8
            local.set 10
            i32.const 0
            local.set 8
            br 1 (;@3;)
          end
          local.get 8
          i32.const 1
          i32.shr_u
          local.set 10
          local.get 8
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 8
        end
        local.get 10
        i32.const 1
        i32.add
        local.set 10
        block  ;; label = @3
          loop  ;; label = @4
            local.get 10
            i32.const -1
            i32.add
            local.tee 10
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=4
        local.set 9
        i32.const 1
        local.set 10
        local.get 0
        i32.load offset=24
        local.get 3
        local.get 4
        local.get 0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 8
        i32.const 1
        i32.add
        local.set 8
        local.get 0
        i32.load offset=28
        local.set 2
        local.get 0
        i32.load offset=24
        local.set 0
        loop  ;; label = @3
          block  ;; label = @4
            local.get 8
            i32.const -1
            i32.add
            local.tee 8
            br_if 0 (;@4;)
            i32.const 0
            return
          end
          i32.const 1
          local.set 10
          local.get 0
          local.get 9
          local.get 2
          i32.load offset=16
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 0
      i32.load offset=4
      local.set 7
      i32.const 1
      local.set 10
      local.get 0
      local.get 8
      local.get 1
      local.get 2
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha248e7d03275de78E
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=24
      local.get 3
      local.get 4
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 0)
      br_if 0 (;@1;)
      local.get 9
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load offset=28
      local.set 9
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 8
          i32.const -1
          i32.add
          local.tee 8
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        i32.const 1
        local.set 10
        local.get 0
        local.get 7
        local.get 9
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 10)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha248e7d03275de78E (type 13) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1114112
        i32.eq
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type 1)
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=24
      local.get 2
      local.get 3
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      local.set 4
    end
    local.get 4)
  (func $_ZN4core5slice22slice_index_order_fail17h10af61aef1934e6aE (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 128
    i32.store offset=4
    local.get 1
    local.get 0
    i32.store
    local.get 1
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 1
    i32.const 44
    i32.add
    i32.const 5
    i32.store
    local.get 1
    i64.const 2
    i64.store offset=12 align=4
    local.get 1
    i32.const 33476
    i32.store offset=8
    local.get 1
    i32.const 5
    i32.store offset=36
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 1
    local.get 1
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 1
    local.get 1
    i32.store offset=32
    local.get 1
    i32.const 8
    i32.add
    i32.const 33492
    call $_ZN4core9panicking9panic_fmt17h3d02c4752a84b829E
    unreachable)
  (func $_ZN4core3fmt9Formatter3pad17h339d16aa5bcc9066E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=16
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 4
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 0)
            local.set 3
            br 3 (;@1;)
          end
          local.get 3
          i32.eqz
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          local.get 2
          i32.add
          local.set 5
          local.get 0
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 7
          local.get 1
          local.set 3
          local.get 1
          local.set 8
          loop  ;; label = @4
            local.get 3
            i32.const 1
            i32.add
            local.set 9
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.load8_s
                  local.tee 10
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 9
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 11
                      local.get 5
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u offset=1
                    i32.const 63
                    i32.and
                    local.set 11
                    local.get 3
                    i32.const 2
                    i32.add
                    local.tee 9
                    local.set 3
                  end
                  local.get 10
                  i32.const 31
                  i32.and
                  local.set 12
                  block  ;; label = @8
                    local.get 10
                    i32.const 255
                    i32.and
                    local.tee 10
                    i32.const 223
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 13
                      local.get 5
                      local.set 14
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 13
                    local.get 3
                    i32.const 1
                    i32.add
                    local.tee 9
                    local.set 14
                  end
                  local.get 13
                  local.get 11
                  i32.const 6
                  i32.shl
                  i32.or
                  local.set 11
                  block  ;; label = @8
                    local.get 10
                    i32.const 240
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 14
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 10
                      local.get 9
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 14
                    i32.const 1
                    i32.add
                    local.set 3
                    local.get 14
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 10
                  end
                  local.get 11
                  i32.const 6
                  i32.shl
                  local.get 12
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.get 10
                  i32.or
                  local.tee 10
                  i32.const 1114112
                  i32.ne
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                local.get 10
                i32.const 255
                i32.and
                local.set 10
              end
              local.get 9
              local.set 3
            end
            block  ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 8
              i32.sub
              local.get 3
              i32.add
              local.set 7
              local.get 3
              local.set 8
              local.get 5
              local.get 3
              i32.ne
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
          end
          local.get 10
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 2
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              local.get 7
              local.get 2
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 1
            local.set 3
          end
          local.get 7
          local.get 2
          local.get 3
          select
          local.set 2
          local.get 3
          local.get 1
          local.get 3
          select
          local.set 1
        end
        local.get 4
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        return
      end
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 2
        local.get 9
        i32.sub
        local.get 0
        i32.load offset=12
        local.tee 6
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        return
      end
      i32.const 0
      local.set 7
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 9
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 2
      i32.sub
      local.get 6
      i32.add
      local.set 10
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            local.get 0
            i32.load8_u offset=48
            local.tee 3
            local.get 3
            i32.const 3
            i32.eq
            select
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 10
          local.set 7
          i32.const 0
          local.set 10
          br 1 (;@2;)
        end
        local.get 10
        i32.const 1
        i32.shr_u
        local.set 7
        local.get 10
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 10
      end
      local.get 7
      i32.const 1
      i32.add
      local.set 3
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=28
          i32.load offset=16
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      local.get 0
      i32.load offset=4
      local.set 9
      i32.const 1
      local.set 3
      local.get 0
      i32.load offset=24
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 0)
      br_if 0 (;@1;)
      local.get 10
      i32.const 1
      i32.add
      local.set 3
      local.get 0
      i32.load offset=28
      local.set 10
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 0
        local.get 9
        local.get 10
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      return
    end
    local.get 3)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdf23d6049b4697d9E (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h339d16aa5bcc9066E)
  (func $_ZN4core3fmt10ArgumentV110show_usize17hda1dd3e85f5b207cE (type 1) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h2d8ddd8455dd63c2E)
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h2a9057d1634d3fa9E (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 33702
    i32.const 14
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0))
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he9a1257f53511431E (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 1))
  (func $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hc08d2e58679b49f8E (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 33773
    i32.const 5
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0))
  (func $_ZN72_$LT$ontio_bump_alloc..BumpAlloc$u20$as$u20$core..alloc..GlobalAlloc$GT$5alloc17h9ddc05dbd1c5c3afE (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      i32.const 0
      i32.load offset=34020
      br_if 0 (;@1;)
      i32.const 0
      i32.const -1
      i32.store offset=34020
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=34024
          local.tee 2
          local.get 0
          i32.add
          local.tee 3
          i32.const 0
          i32.load offset=34028
          local.tee 4
          i32.le_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 3
            local.get 4
            i32.sub
            i32.const 65535
            i32.add
            local.tee 4
            i32.const 16
            i32.shr_u
            memory.grow
            local.tee 2
            i32.const -1
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 2 (;@2;)
          end
          local.get 2
          i32.const 16
          i32.shl
          local.set 3
          block  ;; label = @4
            i32.const 0
            i32.load offset=34024
            local.tee 2
            br_if 0 (;@4;)
            i32.const 0
            local.get 3
            i32.store offset=34024
            local.get 3
            local.set 2
          end
          i32.const 0
          local.get 3
          local.get 4
          i32.const -65536
          i32.and
          i32.add
          i32.store offset=34028
          local.get 2
          local.get 0
          i32.add
          local.set 3
        end
        i32.const 0
        local.get 3
        i32.store offset=34024
      end
      i32.const 0
      i32.const 0
      i32.load offset=34020
      i32.const 1
      i32.add
      i32.store offset=34020
      local.get 1
      i32.const 16
      i32.add
      global.set 0
      local.get 2
      return
    end
    i32.const 33778
    i32.const 16
    local.get 1
    i32.const 8
    i32.add
    i32.const 33796
    call $_ZN4core6result13unwrap_failed17h3987edb2614d0bc9E
    unreachable)
  (func $_ZN4core3ptr18real_drop_in_place17h4912c9903427c192E (type 3) (param i32))
  (func $_ZN4core3ptr18real_drop_in_place17h0383ec85acded427E (type 3) (param i32))
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1af775f68a8cc3d5E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 1
    i32.load offset=24
    local.set 4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.load
    local.tee 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 4
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb9bf49115b8795e8E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h5e238c3a303a5a13E (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h339d16aa5bcc9066E)
  (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h26703904c6deeae8E (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    call $_ZN4core3fmt9Formatter3pad17h339d16aa5bcc9066E)
  (func $_ZN9ontio_std7runtime5panic17h009a1911cc88a84bE (type 7) (param i32 i32)
    local.get 0
    local.get 1
    call $ontio_panic
    unreachable)
  (func $_ZN4core3ptr18real_drop_in_place17h0720dd6827beddabE (type 3) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h9f88e15dde29fa51E (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 0
    i32.load offset=8
    local.get 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h51c9a4b07e954c68E
    local.get 0
    local.get 0
    i32.load offset=8
    local.tee 3
    local.get 2
    i32.add
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    i32.const 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h51c9a4b07e954c68E (type 6) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 3
          local.get 1
          i32.sub
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.lt_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 1
          i32.shl
          local.tee 1
          local.get 2
          local.get 1
          local.get 2
          i32.gt_u
          select
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              local.get 1
              call $__rust_alloc
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            i32.load
            local.get 3
            local.get 1
            call $__rust_realloc
            local.set 2
          end
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store
        end
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h64ecb6f7cf049a7dE
      unreachable
    end
    local.get 1
    i32.const 1
    call $rust_oom
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h83357c1cb9797c8eE (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 224
              i32.or
              i32.store8 offset=12
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load offset=4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            i32.const 1
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h51c9a4b07e954c68E
            local.get 0
            i32.load offset=8
            local.set 3
          end
          local.get 0
          i32.load
          local.get 3
          i32.add
          local.get 1
          i32.store8
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 31
        i32.and
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      local.get 0
      local.get 0
      i32.load offset=8
      local.get 1
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h51c9a4b07e954c68E
      local.get 0
      local.get 0
      i32.load offset=8
      local.tee 3
      local.get 1
      i32.add
      i32.store offset=8
      local.get 3
      local.get 0
      i32.load
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h11e940a78a481157E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 33972
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hb9bf49115b8795e8E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $memset (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcpy (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcmp (type 0) (param i32 i32 i32) (result i32)
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
  (table (;0;) 24 24 anyfunc)
  (memory (;0;) 1 1000)
  (global (;0;) (mut i32) (i32.const 32768))
  (global (;1;) i32 (i32.const 34032))
  (global (;2;) i32 (i32.const 34032))
  (export "invoke" (func $invoke))
  (elem (;0;) (i32.const 1) $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hbe5ab8d7970b6d6bE $_ZN4core3fmt10ArgumentV110show_usize17hda1dd3e85f5b207cE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he9a1257f53511431E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdf23d6049b4697d9E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h3e4eb74648f24612E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1af775f68a8cc3d5E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h5e238c3a303a5a13E $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h26703904c6deeae8E $_ZN4core3ptr18real_drop_in_place17h0999c24ec24a2194E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h3e50052967fc4094E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h8e310dcc26e63065E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hfab4a9c8f73cb395E $_ZN4core3ptr18real_drop_in_place17heb402bda5ba4a707E $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hc08d2e58679b49f8E $_ZN4core3ptr18real_drop_in_place17h940a829d26b18402E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h80c760f782d10510E $_ZN4core3ptr18real_drop_in_place17h4912c9903427c192E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h2a9057d1634d3fa9E $_ZN4core3ptr18real_drop_in_place17h0383ec85acded427E $_ZN4core3ptr18real_drop_in_place17h0720dd6827beddabE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h9f88e15dde29fa51E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h83357c1cb9797c8eE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h11e940a78a481157E)
  (data (;0;) (i32.const 32768) "get_current_block_hashget_current_tx_hashget_timestampget_block_heightself_addresscaller_addressentry_addresscheck_witnesssha256test080af0b0156c5dd12c820b2b1b4fbfa315d05ac5a0ea2f9a657d4c8881d0869f\dc\80\00\00\11\00\00\00\cc\84\00\00\17\00\00\00\09\03\00\00\05\00\00\00capacity overflow\00\00\00`\81\00\00F\00\00\00c\01\00\00\13\00\00\00\09\00\00\00\04\00\00\00\04\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00a formatting trait implementation returned an error\00\0d\00\00\00\00\00\00\00\01\00\00\00\0e\00\00\00\00\00\00\00/rustc/e413dc36a83a5aad3ab6270373000693a917e92b/src/libcore/fmt/mod.rs\00\00\c8\81\00\00 \00\00\00\e8\81\00\00\12\00\00\00\0f\00\00\00\00\00\00\00\01\00\00\00\10\00\00\00index out of bounds: the len is  but the index is 00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00\e4\82\00\00\16\00\00\00\fa\82\00\00\0d\00\00\00t\84\00\00\18\00\00\00\1e\0a\00\00\05\00\00\00slice index starts at  but ends at 0x\00\00\00f\83\00\00+\00\00\00\91\83\00\00\15\00\00\00z\01\00\00\15\00\00\00P\83\00\00\16\00\00\00`\04\00\00\11\00\00\00P\83\00\00\16\00\00\00T\04\00\00(\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00src/libcore/fmt/mod.rscalled `Option::unwrap()` on a `None` valuesrc/libcore/option.rsBorrowMutError\94\84\00\00\00\00\00\00\c4\83\00\00\02\00\00\00: \00\00\d8\83\00\00\15\00\00\00\8d\04\00\00\05\00\00\00src/libcore/result.rsErroralready borrowed\00\00\11\00\00\00\00\00\00\00\01\00\00\00\12\00\00\00\01\00\00\00\00\00\00\00 \00\00\00\08\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\03\00\00\00called `Result::unwrap()` on an `Err` value\00\13\00\00\00\00\00\00\00\01\00\00\00\0e\00\00\00src/libcore/slice/mod.rs\94\84\00\00\00\00\00\00\94\84\00\00\00\00\00\00\ac\84\00\00\04\00\00\00\b0\84\00\00\01\00\00\00 at :\00\00\00\14\00\00\00\04\00\00\00\04\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00src/liballoc/raw_vec.rs")
  (data (;1;) (i32.const 34020) "\00\00\00\00\00\00\00\00\00\00\00\00"))
