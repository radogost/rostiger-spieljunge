use crate::sound::common::FRAME_TICKS;

pub(in crate::sound) struct PulseSweepChannel {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,

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
    volume_envelope_active: bool,
    volume_envelope_sweep_counter: u8,
    // counter of cycles in current frame
    frame_counter: u16,
}

impl PulseSweepChannel {
    pub fn new() -> Self {
        Self {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            volume: 0,
            duty_index: 0,
            enabled: false,
            duty_advance_countdown: 0,
            sweep_counter: 0,
            volume_envelope_active: true,
            volume_envelope_sweep_counter: 0,
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
        if self.enabled && self.duty_high() {
            (self.volume as f32) / 15.0
        } else {
            0.0
        }
    }

    fn length_click(&mut self) {
        if self.length_counter > 0 && self.stop_output_when_length_expires() {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn volume_envelope_click(&mut self) {
        let sweeps = self.volume_envelope_sweeps();
        if sweeps == 0 || !self.volume_envelope_active {
            return;
        }
        self.volume_envelope_sweep_counter += 1;
        if self.volume_envelope_sweep_counter == sweeps {
            self.volume_envelope_sweep_counter = 0;
            let incremental = self.incremental_volume_envelope();
            let new_volume = self.volume as i8 + if incremental { 1 } else { -1 };
            if (0..=15).contains(&new_volume) {
                self.volume = new_volume as u8;
            } else {
                self.volume_envelope_active = false;
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
        let duty = self.nr11 >> 6;
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
        (((self.nr14 & 0b111) as u16) << 8) | self.nr13 as u16
    }

    fn set_frequency(&mut self, freq: u16) {
        self.nr13 = freq as u8;
        self.nr14 &= 0b11111000 | ((freq >> 8) & 0b111) as u8;
    }

    fn stop_output_when_length_expires(&self) -> bool {
        (self.nr14 >> 6 & 1) == 1
    }

    fn restart(&mut self) {
        self.enabled = true;
        self.volume = self.nr12 >> 4;
        self.volume_envelope_active = true;
        self.volume_envelope_sweep_counter = 0;
        self.sweep_counter = 0;
    }

    fn sweep_time(&self) -> u8 {
        self.nr10 & 0b01110000
    }

    fn incremental_sweep(&self) -> bool {
        (self.nr10 >> 3 & 1) == 0
    }

    fn sweep_shifts(&self) -> u8 {
        self.nr10 & 0b111
    }

    fn incremental_volume_envelope(&self) -> bool {
        (self.nr12 >> 3 & 1) == 1
    }

    fn volume_envelope_sweeps(&self) -> u8 {
        self.nr12 & 0b111
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.nr10,
            0xff11 => self.nr11,
            0xff12 => self.nr12,
            0xff13 => self.nr13,
            0xff14 => self.nr14,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff10 => self.nr10 = value,
            0xff11 => {
                self.nr11 = value;
                self.length_counter = value & 0x1f;
            }
            0xff12 => {
                self.nr12 = value;
                self.volume = value >> 4;
                self.volume_envelope_sweep_counter = 0;
            }
            0xff13 => self.nr13 = value,
            0xff14 => {
                self.nr14 = value;
                if (value >> 7 & 1) == 1 {
                    self.restart();
                }
            }
            _ => unreachable!(),
        }
    }
}
