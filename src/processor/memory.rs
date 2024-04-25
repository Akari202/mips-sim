use std::ptr::NonNull;

pub struct Memory {
    pointer: NonNull<u8>,
    capacity: u32
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            pointer: NonNull::dangling(),
            capacity: 0
        }
    }

    pub fn new_with_capacity(word_capacity: u32) -> Self {
        let layout = std::alloc::Layout::array::<u8>((word_capacity << 2) as usize).unwrap();
        let pointer = unsafe { std::alloc::alloc(layout) };
        if pointer.is_null() {
            panic!("Failed to allocate memory");
        }
        Memory {
            pointer: NonNull::new(pointer).unwrap(),
            capacity: word_capacity * 4
        }
    }

    pub fn get_stack_pointer(&self) -> u32 {
        0
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.read() }
    }

    pub fn write_byte(&self, address: u32, value: u8) {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.write(value); }
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u16>().read() }
    }

    pub fn write_halfword(&self, address: u32, value: u16) {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u16>().write(value); }
    }

    pub fn read_word(&self, address: u32) -> u32 {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u32>().read() }
    }

    pub fn write_word(&self, address: u32, value: u32) {
        self.check_address(address);
        let pointer = unsafe { self.pointer.offset(address as isize) };
        unsafe { pointer.cast::<u32>().write(value); }
    }

    pub fn read_cstring(&self, address: u32) -> String {
        self.check_address(address);
        let mut result = String::new();
        let mut index = address;
        loop {
            let byte: u8 = self.read_byte(index);
            if byte == "\0".as_bytes()[0] {
                break;
            }
            result.push(byte as char);
            index += 1;
        }
        result
    }

    fn check_address(&self, address: u32) {
        if address > self.capacity || self.capacity == 0 {
            panic!("Address {} out of range", address);
        }
    }
}

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

    pub fn read_byte(&self, address: u32) -> u8 {
        0
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        0
    }

    pub fn read_word(&self, address: u32) -> u32 {
        0
    }

    pub fn read_cstring(&self, address: u32) -> String {
        String::from("")
    }
}
