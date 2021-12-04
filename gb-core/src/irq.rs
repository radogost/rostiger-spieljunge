pub(crate) struct Irq {
    interrupt_flag: u8,
}

impl Irq {
    pub fn new() -> Self {
        Self { interrupt_flag: 0 }
    }

    pub fn interrupt_flag(&self) -> u8 {
        self.interrupt_flag
    }

    pub fn set_interrupt_flag(&mut self, interrupt_flag: u8) {
        self.interrupt_flag = interrupt_flag;
    }

    pub fn vblank_interrupt(&mut self) {
        self.interrupt_flag |= 1;
    }

    pub fn lcd_stat_interrupt(&mut self) {
        self.interrupt_flag |= 1 << 1;
    }

    pub fn timer_interrupt(&mut self) {
        self.interrupt_flag |= 1 << 2;
    }

    pub fn joypad_interrupt(&mut self) {
        self.interrupt_flag |= 1 << 4;
    }
}
