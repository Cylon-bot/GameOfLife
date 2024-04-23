use crate::item::{BoxGame, Pixel};

use super::Drawing;

pub struct CellState {
    iteration: u32,
    box_coordonate: BoxGame,
}


impl Drawing for CellState {
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