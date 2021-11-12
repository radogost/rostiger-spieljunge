use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use ggez::{event, graphics, timer, Context, GameResult};

use gb_core::{Board, HEIGHT, WIDTH};

const PIXEL_SIZE: i32 = 2;

const SCREEN_SIZE: (f32, f32) = (
    PIXEL_SIZE as f32 * WIDTH as f32,
    PIXEL_SIZE as f32 * HEIGHT as f32,
);

const DESIRED_FPS: u32 = 60;

const BLACK_SCREEN: [u8; HEIGHT * WIDTH * 4] = [0x00; HEIGHT * WIDTH * 4];

struct GameBoy {
    board: Board,
    screen: [u8; HEIGHT * WIDTH * 4],
    param: graphics::DrawParam,
}

impl GameBoy {
    fn new(board: Board) -> Self {
        let scale = graphics::mint::Vector2 { x: 2.0, y: 2.0 };
        let param = graphics::DrawParam::new().scale(scale);
        Self {
            board,
            screen: BLACK_SCREEN,
            param,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameBoy {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.step();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let frame = self.board.frame();
        for (y, row) in frame.into_iter().enumerate() {
            for (x, color) in row.into_iter().enumerate() {
                let idx = (x + y * WIDTH) * 4;
                self.screen[idx] = color.r;
                self.screen[idx + 1] = color.g;
                self.screen[idx + 2] = color.b;
                self.screen[idx + 3] = 0xFF;
            }
        }
        let image = graphics::Image::from_rgba8(ctx, WIDTH as u16, HEIGHT as u16, &self.screen)?;
        graphics::draw(ctx, &image, self.param)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn load_file(path: String) -> Vec<u8> {
    let path = PathBuf::from(path);
    let mut data = Vec::new();
    File::open(&path)
        .and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read cartridge")
        .unwrap();
    data
}

fn main() -> GameResult {
    env_logger::init();

    let bios = std::env::args().nth(1).unwrap();
    let cartridge = std::env::args().nth(2).unwrap();

    let bios_data = load_file(bios);
    let cartridge_data = load_file(cartridge);

    let mut data = Vec::new();
    for byte in bios_data.iter() {
        data.push(*byte);
    }
    for byte in cartridge_data.iter().skip(bios_data.len()) {
        data.push(*byte);
    }

    let (ctx, event_loop) = ggez::ContextBuilder::new("Rostiger Spieljunge", "radogost")
        .window_setup(ggez::conf::WindowSetup::default().title("Rostiger Spieljunge"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let gameboy = GameBoy::new(Board::new(&data));

    event::run(ctx, event_loop, gameboy);
}
