use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::irq::Irq;
use crate::joypad::JoyPad;
use crate::ppu::Ppu;
use crate::timer::Timer;

const MEMORY_SIZE: usize = 0x10000;

pub(crate) struct Mmu {
    timer: Timer,
    irq: Rc<RefCell<Irq>>,
    ppu: Rc<RefCell<Ppu>>,
    joypad: Rc<RefCell<JoyPad>>,
    cartridge: Cartridge,
    memory: [u8; MEMORY_SIZE],
    interrupt_enable: u8,
}

impl Mmu {
    pub fn new(
        irq: Rc<RefCell<Irq>>,
        ppu: Rc<RefCell<Ppu>>,
        joypad: Rc<RefCell<JoyPad>>,
        cartridge: Cartridge,
    ) -> Self {
        Self {
            timer: Timer::new(Rc::clone(&irq)),
            irq,
            ppu,
            joypad,
            cartridge,
            memory: [0; MEMORY_SIZE],
            interrupt_enable: 0,
        }
    }

    pub fn step(&mut self, steps: u8) {
        let mut ppu = self.ppu.borrow_mut();
        ppu.step(steps);

        self.timer.step(steps);
    }

    pub fn interrupt_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.read_byte(addr),
            0x8000..=0x9fff => self.ppu.borrow().read_byte(addr),
            0xff00 => self.joypad.borrow().read_byte(),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow().read_byte(addr),
            0xfe00..=0xfe9f => self.ppu.borrow().read_byte(addr),
            0xff04..=0xff07 => self.timer.read_byte(addr),
            0xff0f => self.irq.borrow().interrupt_flag(),
            0xff50 => self.cartridge.read_byte(addr),
            0xffff => self.interrupt_enable,
            _ => self.memory[addr as usize],
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.write_byte(addr, value),
            0x8000..=0x9fff => self.ppu.borrow_mut().write_byte(addr, value),
            0xff00 => self.joypad.borrow_mut().write_byte(value),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow_mut().write_byte(addr, value),
            0xff46 => self.dma_transfer(value),
            0xfe00..=0xfe9f => self.ppu.borrow_mut().write_byte(addr, value),
            0xff04..=0xff07 => self.timer.write_byte(addr, value),
            0xff0f => self.irq.borrow_mut().set_interrupt_flag(value),
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
