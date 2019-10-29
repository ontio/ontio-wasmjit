(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $sum (export "sum") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
	(local $l0 i32)
	block $B0
	get_local $p1
	i32.eqz
	br_if $B0
	get_local $p1
	i32.const 2
	i32.shl
	set_local $l0
	i32.const 0
	set_local $p1
	loop $L1
	get_local $p0
	i32.load
	get_local $p1
	i32.add
	set_local $p1
	get_local $p0
	i32.const 4
	i32.add
	set_local $p0
	get_local $l0
	i32.const -4
	i32.add
	tee_local $l0
	br_if $L1
	end
	get_local $p1
	return
	end
	i32.const 0)
    (memory $memory 17))


