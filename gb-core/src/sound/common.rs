// The frame sequencer is clocked by a 512Hz timer, so we have to wait 8192 cycles for a step
pub(in crate::sound) const FRAME_TICKS: u16 = 8192;
