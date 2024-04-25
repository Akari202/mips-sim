use crate::processor::instruction::Instruction;

#[derive(Copy, Clone)]
pub struct IFIDBuffer {
    pub instruction: Option<Instruction>,
    pub pc: u32
}

pub struct IDEXBuffer {
    pub instruction: Option<Instruction>,
    pub data_1: u32,
    pub data_2: u32,
    pub sign_extended: u32,
    pub pc: u32
}

pub struct EXMEMBuffer {
    pub instruction: Option<Instruction>,
    pub alu_result: u32,
    pub pc: u32
}

pub struct MEMWBBuffer {}

impl IFIDBuffer {
    pub fn new() -> Self {
        Self {
            instruction: None,
            pc: 0
        }
    }

    pub fn set(&mut self, instruction: Instruction, pc: u32) {
        self.instruction = Some(instruction);
        self.pc = pc;
    }
}

impl IDEXBuffer {
    pub fn new() -> Self {
        Self {
            instruction: None,
            data_1: 0,
            data_2: 0,
            sign_extended: 0,
            pc: 0
        }
    }
}

impl EXMEMBuffer {
    pub fn new() -> Self {
        Self {
            instruction: None,
            alu_result: 0,
            pc: 0
        }
    }
}

impl MEMWBBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::fmt::Display for IFIDBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "IF/ID Buffer:")?;
        match &self.instruction {
            Some(instruction) => write!(f, "    Instruction:\n{}", instruction)?,
            None => writeln!(f, "    No instruction")?
        }
        writeln!(f, "    PC: {:#x}", self.pc)?;
        Ok(())
    }
}

impl std::fmt::Display for IDEXBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "ID/EX Buffer:")?;
        match &self.instruction {
            Some(instruction) => write!(f, "    Instruction:\n{}", instruction)?,
            None => writeln!(f, "    No instruction")?
        }
        writeln!(f, "    Data 1: {:#x}", self.data_1)?;
        writeln!(f, "    Data 2: {:#x}", self.data_2)?;
        writeln!(f, "    Sign Extended: {:#x}", self.sign_extended)?;
        writeln!(f, "    PC: {:#x}", self.pc)?;
        Ok(())
    }
}

impl std::fmt::Display for EXMEMBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "EX/MEM Buffer:")?;
        match &self.instruction {
            Some(instruction) => write!(f, "    Instruction:\n{}", instruction)?,
            None => writeln!(f, "    No instruction")?
        }
        writeln!(f, "    ALU Result: {:#x}", self.alu_result)?;
        writeln!(f, "    PC: {:#x}", self.pc)?;
        Ok(())
    }
}

impl std::fmt::Display for MEMWBBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "MEM/WB Buffer")?;
        Ok(())
    }
}
