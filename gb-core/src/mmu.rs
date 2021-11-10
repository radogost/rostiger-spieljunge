const MEMORY_SIZE: usize = 0x10000;

pub(crate) struct Mmu {
    memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;
        self.write_byte(addr, low);
        self.write_byte(addr, high);
    }
}
