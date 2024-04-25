use log::info;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::processor::alu::FunctionCode;
use crate::processor::memory::InstructionMemory;
use crate::processor::registers::Register;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub instruction_type: InstructionType,
    pub rs: Option<u8>,
    pub rt: Option<u8>,
    pub rd: Option<u8>,
    pub shamt: Option<u8>,
    pub funct: Option<u8>,
    pub imm: Option<u32>,
    pub addr: Option<u32>,
    // fmt: Option<u8>,
    // ft: Option<u8>,
    // fs: Option<u8>,
    // fd: Option<u8>
}

#[derive(Clone, Copy)]
pub enum InstructionType {
    R,
    I,
    J,
    Fr,
    Fi
}

impl Instruction {
    pub fn load(data: u32) -> Self {
        info!("Loading instruction: {:#b}", data);
        let opcode: u8 = (data >> 26) as u8;
        match opcode {
            0 => {
                Self {
                    opcode,
                    instruction_type: InstructionType::R,
                    rs: Some(((data >> 21) & 0x1F) as u8),
                    rt: Some(((data >> 16) & 0x1F) as u8),
                    rd: Some(((data >> 11) & 0x1F) as u8),
                    shamt: Some(((data >> 6) & 0x1F) as u8),
                    funct: Some(((data >> 0) & 0x3F) as u8),
                    imm: None,
                    addr: None
                    // fmt: None,
                    // ft: None,
                    // fs: None,
                    // fd: None
                }
            },
            1 | 2 => {
                Self {
                    opcode,
                    instruction_type: InstructionType::J,
                    rs: None,
                    rt: None,
                    rd: None,
                    shamt: None,
                    funct: None,
                    imm: None,
                    addr: Some(((data >> 0) & 0x3FFFFFF) as u32)
                    // fmt: None,
                    // ft: None,
                    // fs: None,
                    // fd: None
                }
            },
            // 11 => {
            //     let fmt: u8 = ((data >> 21) & 0x1F) as u8;
            //     match fmt {
            //         0x8 => {
            //             Self {
            //                 opcode,
            //                 instruction_type: InstructionType::Fi,
            //                 rs: None,
            //                 rt: None,
            //                 rd: None,
            //                 shamt: None,
            //                 funct: None,
            //                 imm: Some(((data >> 0) & 0xFFFF) as u32),
            //                 addr: None,
            //                 fmt: Some(fmt),
            //                 ft: Some(((data >> 16) & 0x1F) as u8),
            //                 fs: Some(((data >> 11) & 0x1F) as u8),
            //                 fd: Some(((data >> 6) & 0x1F) as u8)
            //             }
            //         },
            //         0x10 | 0x11 => {
            //             Self {
            //                 opcode,
            //                 instruction_type: InstructionType::Fr,
            //                 rs: None,
            //                 rt: None,
            //                 rd: None,
            //                 shamt: None,
            //                 funct: Some(((data >> 0) & 0x3F) as u8),
            //                 imm: None,
            //                 addr: None,
            //                 fmt: Some(fmt),
            //                 ft: Some(((data >> 16) & 0x1F) as u8),
            //                 fs: Some(((data >> 11) & 0x1F) as u8),
            //                 fd: Some(((data >> 6) & 0x1F) as u8)
            //             }
            //         },
            //         _ => {
            //             panic!("Invalid instruction")
            //         }
            //     }
            // }
            _ => {
                Self {
                    opcode,
                    instruction_type: InstructionType::I,
                    rs: Some(((data >> 21) & 0x1F) as u8),
                    rt: Some(((data >> 16) & 0x1F) as u8),
                    rd: None,
                    shamt: None,
                    funct: None,
                    // NOTE: This is a sign-extended immediate value
                    imm: Some(((data >> 0) & 0xFFFF) as i32 as u32),
                    addr: None
                    // fmt: None,
                    // ft: None,
                    // fs: None,
                    // fd: None
                }
            }
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.instruction_type {
            InstructionType::R => {
                writeln!(f, "        R-Type:")?;
                writeln!(f, "            Opcode: {:#x}", self.opcode)?;
                writeln!(f, "            RS: {:?}", Register::from_u8(self.rs.unwrap()).unwrap())?;
                writeln!(f, "            RT: {:?}", Register::from_u8(self.rt.unwrap()).unwrap())?;
                writeln!(f, "            RD: {:?}", Register::from_u8(self.rd.unwrap()).unwrap())?;
                writeln!(f, "            Shift Ammount: {:#x}", self.shamt.unwrap())?;
                writeln!(f, "            Function Code: {:?}", FunctionCode::from_u8(self.funct.unwrap()).unwrap())?;
            },
            InstructionType::I => {
                writeln!(f, "        I-Type:")?;
                writeln!(f, "            Opcode: {:#x}", self.opcode)?;
                writeln!(f, "            RS: {:?}", Register::from_u8(self.rs.unwrap()).unwrap())?;
                writeln!(f, "            RT: {:?}", Register::from_u8(self.rt.unwrap()).unwrap())?;
                writeln!(f, "            Immediate: {:#x}", self.imm.unwrap())?;
            },
            InstructionType::J => {
                writeln!(f, "        J-Type:")?;
                writeln!(f, "            Opcode: {:#x}", self.opcode)?;
                writeln!(f, "            Address: {:#x}", self.addr.unwrap())?;
            },
            InstructionType::Fr => {
                writeln!(f, "Fr-Type")?;
            },
            InstructionType::Fi => {
                writeln!(f, "Fi-Type")?;
            }
        }
        Ok(())
    }
}
