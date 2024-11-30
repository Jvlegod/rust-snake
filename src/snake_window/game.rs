use crate::snake_character::character::{Snake, Food, Draw};
use crate::common::{FPS, i32tof64};
use piston_window::*;
use std::time::{Instant};

pub struct Game{
    pub window: PistonWindow,

    last_update: Instant,
    update_interval: f32
}

impl Game {
    pub fn new(title: String, width: u32, height: u32) -> Game {
        let window = WindowSettings::new(title.clone(), [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            window,
            last_update: Instant::now(),
            update_interval: FPS
        }
    }

    pub fn run(&mut self) {
        let mut snake = Snake::new();
        let mut food = Food::new();

        while let Some(e) = self.window.next() {
            let now = Instant::now();
            let delta_time = now.duration_since(self.last_update).as_secs_f32();

            if let Some(Button::Keyboard(key)) = e.press_args() {
                snake.change_direction(key);
            }
            
            if delta_time >= self.update_interval {
                if snake.head.node == food.node {
                    snake.push_back();
                    food.move_pos_now();
                }
                
                snake.move_pos();
                food.move_pos();

                if snake.died() {
                    break;
                }
                self.last_update = now;
            }

            self.window.draw_2d(&e, |c: Context, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                snake.draw(&c, g);
                food.draw(&c, g);
            });
        }
    }

    pub fn draw_rect(
        color: [f32;4],
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        con: &Context,
        g: &mut G2d,
    ) {
        let gui_x = i32tof64(x);
        let gui_y = i32tof64(y);
        let width = i32tof64(width);
        let height = i32tof64(height);
        rectangle(color, [gui_x, gui_y, width, height], con.transform, g);
    }
}