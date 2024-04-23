use crate::item::{BoxGame, Cell, Pixel};

use super::Drawing;

pub struct CellState {
    iteration: u32,
    box_coordonate: BoxGame,
}

impl Drawing for CellState {
    fn draw(
        self,
        _all_pixels: &Vec<Pixel>,
        all_cells: &Vec<Cell>,
        _loop_iteration: u32,
    ) -> (Vec<Pixel>, Vec<Cell>) {
        let mut all_cells = all_cells.clone();
        let mut iterator = 1;
        for cell in &mut all_cells {
            if iterator % 3 == 0 {
                cell.is_alive = true;
            }
            iterator += 1
        }
        (_all_pixels.to_vec(), all_cells)
    }
}
