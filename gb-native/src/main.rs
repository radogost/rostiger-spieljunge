use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[macro_use]
extern crate clap;

use sdl2::audio;
use sdl2::event;
use sdl2::keyboard;

use gb_core::{Board, Button, AUDIO_SAMPLE_RATE, HEIGHT, WIDTH};

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

    fn audio(&mut self) -> Vec<f32> {
        self.board.audio()
    }

    fn button_pressed(&mut self, button: Button) {
        self.board.button_pressed(button);
    }

    fn button_released(&mut self, button: Button) {
        self.board.button_released(button);
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
        .present_vsync()
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

    let audio_subsystem = sdl_context.audio()?;
    let desired_spec = audio::AudioSpecDesired {
        freq: Some(AUDIO_SAMPLE_RATE as i32),
        channels: Some(2),
        samples: Some(4096),
    };
    let device = audio_subsystem.open_queue(None, &desired_spec)?;
    device.resume();

    loop {
        let frame = gameboy.next_frame();
        texture.with_lock(None, |buffer, _| buffer.clone_from_slice(frame.as_slice()))?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        let audio = gameboy.audio();
        device.queue(&audio);

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
