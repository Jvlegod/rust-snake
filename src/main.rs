use snake::snake_window::game::Game;
use snake::common::{TITLE, WIDTH_px, HEIGHT_px};

fn main() {
    let mut game = Game::new(TITLE.to_string(), WIDTH_px as u32, HEIGHT_px as u32);
    game.run();
}
