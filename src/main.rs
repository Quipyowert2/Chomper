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
use rand::prelude::random;

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
    size: f32,
    color: Color,
    id: i32,
    mouth_angle: f64,
    mouth_closing: bool,
    pacmen_eaten: i32
}
fn angle(x: i32,y: i32) -> f64 {
    let xf = x as f64;
    let yf = y as f64;
    return 180.0*yf.atan2(xf)/PI;
}
fn area(radius: f64) -> f64 {
    radius.powf(2.0)*PI
}
impl Pacman {
    fn animate_mouth(self: &mut Pacman) {
        if self.mouth_closing {
            self.mouth_angle += 5.0;
            if self.mouth_angle > 45.0 {
                self.mouth_closing = false;
                self.mouth_angle = 45.0 - (self.mouth_angle - 45.0);
            }
        }
        else {
            self.mouth_angle -= 5.0;
            if self.mouth_angle < 0.0 {
                self.mouth_closing = true;
                self.mouth_angle = self.mouth_angle.abs();
            }
        }
    }
    fn draw(self: Pacman, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let size_squared = self.size.powf(2.0) as i32;
        let selfx = self.x as i32;
        let selfy = self.y as i32;
        let size = self.size as i32;
        let mut lines: Vec<(Point, Point)> = vec![];
        // draw circle with a part missing
        for x in selfx-size..selfx+size {
            for y in selfy-size..selfy+size {
                if (x - selfx).pow(2)+(y - selfy).pow(2) < size_squared { //hypotenuse
                    let mut isbody: bool = false;// not mouth of pacman
                    let angle=angle(x-selfx, y-selfy);
                    match self.direction {
                    Direction::RIGHT => {
                        if angle > 45.0-self.mouth_angle || angle < -45.0+self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::LEFT => {
                        if angle < 135.0+self.mouth_angle && angle > -135.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::UP => {
                        if angle < -135.0+self.mouth_angle || angle > -45.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::DOWN => {
                        if angle < 45.0+self.mouth_angle || angle > 135.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::UPLEFT => {
                        if angle < -180.0+self.mouth_angle || angle > -90.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::UPRIGHT => {
                        if angle < -90.0+self.mouth_angle || angle > 0.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::DOWNLEFT => {
                        if angle < 90.0+self.mouth_angle || angle > 180.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    Direction::DOWNRIGHT => {
                        if angle < 0.0+self.mouth_angle || angle > 90.0-self.mouth_angle {
                            isbody = true;
                        }
                    },
                    }
                    if isbody {
                        if lines.len() > 0 {
                            let last = lines.len()-1;
                            if lines[last].1.x == x && lines[last].1.y == y-1 {
                                lines[last].1.y = y;
                            }
                            else {
                                lines.push((Point::new(x, y), Point::new(x, y)));
                            }
                        }
                        else {
                            lines.push((Point::new(x, y), Point::new(x, y)));
                        }
                    }
                }
            }
        }
        for line in &lines {
            canvas.draw_line(line.0, line.1).unwrap();
        }
    }
    fn can_chomp(&mut self, enemy: Pacman) -> bool {
        if self.size < enemy.size {
            return false;
        }
        let distance = (((self.x - enemy.x).pow(2) + (self.y - enemy.y).pow(2)) as f32).sqrt();
        return distance + enemy.size == self.size
            || distance + enemy.size < self.size;
    }
    fn calculate_new_size(self: Pacman, other: Pacman) -> f32 {
            // pi r**2 = area
            let enemy_area = ((other.size.powf(2.0) as f64)*PI) as f32;
            let self_area = ((self.size.powf(2.0) as f64)*PI) as f32;
            let combined_area = enemy_area + self_area;
            // r**2 = area/pi
            let rsquared = (combined_area as f64)/PI;
            return rsquared.sqrt() as f32;
    }
    fn player_step(&mut self, enemies: &mut Vec<Pacman>, rng: &mut ThreadRng) {
        for x in 0..enemies.len() {
            if self.can_chomp(enemies[x]) {
                self.pacmen_eaten += 1;
                self.size = self.calculate_new_size(enemies[x]);
                enemies[x] = Pacman{
                    x:rng.gen_range(0..WINDOW_WIDTH) as i32,
                    y:rng.gen_range(0..WINDOW_HEIGHT) as i32,
                    direction:random_direction(rng).unwrap(),
                    size:5.0,
                    color:random_color(rng),
                    id:(x+1) as i32,
                    mouth_closing: random(),
                    mouth_angle: random_angle(rng),
                    pacmen_eaten: 0};
            }
        }
    }
    // Returns whether the game is over.
    fn ai_step(&mut self, enemies: &mut Vec<Pacman>, player: &mut Pacman, rng: &mut ThreadRng) -> bool {
        let mut best_target: Vec<i32> = Vec::new();
        let mut nearest: usize = usize::MAX;
        let mut best_distance: f32 = -1.0;
        for x in 0..enemies.len() {
            if enemies[x].id != self.id && self.can_chomp(enemies[x]) {
                self.size = self.calculate_new_size(enemies[x]);
                enemies[x] = Pacman{
                    x:rng.gen_range(0..WINDOW_WIDTH) as i32,
                    y:rng.gen_range(0..WINDOW_HEIGHT) as i32,
                    direction:random_direction(rng).unwrap(),
                    size:5.0,
                    color:random_color(rng),
                    id:(x+1) as i32,
                    mouth_closing: random(),
                    mouth_angle: random_angle(rng),
                    pacmen_eaten: 0};
                continue;
            }
            if self.can_chomp(*player) {
                self.pacmen_eaten += 1;
                self.size = self.calculate_new_size(*player);
                println!("Player was eaten by chomper {:?}", self);
                println!("Score {} pacmen eaten: {}", area(player.size as f64) - area(40.0), player.pacmen_eaten);
                player.size = 0.0;
                return true;
                // game over
            }
            if enemies[x].id != self.id && enemies[x].size <= self.size {
                best_target.push(enemies[x].id);
            }
        }
        for x in &best_target {
            let index = (x-1) as usize;
            let enemy_distance: f32 = (((enemies[index].x - self.x).pow(2) + (enemies[index].y - self.y).pow(2)) as f32).sqrt();
            if enemy_distance < best_distance || best_distance < 0.0 {
                best_distance = enemy_distance;
                nearest = *x as usize;
            }
        }
        if nearest == usize::MAX {
            return false;
        }
        let nearest_enemy: Pacman = enemies[nearest-1];
        //println!("Pacman x={} y={} color={:?} distance={} nearest={:?}", self.x, self.y, self.color, best_distance, nearest_enemy);
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
        return false;
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
fn random_angle(rng: &mut ThreadRng) -> f64 {
    rng.gen_range(0..45) as f64
}
const NUM_ENEMIES:usize = 100;
const WINDOW_WIDTH:u32 = 800;
const WINDOW_HEIGHT:u32 = 600;
pub fn main() {
    let mut player = Pacman {x:400, y:300, direction:Direction::RIGHT, size:40.0, color:Color::RGB(255,255,0), id:0, mouth_closing: true, mouth_angle: 0.0, pacmen_eaten: 0};
    let mut rng = rand::thread_rng();

    let mut enemies: Vec<Pacman> = (0..NUM_ENEMIES).into_iter().map(|x| Pacman{
        x:rng.gen_range(0..WINDOW_WIDTH) as i32,
        y:rng.gen_range(0..WINDOW_HEIGHT) as i32,
        direction:random_direction(&mut rng).unwrap(),
        size:5.0,
        color:random_color(&mut rng),
        id:(x+1) as i32,
        mouth_closing: random(),
        mouth_angle: random_angle(&mut rng),
        pacmen_eaten: 0}).collect();

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
        // Draw pacman here.
        for x in 0..NUM_ENEMIES {
            enemies[x].animate_mouth();
            enemies[x].draw(&mut canvas);
            let mut enemy = enemies[x];
            if enemy.ai_step(&mut enemies, &mut player, &mut rng) {
                return;
            }
            enemies[x] = enemy;
            enemies[x].move_pacman();
        }
        player.animate_mouth();
        player.draw(&mut canvas);
        player.player_step(&mut enemies, &mut rng);
        player.move_pacman();

        canvas.present();
        
        frame_counter = frame_counter + 1;
        // Sleep for 1/60th of a second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
