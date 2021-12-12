pub(in crate::sound) struct PulseChannel {
    length_pattern_register: u8,
    volume_envelope_register: u8,
    frequency_low_register: u8,
    frequency_high_register: u8,
}

impl PulseChannel {
    pub fn new() -> Self {
        Self {
            length_pattern_register: 0,
            volume_envelope_register: 0,
            frequency_low_register: 0,
            frequency_high_register: 0,
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff16 => self.length_pattern_register,
            0xff17 => self.volume_envelope_register,
            0xff18 => self.frequency_low_register,
            0xff19 => self.frequency_high_register,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff16 => self.length_pattern_register = value,
            0xff17 => self.volume_envelope_register = value,
            0xff18 => self.frequency_low_register = value,
            0xff19 => self.frequency_high_register = value,
            _ => unreachable!(),
        }
    }
}
