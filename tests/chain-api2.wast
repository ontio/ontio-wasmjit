(module
  (type $t1 (func (param i32) (result i32)))
  (import "env" "ontio_current_blockhash" (func $current_blockhash (type $t1)))
  (func $get_block_hash (export "get_block_hash")  (type $t1) (param $p0 i32) (result i32)
   get_local $p0
   call $current_blockhash)
   (memory $memory 17))
)

