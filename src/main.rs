extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;
use std::time::Duration;

// a chomper. Size increases when eating other pacmen.
#[derive(Clone, Copy)]
struct Pacman {
    x: i32,
    y: i32,
    direction: i8,
    size: i32
}
impl Pacman {
    fn draw(self: Pacman, canvas: &mut Canvas<Window>) {
        let size_squared = self.size.pow(2);
        // draw circle with a part missing
        for x in self.x-self.size..self.x+self.size {
            for y in self.y-self.size..self.y+self.size {
                if (x - self.x).pow(2)+(y - self.y).pow(2) < size_squared {
                    canvas.draw_point(Point::new(x, y));
                }
            }
        }
    }
}
pub fn main() {
    let player = Pacman {x:1, y:1, direction:1, size:1};

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // Draw pacman here.
        player.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
