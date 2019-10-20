pub mod executor;

pub mod disassm;

#[test]
fn test_add() {
    let wat = include_str!("../tests/add.wast");

    for i in 0..100 {
        let (a, b) = rand::random();
        let sum = executor::execute(wat, a, b, false);
        assert_eq!(sum, a.wrapping_add(b));
    }
}

#[test]
fn test_load_memory() {
    let wat = include_str!("../tests/memory-load.wast");

    let sum = executor::execute(wat, 0, 4, false);
    assert_eq!(sum, 1);
}

#[test]
fn test_sum() {
    let wat = include_str!("../tests/sum.wast");

    let sum = executor::execute(wat, 0, 100, false);
    assert_eq!(sum, 4950);
}
