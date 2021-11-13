use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::ppu::Ppu;

const MEMORY_SIZE: usize = 0x10000;

pub(crate) struct Mmu {
    ppu: Rc<RefCell<Ppu>>,
    cartridge: Cartridge,
    memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new(ppu: Rc<RefCell<Ppu>>, cartridge: Cartridge) -> Self {
        Self {
            ppu,
            cartridge,
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.read_byte(addr),
            0x8000..=0x9fff => self.ppu.borrow().read_byte(addr),
            0xff00 => 0xff,
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow().read_byte(addr),
            0xfe00..=0xfe9f => self.ppu.borrow().read_byte(addr),
            0xff50 => self.cartridge.read_byte(addr),
            _ => self.memory[addr as usize],
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.write_byte(addr, value),
            0x8000..=0x9fff => self.ppu.borrow_mut().write_byte(addr, value),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow_mut().write_byte(addr, value),
            0xfe00..=0xfe9f => self.ppu.borrow_mut().write_byte(addr, value),
            0xff50 => self.cartridge.write_byte(addr, value),
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
