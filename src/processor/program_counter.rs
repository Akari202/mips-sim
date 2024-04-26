use log::{info, trace};

pub struct ProgramCounter {
    pc: u32
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { pc: 0 }
    }

    pub fn get(&self) -> u32 {
        self.pc
    }

    pub fn set(&mut self, value: u32) {
        info!("Setting program counter to {:#x}", value);
        self.pc = value;
    }

    pub fn increment(&mut self) {
        self.pc += 4;
    }
}
