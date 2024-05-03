use crate::item::{BoxGame, Cell, Pixel};

use super::{grid_game::GridCreation, Drawing};
pub struct CellState {
    all_cells: Vec<Cell>,
    cells_alive: Vec<Vec<bool>>,
}
impl CellState {
    pub fn new(my_grid: &GridCreation, all_pixels: &mut Vec<Pixel>) -> Self {
        let (mut all_cells, mut cells_alive) = Cell::create_all_from_grid(my_grid, all_pixels);

        for (iterator, cell) in all_cells.iter_mut().enumerate() {
            if iterator % 10 == 0 {
                for pixel_id in &cell.pixels_associated_id {
                    all_pixels[*pixel_id].r = 255;
                    all_pixels[*pixel_id].g = 255;
                    all_pixels[*pixel_id].b = 255;
                    all_pixels[*pixel_id].a = 255;
                    cell.is_alive = true;
                    cells_alive[cell.cell_coordonate.0][cell.cell_coordonate.1] = cell.is_alive;
                }
            }
        }
        CellState {
            all_cells,
            cells_alive,
        }
    }
}
impl Drawing for CellState {
    fn draw(&mut self, all_pixels: &mut Vec<Pixel>) {
        for cell in &mut self.all_cells {
            cell.is_alive(&self.cells_alive);
            if cell.is_alive {
                for pixel_id in &cell.pixels_associated_id {
                    all_pixels[*pixel_id].r = 255;
                    all_pixels[*pixel_id].g = 255;
                    all_pixels[*pixel_id].b = 255;
                    all_pixels[*pixel_id].a = 255;
                }
            } else {
                for pixel_id in &cell.pixels_associated_id {
                    all_pixels[*pixel_id].r = 0;
                    all_pixels[*pixel_id].g = 0;
                    all_pixels[*pixel_id].b = 0;
                    all_pixels[*pixel_id].a = 255;
                }
            }
        }
        for cell in &self.all_cells {
            self.cells_alive[cell.cell_coordonate.0][cell.cell_coordonate.1] = cell.is_alive;
        }
    }
}
