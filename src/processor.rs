use std::cmp::PartialEq;
use log::{debug, trace};
use text_io::read;
use crate::processor::alu::ALU;
use crate::processor::buffer::{EXMEMBuffer, IDEXBuffer, IFIDBuffer, MEMWBBuffer};
use crate::processor::instruction::Instruction;
use crate::processor::memory::{DataMemory, Memory};
use crate::processor::program_counter::ProgramCounter;
use crate::processor::registers::{DecodeReturn, Register, Registers};

pub mod alu;
pub mod buffer;
pub mod instruction;
pub mod memory;
pub mod program_counter;
pub mod registers;

const MEMORY_SIZE: u32 = 1024;

pub struct Processor {
    program_counter: ProgramCounter,
    memory: Memory,
    if_id_buffer: IFIDBuffer,
    registers: Registers,
    id_ex_buffer: IDEXBuffer,
    alu: ALU,
    ex_mem_buffer: EXMEMBuffer,
    mem_wb_buffer: MEMWBBuffer,
    stall: bool
}

impl Processor {
    pub fn new() -> Self {
        let mut processor = Processor {
            program_counter: ProgramCounter::new(),
            memory: Memory::new_with_capacity(MEMORY_SIZE),
            if_id_buffer: IFIDBuffer::new(),
            registers: Registers::new(),
            id_ex_buffer: IDEXBuffer::new(),
            alu: ALU::new(),
            ex_mem_buffer: EXMEMBuffer::new(),
            mem_wb_buffer: MEMWBBuffer::new(),
            stall: false
        };
        processor
            .registers
            .set(Register::Sp, processor.memory.get_stack_pointer());
        processor
    }

    pub fn load_program(&mut self, program: Vec<u32>) {
        self.memory.load_program(program);
        self.registers
            .set(Register::Sp, self.memory.get_stack_pointer());
    }

    pub fn set_entry_point(&mut self, address: u32) {
        self.program_counter.set(address);
    }

    pub fn cycle(&mut self) {
        trace!("Cycle start");
        DataMemory::execute(&self.ex_mem_buffer, &mut self.mem_wb_buffer, &mut self.memory);
        self.alu
            .execute(&self.id_ex_buffer, &mut self.ex_mem_buffer);
        let decode = self.registers.execute(
            &self.if_id_buffer,
            &mut self.id_ex_buffer,
            &mut self.mem_wb_buffer
        );
        match decode {
            DecodeReturn::Jump(address) => { self.program_counter.set(address) }
            DecodeReturn::Syscall => { self.syscall() }
            DecodeReturn::Stall => { self.stall = true }
            DecodeReturn::None => {}
        }
        let instruction;
        if self.stall {
            if decode == DecodeReturn::None {
                self.stall = false;
            }
            instruction = None;
        } else {
            instruction = Some(Instruction::load(self.memory.read_word(self.program_counter.get())));
            self.program_counter.increment();
        }
        self.if_id_buffer.instruction = instruction;
        self.if_id_buffer.pc = self.program_counter.get();
    }

    fn syscall(&mut self) {
        let syscall_code = self.registers.get(Register::V0);
        debug!("Syscall code: {:#x}", syscall_code);
        match syscall_code {
            1 => {
                println!(
                    "{}",
                    self.memory.read_word(self.registers.get(Register::A0))
                )
            }
            4 => {
                println!(
                    "{}",
                    self.memory.read_cstring(self.registers.get(Register::A0))
                )
            }
            5 => {
                let num: i32 = read!();
                self.registers.set(Register::V0, num as u32);
            }
            10 => std::process::exit(0),
            _ => {}
        }
    }
}

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Program Counter: {:#x}\n", self.program_counter.get())?;
        writeln!(f, "{}", self.memory)?;
        writeln!(f, "{}", self.registers)?;
        writeln!(f, "{}", self.if_id_buffer)?;
        writeln!(f, "{}", self.id_ex_buffer)?;
        writeln!(f, "{}", self.ex_mem_buffer)?;
        writeln!(f, "{}", self.mem_wb_buffer)?;
        Ok(())
    }
}
