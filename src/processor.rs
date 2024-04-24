use crate::processor::buffer::IFIDBuffer;
use crate::processor::program_counter::ProgramCounter;
use crate::processor::registers::Registers;

mod registers;
mod instruction;
mod program_counter;
mod alu;
mod buffer;

pub struct Processor {
    registers: Registers,
    program_counter: ProgramCounter,
    if_id_buffer: IFIDBuffer
}

impl Processor {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            program_counter: ProgramCounter::new(),
            if_id_buffer: IFIDBuffer::new(None, 0)
        }
    }

    pub fn cycle(&mut self) {
        let pc = self.program_counter.get();
        self.program_counter.increment();
    }
}
