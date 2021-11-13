use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::{Color, Ppu, HEIGHT, WIDTH};

pub struct Board {
    cpu: Cpu,
    ppu: Rc<RefCell<Ppu>>,
    ticks: usize,
}

impl Board {
    pub fn new(boot: &[u8], game: &[u8]) -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));

        let cartridge = Cartridge::new(boot, game);
        let mmu = Mmu::new(Rc::clone(&ppu), cartridge);

        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::new(Rc::clone(&mmu));
        Self { cpu, ppu, ticks: 0 }
    }

    /// Creates a board which doesn't have a boot rom.
    /// The memory and registers will be initialized such as if the execution
    /// of the boot rom just ended
    pub fn no_boot(game: &[u8]) -> Self {
        let cartridge = Cartridge::no_boot(game);
        let ppu = Rc::new(RefCell::new(Ppu::new()));

        let mut mmu = Mmu::new(Rc::clone(&ppu), cartridge);

        mmu.write_byte(0xff05, 0);
        mmu.write_byte(0xff06, 0);
        mmu.write_byte(0xff07, 0);
        mmu.write_byte(0xff10, 0x80);
        mmu.write_byte(0xff11, 0xbf);
        mmu.write_byte(0xff12, 0xf3);
        mmu.write_byte(0xff14, 0xbf);
        mmu.write_byte(0xff16, 0x3f);
        mmu.write_byte(0xff17, 0);
        mmu.write_byte(0xff19, 0xbf);
        mmu.write_byte(0xff1a, 0x7f);
        mmu.write_byte(0xff1b, 0xff);
        mmu.write_byte(0xff1c, 0x9f);
        mmu.write_byte(0xff1e, 0xff);
        mmu.write_byte(0xff20, 0xff);
        mmu.write_byte(0xff21, 0);
        mmu.write_byte(0xff22, 0);
        mmu.write_byte(0xff23, 0xbf);
        mmu.write_byte(0xff24, 0x77);
        mmu.write_byte(0xff25, 0xf3);
        mmu.write_byte(0xff26, 0xf1);
        mmu.write_byte(0xff40, 0x91);
        mmu.write_byte(0xff42, 0);
        mmu.write_byte(0xff43, 0);
        mmu.write_byte(0xff45, 0);
        mmu.write_byte(0xff47, 0xfc);
        mmu.write_byte(0xff48, 0xff);
        mmu.write_byte(0xff49, 0xff);
        mmu.write_byte(0xff4a, 0);
        mmu.write_byte(0xff4b, 0);

        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::no_boot(Rc::clone(&mmu));
        Self { cpu, ppu, ticks: 0 }
    }

    pub fn step(&mut self) {
        let mut leftticks = (70224 - self.ticks) as isize;
        while leftticks > 0 {
            let steps = self.cpu.step();
            self.ppu.borrow_mut().step(steps);
            leftticks -= steps as isize;
        }
        self.ticks = leftticks.abs() as usize;
    }

    pub fn frame(&self) -> [[Color; WIDTH]; HEIGHT] {
        self.ppu.borrow().frame()
    }
}
