use crate::item::{BoxGame, Cell, Pixel};

use super::{grid_game::GridCreation, Drawing};
pub struct CellState {
    all_cells: Vec<Cell>,
}
impl CellState {
    pub fn new(my_grid: &GridCreation, all_pixels: &'static Vec<Pixel>) -> Self {
        let all_cells = Cell::create_all_from_grid(
            my_grid.column_number,
            my_grid.line_number,
            my_grid.grid_thickness,
            &all_pixels,
        );
        CellState { all_cells }
    }
}
impl Drawing for CellState {
    fn draw(self, all_pixels: &mut Vec<Pixel>) -> &mut Vec<Pixel> {
        let mut iterator = 1;

        for mut cell in self.all_cells {
            if iterator % 3 == 0 {
                cell.is_alive = true;
                for pixel_id in cell.cell_coordonate.pixels_associated_id {
                    all_pixels[pixel_id].r = 255;
                    all_pixels[pixel_id].g = 255;
                    all_pixels[pixel_id].b = 255;
                    all_pixels[pixel_id].a = 255;
                }
            }
            iterator += 1;
        }
        all_pixels
    }
}
