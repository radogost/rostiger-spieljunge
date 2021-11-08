const MEMORY: usize = 0x10000;

pub(crate) struct Mmu {
    memory: [u8; MEMORY],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        let low = self.memory[addr] as u16;
        let high = self.memory[addr + 1] as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        let addr = addr as usize;
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;
        self.memory[addr] = low;
        self.memory[addr + 1] = high;
    }
}
