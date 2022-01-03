use crate::sound::common::FRAME_TICKS;

pub(in crate::sound) struct PulseSweepChannel {
    sweep_register: u8,
    length_pattern_register: u8,
    volume_envelope_register: u8,
    frequency_low_register: u8,
    frequency_high_register: u8,

    // emulator internal
    enabled: bool,
    duty_advance_countdown: u16,
    frame_step: u8,
    length_counter: u8,
    volume: u8,
    duty_index: u8,
    // sweep envelope specifics
    sweep_counter: u8,
    // volume envelope specifics
    active_envelope: bool,
    envelope_counter: u8,
    // counter of cycles in current frame
    frame_counter: u16,
}

impl PulseSweepChannel {
    pub fn new() -> Self {
        Self {
            sweep_register: 0,
            length_pattern_register: 0,
            volume_envelope_register: 0,
            frequency_low_register: 0,
            frequency_high_register: 0,
            volume: 0,
            duty_index: 0,
            enabled: true,
            duty_advance_countdown: 0,
            sweep_counter: 0,
            active_envelope: true,
            envelope_counter: 0,
            frame_step: 0,
            length_counter: 0,
            frame_counter: 0,
        }
    }

    pub fn single_step(&mut self) {
        self.frame_counter += 1;
        if self.frame_counter == FRAME_TICKS {
            self.frame_counter = 0;
            match self.frame_step {
                2 | 6 => {
                    self.length_click();
                    self.sweep_click();
                }
                0 | 4 => self.length_click(),
                7 => self.volume_envelope_click(),
                _ => (),
            };
            self.frame_step += 1;
            if self.frame_step == 8 {
                self.frame_step = 0;
            }
        }

        if self.duty_advance_countdown > 0 {
            self.duty_advance_countdown -= 1;
        } else {
            self.duty_advance_countdown = self.required_cycles_for_duty_advance();
            self.duty_index += 1;
            if self.duty_index == 8 {
                self.duty_index = 0;
            }
        }
    }

    pub fn get_volume(&self) -> f32 {
        if self.duty_high() {
            (self.volume as f32) / 15.0
        } else {
            0.0
        }
    }

    fn length_click(&mut self) {
        if self.length_counter > 0 && self.counter_selection() {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn volume_envelope_click(&mut self) {
        let sweeps = self.envelope_sweeps();
        if sweeps == 0 || !self.active_envelope {
            return;
        }
        self.envelope_counter += 1;
        if self.envelope_counter == sweeps {
            self.envelope_counter = 0;
            let new_volume = self.volume as i8 + if self.incremental_envelope() { 1 } else { -1 };
            if (0..=15).contains(&new_volume) {
                self.volume = new_volume as u8;
            } else {
                self.active_envelope = false;
            }
        }
    }

    fn sweep_click(&mut self) {
        let sweep_time = self.sweep_time();
        if sweep_time == 0 {
            return;
        }
        self.sweep_counter += 1;
        if self.sweep_counter == sweep_time {
            self.sweep_counter = 0;
            let sweep_shifts = self.sweep_shifts();
            let freq = self.frequency();
            let dfreq = freq >> sweep_shifts;
            let new_freq = if self.incremental_sweep() {
                freq + dfreq
            } else {
                freq - dfreq
            };
            if new_freq < 2048 {
                self.set_frequency(new_freq);
            } else {
                self.enabled = false;
            }
        }
    }

    /// Returns how many cycles have to pass, until the index of the wave duty should be
    /// incremented
    fn required_cycles_for_duty_advance(&self) -> u16 {
        // A wave duty consists of 8 steps
        // the cpu freq is 4194304Hz
        // the freq of a complete wave duty is 131072 / (2048 - frequency)
        // therefore, we have to wait this many cycles to advance in our wave duty
        4 * (2048 - self.frequency())
    }

    /// Returns if the current duty index is at a high state
    fn duty_high(&self) -> bool {
        let duty = self.length_pattern_register >> 6;
        let required_index_for_high = match duty {
            0 => 1,
            1 => 2,
            2 => 4,
            3 => 6,
            _ => unreachable!(),
        };
        self.duty_index >= required_index_for_high
    }

    fn frequency(&self) -> u16 {
        (((self.frequency_high_register & 0b111) as u16) << 8) | self.frequency_low_register as u16
    }

    fn set_frequency(&mut self, freq: u16) {
        self.frequency_low_register = freq as u8;
        self.frequency_high_register &= 0b11111000 | ((freq >> 8) & 0b111) as u8;
    }

    fn counter_selection(&self) -> bool {
        (self.frequency_high_register >> 6 & 1) != 0
    }

    fn restart(&mut self) {
        self.enabled = true;
        self.active_envelope = true;
        self.envelope_counter = 0;
        self.sweep_counter = 0;
    }

    fn sweep_time(&self) -> u8 {
        self.sweep_register & 0b01110000
    }

    fn incremental_sweep(&self) -> bool {
        (self.sweep_register >> 3 & 1) == 0
    }

    fn sweep_shifts(&self) -> u8 {
        self.sweep_register & 0b111
    }

    fn incremental_envelope(&self) -> bool {
        (self.volume_envelope_register >> 3 & 1) != 0
    }

    fn envelope_sweeps(&self) -> u8 {
        self.volume_envelope_register & 0b111
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
            0xff11 => {
                self.length_pattern_register = value;
                self.length_counter = value & 0x1f;
            }
            0xff12 => {
                self.volume_envelope_register = value;
                self.volume = value >> 4;
                self.envelope_counter = 0;
            }
            0xff13 => self.frequency_low_register = value,
            0xff14 => {
                self.frequency_high_register = value;
                if (value >> 7 & 1) != 0 {
                    self.restart();
                }
            }
            _ => unreachable!(),
        }
    }
}
