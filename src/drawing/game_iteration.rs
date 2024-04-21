use crate::item::{BoxGame, Pixel};

use super::Drawing;

pub struct GameIteration {
    iteration: u32,
    box_coordonate: BoxGame,
}

impl GameIteration {
    pub fn new(box_coordonate: [(u16, u16); 2]) -> Self {
        GameIteration {
            iteration: 0,
            box_coordonate: BoxGame::new(
                box_coordonate[0].0.min(box_coordonate[1].0),
                box_coordonate[0].1.min(box_coordonate[1].1),
                box_coordonate[0].0.max(box_coordonate[1].0),
                box_coordonate[0].1.max(box_coordonate[1].1),
            ),
        }
    }
}

impl Drawing for GameIteration {
    fn draw(&self, all_pixels: &Vec<Pixel>, loop_iteration: u32) -> Vec<Pixel> {
        let mut all_pixels = all_pixels.clone();
        for pixel in &mut all_pixels {
            if self.box_coordonate.is_inside(pixel) {
                pixel.r = (loop_iteration % 255) as u8;
                pixel.g = (loop_iteration % 255) as u8;
                pixel.b = (loop_iteration % 255) as u8;
            };
        }
        all_pixels
    }
}
