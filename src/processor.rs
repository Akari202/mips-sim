use text_io::read;
use crate::processor;
use crate::processor::alu::ALU;
use crate::processor::buffer::{EXMEMBuffer, IDEXBuffer, IFIDBuffer, MEMWBBuffer};
use crate::processor::instruction::Instruction;
use crate::processor::memory::{DataMemory, InstructionMemory, Memory};
use crate::processor::program_counter::ProgramCounter;
use crate::processor::registers::{DecodeReturn, Register, Registers};

mod registers;
mod instruction;
mod program_counter;
mod alu;
mod buffer;
mod memory;

const MEMORY_SIZE: u32 = 1024;

pub struct Processor {
    program_counter: ProgramCounter,
    instruction_memory: InstructionMemory,
    if_id_buffer: IFIDBuffer,
    registers: Registers,
    id_ex_buffer: IDEXBuffer,
    alu: ALU,
    ex_mem_buffer: EXMEMBuffer,
    mem_wb_buffer: MEMWBBuffer,
    data_memory: Memory
}

impl Processor {
    pub fn new() -> Self {
        let mut processor = Processor {
            program_counter: ProgramCounter::new(),
            instruction_memory: InstructionMemory::new(),
            if_id_buffer: IFIDBuffer::new(),
            registers: Registers::new(),
            id_ex_buffer: IDEXBuffer::new(),
            alu: ALU::new(),
            ex_mem_buffer: EXMEMBuffer::new(),
            mem_wb_buffer: MEMWBBuffer::new(),
            data_memory: Memory::new_with_capacity(MEMORY_SIZE)
        };
        processor.registers.set(Register::Sp, processor.data_memory.get_stack_pointer());
        processor
    }

    pub fn cycle(&mut self) {
        self.alu.execute(&self.id_ex_buffer, &mut self.ex_mem_buffer);
        let decode = self.registers.read_write(&self.if_id_buffer, &mut self.id_ex_buffer, &mut self.mem_wb_buffer);
        match decode {
            DecodeReturn::Jump(address) => { self.program_counter.set(address) },
            DecodeReturn::Syscall => { self.syscall() },
            DecodeReturn::None => {}
        }
        let instruction = self.instruction_memory.load(self.program_counter.get());
        self.program_counter.increment();
        let instruction = Instruction::load(instruction);
        self.if_id_buffer.set(instruction, self.program_counter.get());
    }

    fn syscall(&mut self) {
        let syscall_code = self.registers.get(Register::V0);
        match syscall_code {
            1 => { println!("{}", self.data_memory.read_word(self.registers.get(Register::A0))) },
            4 => { println!("{}", self.data_memory.read_cstring(self.registers.get(Register::A0))) },
            5 => {
                let num: i32 = read!();
                self.registers.set(Register::V0, num as u32);
            }
            _ => {}
        }
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
