use crate::processor::instruction::Instruction;

pub struct IFIDBuffer {
    instruction: Option<Instruction>,
    pc: u32
}

pub struct IDEXBuffer {}

pub struct EXMEMBuffer {}

pub struct MEMWBBuffer {}

impl IFIDBuffer {
    pub fn new(instruction: Option<Instruction>, pc: u32) -> Self {
        Self { instruction, pc }
    }
}
