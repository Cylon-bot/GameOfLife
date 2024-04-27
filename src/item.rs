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
    pub fn create_all_from_grid(box_righ: Coordonate) -> Vec<Pixel> {
        let number_of_pixels: u32 = box_righ.x as u32 * box_righ.y as u32;

        let mut all_pixels = vec![];
        let mut pixel_to_create: u32 = 0;
        while pixel_to_create < number_of_pixels {
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
        all_pixels
    }
}
#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_coordonate: BoxGame,
    pub is_alive: bool,
    // neighboors: [Box<Cell>;8]
}

impl Cell {
    pub fn new(cell_coordonate: BoxGame) -> Self {
        Cell {
            cell_coordonate,
            is_alive: false,
            // neighboors,
        }
    }

    pub fn create_all_from_grid(
        column_number: u16,
        line_number: u16,
        grid_thickness: u16,
        all_pixels: &'static Vec<Pixel>,
    ) -> Vec<Cell> {
        let mut all_cell: Vec<Cell> = vec![];
        let mut iteration_loop_x = 0;
        let mut iteration_loop_y = 0;
        let mut all_box_cell: Vec<BoxGame> = vec![];
        let modulo_for_column = column_number + ((column_number + 1) * grid_thickness);
        let modulo_for_line = line_number + ((line_number + 1) * grid_thickness);
        loop {
            let x = modulo_for_column * iteration_loop_x;
            let y = modulo_for_line * iteration_loop_y;
            all_box_cell.push(BoxGame::new(
                x + grid_thickness,
                y + grid_thickness,
                x + modulo_for_column - grid_thickness,
                y + modulo_for_line - grid_thickness,
                all_pixels,
            ));
            if iteration_loop_x == column_number {
                if iteration_loop_y == line_number {
                    break;
                }
                iteration_loop_x = 0;
                iteration_loop_y += 1;
            } else {
                iteration_loop_x += 1;
            }
        }
        for box_cell in all_box_cell {
            all_cell.push(Cell::new(box_cell))
        }
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
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16, all_pixels: &Vec<Pixel>) -> Self {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        let top_left = Coordonate::new(x_min, y_min);
        let bot_righ = Coordonate::new(x_max, y_max);
        let mut pixels_associated_id: Vec<usize> = vec![];
        for (key, pixel) in all_pixels.iter().enumerate() {
            if pixel.coordonate.x >= top_left.x
                && pixel.coordonate.x <= bot_righ.x
                && pixel.coordonate.y >= top_left.y
                && pixel.coordonate.y <= bot_righ.y
            {
                pixels_associated_id.push(key);
            }
        }
        BoxGame {
            top_left: Coordonate::new(x_min, y_min),
            bot_righ: Coordonate::new(x_max, y_max),
            size: x2 as usize * y2 as usize,
            number_pixel_width: x_max - x_min + 1,
            number_pixel_height: y_max - y_min + 1,
            pixels_associated_id,
        }
    }
}
