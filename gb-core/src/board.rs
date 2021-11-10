use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::mmu::Mmu;

pub struct Board {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
}

impl Board {
    pub fn new() -> Self {
        let mmu = Rc::new(RefCell::new(Mmu::new()));
        let cpu = Cpu::new(Rc::clone(&mmu));
        Self { cpu, mmu }
    }
}
