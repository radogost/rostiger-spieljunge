#![feature(exclusive_range_pattern)]

mod alu;
mod board;
mod cartridge;
mod cpu;
mod mmu;
mod ppu;
mod registers;

pub use board::Board;
pub use ppu::{Color, HEIGHT, WIDTH};
