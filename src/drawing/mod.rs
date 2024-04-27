use crate::item::{Cell, Pixel};

pub mod cell_state;
pub mod grid_game;

pub trait Drawing {
    fn draw(self, all_pixels: &mut Vec<Pixel>) -> &mut Vec<Pixel>;
}
