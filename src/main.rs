extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::ttf;
use std::time::{Duration, Instant};
use std::f64::consts::PI;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::prelude::random;
use std::convert::TryInto;

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
fn abs(x: i32) -> i32 {
    if x >= 0 {
        return x;
    }
    else {
        return -x;
    }
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
        for x in selfx-size..=selfx+size {
            for y in selfy-size..=selfy+size {
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
        let (enemyx, enemyy) = self.wraparound(enemy.x, enemy.y, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        let distance = (((self.x - enemyx).pow(2) + (self.y - enemyy).pow(2)) as f32).sqrt();
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
    fn wraparound(self: Pacman, x: i32, y: i32, window_width: i32, window_height: i32) -> (i32, i32) {
        let mut wrapx = x;
        let mut wrapy = y;
            if abs(wrapx + window_width - self.x) < abs(wrapx - self.x) {
                wrapx += window_width;
            }
            else if abs(wrapx - window_width - self.x) < abs(wrapx - self.x) {
                wrapx -= window_width;
            }
            if abs(wrapy + window_height - self.x) < abs(wrapy - self.y) {
                wrapy += window_height;
            }
            else if abs(wrapy - window_height - self.x) < abs(wrapy - self.y) {
                wrapy -= window_height;
            }
        return (wrapx, wrapy);
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
        let window_width = WINDOW_WIDTH as i32;
        let window_height = WINDOW_HEIGHT as i32;
        for x in 0..enemies.len() {
            if enemies[x].id != self.id && self.can_chomp(enemies[x]) {
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
                continue;
            }
            if self.can_chomp(*player) {
                self.pacmen_eaten += 1;
                self.size = self.calculate_new_size(*player);
                println!("Player was eaten by chomper {:?}", self);
                println!("Score {} pacmen eaten: {}", player.score(), player.pacmen_eaten);
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

            // Wraparound at edges of screen
            let (wrapx, wrapy) = self.wraparound(enemies[index].x, enemies[index].y, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
            let enemy_distance: f32 = (((wrapx - self.x).pow(2) + (wrapy - self.y).pow(2)) as f32).sqrt();
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
        let (wrapx, wrapy) = self.wraparound(nearest_enemy.x, nearest_enemy.y, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        if wrapx < self.x {
            if wrapy < self.y {
                self.direction = Direction::UPLEFT;
            }
            else if wrapy == self.y {
                self.direction = Direction::LEFT;
            }
            else {
                self.direction = Direction::DOWNLEFT;
            }
        }
        else if wrapx == self.x {
            if wrapy < self.y {
                self.direction = Direction::UP;
            }
            else if wrapy == self.y {
                // ???
            }
            else {
                self.direction = Direction::DOWN;
            }
        }
        else {// enemy.x > self.x
            if wrapy < self.y {
                self.direction = Direction::UPRIGHT;
            }
            else if wrapy == self.y {
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

        // Wraparound at edges of screen
        let width: i32 = WINDOW_WIDTH.try_into().unwrap();
        let height: i32 = WINDOW_HEIGHT.try_into().unwrap();
        if self.x < 0 || self.x > width {
            self.x = width - self.x;
        }
        if self.y < 0 || self.y > height {
            self.y = height - self.y;
        }
    }
    fn score(self: Pacman) -> i64 {
        (area(self.size as f64) - area(INITIAL_PLAYER_SIZE)) as i64
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
const INITIAL_PLAYER_SIZE:f64 = 40.0;
pub fn main() {
    let mut player = Pacman {x:400, y:300, direction:Direction::RIGHT, size:INITIAL_PLAYER_SIZE as f32, color:Color::RGB(255,255,0), id:0, mouth_closing: true, mouth_angle: 0.0, pacmen_eaten: 0};
    let mut rng = rand::thread_rng();

    let version = option_env!("CARGO_PKG_VERSION").unwrap();

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
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let title = format!("{} {}", "Chomper", version);
    let window = video_subsystem.window(&title, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Helvetica for Linux, Arial for Windows, ??? for macOS
    let font = ttf_context.load_font("C:\\Windows\\Fonts\\arial.ttf", 12).unwrap();
    let tc = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut left_pressed: bool = false;
    let mut up_pressed: bool = false;
    let mut right_pressed: bool = false;
    let mut down_pressed: bool = false;

    let mut frame_counter: i64 = 0;
    let mut fps_enabled = false;
    let mut fps_calculated_counter = 0; // fps counter when last calculated

    let mut now = Instant::now();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F7), ..} => {
                    fps_enabled = !fps_enabled;
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

        // Draw score on screen
        let score_text = format!("{}{}", "Score: ", player.score());
        let partial = font.render(&score_text);
        let (fontwidth, fontheight) = font.size_of(&score_text).unwrap();
        let result = partial.solid(Color::RGB(255, 255, 255)).unwrap();
        let tex = tc.create_texture_from_surface(result).unwrap();
        canvas.copy(&tex, None, Rect::new((WINDOW_WIDTH-fontwidth) as i32, 0, fontwidth, fontheight)).unwrap();

        let elapsed = now.elapsed();
        let seconds = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        let fps = (frame_counter - fps_calculated_counter) as f64 / seconds;
        if elapsed > Duration::from_secs(1) {
            now = Instant::now();
            fps_calculated_counter = frame_counter;
        }

        if fps_enabled {
            let fps_text = format!("{}{}", "FPS: ", fps as i64);
            let partial = font.render(&fps_text);
            let (fontwidth, fontheight) = font.size_of(&fps_text).unwrap();
            let result = partial.solid(Color::RGB(255, 255, 255)).unwrap();
            let tex = tc.create_texture_from_surface(result).unwrap();
            canvas.copy(&tex, None, Rect::new(0, 0, fontwidth, fontheight)).unwrap();
        }

        canvas.present();
        
        frame_counter = frame_counter + 1;
        // Sleep for 1/60th of a second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
