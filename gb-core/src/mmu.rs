use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::ppu::Ppu;

const MEMORY_SIZE: usize = 0x10000;

pub(crate) struct Mmu {
    ppu: Rc<RefCell<Ppu>>,
    cartridge: Cartridge,
    memory: [u8; MEMORY_SIZE],

    interrupt_enable: u8,
    interrupt_flag: u8,
    joyp: u8, // joypad
}

impl Mmu {
    pub fn new(ppu: Rc<RefCell<Ppu>>, cartridge: Cartridge) -> Self {
        Self {
            ppu,
            cartridge,
            memory: [0; MEMORY_SIZE],
            interrupt_enable: 0,
            interrupt_flag: 0,
            joyp: 0xff,
        }
    }

    pub fn step(&mut self, steps: u8) {
        let mut ppu = self.ppu.borrow_mut();

        ppu.step(steps);
        self.interrupt_flag |= ppu.interrupt_flag();
        ppu.clear_interrupt_flag();
    }

    pub fn interrupt_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub fn interrupt_flag(&self) -> u8 {
        self.interrupt_flag
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.interrupt_flag = flag;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.read_byte(addr),
            0x8000..=0x9fff => self.ppu.borrow().read_byte(addr),
            0xff00 => self.joyp,
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow().read_byte(addr),
            0xfe00..=0xfe9f => self.ppu.borrow().read_byte(addr),
            0xff0f => self.interrupt_flag,
            0xff50 => self.cartridge.read_byte(addr),
            0xffff => self.interrupt_enable,
            _ => self.memory[addr as usize],
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.write_byte(addr, value),
            0x8000..=0x9fff => self.ppu.borrow_mut().write_byte(addr, value),
            0xff00 => {
                if (value & 0xf) == 0 {
                    return;
                }
                self.joyp = value;
                self.interrupt_flag |= 1;
            }
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow_mut().write_byte(addr, value),
            0xff46 => self.dma_transfer(value),
            0xfe00..=0xfe9f => self.ppu.borrow_mut().write_byte(addr, value),
            0xff0f => self.interrupt_flag = value,
            0xff50 => self.cartridge.write_byte(addr, value),
            0xffff => self.interrupt_enable = value,
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

    fn dma_transfer(&mut self, value: u8) {
        let high_byte = (value as u16) << 8;
        for offset in 0u16..=0x9f {
            let source = high_byte | offset;
            let destination = 0xfe00 | offset;
            let byte = self.read_byte(source);
            self.write_byte(destination, byte);
        }
    }
}
