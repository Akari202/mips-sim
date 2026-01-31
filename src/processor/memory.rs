use std::fmt::{Display, Formatter};
use std::ptr::NonNull;
use log::{debug, info, trace};
use crate::processor::buffer::{EXMEMBuffer, MEMWBBuffer};

#[derive(Debug)]
pub struct Memory {
    pointer: NonNull<u8>,
    capacity: u32,
    stack_pointer: u32,
    alloc: u32
}

pub enum Size {
    Byte,
    Halfword,
    Word,
    Quad
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            pointer: NonNull::dangling(),
            capacity: 0,
            stack_pointer: 0,
            alloc: 0
        }
    }

    pub fn new_with_capacity(word_capacity: u32) -> Self {
        let layout = std::alloc::Layout::array::<u8>((word_capacity << 2) as usize).unwrap();
        let pointer = unsafe { std::alloc::alloc(layout) };
        if pointer.is_null() {
            panic!("Failed to allocate memory");
        }
        info!(
            "Allocated {} bytes at address {:#x}",
            word_capacity << 2,
            pointer as u32
        );
        Memory {
            pointer: NonNull::new(pointer).unwrap(),
            capacity: word_capacity * 4,
            stack_pointer: 0,
            alloc: 0
        }
    }

    pub fn get_stack_pointer(&self) -> u32 {
        self.stack_pointer
    }

    pub fn load_program(&mut self, program: Vec<u32>) {
        let mut index = self.alloc;
        for instruction in program {
            self.write_word(index, instruction);
            index += 4;
        }
        self.stack_pointer = index;
        self.alloc = self.alloc.max(index);
    }

    pub fn read<T>(&self, address: u32, size: Size) -> T
    where
        T: From<u8> + From<u16> + From<u32> + From<u64>
    {
        match size {
            Size::Byte => self.read_byte(address).into(),
            Size::Halfword => self.read_halfword(address).into(),
            Size::Word => self.read_word(address).into(),
            Size::Quad => self.read_quad(address).into()
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        trace!("Reading byte from address {:#x}", address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.read() }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        debug!("Writing byte {:#x} to address {:#x}", value, address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe {
            pointer.write(value);
        }
        self.alloc = self.alloc.max(address + 1);
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        trace!("Reading halfword from address {:#x}", address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u16>().read() }
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        debug!("Writing halfword {:#x} to address {:#x}", value, address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe {
            pointer.cast::<u16>().write(value);
        }
        self.alloc = self.alloc.max(address + 2);
    }

    pub fn read_word(&self, address: u32) -> u32 {
        trace!("Reading word from address {:#x}", address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u32>().read() }
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        debug!("Writing word {:#x} to address {:#x}", value, address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe {
            pointer.cast::<u32>().write(value);
        }
        self.alloc = self.alloc.max(address + 4);
    }

    pub fn read_quad(&self, address: u32) -> u64 {
        trace!("Reading quad from address {:#x}", address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u64>().read() }
    }

    pub fn write_quad(&mut self, address: u32, value: u64) {
        debug!("Writing quad {:#x} to address {:#x}", value, address);
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe {
            pointer.cast::<u64>().write(value);
        }
        self.alloc = self.alloc.max(address + 8);
    }

    pub fn read_cstring(&self, address: u32) -> String {
        trace!("Reading cstring from address {:#x}", address);
        self.check_address(address);
        let mut result = String::new();
        let mut index = address;
        loop {
            let byte: u8 = self.read_byte(index);
            if byte == "\0".as_bytes()[0] {
                break;
            }
            result.push(byte as char);
            index += 1;
        }
        result
    }

    pub fn write_cstring(&mut self, address: u32, value: &str) {
        debug!("Writing cstring {:?} to address {:#x}", value, address);
        self.check_address(address);
        let bytes = value.as_bytes();
        let mut index = address;
        for byte in bytes {
            self.write_byte(index, *byte);
            index += 1;
        }
        self.write_byte(index, "\0".as_bytes()[0]);
        self.alloc = self.alloc.max(index + 1);
    }

    fn check_address(&self, address: u32) {
        if address > self.capacity || self.capacity == 0 {
            panic!("Address {} out of range", address);
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Memory: {} bytes", self.capacity)?;
        writeln!(f, "Stack Pointer: {:#x}", self.stack_pointer)?;
        writeln!(f, "Allocated: {:#x}", self.alloc)?;
        for i in 0..(self.alloc / 4) {
            let index = i * 4;
            let byte = self.read_word(index);
            write!(f, "    {:#04x}: {:#010x}    |    ", index, byte)?;
            for j in 0..4 {
                let byte = self.read_byte(index + j);
                write!(f, "{:#04x} ", byte)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub mod DataMemory {
    use log::{debug, info, trace};
    use num_traits::FromPrimitive;
    use crate::processor::alu::OpCode;
    use crate::processor::buffer::{EXMEMBuffer, MEMWBBuffer};
    use crate::processor::memory::Memory;

    pub fn execute(exmem: &EXMEMBuffer, memwb: &mut MEMWBBuffer, memory: &mut Memory) {
        info!("Executing memory stage");
        let instruction = exmem.instruction;
        if instruction.is_none() {
            return;
        }
        let instruction = instruction.unwrap();
        let opcode = OpCode::from_u8(instruction.opcode).unwrap();
        match opcode {
            OpCode::Lbu => {
                let address = exmem.alu_result;
                let value = memory.read_byte(address);
                memwb.data = value as u32;
            }
            OpCode::Lhu => {
                let address = exmem.alu_result;
                let value = memory.read_halfword(address);
                memwb.data = value as u32;
            }
            OpCode::Lw => {
                let address = exmem.alu_result;
                let value = memory.read_word(address);
                memwb.data = value;
            }
            OpCode::Sb => {
                let address = exmem.alu_result;
                let value = exmem.data_2 as u8;
                memory.write_byte(address, value);
            }
            OpCode::Sh => {
                let address = exmem.alu_result;
                let value = exmem.data_2 as u16;
                memory.write_halfword(address, value);
            }
            OpCode::Sw => {
                let address = exmem.alu_result;
                let value = exmem.data_2;
                memory.write_word(address, value);
            }
            _ => {}
        }
    }
}
