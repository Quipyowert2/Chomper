extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;
use std::time::Duration;

#[derive(Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    UPLEFT,
    UPRIGHT,
    DOWN,
    DOWNLEFT,
    DOWNRIGHT
}
// a chomper. Size increases when eating other pacmen.
#[derive(Clone, Copy)]
struct Pacman {
    x: i32,
    y: i32,
    direction: Direction,
    size: i32,
    color: Color
}
impl Pacman {
    fn draw(self: Pacman, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
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
    fn move_pacman(&mut self) {
        match self.direction {
            Direction::RIGHT => {
                self.x = self.x + 1 }
            Direction::LEFT => {self.x = self.x - 1 }
            Direction::UP => {self.y = self.y - 1 }
            Direction::UPRIGHT => {self.y = self.y - 1; self.x = self.x + 1}
            Direction::UPLEFT => {self.y = self.y - 1;self.x = self.x - 1}
            Direction::DOWN => {self.y = self.y + 1}
            Direction::DOWNLEFT => {self.y = self.y + 1;self.x = self.x - 1}
            Direction::DOWNRIGHT => {self.y = self.y + 1;self.x = self.x + 1}
        }
    }
}
pub fn main() {
    let mut player = Pacman {x:400, y:300, direction:Direction::RIGHT, size:40, color:Color::RGB(255,255,0)};

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
        player.move_pacman();


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
