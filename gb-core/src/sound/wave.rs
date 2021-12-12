use log::error;

const WAVE_PATTERN_SIZE: usize = 0x40;

pub(in crate::sound) struct WaveChannel {
    on_off_register: u8,
    length_register: u8,
    output_level_register: u8,
    frequency_low_register: u8,
    frequency_high_register: u8,
    wave_pattern: [u8; WAVE_PATTERN_SIZE],
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            on_off_register: 0,
            length_register: 0,
            output_level_register: 0,
            frequency_low_register: 0,
            frequency_high_register: 0,
            wave_pattern: [0; WAVE_PATTERN_SIZE],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff1a => self.on_off_register,
            0xff1b => self.length_register,
            0xff1c => self.output_level_register,
            0xff1d => self.frequency_low_register,
            0xff1e => self.frequency_high_register,
            0xff30..=0xff3f => self.wave_pattern[(addr - 0xff30) as usize],
            _ => {
                error!(
                    "APU wave channel should never read byte from addr {:04x}",
                    addr
                );
                unreachable!();
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff1a => self.on_off_register = value,
            0xff1b => self.length_register = value,
            0xff1c => self.output_level_register = value,
            0xff1d => self.frequency_low_register = value,
            0xff1e => self.frequency_high_register = value,
            0xff30..=0xff3f => self.wave_pattern[(addr - 0xff30) as usize] = value,
            _ => {
                error!(
                    "APU wave channel should never write byte to addr {:04x}",
                    addr
                );
                unreachable!();
            }
        }
    }
}
