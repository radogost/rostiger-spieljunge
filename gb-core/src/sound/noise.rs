pub(in crate::sound) struct NoiseChannel {
    length_register: u8,
    volume_envelope_register: u8,
    polynomial_counter_register: u8,
    consecutive_register: u8,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            length_register: 0,
            volume_envelope_register: 0,
            polynomial_counter_register: 0,
            consecutive_register: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff20 => self.length_register,
            0xff21 => self.volume_envelope_register,
            0xff22 => self.polynomial_counter_register,
            0xff23 => self.consecutive_register,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff20 => self.length_register = value,
            0xff21 => self.volume_envelope_register = value,
            0xff22 => self.polynomial_counter_register = value,
            0xff23 => self.consecutive_register = value,
            _ => unreachable!(),
        }
    }
}
