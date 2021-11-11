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

struct GameBoy {
    board: Board,
}

impl ggez::event::EventHandler<ggez::GameError> for GameBoy {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.step();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let frame = self.board.frame();
        for (y, row) in frame.into_iter().enumerate() {
            for (x, color) in row.into_iter().enumerate() {
                let x = x as i32 * PIXEL_SIZE;
                let y = y as i32 * PIXEL_SIZE;
                let pixel = graphics::Rect {
                    x: x as f32,
                    y: y as f32,
                    w: PIXEL_SIZE as f32,
                    h: PIXEL_SIZE as f32,
                };
                let color = graphics::Color::from_rgb(color.r, color.g, color.b);
                let pixel =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), pixel, color)?;
                graphics::draw(ctx, &pixel, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }

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

    let gameboy = GameBoy {
        board: Board::new(&data),
    };

    event::run(ctx, event_loop, gameboy);
}
