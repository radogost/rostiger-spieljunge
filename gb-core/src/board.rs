use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::{Color, Ppu, HEIGHT, WIDTH};

pub struct Board {
    cpu: Cpu,
    ppu: Rc<RefCell<Ppu>>,
    ticks: usize,
}

impl Board {
    pub fn new(cartridge: &[u8]) -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));

        let mut mmu = Mmu::new(Rc::clone(&ppu));
        for (addr, byte) in cartridge.iter().enumerate() {
            mmu.write_byte(addr as u16, *byte);
        }

        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::new(Rc::clone(&mmu));
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
