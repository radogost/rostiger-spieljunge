use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::irq::Irq;
use crate::joypad::{Button, JoyPad};
use crate::mmu::Mmu;
use crate::ppu::{Color, Ppu, HEIGHT, WIDTH};

pub struct Board {
    cpu: Cpu,
    ppu: Rc<RefCell<Ppu>>,
    mmu: Rc<RefCell<Mmu>>,
    joypad: Rc<RefCell<JoyPad>>,
    ticks: usize,
}

impl Board {
    pub fn new(boot: &[u8], game: &[u8]) -> Self {
        let irq = Rc::new(RefCell::new(Irq::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new(Rc::clone(&irq))));
        let joypad = Rc::new(RefCell::new(JoyPad::new(Rc::clone(&irq))));

        let cartridge = Cartridge::new(boot, game);
        let mmu = Mmu::new(
            Rc::clone(&irq),
            Rc::clone(&ppu),
            Rc::clone(&joypad),
            cartridge,
        );

        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::new(Rc::clone(&irq), Rc::clone(&mmu));
        Self {
            cpu,
            ppu,
            mmu,
            joypad,
            ticks: 0,
        }
    }

    /// Creates a board which doesn't have a boot rom.
    /// The memory and registers will be initialized such as if the execution
    /// of the boot rom just ended
    pub fn no_boot(game: &[u8]) -> Self {
        let cartridge = Cartridge::no_boot(game);
        let irq = Rc::new(RefCell::new(Irq::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new(Rc::clone(&irq))));
        let joypad = Rc::new(RefCell::new(JoyPad::new(Rc::clone(&irq))));

        let mut mmu = Mmu::new(
            Rc::clone(&irq),
            Rc::clone(&ppu),
            Rc::clone(&joypad),
            cartridge,
        );

        mmu.write_byte(0xff10, 0x80);
        mmu.write_byte(0xff11, 0xbf);
        mmu.write_byte(0xff12, 0xf3);
        mmu.write_byte(0xff14, 0xbf);
        mmu.write_byte(0xff16, 0x3f);
        mmu.write_byte(0xff19, 0xbf);
        mmu.write_byte(0xff1a, 0x7f);
        mmu.write_byte(0xff1b, 0xff);
        mmu.write_byte(0xff1c, 0x9f);
        mmu.write_byte(0xff1e, 0xff);
        mmu.write_byte(0xff20, 0xff);
        mmu.write_byte(0xff23, 0xbf);
        mmu.write_byte(0xff24, 0x77);
        mmu.write_byte(0xff25, 0xf3);
        mmu.write_byte(0xff26, 0xf1);
        mmu.write_byte(0xff40, 0x91);
        mmu.write_byte(0xff47, 0xfc);
        mmu.write_byte(0xff48, 0xff);
        mmu.write_byte(0xff49, 0xff);

        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::no_boot(Rc::clone(&irq), Rc::clone(&mmu));
        Self {
            cpu,
            ppu,
            mmu,
            joypad,
            ticks: 0,
        }
    }

    pub fn step(&mut self) {
        let mut leftticks = (70224 - self.ticks) as isize;
        while leftticks > 0 {
            let steps = self.cpu.step();
            self.mmu.borrow_mut().step(steps);
            leftticks -= steps as isize;
        }
        self.ticks = leftticks.abs() as usize;
    }

    pub fn frame(&self) -> [[Color; WIDTH]; HEIGHT] {
        self.ppu.borrow().frame()
    }

    pub fn button_pressed(&mut self, button: Button) {
        self.joypad.borrow_mut().button_pressed(button);
    }

    pub fn button_released(&mut self, button: Button) {
        self.joypad.borrow_mut().button_released(button);
    }
}
