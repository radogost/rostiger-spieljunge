#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    A,
    B,
}

pub(crate) struct JoyPad {
    direction_buttons: u8,
    action_buttons: u8,
    selection_flag: u8,
    interrupt_flag: u8,
}

impl JoyPad {
    pub fn new() -> Self {
        Self {
            direction_buttons: 0xf,
            action_buttons: 0xf,
            selection_flag: 0,
            interrupt_flag: 0,
        }
    }

    pub fn button_pressed(&mut self, button: Button) {
        match button {
            Button::Right => self.direction_buttons = !(1 << 0),
            Button::Left => self.direction_buttons = !(1 << 1),
            Button::Up => self.direction_buttons = !(1 << 2),
            Button::Down => self.direction_buttons = !(1 << 3),
            Button::A => self.action_buttons = !(1 << 0),
            Button::B => self.action_buttons = !(1 << 1),
            Button::Select => self.action_buttons = !(1 << 2),
            Button::Start => self.action_buttons = !(1 << 3),
        }
        self.interrupt_flag = 1;
    }

    pub fn interrupt_flag(&self) -> u8 {
        self.interrupt_flag
    }

    pub fn clear_interrupt_flag(&mut self) {
        self.interrupt_flag = 0;
    }

    pub fn write_byte(&mut self, value: u8) {
        self.selection_flag = value;
    }

    pub fn read_byte(&self) -> u8 {
        if (self.selection_flag & 0x20) == 0 {
            self.action_buttons
        } else if (self.selection_flag & 0x10) == 0 {
            self.direction_buttons
        } else {
            0xf
        }
    }
}
