use std::cell::RefCell;
use std::rc::Rc;

use crate::irq::Irq;

pub(crate) struct Timer {
    irq: Rc<RefCell<Irq>>,
    divider: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: u8,
    internal_divider: u16,
    internal_timer: u16,
}

impl Timer {
    pub fn new(irq: Rc<RefCell<Irq>>) -> Self {
        Self {
            irq,
            divider: 0,
            timer_counter: 0,
            timer_modulo: 0,
            timer_control: 0,
            internal_divider: 0,
            internal_timer: 0,
        }
    }

    pub fn step(&mut self, steps: u8) {
        let steps = steps as u16;
        self.internal_divider += steps;
        if self.internal_divider >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.internal_divider -= 256;
        }

        let timer_enabled = (self.timer_control & (1 << 2)) != 0;
        if timer_enabled {
            self.internal_timer += steps;
            let steps_to_wait = match self.timer_control & 0b11 {
                0 => 1024,
                1 => 16,
                2 => 65,
                3 => 256,
                _ => unreachable!(),
            };
            if self.internal_timer >= steps_to_wait {
                self.internal_timer -= steps_to_wait;
                self.timer_counter = self.timer_counter.wrapping_add(1);
                if self.timer_counter == 0 {
                    self.timer_counter = self.timer_modulo;
                    self.irq.borrow_mut().timer_interrupt();
                }
            }
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff04 => self.divider,
            0xff05 => self.timer_counter,
            0xff06 => self.timer_modulo,
            0xff07 => self.timer_control,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xff04 => self.divider = 0,
            0xff05 => self.timer_counter = val,
            0xff06 => self.timer_modulo = val,
            0xff07 => self.timer_control = val,
            _ => unreachable!(),
        }
    }
}
