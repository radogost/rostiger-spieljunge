use log::error;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xa0;

/// Picture Processing Unit
pub(crate) struct Ppu {
    // video ram
    vram: [u8; VRAM_SIZE],

    // sprite attribute table
    oam: [u8; OAM_SIZE],

    // control register
    lcdc: u8,

    // status register
    stat: u8,

    // registers for positioning and scrolling
    scy: u8, // scroll y
    scx: u8, // scroll x
    ly: u8,  // current horizontal line
    lyc: u8, // ly compare
    wy: u8,  // window y position
    wx: u8,  // window x position

    // color palette registers
    bgp: u8,  // gray shades of the BG and Window tiles
    obp0: u8, // gray shades of objects
    obp1: u8, // gray shades of objects
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[addr as usize],
            0xfe00..=0xfe9f => self.oam[addr as usize],
            0xff40 => self.lcdc,
            0xff41 => self.stat,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => {
                error!("Unimplemented read byte from addr {:04x}", addr);
                unimplemented!();
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => self.vram[addr as usize] = val,
            0xfe00..=0xfe9f => self.oam[addr as usize] = val,
            0xff40 => self.lcdc = val,
            0xff41 => self.stat = val,
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff45 => self.lyc = val,
            0xff47 => self.bgp = val,
            0xff48 => self.obp0 = val,
            0xff49 => self.obp1 = val,
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            _ => {
                error!("Unimplemented write byte to addr {:04x}", addr);
                unimplemented!();
            }
        }
    }
}
