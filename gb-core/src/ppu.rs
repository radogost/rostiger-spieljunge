use std::cell::RefCell;
use std::rc::Rc;

use log::error;

use crate::irq::Irq;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xa0;

pub const HEIGHT: usize = 144;
pub const WIDTH: usize = 160;

const OAM_SCAN_END: usize = 80;
const TRANSFER_END: usize = OAM_SCAN_END + 172;

enum Mode {
    HBlank,
    VBlank,
    OAMSearch,
    Transfer,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    fn white() -> Self {
        Color {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }

    fn lightgrey() -> Self {
        Color {
            r: 0xaa,
            g: 0xaa,
            b: 0xaa,
        }
    }

    fn darkgrey() -> Self {
        Color {
            r: 0x55,
            g: 0x55,
            b: 0x55,
        }
    }
}

/// Picture Processing Unit
pub(crate) struct Ppu {
    irq: Rc<RefCell<Irq>>,

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

    // emulator internal position of the current mode
    clock: usize,

    screen: [[Color; WIDTH]; HEIGHT],
}

impl Ppu {
    pub fn new(irq: Rc<RefCell<Irq>>) -> Self {
        Self {
            irq,
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
            clock: 0,
            screen: [[Color::white(); WIDTH]; HEIGHT],
        }
    }

    pub fn frame(&self) -> [[Color; WIDTH]; HEIGHT] {
        self.screen
    }

    /// The emulator is driven by the CPU and the other components have to catch up.
    pub fn step(&mut self, steps: u8) {
        if !self.lcd_enabled() {
            return;
        }
        for _ in 0..steps {
            self.single_step();
        }
    }

    fn single_step(&mut self) {
        self.clock = (self.clock + 1) % 456;
        if self.clock == 0 {
            self.ly = (self.ly + 1) % 154;
            self.set_lyc_ly_flag();
        }

        match (self.clock, self.ly as usize) {
            (0, 0..HEIGHT) => {
                self.set_mode(Mode::OAMSearch);
            }
            (OAM_SCAN_END, 0..HEIGHT) => {
                self.set_mode(Mode::Transfer);
            }
            (TRANSFER_END, 0..HEIGHT) => {
                self.set_mode(Mode::HBlank);
                self.draw_line();
            }
            (0, HEIGHT) => {
                self.set_mode(Mode::VBlank);
            }
            _ => {}
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[addr as usize - 0x8000],
            0xfe00..=0xfe9f => self.oam[addr as usize - 0xfe00],
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
            0x8000..=0x9fff => self.vram[addr as usize - 0x8000] = val,
            0xfe00..=0xfe9f => self.oam[addr as usize - 0xfe00] = val,
            0xff40 => self.lcdc = val,
            0xff41 => self.stat = val,
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => {} // ly register is read-only
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

    fn set_mode(&mut self, mode: Mode) {
        let (fire_stat_interrupt, mask) = match mode {
            Mode::HBlank => {
                let interrupt = (self.stat & (1 << 3)) != 0;
                (interrupt, 0)
            }
            Mode::VBlank => {
                self.irq.borrow_mut().vblank_interrupt();
                let interrupt = (self.stat & (1 << 4)) != 0;
                (interrupt, 1)
            }
            Mode::OAMSearch => {
                let interrupt = (self.stat & (1 << 5)) != 0;
                (interrupt, 2)
            }
            Mode::Transfer => (false, 3),
        };
        self.stat = (self.stat & 0xfc) | mask;
        if fire_stat_interrupt {
            self.irq.borrow_mut().lcd_stat_interrupt();
        }
    }

    fn set_lyc_ly_flag(&mut self) {
        if self.lyc == self.ly {
            self.stat |= 1 << 2;
            if self.stat & (1 << 6) != 0 {
                self.irq.borrow_mut().lcd_stat_interrupt();
            }
        } else {
            self.stat &= !(1 << 2);
        }
    }

    fn lcd_enabled(&self) -> bool {
        (self.lcdc & (1 << 7)) != 0
    }

    fn window_tile_map_base(&self) -> u16 {
        if (self.lcdc & (1 << 6)) == 0 {
            0x9800
        } else {
            0x9c00
        }
    }

    fn window_enabled(&self) -> bool {
        (self.lcdc & (1 << 5)) != 0
    }

    fn tile_data_base(&self) -> u16 {
        if (self.lcdc & (1 << 4)) == 0 {
            0x8800
        } else {
            0x8000
        }
    }

    fn bg_tile_map_base(&self) -> u16 {
        if (self.lcdc & (1 << 3)) == 0 {
            0x9800
        } else {
            0x9c00
        }
    }

    fn vertical_sprite_size(&self) -> u8 {
        if self.lcdc & (1 << 2) == 0 {
            8
        } else {
            16
        }
    }

    fn sprites_enabled(&self) -> bool {
        (self.lcdc & (1 << 1)) != 0
    }

    fn bg_and_window_enabled(&self) -> bool {
        (self.lcdc & 1) != 0
    }

    fn bg_window_tile_addr(&self, tile_id: u8) -> u16 {
        let tile_id = tile_id as u16;
        let tile_data_base_addr = self.tile_data_base();
        if tile_data_base_addr == 0x8000 {
            tile_data_base_addr + 16 * tile_id
        } else if tile_id < 128 {
            0x9000 + 16 * tile_id
        } else {
            tile_data_base_addr + 16 * (tile_id - 128)
        }
    }

    fn tile_color_index(&self, tile_addr: u16, tile_x: u8, tile_y: u8) -> u8 {
        let offset = (tile_y * 2) as u16;
        let low_byte = self.read_byte(tile_addr + offset);
        let high_byte = self.read_byte(tile_addr + offset + 1);

        let low = (low_byte >> (7 - tile_x)) & 1;
        let high = ((high_byte >> (7 - tile_x)) & 1) << 1;

        high | low
    }

    fn bg_window_color(&self, tile_id: u8, tile_x: u8, tile_y: u8) -> Color {
        let tile_addr = self.bg_window_tile_addr(tile_id);

        let color_index = self.tile_color_index(tile_addr, tile_x, tile_y);
        let color_value = (self.bgp >> (2 * color_index)) & 0x03;

        match color_value {
            0x00 => Color::white(),
            0x01 => Color::lightgrey(),
            0x02 => Color::darkgrey(),
            0x03 => Color::black(),
            _ => {
                error!(
                    "Color value in BGP register for index {} should be between {} and {}, but was {}",
                    color_index, 0, 3, color_value);
                unreachable!();
            }
        }
    }

    fn draw_line(&mut self) {
        if self.bg_and_window_enabled() {
            self.draw_background_line();
            self.draw_window_line();
        }
        if self.sprites_enabled() {
            self.draw_sprites();
        }
    }

    fn draw_background_line(&mut self) {
        let y = self.ly;
        for x in 0..WIDTH {
            let pixel_x = (x as u16 + self.scx as u16) % 256;
            let pixel_y = (y as u16 + self.scy as u16) % 256;

            let tile_map_index = (pixel_x / 8) + 32 * (pixel_y / 8);
            let tile_map_base = self.bg_tile_map_base();
            let tile_id = self.read_byte(tile_map_base + tile_map_index);

            let tile_x = pixel_x % 8;
            let tile_y = pixel_y % 8;
            let color = self.bg_window_color(tile_id, tile_x as u8, tile_y as u8);
            self.screen[y as usize][x as usize] = color;
        }
    }

    fn draw_window_line(&mut self) {
        if !self.window_enabled() || self.ly < self.wy {
            return;
        }

        let y = self.ly;
        let window_start = (if self.wx < 7 { 0 } else { self.wx - 7 }) as usize;
        for x in window_start..WIDTH {
            let pixel_x = x as u16 + 7 - self.wx as u16;
            let pixel_y = (y - self.wy) as u16;

            let tile_map_index = (pixel_x / 8) + 32 * (pixel_y / 8);
            let tile_map_base = self.window_tile_map_base();
            let tile_id = self.read_byte(tile_map_base + tile_map_index);

            let tile_x = pixel_x % 8;
            let tile_y = pixel_y % 8;
            let color = self.bg_window_color(tile_id, tile_x as u8, tile_y as u8);
            self.screen[y as usize][x as usize] = color;
        }
    }

    fn draw_sprites(&mut self) {
        let y = self.ly;
        let tile_y_size = self.vertical_sprite_size();

        for sprite in 0..40 {
            let sprite_base_addr = 0xfe00 + 4 * sprite;
            let y_pos = self.read_byte(sprite_base_addr);
            let x_pos = self.read_byte(sprite_base_addr + 1);

            if y + 16 < y_pos {
                continue;
            }
            let tile_y = y + 16 - y_pos;
            if tile_y >= tile_y_size {
                continue;
            }

            let tile_id = self.read_byte(sprite_base_addr + 2);
            let attributes = self.read_byte(sprite_base_addr + 3);

            let palette_number = (attributes >> 4) & 1;
            let x_flip = (attributes & (1 << 5)) != 0;
            let y_flip = (attributes & (1 << 6)) != 0;
            let bg_over_sprite = (attributes & (1 << 7)) != 0;

            let palette = if palette_number == 0 {
                self.obp0
            } else {
                self.obp1
            };

            let tile_y = if y_flip {
                tile_y_size - tile_y - 1
            } else {
                tile_y
            };

            let tile_id = if tile_y_size == 8 {
                tile_id
            } else {
                if tile_y < 8 {
                    tile_id & 0xfe
                } else {
                    tile_id | 1
                }
            } as u16;

            for x in 0..WIDTH {
                if bg_over_sprite && self.screen[y as usize][x as usize] != Color::white() {
                    continue;
                }
                let x = x as u8;
                if x + 8 < x_pos {
                    continue;
                }

                let tile_x = x + 8 - x_pos;
                if tile_x >= 8 {
                    continue;
                }

                let tile_x = if x_flip { 7 - tile_x } else { tile_x };

                let tile_addr = 0x8000 + tile_id * 16;
                let color_index = self.tile_color_index(tile_addr, tile_x, tile_y);
                if color_index == 0 {
                    // transparent
                    continue;
                }

                let color_value = (palette >> (2 * color_index)) & 0x03;
                let color = match color_value {
                    0x00 => Color::white(),
                    0x01 => Color::lightgrey(),
                    0x02 => Color::darkgrey(),
                    0x03 => Color::black(),
                    _ => {
                        unreachable!();
                    }
                };

                self.screen[y as usize][x as usize] = color;
            }
        }
    }
}
