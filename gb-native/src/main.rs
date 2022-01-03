use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use sdl2::event;
use sdl2::keyboard;

#[macro_use]
extern crate clap;

use gb_core::{Board, Button, HEIGHT, WIDTH};

const PIXEL_SCALE: usize = 2;

struct GameBoy {
    board: Board,
}

fn keycode_to_button(keycode: keyboard::Keycode) -> Option<Button> {
    match keycode {
        keyboard::Keycode::A => Some(Button::A),
        keyboard::Keycode::F => Some(Button::B),
        keyboard::Keycode::Left => Some(Button::Left),
        keyboard::Keycode::Right => Some(Button::Right),
        keyboard::Keycode::Up => Some(Button::Up),
        keyboard::Keycode::Down => Some(Button::Down),
        keyboard::Keycode::Return => Some(Button::Start),
        keyboard::Keycode::Space => Some(Button::Select),
        _ => None,
    }
}

impl GameBoy {
    fn new(board: Board) -> Self {
        Self { board }
    }

    fn next_frame(&mut self) -> Vec<u8> {
        self.board.run_to_next_frame();
        let frame = self.board.frame();
        frame
            .into_iter()
            .flat_map(|row| row.into_iter())
            .flat_map(|c| [c.r, c.g, c.b])
            .collect()
    }

    fn button_pressed(&mut self, button: Button) {
        self.board.button_pressed(button);
    }

    fn button_released(&mut self, button: Button) {
        self.board.button_released(button);
    }
}

//impl ggez::event::EventHandler<ggez::GameError> for GameBoy {
//    fn update(&mut self, ctx: &mut Context) -> GameResult {
//        while timer::check_update_time(ctx, DESIRED_FPS) {
//            self.board.step();
//        }
//        Ok(())
//    }
//
//    fn key_down_event(
//        &mut self,
//        _ctx: &mut Context,
//        keycode: keyboard::KeyCode,
//        _keymods: keyboard::KeyMods,
//        _repeat: bool,
//    ) {
//        if let Some(button) = keycode_to_button(keycode) {
//            self.board.button_pressed(button);
//        }
//    }
//
//    fn key_up_event(
//        &mut self,
//        _ctx: &mut Context,
//        keycode: keyboard::KeyCode,
//        _keymods: keyboard::KeyMods,
//    ) {
//        if let Some(button) = keycode_to_button(keycode) {
//            self.board.button_released(button);
//        }
//    }
//
//    fn draw(&mut self, ctx: &mut Context) -> GameResult {
//        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
//
//        let frame = self.board.frame();
//        for (y, row) in frame.into_iter().enumerate() {
//            for (x, color) in row.into_iter().enumerate() {
//                let idx = (x + y * WIDTH) * 4;
//                self.screen[idx] = color.r;
//                self.screen[idx + 1] = color.g;
//                self.screen[idx + 2] = color.b;
//                self.screen[idx + 3] = 0xFF;
//            }
//        }
//        let image = graphics::Image::from_rgba8(ctx, WIDTH as u16, HEIGHT as u16, &self.screen)?;
//        graphics::draw(ctx, &image, self.param)?;
//
//        graphics::present(ctx)?;
//        ggez::timer::yield_now();
//        Ok(())
//    }
//}

fn load_file(path: &str) -> Vec<u8> {
    let path = PathBuf::from(path);
    let mut data = Vec::new();
    File::open(&path)
        .and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read cartridge")
        .unwrap();
    data
}

fn main() -> Result<(), String> {
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

    let mut gameboy = GameBoy::new(board);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Rostiger Spieljunge",
            (WIDTH * PIXEL_SCALE) as u32,
            (HEIGHT * PIXEL_SCALE) as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| format!("{:?}", e))?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| format!("{:?}", e))?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            WIDTH as u32,
            HEIGHT as u32,
        )
        .map_err(|e| format!("{:?}", e))?;

    let mut event_pump = sdl_context.event_pump()?;

    loop {
        let frame = gameboy.next_frame();
        texture.with_lock(None, |buffer, _| buffer.clone_from_slice(frame.as_slice()))?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                event::Event::Quit { .. } => return Ok(()),
                event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(button) = keycode_to_button(keycode) {
                        gameboy.button_pressed(button);
                    }
                }
                event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(button) = keycode_to_button(keycode) {
                        gameboy.button_released(button);
                    }
                }
                _ => (),
            }
        }
    }
}
