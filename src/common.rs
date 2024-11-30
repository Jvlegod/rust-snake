use piston_window::types::Color;

pub const WIDTH_px: i32 = 800;
pub const HEIGHT_px: i32 = 600;
pub const BLOCK_SIZE: f64 = 20.0;
pub const WIDTH: i32 = WIDTH_px / BLOCK_SIZE as i32;
pub const HEIGHT: i32 = HEIGHT_px / BLOCK_SIZE as i32;

pub const TITLE: &str = "Snake";

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];

pub const FPS: f32 = 1.0 / 10.0;
pub const FLASH_TIMES: i32 = 30;

pub fn i32tof64(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}