use ontio_wasmjit::executor;

const ADD: &str = include_str!("../tests/add.wast");

fn main() {
    env_logger::init();
    for i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100 + 1;
        let sum: i32 = executor::execute(ADD, "add", (a, b), i == 0);
        println!("{:3} / {:3} = {:4}", a, b, sum);
    }

    let wat = include_str!("../tests/chain-api.wast");

    let time: u64 = executor::execute(wat, "get_time", (), true);
    println!("timestamp: {}", time);

    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        let time: u32 = executor::execute(wat, "br_table", (i, 3), i == 0);
        println!("br_table: {}", time);
    }
}
