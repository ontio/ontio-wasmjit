(module
  (type $t0 (func (result i64)))
  (import "env" "ontio_timestamp" (func $timestamp (type $t0)))
  (func $get_time (export "get_time") (type $t0) (result i64)
    call $timestamp)
)

