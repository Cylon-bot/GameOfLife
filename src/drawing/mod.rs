use crate::item::Pixel;

pub mod game_iteration;
pub mod grid_game;
pub mod cell_state;

pub trait Drawing {
    fn draw(&self, all_pixels: &Vec<Pixel>, loop_iteration: u32) -> Vec<Pixel>;
}
