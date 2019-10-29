pub mod executor;

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
