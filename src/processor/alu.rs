use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::processor::buffer::{EXMEMBuffer, IDEXBuffer};
use crate::processor::instruction::InstructionType;

pub struct ALU {
    hi: u32,
    lo: u32
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum FunctionCode {
    Add = 0x20,
    Addu = 0x21,
    And = 0x24,
    Jr = 0x08,
    Nor = 0x27,
    Or = 0x25,
    Slt = 0x2A,
    Sltu = 0x2B,
    Sll = 0x00,
    Srl = 0x02,
    Sub = 0x22,
    Subu = 0x23,
    Div = 0x1A,
    Divu = 0x1B,
    Mfhi = 0x10,
    Mflo = 0x12,
    Mult = 0x18,
    Multu = 0x19,
    Sra = 0x03,
    Syscall = 0x0C
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum OpCode {
    Addi = 0x08,
    Addiu = 0x09,
    Andi = 0x0C,
    Beq = 0x04,
    Bne = 0x05,
    Lbu = 0x24,
    Lhu = 0x25,
    Ll = 0x30,
    Lui = 0x0F,
    Lw = 0x23,
    Ori = 0x0D,
    Slti = 0x0A,
    Sltiu = 0x0B,
    Sb = 0x28,
    Sc = 0x38,
    Sh = 0x29,
    Sw = 0x2B
}

impl ALU {
    pub fn new() -> Self {
        Self {
            hi: 0,
            lo: 0
        }
    }

    pub fn execute(&mut self, idex: &IDEXBuffer, exmem: &mut EXMEMBuffer) {
        let instruction = idex.instruction;
        exmem.pc = idex.pc;
        exmem.instruction = instruction;
        exmem.alu_result = 0;
        if instruction.is_none() {
            return;
        }
        let instruction = instruction.unwrap();
        match instruction.instruction_type {
            InstructionType::R => {
                match FunctionCode::from_u8(instruction.funct.unwrap()).unwrap() {
                    FunctionCode::Add => {
                        exmem.alu_result = (idex.data_1 as i32).wrapping_add(idex.data_2 as i32) as u32;
                    },
                    FunctionCode::Addu => {
                        exmem.alu_result = idex.data_1.wrapping_add(idex.data_2);
                    },
                    FunctionCode::And => {
                        exmem.alu_result = idex.data_1 & idex.data_2;
                    },
                    FunctionCode::Jr => {
                        exmem.pc = idex.data_1;
                    },
                    FunctionCode::Nor => {
                        exmem.alu_result = !(idex.data_1 | idex.data_2);
                    },
                    FunctionCode::Or => {
                        exmem.alu_result = idex.data_1 | idex.data_2;
                    },
                    FunctionCode::Slt => {
                        exmem.alu_result = if (idex.data_1 as i32) < (idex.data_2 as i32) { 1 } else { 0 };
                    },
                    FunctionCode::Sltu => {
                        exmem.alu_result = if idex.data_1 < idex.data_2 { 1 } else { 0 };
                    },
                    FunctionCode::Sll => {
                        exmem.alu_result = idex.data_2 << instruction.shamt.unwrap();
                    },
                    FunctionCode::Srl => {
                        exmem.alu_result = idex.data_2 >> instruction.shamt.unwrap();
                    },
                    FunctionCode::Sub => {
                        exmem.alu_result = (idex.data_1 as i32).wrapping_sub(idex.data_2 as i32) as u32;
                    },
                    FunctionCode::Subu => {
                        exmem.alu_result = idex.data_1.wrapping_sub(idex.data_2);
                    },
                    FunctionCode::Div => {
                        self.lo = (idex.data_1 as i32 / idex.data_2 as i32) as u32;
                        self.hi = (idex.data_1 as i32 % idex.data_2 as i32) as u32;
                    },
                    FunctionCode::Divu => {
                        self.lo = idex.data_1 / idex.data_2;
                        self.hi = idex.data_1 % idex.data_2;
                    },
                    FunctionCode::Mfhi => {
                        exmem.alu_result = self.hi;
                    },
                    FunctionCode::Mflo => {
                        exmem.alu_result = self.lo;
                    },
                    FunctionCode::Mult => {
                        let result = (idex.data_1 as i64) * (idex.data_2 as i64);
                        self.lo = result as u32;
                        self.hi = (result >> 32) as u32;
                    },
                    FunctionCode::Multu => {
                        let result = (idex.data_1 as u64) * (idex.data_2 as u64);
                        self.lo = result as u32;
                        self.hi = (result >> 32) as u32;
                    },
                    FunctionCode::Sra => {
                        exmem.alu_result = ((idex.data_2 as i32) >> instruction.shamt.unwrap()) as u32;
                    },
                    FunctionCode::Syscall => {
                        // TODO: Implement syscall
                    }
                }
            },
            InstructionType::I => {
                match OpCode::from_u8(instruction.opcode).unwrap() {
                    OpCode::Addi => {
                        exmem.alu_result = (idex.data_1 as i32).wrapping_add(idex.sign_extended as i32) as u32;
                    },
                    OpCode::Addiu => {
                        exmem.alu_result = idex.data_1.wrapping_add(idex.sign_extended);
                    },
                    OpCode::Andi => {
                        exmem.alu_result = idex.data_1 & idex.sign_extended;
                    },
                    OpCode::Beq => {
                        if idex.data_1 == idex.data_2 {
                            exmem.pc = idex.pc.wrapping_add(4).wrapping_add(idex.sign_extended);
                        }
                    },
                    OpCode::Bne => {
                        if idex.data_1 != idex.data_2 {
                            exmem.pc = idex.pc.wrapping_add(4).wrapping_add(idex.sign_extended);
                        }
                    },
                    // TODO: Implement remaining I-type instructions
                    OpCode::Lbu => {},
                    OpCode::Lhu => {},
                    OpCode::Ll => {},
                    OpCode::Lui => {},
                    OpCode::Lw => {},
                    OpCode::Ori => {},
                    OpCode::Slti => {},
                    OpCode::Sltiu => {},
                    OpCode::Sb => {},
                    OpCode::Sc => {},
                    OpCode::Sh => {},
                    OpCode::Sw => {}
                }
            },
            InstructionType::J => {

            },
            _ => {}
        }
    }
}

