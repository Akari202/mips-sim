use crate::processor::alu::ALU;
use crate::processor::buffer::{EXMEMBuffer, IDEXBuffer, IFIDBuffer, MEMWBBuffer};
use crate::processor::instruction::Instruction;
use crate::processor::memory::{DataMemory, InstructionMemory};
use crate::processor::program_counter::ProgramCounter;
use crate::processor::registers::Registers;

mod registers;
mod instruction;
mod program_counter;
mod alu;
mod buffer;
mod memory;

pub struct Processor {
    program_counter: ProgramCounter,
    instruction_memory: InstructionMemory,
    if_id_buffer: IFIDBuffer,
    registers: Registers,
    id_ex_buffer: IDEXBuffer,
    alu: ALU,
    ex_mem_buffer: EXMEMBuffer,
    mem_wb_buffer: MEMWBBuffer,
    data_memory: DataMemory
}

impl Processor {
    pub fn new() -> Self {
        Self {
            program_counter: ProgramCounter::new(),
            instruction_memory: InstructionMemory::new(),
            if_id_buffer: IFIDBuffer::new(),
            registers: Registers::new(),
            id_ex_buffer: IDEXBuffer::new(),
            alu: ALU::new(),
            ex_mem_buffer: EXMEMBuffer::new(),
            mem_wb_buffer: MEMWBBuffer::new(),
            data_memory: DataMemory::new()
        }
    }

    pub fn cycle(&mut self) {
        let pc = self.program_counter.get();
        self.alu.execute(&self.id_ex_buffer, &mut self.ex_mem_buffer);
        self.registers.read_write(&self.if_id_buffer, &mut self.id_ex_buffer, &mut self.mem_wb_buffer);
        let instruction = self.instruction_memory.load(pc);
        let instruction = Instruction::load(instruction);
        self.if_id_buffer.set(instruction, pc);
        self.program_counter.increment();
    }
}

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Program Counter: {:#x}\n", self.program_counter.get())?;
        writeln!(f, "{}", self.registers)?;
        writeln!(f, "{}", self.if_id_buffer)?;
        writeln!(f, "{}", self.id_ex_buffer)?;
        writeln!(f, "{}", self.ex_mem_buffer)?;
        writeln!(f, "{}", self.mem_wb_buffer)?;
        Ok(())
    }
}
