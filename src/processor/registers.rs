pub struct Registers {
    r: [u32; 32]
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum Register {
    Zero = 0,
    At = 1,
    V0 = 2,
    V1 = 3,
    A0 = 4,
    A1 = 5,
    A2 = 6,
    A3 = 7,
    T0 = 8,
    T1 = 9,
    T2 = 10,
    T3 = 11,
    T4 = 12,
    T5 = 13,
    T6 = 14,
    T7 = 15,
    S0 = 16,
    S1 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    T8 = 24,
    T9 = 25,
    K0 = 26,
    K1 = 27,
    Gp = 28,
    Sp = 29,
    Fp = 30,
    Ra = 31
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 32]
        }
    }

    pub fn get(&self, reg: Register) -> u32 {
        self.r[reg]
    }

    pub fn set(&mut self, reg: Register, value: u32) {
        if (reg as usize) == 0 {
            return;
        }
        self.r[reg] = value;
    }
}
