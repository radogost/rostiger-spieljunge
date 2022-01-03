use log::error;

pub(in crate::sound) mod common;
mod noise;
mod pulse;
mod pulsesweep;
mod wave;

use noise::NoiseChannel;
use pulse::PulseChannel;
use pulsesweep::PulseSweepChannel;
use wave::WaveChannel;

pub const SAMPLE_RATE: usize = 44_100;

// The CPU ticks with 4194304Hz, the PCM sample rate is 44.1kHz, so for single sample,
// we have to wait around 95 cycles.
const SAMPLE_TICKS: u8 = 95;

/// Audio Processing Unit
pub(crate) struct Apu {
    // channels
    pulse_channel: PulseChannel,
    pulsesweep_channel: PulseSweepChannel,
    wave_channel: WaveChannel,
    noise_channel: NoiseChannel,

    // registers
    volume_register: u8,
    output_register: u8,
    on_off_register: u8,

    // emulator internal counter of cycles in current sample
    sample_counter: u8,
    // audio buffer stores PCM data and is cleared every time a frontend implementation
    // fetches it. To prevent it from growing to infinity, in case it is never fetched, it's
    // regularly cleared when it reaches a certain size.
    audio_buffer: Vec<f32>,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse_channel: PulseChannel::new(),
            pulsesweep_channel: PulseSweepChannel::new(),
            wave_channel: WaveChannel::new(),
            noise_channel: NoiseChannel::new(),
            volume_register: 0,
            output_register: 0,
            on_off_register: 0,
            sample_counter: 0,
            audio_buffer: Vec::new(),
        }
    }

    pub fn step(&mut self, steps: u8) {
        for _ in 0..steps {
            self.pulsesweep_channel.single_step();

            self.sample_counter += 1;
            if self.sample_counter == SAMPLE_TICKS {
                self.sample_counter = 0;
                self.audio_buffer.push(self.pulsesweep_channel.get_volume());
                self.audio_buffer.push(self.pulsesweep_channel.get_volume());

                // clear the audio buffer if it wasn't requested for half a second.
                if self.audio_buffer.len() == SAMPLE_RATE {
                    self.audio_buffer.clear();
                }
            }
        }
    }

    pub fn audio_buffer(&mut self) -> Vec<f32> {
        let buffer = self.audio_buffer.clone();
        self.audio_buffer.clear();
        buffer
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff10..=0xff14 => self.pulsesweep_channel.read_byte(addr),
            0xff16..=0xff19 => self.pulse_channel.read_byte(addr),
            0xff1a..=0xff1e | 0xff30..=0xff3f => self.wave_channel.read_byte(addr),
            0xff20..=0xff23 => self.noise_channel.read_byte(addr),
            0xff24 => self.volume_register,
            0xff25 => self.output_register,
            0xff26 => self.on_off_register,
            _ => {
                error!("APU should never read byte from addr {:04x}", addr);
                unreachable!();
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xff10..=0xff14 => self.pulsesweep_channel.write_byte(addr, value),
            0xff16..=0xff19 => self.pulse_channel.write_byte(addr, value),
            0xff1a..=0xff1e | 0xff30..=0xff3f => self.wave_channel.write_byte(addr, value),
            0xff20..=0xff23 => self.noise_channel.write_byte(addr, value),
            0xff24 => self.volume_register = value,
            0xff25 => self.output_register = value,
            0xff26 => self.on_off_register = value,
            _ => {
                error!("APU should never write byte to addr {:04x}", addr);
                unreachable!();
            }
        }
    }
}
