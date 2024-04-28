use crate::item::{BoxGame, Cell, Pixel};

use super::{grid_game::GridCreation, Drawing};
pub struct CellState<'a> {
    all_cells: Vec<Cell<'a>>,
}
impl<'a> CellState<'a> {
    pub fn new(my_grid: &GridCreation, all_pixels: &Vec<Pixel>) -> Self {
        let all_cells = Cell::create_all_from_grid(my_grid, all_pixels);
        CellState { all_cells }
    }
}
impl<'a> Drawing for CellState<'a> {
    fn draw(&mut self, all_pixels: &mut Vec<Pixel>) {
        let mut iterator = 1;

        for cell in &mut self.all_cells {
            if iterator % 10 == 0 {
                cell.is_alive = true;
                for pixel_id in &cell.cell_coordonate.pixels_associated_id {
                    all_pixels[*pixel_id].r = 255;
                    all_pixels[*pixel_id].g = 255;
                    all_pixels[*pixel_id].b = 255;
                    all_pixels[*pixel_id].a = 255;
                }
            }
            iterator += 1;
        }
    }
}
