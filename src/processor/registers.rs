use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::processor::buffer::{IDEXBuffer, IFIDBuffer, MEMWBBuffer};
use crate::processor::instruction::InstructionType;

pub struct Registers {
    r: [u32; 32]
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
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

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 32]
        }
    }

    pub fn get(&self, reg: Register) -> u32 {
        self.r[reg as usize]
    }

    pub fn set(&mut self, reg: Register, value: u32) {
        if (reg as usize) == 0 {
            return;
        }
        self.r[reg as usize] = value;
    }

    pub fn read_write(&mut self, ifid: &IFIDBuffer, idex: &mut IDEXBuffer, memwb: &mut MEMWBBuffer) {
        let instruction = ifid.instruction;
        idex.instruction = instruction;
        idex.pc = ifid.pc;
        if instruction.is_none() {
            idex.data_1 = 0;
            idex.data_2 = 0;
            idex.sign_extended = 0;
            return;
        }
        let instruction = instruction.unwrap();
        match instruction.instruction_type {
            InstructionType::R => {
                idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                idex.data_2 = self.get(Register::from_u8(instruction.rt.unwrap()).unwrap());
            },
            InstructionType::I => {
                if instruction.opcode == 0x04 || instruction.opcode == 0x05 {
                    idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                    idex.data_2 = self.get(Register::from_u8(instruction.rt.unwrap()).unwrap());
                } else {
                    idex.data_1 = self.get(Register::from_u8(instruction.rs.unwrap()).unwrap());
                    idex.data_2 = instruction.imm.unwrap();
                }
            },
            InstructionType::J => {
                idex.data_1 = 0;
                idex.data_2 = 0;
            },
            _ => {}
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
