use crate::chain_api::ChainCtx;
mod chain_api;
pub mod executor;
pub mod resolver;

pub mod disassm;

#[test]
fn test_add_one() {
    let wat = include_str!("../tests/add_one.wast");

    for _i in 0..100 {
        let a: i32 = rand::random();
        let sum: i32 = executor::execute(wat, "add_one", (a,), false);
        assert_eq!(sum, a.wrapping_add(1));
    }
}

#[test]
fn test_add() {
    let wat = include_str!("../tests/add.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum: i32 = executor::execute(wat, "add", (a, b), false);
        assert_eq!(sum, a.wrapping_add(b));
    }
}

#[test]
fn test_load_memory() {
    let wat = include_str!("../tests/memory-load.wast");

    let sum: i32 = executor::execute(wat, "load_add", (0, 4), false);
    assert_eq!(sum, 1);
}

#[test]
fn test_sum() {
    let wat = include_str!("../tests/sum.wast");
    let sum: i32 = executor::execute(wat, "sum", (0i32, 100i32), false);
    assert_eq!(sum, 4950);
}

#[test]
fn test_subtract() {
    let wat = include_str!("../tests/subtract.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sub: i32 = executor::execute(wat, "sub", (a, b), false);
        assert_eq!(sub, a.wrapping_sub(b));
    }
}

#[test]
fn test_load_subtract() {
    let wat = include_str!("../tests/load_sub.wast");
    let sub: i32 = executor::execute(wat, "sub", (4, 0), false);
    assert_eq!(sub, 1)
}

#[test]
fn test_multiply() {
    let wat = include_str!("../tests/multiply.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum: i32 = executor::execute(wat, "mul", (a, b), false);
        assert_eq!(sum, a.wrapping_mul(b));
    }
}

#[test]
fn test_load_multiply() {
    let wat = include_str!("../tests/load_mul.wast");
    let mul: i32 = executor::execute(wat, "mul", (4, 8), false);
    assert_eq!(mul, 2);
}

#[test]
fn test_div() {
    let wat = include_str!("../tests/div.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        if b == 0 {
            continue;
        }
        let sum: i32 = executor::execute(wat, "div", (a, b), false);
        assert_eq!(sum, a.wrapping_div(b));
    }
}

////#[test]
//fn test_fibonacci() {
//    fn fib(x: i32) -> i32 {
//        if x < 0 {
//            return 0;
//        } else if x == 1 || x == 2 {
//            return 1;
//        } else {
//            return fib(x - 1) + fib(x - 2);
//        }
//    }
//
//    let wat = include_str!("../tests/fibonacci.wast");
//    for i in 0..100 {
//        let a: i32 = rand::random();
//        let sum: i32 = executor::execute(wat, "fib", (a,), false);
//        assert_eq!(sum, fib(a));
//    }
//}

#[test]
fn test_global() {
    let wat = include_str!("../tests/global.wast");
    for i in 0..100 {
        let a: i32 = rand::random();
        let sum: i32 = executor::execute(wat, "get-global", (a,), false);
        assert_eq!(sum, a + 1);
    }
}

//#[test]
//fn test_chain() {
//    let wat = include_str!("../tests/runtime_test.wast");
//    let sum: u64 = executor::execute(wat, "get_block_hash", (), false);
//    assert_eq!(sum, 32);
//}
