pub struct Instruction {
    opcode: u8,
    rs: Option<u8>,
    rt: Option<u8>,
    rd: Option<u8>,
    shamt: Option<u8>,
    funct: Option<u8>,
    imm: Option<u16>,
    addr: Option<u32>
}

enum InstructionType {
    R,
    I,
    J
}

impl Instruction {
    pub fn get_type(&self) -> InstructionType {
        if self.opcode == 0 {
            InstructionType::R
        } else if self.opcode == 2 || self.opcode == 3 {
            InstructionType::J
        } else {
            InstructionType::I
        }
    }
}
