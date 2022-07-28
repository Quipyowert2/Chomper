extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;
use std::time::Duration;
use std::f64::consts::PI;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, Debug)]
struct Pacman {
    x: i32,
    y: i32,
    direction: Direction,
    size: i32,
    color: Color,
    id: i32
}
fn angle(x: i32,y: i32) -> f64 {
    let xf = x as f64;
    let yf = y as f64;
    return 180.0*yf.atan2(xf)/PI;
}
impl Pacman {
    fn draw(self: Pacman, canvas: &mut Canvas<Window>, draw_mouth: bool) {
        canvas.set_draw_color(self.color);
        let size_squared = self.size.pow(2);
        // draw circle with a part missing
        for x in self.x-self.size..self.x+self.size {
            for y in self.y-self.size..self.y+self.size {
                if (x - self.x).pow(2)+(y - self.y).pow(2) < size_squared { //hypotenuse
                    if draw_mouth {
                        canvas.draw_point(Point::new(x, y)).unwrap();
                        continue;
                    }
                    let mut isbody: bool = false;// not mouth of pacman
                    let angle=angle(x-self.x, y-self.y);
                    match self.direction {
                    Direction::RIGHT => {
                        if angle > 45.0 || angle < -45.0 {
                            isbody = true;
                        }
                    },
                    Direction::LEFT => {
                        if angle < 135.0 && angle > -135.0 {
                            isbody = true;
                        }
                    },
                    Direction::UP => {
                        if !(angle > -135.0 && angle < -45.0) {
                            isbody = true;
                        }
                    },
                    Direction::DOWN => {
                        if !(angle > 45.0 && angle < 135.0) {
                            isbody = true;
                        }
                    },
                    Direction::UPLEFT => {
                        if angle < -180.0 || angle > -90.0 {
                            isbody = true;
                        }
                    },
                    Direction::UPRIGHT => {
                        if angle < -90.0 || angle > 0.0 {
                            isbody = true;
                        }
                    },
                    Direction::DOWNLEFT => {
                        if angle < 90.0 || angle > 180.0 {
                            isbody = true;
                        }
                    },
                    Direction::DOWNRIGHT => {
                        if angle < 0.0 || angle > 90.0 {
                            isbody = true;
                        }
                    },
                    }
                    if isbody {
                        canvas.draw_point(Point::new(x, y)).unwrap();
                    }
                }
            }
        }
    }
    fn ai_step(&mut self, enemies: &Vec<Pacman>) {
        let mut best_target: Vec<Pacman> = Vec::new();
        let mut nearest: usize = usize::MAX;
        let mut best_distance: f32 = -1.0;
        for x in 0..enemies.len() {
            if enemies[x].id != self.id && enemies[x].size <= self.size {
                best_target.push(enemies[x]);
            }
        }
        for x in 0..best_target.len() {
            let enemy_distance: f32 = (((enemies[x].x - self.x).pow(2) + (enemies[x].y - self.y).pow(2)) as f32).sqrt();
            if enemy_distance < best_distance || best_distance < 0.0 {
                best_distance = enemy_distance;
                nearest = x;
            }
        }
        if nearest == usize::MAX {
            return;
        }
        let nearest_enemy: Pacman = best_target[nearest];
        println!("Pacman color={:?} distance={} nearest={:?}", self.color, best_distance, nearest_enemy);
        if nearest_enemy.x < self.x {
            if nearest_enemy.y < self.y {
                self.direction = Direction::UPLEFT;
            }
            else if nearest_enemy.y == self.y {
                self.direction = Direction::LEFT;
            }
            else {
                self.direction = Direction::DOWNLEFT;
            }
        }
        else if nearest_enemy.x == self.x {
            if nearest_enemy.y < self.y {
                self.direction = Direction::UP;
            }
            else if nearest_enemy.y == self.y {
                // ???
            }
            else {
                self.direction = Direction::DOWN;
            }
        }
        else {// enemy.x > self.x
            if nearest_enemy.y < self.y {
                self.direction = Direction::UPRIGHT;
            }
            else if nearest_enemy.y == self.y {
                self.direction = Direction::RIGHT;
            }
            else {
                self.direction = Direction::DOWNRIGHT;
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
fn random_direction(rng: &mut ThreadRng) -> Option<Direction> {
    let dir: u8 = rng.gen_range(1..8);
    match dir {
        1 => {return Some(Direction::LEFT)},
        2 => {return Some(Direction::RIGHT)},
        3 => {return Some(Direction::UP)},
        4 => {return Some(Direction::UPLEFT)},
        5 => {return Some(Direction::UPRIGHT)},
        6 => {return Some(Direction::DOWN)},
        7 => {return Some(Direction::DOWNLEFT)},
        8 => {return Some(Direction::DOWNRIGHT)},
        _ => {return None}
    }
}
fn random_color(rng: &mut ThreadRng) -> Color {
    let red: u8 = rng.gen();
    let green: u8 = rng.gen();
    let blue: u8 = rng.gen();
    return Color::RGB(red, green, blue);
}
pub fn main() {
    let mut player = Pacman {x:400, y:300, direction:Direction::RIGHT, size:40, color:Color::RGB(255,255,0), id:0};
    const NUM_ENEMIES:usize = 100;
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;
    let mut rng = rand::thread_rng();

    let mut enemies: Vec<Pacman> = (0..NUM_ENEMIES).into_iter().map(|x| Pacman{
        x:rng.gen_range(0..WINDOW_WIDTH) as i32,
        y:rng.gen_range(0..WINDOW_HEIGHT) as i32,
        direction:random_direction(&mut rng).unwrap(),
        size:5,
        color:random_color(&mut rng),
        id:(x+1) as i32}).collect();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut left_pressed: bool = false;
    let mut up_pressed: bool = false;
    let mut right_pressed: bool = false;
    let mut down_pressed: bool = false;
    let mut draw_mouth: bool = false;

    let mut frame_counter: i64 = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    left_pressed = true;
                    if up_pressed {
                        player.direction = Direction::UPLEFT;
                    }
                    else if down_pressed {
                        player.direction = Direction::DOWNLEFT;
                    }
                    else {
                        player.direction = Direction::LEFT;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    right_pressed = true;
                    if up_pressed {
                        player.direction = Direction::UPRIGHT;
                    }
                    else if down_pressed {
                        player.direction = Direction::DOWNRIGHT;
                    }
                    else {
                        player.direction = Direction::RIGHT;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    up_pressed = true;
                    if left_pressed {
                        player.direction = Direction::UPLEFT;
                    }
                    else if right_pressed {
                        player.direction = Direction::UPRIGHT;
                    }
                    else {
                        player.direction = Direction::UP;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    down_pressed = true;
                    if left_pressed {
                        player.direction = Direction::DOWNLEFT;
                    }
                    else if right_pressed {
                        player.direction = Direction::DOWNRIGHT;
                    }
                    else {
                        player.direction = Direction::DOWN;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Left), ..} => {
                    left_pressed = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), ..} => {
                    right_pressed = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), ..} => {
                    up_pressed = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
                    down_pressed = false;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if frame_counter % 15 == 0 {
            draw_mouth = !draw_mouth;
        }
        // Draw pacman here.
        for x in 0..NUM_ENEMIES {
            enemies[x].draw(&mut canvas, draw_mouth);
            let mut enemy = enemies[x];
            enemy.ai_step(&mut enemies);
            enemies[x].move_pacman();
        }
        player.draw(&mut canvas, draw_mouth);
        player.move_pacman();

        canvas.present();
        
        frame_counter = frame_counter + 1;
        // Sleep for 1/60th of a second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
