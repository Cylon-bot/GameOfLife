use std::collections::HashMap;

use crate::drawing::grid_game::GridCreation;

#[derive(Clone, Debug)]
pub struct Pixel {
    pub id: u32,
    pub coordonate: Coordonate,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug)]
pub struct Coordonate {
    pub x: u16,
    pub y: u16,
}

impl Coordonate {
    pub fn new(x: u16, y: u16) -> Self {
        Coordonate { x, y }
    }
}
impl Pixel {
    pub fn new(id: u32, coordonate: Coordonate, r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel {
            id,
            coordonate,
            r,
            g,
            b,
            a,
        }
    }
    pub fn create_all_from_grid(box_righ: Coordonate) -> (Vec<Pixel>, Vec<usize>) {
        let number_of_pixels: u32 = box_righ.x as u32 * box_righ.y as u32;

        let mut all_pixels = vec![];
        let mut pixel_to_create: u32 = 0;
        let mut id_pixels: Vec<usize> = vec![];
        while pixel_to_create < number_of_pixels {
            id_pixels.push(pixel_to_create as usize);
            all_pixels.push(Pixel {
                id: pixel_to_create,
                coordonate: Coordonate::new(
                    (pixel_to_create % box_righ.x as u32) as u16,
                    (pixel_to_create / box_righ.x as u32) as u16,
                ),
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });
            pixel_to_create += 1;
        }
        (all_pixels, id_pixels)
    }
}
#[derive(Clone, Debug)]
pub struct Cell {
    pub pixels_associated_id: Vec<usize>,
    pub cell_coordonate: (usize, usize),
    pub is_alive: bool,
}

impl Cell {
    pub fn new(pixels_associated_id: Vec<usize>, column: usize, line: usize) -> Self {
        Cell {
            pixels_associated_id,
            cell_coordonate: (column, line),
            is_alive: false,
        }
    }

    pub fn create_all_from_grid(
        grid: &GridCreation,
        all_pixels: &Vec<Pixel>,
    ) -> (Vec<Cell>, Vec<Vec<bool>>) {
        let mut all_cell: Vec<Cell> = Vec::new();
        let mut map_pixel_to_cell = HashMap::new();

        // create a map to associated a Vec of Pixel to a Cell id
        for pixel in all_pixels {
            let x = pixel.coordonate.x;
            let y = pixel.coordonate.y;
            map_pixel_to_cell
                .entry((
                    ((x as f32 / grid.size_cell_column.unwrap() as f32).floor() as usize),
                    ((y as f32 / grid.size_cell_line.unwrap() as f32).floor() as usize),
                ))
                .or_insert_with(Vec::new)
                .push(pixel.id as usize);
        }

        let cells_alive: Vec<Vec<bool>> =
            vec![vec![false; grid.line_number as usize]; grid.column_number as usize];

        // create all Cell struct using the map created before
        for (cell_id, cell_map) in map_pixel_to_cell {
            let cell: Cell = Cell::new(cell_map, cell_id.0, cell_id.1);
            all_cell.push(cell);
        }
        (all_cell, cells_alive)
    }
    pub fn is_alive(&mut self, cells_alive: &Vec<Vec<bool>>) {
        if self.cell_coordonate.0 > 0 && self.cell_coordonate.0 < cells_alive.len() - 1 {
            let all_neighboors_column =
                &cells_alive[self.cell_coordonate.0 - 1..self.cell_coordonate.0 + 2];
            if self.cell_coordonate.1 > 0 && self.cell_coordonate.1 < cells_alive[0].len() - 1 {
                let all_neighboors_left = &all_neighboors_column[0]
                    [self.cell_coordonate.1 - 1..self.cell_coordonate.1 + 2];
                let neighboor_above = &[all_neighboors_column[1][self.cell_coordonate.1 - 1]];
                let neighboor_below = &[all_neighboors_column[1][self.cell_coordonate.1 + 1]];
                let all_neighboors_right = &all_neighboors_column[2]
                    [self.cell_coordonate.1 - 1..self.cell_coordonate.1 + 2];

                let all_neighboors = [
                    neighboor_above,
                    neighboor_below,
                    all_neighboors_right,
                    all_neighboors_left,
                ];
                let mut count_neighboors_alive = 0;
                for neighs in all_neighboors.iter() {
                    for neigh_solo in neighs.iter() {
                        if *neigh_solo {
                            count_neighboors_alive += 1;
                        }
                    }
                }
                if self.is_alive {
                    if count_neighboors_alive < 2 {
                        self.is_alive = false
                    }
                    if count_neighboors_alive == 2 || count_neighboors_alive == 3 {
                        self.is_alive = true
                    }
                    if count_neighboors_alive > 3 {
                        self.is_alive = false
                    }
                } else if count_neighboors_alive == 3 {
                    self.is_alive = true
                }
            } else {
                self.is_alive = false;
            }
        } else {
            self.is_alive = false;
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxGame {
    pub top_left: Coordonate,
    pub bot_righ: Coordonate,
    pub size: usize,
    pub number_pixel_width: u16,
    pub number_pixel_height: u16,
    pub pixels_associated_id: Vec<usize>,
}

impl BoxGame {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16, pixels_associated_id: Vec<usize>) -> Self {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        let top_left = Coordonate::new(x_min, y_min);
        let bot_righ = Coordonate::new(x_max, y_max);
        BoxGame {
            top_left,
            bot_righ,
            size: x2 as usize * y2 as usize,
            number_pixel_width: x_max - x_min + 1,
            number_pixel_height: y_max - y_min + 1,
            pixels_associated_id,
        }
    }
}
