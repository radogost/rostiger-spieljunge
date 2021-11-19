use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[macro_use]
extern crate clap;

use ggez::{event, graphics, input::keyboard, timer, Context, GameResult};

use gb_core::{Board, Button, HEIGHT, WIDTH};

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

fn keycode_to_button(keycode: keyboard::KeyCode) -> Option<Button> {
    match keycode {
        keyboard::KeyCode::A => Some(Button::A),
        keyboard::KeyCode::F => Some(Button::B),
        keyboard::KeyCode::Left => Some(Button::Left),
        keyboard::KeyCode::Right => Some(Button::Right),
        keyboard::KeyCode::Up => Some(Button::Up),
        keyboard::KeyCode::Down => Some(Button::Down),
        keyboard::KeyCode::Return => Some(Button::Start),
        keyboard::KeyCode::Space => Some(Button::Select),
        _ => None,
    }
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
        _repeat: bool,
    ) {
        if let Some(button) = keycode_to_button(keycode) {
            self.board.button_pressed(button);
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
    ) {
        if let Some(button) = keycode_to_button(keycode) {
            self.board.button_released(button);
        }
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

fn load_file(path: &str) -> Vec<u8> {
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

    let matches = clap_app!(app =>
        (version: "0.0")
        (author: "radogost")
        (about: "A GameBoy emulator written in Rust")
        (@arg BOOT: --boot +takes_value "Boot rom file")
        (@arg CARTRIDGE: +required "file with game data")
    )
    .get_matches();

    let cartridge = matches.value_of("CARTRIDGE").unwrap();
    let cartridge_data = load_file(cartridge);

    let board = if let Some(path) = matches.value_of("BOOT") {
        let boot_data = load_file(path);
        Board::new(&boot_data, &cartridge_data)
    } else {
        println!("No boot rom provided.");
        Board::no_boot(&cartridge_data)
    };

    let gameboy = GameBoy::new(board);

    let (ctx, event_loop) = ggez::ContextBuilder::new("Rostiger Spieljunge", "radogost")
        .window_setup(ggez::conf::WindowSetup::default().title("Rostiger Spieljunge"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    event::run(ctx, event_loop, gameboy);
}
