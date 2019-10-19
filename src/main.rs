use ontio_wasmjit::executor;

const ADD: &str = include_str!("../tests/add.wast");

fn main() {
    for i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100;
        let sum = executor::execute(ADD, a, b, i == 0);
        println!("{:3} + {:3} = {:4}", a, b, sum);
    }
}
