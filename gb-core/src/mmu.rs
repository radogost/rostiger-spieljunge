use std::cell::RefCell;
use std::rc::Rc;

use log::error;

use crate::cartridge::Cartridge;
use crate::irq::Irq;
use crate::joypad::JoyPad;
use crate::ppu::Ppu;
use crate::sound::Apu;
use crate::timer::Timer;

const WRAM_SIZE: usize = 0x2000;
const ECHO_RAM_SIZE: usize = 0x1e00;
const HRAM_SIZE: usize = 0xfe;
const SERIAL_RAM: usize = 0x2;

pub(crate) struct Mmu {
    timer: Timer,
    apu: Rc<RefCell<Apu>>,
    irq: Rc<RefCell<Irq>>,
    ppu: Rc<RefCell<Ppu>>,
    joypad: Rc<RefCell<JoyPad>>,
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    echo_ram: [u8; ECHO_RAM_SIZE],
    hram: [u8; HRAM_SIZE],
    serial_ram: [u8; SERIAL_RAM],
    interrupt_enable: u8,
}

impl Mmu {
    pub fn new(
        apu: Rc<RefCell<Apu>>,
        irq: Rc<RefCell<Irq>>,
        ppu: Rc<RefCell<Ppu>>,
        joypad: Rc<RefCell<JoyPad>>,
        cartridge: Cartridge,
    ) -> Self {
        Self {
            timer: Timer::new(Rc::clone(&irq)),
            apu,
            irq,
            ppu,
            joypad,
            cartridge,
            wram: [0; WRAM_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            hram: [0; HRAM_SIZE],
            serial_ram: [0; SERIAL_RAM],
            interrupt_enable: 0,
        }
    }

    pub fn step(&mut self, steps: u8) {
        let mut ppu = self.ppu.borrow_mut();
        ppu.step(steps);

        let mut apu = self.apu.borrow_mut();
        apu.step(steps);

        self.timer.step(steps);
    }

    pub fn interrupt_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.read_byte(addr),
            0x8000..=0x9fff => self.ppu.borrow().read_byte(addr),
            0xc000..=0xdfff => self.wram[addr as usize - 0xc000],
            0xe000..=0xfdff => self.echo_ram[addr as usize - 0xe000],
            0xfe00..=0xfe9f => self.ppu.borrow().read_byte(addr),
            0xff00 => self.joypad.borrow().read_byte(),
            0xff01..=0xff02 => self.serial_ram[addr as usize - 0xff01],
            0xff04..=0xff07 => self.timer.read_byte(addr),
            0xff0f => self.irq.borrow().interrupt_flag(),
            0xff10..=0xff26 | 0xff30..=0xff3f => self.apu.borrow().read_byte(addr),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow().read_byte(addr),
            0xff50 => self.cartridge.read_byte(addr),
            0xff80..=0xfffe => self.hram[addr as usize - 0xff80],
            0xffff => self.interrupt_enable,
            _ => {
                error!("Unimplemented read byte from addr {:04x}", addr);
                unimplemented!();
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.write_byte(addr, value),
            0x8000..=0x9fff => self.ppu.borrow_mut().write_byte(addr, value),
            0xc000..=0xdfff => self.wram[addr as usize - 0xc000] = value,
            0xe000..=0xfdff => self.echo_ram[addr as usize - 0xe000] = value,
            0xfe00..=0xfe9f => self.ppu.borrow_mut().write_byte(addr, value),
            0xfea0..=0xfeff => (), // not usable
            0xff00 => self.joypad.borrow_mut().write_byte(value),
            0xff01..=0xff02 => self.serial_ram[addr as usize - 0xff01] = value,
            0xff04..=0xff07 => self.timer.write_byte(addr, value),
            0xff0f => self.irq.borrow_mut().set_interrupt_flag(value),
            0xff10..=0xff26 | 0xff30..=0xff3f => self.apu.borrow_mut().write_byte(addr, value),
            0xff40..=0xff45 | 0xff47..=0xff4b => self.ppu.borrow_mut().write_byte(addr, value),
            0xff46 => self.dma_transfer(value),
            0xff50 => self.cartridge.write_byte(addr, value),
            0xff7f => (), // not usable
            0xff80..=0xfffe => self.hram[addr as usize - 0xff80] = value,
            0xffff => self.interrupt_enable = value,
            _ => {
                error!("Unimplemented write byte to addr {:04x}", addr);
                unimplemented!();
            }
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
