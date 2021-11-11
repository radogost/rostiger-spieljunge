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
    pub fn new() -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let mmu = Mmu::new(Rc::clone(&ppu));
        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::new(Rc::clone(&mmu));
        Self { cpu, ppu, ticks: 0 }
    }

    pub fn frame(&mut self) -> [[Color; WIDTH]; HEIGHT] {
        let mut leftticks = (70224 - self.ticks) as isize;
        while leftticks > 0 {
            let steps = self.cpu.step();
            self.ppu.borrow_mut().step(steps);
            leftticks -= steps as isize;
        }
        self.ticks = (-1 * leftticks) as usize;
        self.ppu.borrow().frame()
    }
}
