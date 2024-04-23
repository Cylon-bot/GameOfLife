use crate::item::{Cell, Pixel};

pub mod cell_state;
pub mod grid_game;

pub trait Drawing {
    fn draw(
        self,
        all_pixels: &Vec<Pixel>,
        all_cells: &Vec<Cell>,
        loop_iteration: u32,
    ) -> (Vec<Pixel>, Vec<Cell>);
}
