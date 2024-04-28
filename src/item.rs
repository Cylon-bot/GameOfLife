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
pub struct Cell<'a> {
    pub cell_coordonate: BoxGame,
    pub is_alive: bool,
    pub neighboors: Option<Vec<&'a Cell<'a>>>,
}

impl<'a> Cell<'a> {
    pub fn new(cell_coordonate: BoxGame) -> Self {
        Cell {
            cell_coordonate,
            is_alive: false,
            neighboors: None,
        }
    }

    pub fn create_all_from_grid(grid: &GridCreation, all_pixels: &Vec<Pixel>) -> Vec<Cell<'a>> {
        let mut all_cell: Vec<Cell> = vec![];
        let mut map_pixel_to_cell = HashMap::new();

        // create a map to associated a Vec of Pixel to a Cell id
        for pixel in all_pixels {
            let x = pixel.coordonate.x;
            let y = pixel.coordonate.y;
            map_pixel_to_cell
                .entry((
                    ((x as f32 / grid.size_cell_column.unwrap() as f32).floor() as u32),
                    ((y as f32 / grid.size_cell_line.unwrap() as f32).floor() as u32),
                ))
                .or_insert_with(Vec::new)
                .push(pixel.id as usize);
        }

        // create all Cell struct using the map created before
        for (cell_id, cell_map) in map_pixel_to_cell {
            let cell: Cell = Cell::new(BoxGame::new(
                cell_id.0 as u16,
                cell_id.1 as u16,
                cell_id.0 as u16 + grid.size_cell_column.unwrap(),
                cell_id.1 as u16 + grid.size_cell_line.unwrap(),
                cell_map,
            ));
            all_cell.push(cell);
        }
        let mut cell_neighboors = vec![];

        // iter over all_cell and create a map to identify all neighboors of each cell
        for cell in &all_cell {
            let neighboors = [
                (
                    cell.cell_coordonate.top_left.x - grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y - grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x + grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y + grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x - grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y + grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x + grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y - grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x,
                    cell.cell_coordonate.top_left.y - grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x - grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y,
                ),
                (
                    cell.cell_coordonate.top_left.x,
                    cell.cell_coordonate.top_left.y + grid.size_cell_column.unwrap(),
                ),
                (
                    cell.cell_coordonate.top_left.x + grid.size_cell_column.unwrap(),
                    cell.cell_coordonate.top_left.y,
                ),
            ];
            cell_neighboors.push((cell, neighboors));
        }

        // // iter over the map created before to modify the cell.neighboors field (cannot do it inside the for before because i need to access to all_cell inside the for already itering on it)
        // for (cell, neigboors_coordonate) in cell_neighboors {
        //     let mut neigboors_cell = vec![];
        //     for neighboor_coordonate in neigboors_coordonate.iter() {
        //         if let Some(cell_neighboor) = &all_cell.iter().find(|c: &&Cell| {
        //             c.cell_coordonate.top_left.x == neighboor_coordonate.0
        //                 && c.cell_coordonate.top_left.y == neighboor_coordonate.1
        //         }) {
        //             neigboors_cell.push(*cell_neighboor)
        //         }
        //     }
        //     cell.neighboors = Some(neigboors_cell);
        // }
        // all_cell.clone()
        all_cell
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
