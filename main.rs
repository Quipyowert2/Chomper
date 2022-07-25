extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// a chomper. Size increases when eating other pacmen.
struct Pacman {
    x: u32,
    y: u32,
    direction: u8,
    size: u8
}
pub fn main() {
    let player = Pacman {x:1, y:1, direction:1, size:1};

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
}
