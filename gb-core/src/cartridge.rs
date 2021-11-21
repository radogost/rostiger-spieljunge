pub(crate) struct Cartridge {
    boot_rom: [u8; 0x100],
    // TODO: split into mbc
    game_data: [u8; 0x8000],
    use_boot_rom: bool,
}

impl Cartridge {
    pub fn new(boot: &[u8], game: &[u8]) -> Self {
        let mut boot_rom = [0u8; 0x100];
        for (&x, p) in boot.iter().zip(boot_rom.iter_mut()) {
            *p = x;
        }
        let mut game_data = [0u8; 0x8000];
        for (&x, p) in game.iter().zip(game_data.iter_mut()) {
            *p = x;
        }

        Self {
            boot_rom,
            game_data,
            use_boot_rom: true,
        }
    }

    pub fn no_boot(game: &[u8]) -> Self {
        let mut game_data = [0u8; 0x8000];
        for (&x, p) in game.iter().zip(game_data.iter_mut()) {
            *p = x;
        }

        Self {
            boot_rom: [0u8; 0x100],
            game_data,
            use_boot_rom: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.use_boot_rom && addr < 0x100 {
            self.boot_rom[addr as usize]
        } else {
            self.game_data[addr as usize]
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => {}
            0xff50 => self.use_boot_rom = false,
            _ => self.game_data[addr as usize] = val,
        }
    }
}
