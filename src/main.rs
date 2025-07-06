mod executor;
mod hardware;
mod parser;
mod stack;

use crate::executor::executor::Executor;

fn main() {
    let mut executor = Executor::new();

    let commands = vec![
        "push constant 7".into(),
    ];

    executor.set_stack(commands);
    executor.run();
    executor.run_print();

}
