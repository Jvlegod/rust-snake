use crate::snake_window::game::Game;
use std::collections::LinkedList;
use rand::Rng;
use piston_window::*;
use crate::common::{RED, YELLOW, FLASH_TIMES, WHITE, WIDTH, HEIGHT};
use piston_window::types::Color;

pub trait Draw {
    fn draw(&self, c: &Context, g: &mut G2d);
}

pub struct Food {
    pub node: Point,
    pub color: Color,

    pub times: i32,
}
pub struct Snake{
    pub head: Body,
    pub body: LinkedList<Body>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    pub node: Point,
    pub direction: Direction,
}

impl Body {
    pub fn new(node: Point, direction: Direction) -> Self {
        Body { node, direction }
    }
}

impl Snake {
    pub fn new () -> Self {
        let mut rng = rand::thread_rng();
                
        let direction = rng.gen_range(0..4);
        let direction = match direction {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Up,
        };
        
        let mut head = Body::new(Point{x: rng.gen_range(0..30), y: rng.gen_range(0..30)}, direction);
        let mut body = LinkedList::new();

        Snake {
            head,
            body,
        }
    }

    pub fn died(&mut self) -> bool {
        for &node in self.body.iter() {
            if node == self.head {
                return true;
            }
        }
        false
    }

    pub fn move_pos(&mut self) {
        let head_pos = self.head.node;

        let new_head_pos = match self.head.direction {
            Direction::Up => Point { x: head_pos.x, y: (head_pos.y - 1 + HEIGHT) % HEIGHT },
            Direction::Down => Point { x: head_pos.x, y: (head_pos.y + 1) % HEIGHT },
            Direction::Left => Point { x: (head_pos.x - 1 + WIDTH) % WIDTH, y: head_pos.y },
            Direction::Right => Point { x: (head_pos.x + 1) % WIDTH, y: head_pos.y },
        };

        if !self.body.is_empty() {
            let mut prev_node = self.head.node;
            let mut current_node = self.head.node;

            for node in self.body.iter_mut() {
                current_node = node.node;
                node.node = prev_node;
                prev_node = current_node;
            }
        }

        self.head.node = new_head_pos;
    }

    pub fn push_back(&mut self) {
        if self.body.is_empty() {
            let head = self.head;
            let body_node = Body::new(head.node, self.head.direction);
            self.body.push_back(body_node);
        } else {
            let new_body = self.body.back().clone().unwrap();
            self.body.push_back(*new_body);
        }
    }

    pub fn change_direction(&mut self, key: Key) {
        let new_direction = match key {
            Key::W => Direction::Up,
            Key::A => Direction::Left,
            Key::S => Direction::Down,
            Key::D => Direction::Right,
            _ => return,
        };

        if (self.head.direction == Direction::Up && new_direction != Direction::Down)
            || (self.head.direction == Direction::Down && new_direction != Direction::Up)
            || (self.head.direction == Direction::Left && new_direction != Direction::Right)
            || (self.head.direction == Direction::Right && new_direction != Direction::Left)
        {
            self.head.direction = new_direction;
        }
    }
    

}

impl Draw for Snake {
    fn draw(&self, c: &Context, g: &mut G2d) {

        Game::draw_rect(RED, self.head.node.x, self.head.node.y, 1, 1, &c, g);

        for &node in self.body.iter() {
            let x = node.node.x;
            let y = node.node.y;

            Game::draw_rect(WHITE, x, y, 1, 1, &c, g);
        }
    }
}

impl Food {
    pub fn new () -> Self {
        let mut rng = rand::thread_rng();
                
        let node = Point{x: rng.gen_range(0..WIDTH as i32), y: rng.gen_range(0..HEIGHT as i32)};

        let color = YELLOW;

        let times = FLASH_TIMES;

        Food {
            node,
            color,
            times
        }
    }

    pub fn move_pos_now (&mut self) {
        self.times = 0;
        self.move_pos();
    }

    pub fn move_pos(&mut self) {
        let mut rng = rand::thread_rng();
        if self.times > 0 {
            self.times -= 1;
        }
        if self.times == 0 {
            self.node = Point {x: rng.gen_range(0..WIDTH as i32), y: rng.gen_range(0..HEIGHT as i32)};
            self.times = FLASH_TIMES;
        }
    }
}

impl Draw for Food {
    fn draw(&self, c: &Context, g: &mut G2d) {
        Game::draw_rect(YELLOW, self.node.x, self.node.y, 1, 1, &c, g);
    }
}