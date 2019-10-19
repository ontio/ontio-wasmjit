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
