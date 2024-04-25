pub struct InstructionMemory {}

pub struct DataMemory {}

impl InstructionMemory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self, address: u32) -> u32 {
        let index = address / 4;
        let instructions = vec![
            0b00000010001100101000000000100000,
            0b00000010001100101000000000100001,
            0b00000010001100101000000000100010,
            0b00000010001100101000000000100011
        ];
        instructions[index as usize]
    }
}

impl DataMemory {
    pub fn new() -> Self {
        Self {}
    }
}
