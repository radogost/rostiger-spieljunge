use std::cell::RefCell;
use std::rc::Rc;

use crate::ppu::Ppu;

const MEMORY_SIZE: usize = 0x10000;

pub(crate) struct Mmu {
    memory: [u8; MEMORY_SIZE],
    ppu: Rc<RefCell<Ppu>>,
}

impl Mmu {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            ppu,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.ppu.borrow().read_byte(addr),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow().read_byte(addr),
            0xfe00..=0xfe9f => self.ppu.borrow().read_byte(addr),
            _ => self.memory[addr as usize],
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9fff => self.ppu.borrow_mut().write_byte(addr, value),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow_mut().write_byte(addr, value),
            0xfe00..=0xfe9f => self.ppu.borrow_mut().write_byte(addr, value),
            _ => self.memory[addr as usize] = value,
        }
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
        self.write_byte(addr + 1, high);
    }
}
