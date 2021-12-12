use log::error;

mod noise;
mod pulse;
mod pulsesweep;
mod wave;

use noise::NoiseChannel;
use pulse::PulseChannel;
use pulsesweep::PulseSweepChannel;
use wave::WaveChannel;

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
        }
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
