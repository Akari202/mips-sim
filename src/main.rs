#![feature(non_null_convenience)]

use log::info;
use text_io::read;

mod processor;

fn main() {
    pretty_env_logger::init();

    let instructions = vec![
        // Prompt string
        0b01000101_01101110_01110100_01100101,
        0b01110010_00100000_01100001_00100000,
        0b01101110_01110101_01101101_01100010,
        0b01100101_01110010_00100000_01110100,
        0b01101111_00100000_01100100_01101111,
        0b01110101_01100010_01101100_01100101,
        0b00111010_00100000_00000000_00000000,
        // Set $v0 to 4
        0b001101_00000_00010_0000000000000100,
        // Set $a0 to 0x0 pointer to prompt string
        0b001101_00000_00100_0000000000000100,
        // syscall
        0b000000_00000_00000_00000_00000_001100
    ];

    print!("Enter cycle count: ");
    // let cycle_count: u32 = read!();
    let cycle_count: u32 = 1;
    let mut processor = processor::Processor::new();
    processor.load_program(instructions);
    processor.set_entry_point(0x1F);
    for i in 0..cycle_count {
        info!("Cycle {}", i);
        processor.cycle();
        println!("{}", processor);
    }
}
