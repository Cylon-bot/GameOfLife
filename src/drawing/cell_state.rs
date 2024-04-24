use crate::item::{BoxGame, Cell, Pixel};

use super::Drawing;
#[derive(Copy, Clone)]
pub struct CellState {}
impl CellState {
    pub fn new() -> Self {
        CellState {}
    }
}
fn is_cell_alive(cell: &mut Cell) -> &mut Cell {
    cell.is_alive = true;
    cell
}
impl Drawing for CellState {
    fn draw(
        self,
        all_pixels: &Vec<Pixel>,
        all_cells: &Vec<Cell>,
        _loop_iteration: u32,
    ) -> (Vec<Pixel>, Vec<Cell>) {
        let mut all_cells = all_cells.clone();
        let mut all_pixels = all_pixels.clone();
        let mut iterator = 1;

        for mut cell in &mut all_cells {
            if iterator % 3 == 0 {
                cell = &mut is_cell_alive(cell);
            }
            iterator += 1;
        }

        for pixel in &mut all_pixels {
            for cell in &mut all_cells {
                if cell.is_alive {
                    pixel.r = 255;
                    pixel.g = 255;
                    pixel.b = 255;
                    pixel.a = 255;
                }
                iterator += 1
            }
        }

        (all_pixels, all_cells)
    }
}
