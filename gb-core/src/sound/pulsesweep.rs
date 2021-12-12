pub(in crate::sound) struct PulseSweepChannel {
    sweep_register: u8,
    length_pattern_register: u8,
    volume_envelope_register: u8,
    frequency_low_register: u8,
    frequency_high_register: u8,
}

impl PulseSweepChannel {
    pub fn new() -> Self {
        Self {
            sweep_register: 0,
            length_pattern_register: 0,
            volume_envelope_register: 0,
            frequency_low_register: 0,
            frequency_high_register: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.sweep_register,
            0xff11 => self.length_pattern_register,
            0xff12 => self.volume_envelope_register,
            0xff13 => self.frequency_low_register,
            0xff14 => self.frequency_high_register,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff10 => self.sweep_register = value,
            0xff11 => self.length_pattern_register = value,
            0xff12 => self.volume_envelope_register = value,
            0xff13 => self.frequency_low_register = value,
            0xff14 => self.frequency_high_register = value,
            _ => unreachable!(),
        }
    }
}
