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
    fn new(x: u16, y: u16) -> Self {
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
    pub fn create_all_from_grid(box_window: &BoxGame) -> Vec<Pixel> {
        let number_of_pixels: u32 = box_window.bot_righ.x as u32 * box_window.bot_righ.y as u32;

        let mut all_pixels = vec![];
        let mut pixel_to_create: u32 = 0;
        while pixel_to_create < number_of_pixels {
            all_pixels.push(Pixel {
                id: pixel_to_create,
                coordonate: Coordonate::new(
                    (pixel_to_create % box_window.bot_righ.x as u32) as u16,
                    (pixel_to_create / box_window.bot_righ.x as u32) as u16,
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

struct Cell {
    cell_coordonate: BoxGame,
    is_alive: bool,
    neighboors: [Box<Cell>;8]
}

#[derive(Clone, Debug)]
pub struct BoxGame {
    pub top_left: Coordonate,
    pub bot_righ: Coordonate,
    pub size: usize,
    pub number_pixel_width: u16,
    pub number_pixel_height: u16,
}

impl BoxGame {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        BoxGame {
            top_left: Coordonate::new(x_min, y_min),
            bot_righ: Coordonate::new(x_max, y_max),
            size: x2 as usize * y2 as usize,
            number_pixel_width: x_max - x_min + 1,
            number_pixel_height: y_max - y_min + 1,
        }
    }
    pub fn is_inside(&self, pixel: &Pixel) -> bool {
        pixel.coordonate.x >= self.top_left.x
            && pixel.coordonate.x <= self.bot_righ.x
            && pixel.coordonate.y >= self.top_left.y
            && pixel.coordonate.y <= self.bot_righ.y
    }
}
