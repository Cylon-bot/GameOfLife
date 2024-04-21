#[derive(Clone, Debug)]
pub struct Pixel {
    pub coordonate: Coordonate,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug)]
pub struct Coordonate {
    x: u16,
    y: u16,
}

impl Coordonate {
    fn new(x: u16, y: u16) -> Self {
        Coordonate { x, y }
    }
}
impl Pixel {
    pub fn create_all_from_grid(box_window: &BoxGame) -> Vec<Pixel> {
        let number_of_pixels: u32 = box_window.x2 as u32 * box_window.y2 as u32;
        let mut all_pixels = vec![];
        let mut pixel_to_create: u32 = 0;
        while pixel_to_create < number_of_pixels {
            all_pixels.push(Pixel {
                coordonate: Coordonate::new(
                    (pixel_to_create % box_window.x2 as u32) as u16,
                    (pixel_to_create / box_window.x2 as u32) as u16,
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

struct Cell {}

pub struct BoxGame {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub size: usize,
}

impl BoxGame {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        BoxGame {
            x1: x1.min(x2),
            y1: y1.min(y2),
            x2: x1.max(x2),
            y2: y1.max(y2),
            size: x2 as usize * y2 as usize,
        }
    }
    pub fn is_inside(&self, pixel: &Pixel) -> bool {
        pixel.coordonate.x >= self.x1
            && pixel.coordonate.x <= self.x2
            && pixel.coordonate.y >= self.y1
            && pixel.coordonate.y <= self.y2
    }
}
