use crate::item::Pixel;

pub mod game_iteration;

pub trait Drawing {
    fn draw(&self, all_pixels: &Vec<Pixel>, loop_iteration: u32) -> Vec<Pixel>;
}
