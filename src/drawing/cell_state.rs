use crate::item::{BoxGame, Cell, Coordonate, Pixel};

use super::{grid_game::GridCreation, Drawing};
pub struct CellState {
    all_cells: Vec<Cell>,
    cells_alive: Vec<Vec<bool>>,
    size_cell_column: u16,
    size_cell_line: u16,
}
impl CellState {
    pub fn new(my_grid: &GridCreation, all_pixels: &mut Vec<Pixel>) -> Self {
        let (mut all_cells, cells_alive) = Cell::create_all_from_grid(my_grid, all_pixels);
        CellState {
            all_cells,
            cells_alive,
            size_cell_column: my_grid.size_cell_column.unwrap(),
            size_cell_line: my_grid.size_cell_line.unwrap(),
        }
    }
    pub fn user_cell_interaction(
        &mut self,
        actual_position_cursor: &Coordonate,
        all_pixels: &mut Vec<Pixel>,
    ) {
        for cell in &mut self.all_cells {
            let cursor_inside = cell.is_inside(
                actual_position_cursor,
                self.size_cell_column,
                self.size_cell_line,
            );
            if cursor_inside {
                if cell.is_alive {
                    for pixel_id in &cell.pixels_associated_id {
                        all_pixels[*pixel_id].r = 0;
                        all_pixels[*pixel_id].g = 0;
                        all_pixels[*pixel_id].b = 0;
                        all_pixels[*pixel_id].a = 255;
                        cell.is_alive = false;
                    }
                } else {
                    for pixel_id in &cell.pixels_associated_id {
                        all_pixels[*pixel_id].r = 255;
                        all_pixels[*pixel_id].g = 255;
                        all_pixels[*pixel_id].b = 255;
                        all_pixels[*pixel_id].a = 255;
                        cell.is_alive = true;
                    }
                }
            }
        }
        for cell in &self.all_cells {
            self.cells_alive[cell.cell_coordonate.0][cell.cell_coordonate.1] = cell.is_alive;
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
