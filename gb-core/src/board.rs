use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::Ppu;

pub struct Board {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
    ppu: Rc<RefCell<Ppu>>,
}

impl Board {
    pub fn new() -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let mmu = Mmu::new(Rc::clone(&ppu));
        let mmu = Rc::new(RefCell::new(mmu));
        let cpu = Cpu::new(Rc::clone(&mmu));
        Self { cpu, mmu, ppu }
    }
}
