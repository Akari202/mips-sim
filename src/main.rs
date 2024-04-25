#![feature(non_null_convenience)]

mod processor;

fn main() {
    pretty_env_logger::init();
    let mut processor = processor::Processor::new();
    processor.cycle();
    println!("{}", processor);
    processor.cycle();
    println!("{}", processor);
}
