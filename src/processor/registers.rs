use log::trace;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::processor::alu::{FunctionCode, OpCode};
use crate::processor::buffer::{IDEXBuffer, IFIDBuffer, MEMWBBuffer};
use crate::processor::instruction::InstructionType;

pub struct Registers {
    r: [u32; 32]
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialOrd, PartialEq)]
pub enum Register {
    /// Hard-wired to 0 all writes are ignored
    Zero = 0,
    /// Assembler temporary
    At = 1,
    /// Return value
    V0 = 2,
    /// Return value
    V1 = 3,
    /// Function argument
    A0 = 4,
    /// Function argument
    A1 = 5,
    /// Function argument
    A2 = 6,
    /// Function argument
    A3 = 7,
    /// Temporary
    T0 = 8,
    /// Temporary
    T1 = 9,
    /// Temporary
    T2 = 10,
    /// Temporary
    T3 = 11,
    /// Temporary
    T4 = 12,
    /// Temporary
    T5 = 13,
    /// Temporary
    T6 = 14,
    /// Temporary
    T7 = 15,
    /// Saved temporary
    S0 = 16,
    /// Saved temporary
    S1 = 17,
    /// Saved temporary
    S2 = 18,
    /// Saved temporary
    S3 = 19,
    /// Saved temporary
    S4 = 20,
    /// Saved temporary
    S5 = 21,
    /// Saved temporary
    S6 = 22,
    /// Saved temporary
    S7 = 23,
    /// Temporary
    T8 = 24,
    /// Temporary
    T9 = 25,
    /// Reserved for OS kernel
    K0 = 26,
    /// Reserved for OS kernel
    K1 = 27,
    /// Global Pointer
    Gp = 28,
    /// Stack Pointer
    Sp = 29,
    /// Frame Pointer
    Fp = 30,
    /// Return Address
    Ra = 31
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum DecodeReturn {
    Jump(u32),
    Syscall,
    None,
    Stall
}

impl Registers {
    pub fn new() -> Self {
        Self { r: [0; 32] }
    }

    pub fn get(&self, reg: Register) -> u32 {
        trace!("Reading register: {:?}", reg);
        self.r[reg as usize]
    }

    pub fn set(&mut self, reg: Register, value: u32) {
        trace!("Writing register: {:?} with value {:#x}", reg, value);
        if (reg as usize) == 0 {
            return;
        }
        self.r[reg as usize] = value;
    }

    pub fn execute(
        &mut self,
        ifid: &IFIDBuffer,
        idex: &mut IDEXBuffer,
        memwb: &mut MEMWBBuffer
    ) -> DecodeReturn {
        trace!("Executing register stage");
        trace!("Executing write back");
        let instruction = memwb.instruction;
        if instruction.is_some() {
            let instruction = instruction.unwrap();
            let rd = Register::from_u8(instruction.rd.unwrap()).unwrap();
            self.set(rd, memwb.data);
        }

        trace!("Executing decode");
        let instruction = ifid.instruction;
        idex.instruction = instruction;
        idex.pc = ifid.pc;
        idex.data_1 = 0;
        idex.data_2 = 0;
        idex.sign_extended = 0;
        if instruction.is_none() {
            return DecodeReturn::None;
        }
        let instruction = instruction.unwrap();
        println!("Instruction: {}", instruction);
        match instruction.instruction_type {
            InstructionType::R => {
                let funct = FunctionCode::from_u8(instruction.funct.unwrap()).unwrap();
                match funct {
                    FunctionCode::Syscall => DecodeReturn::Syscall,
                    FunctionCode::Jr => {
                        // NOTE: I think there should be a stall here
                        idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                        DecodeReturn::Jump(idex.data_1)
                    },
                    _ => {
                        idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                        idex.data_2 = self.get(Register::from_u8(instruction.rt.unwrap()).unwrap());
                        DecodeReturn::None
                    }
                }
            }
            // Load instructions might cause a hazard?
            InstructionType::I => {
                let opcode = OpCode::from_u8(instruction.opcode).unwrap();
                idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                idex.data_2 = self.get(Register::from_u8(instruction.rt.unwrap()).unwrap());
                idex.sign_extended = instruction.imm.unwrap();
                match opcode {
                    OpCode::Beq | OpCode::Bne => {
                        DecodeReturn::Stall
                    }
                    _ => {
                        DecodeReturn::None
                    }
                }
            }
            InstructionType::J => {
                if instruction.opcode == 0x3 {
                    // NOTE: it is unclear if the pc should be set to pc + 8 or + 4
                    self.set(Register::Ra, ifid.pc + 4);
                }
                DecodeReturn::Jump(ifid.pc + instruction.addr.unwrap() << 2)
            }
            _ => DecodeReturn::None
        }
    }
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Registers:")?;
        for (i, j) in self.r.iter().enumerate() {
            writeln!(f, "    {:?}: {:#x}", Register::from_usize(i).unwrap(), j)?;
        }
        Ok(())
    }
}
