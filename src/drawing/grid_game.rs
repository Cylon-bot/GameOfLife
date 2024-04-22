use crate::item::{BoxGame, Pixel};

use super::Drawing;

pub struct GridCreation<'a> {
    box_coordonate: &'a BoxGame,
    column_number_appoximation: u16,
    line_number_appoximation: u16,
    grid_thickness: u16,
}
impl<'a> GridCreation<'a> {
    pub fn new(
        box_coordonate: &'a BoxGame,
        column_number_appoximation: u16,
        line_number_appoximation: u16,
        grid_thickness: u16,
    ) -> Self {
        GridCreation {
            box_coordonate,
            column_number_appoximation,
            line_number_appoximation,
            grid_thickness,
        }
    }
}

fn determine_line_column_grid(
    number_wanted: u16,
    max_number_pixel: u16,
    grid_thickness: u16,
) -> u16 {
    let mut real_number = 1;
    while max_number_pixel % (real_number + ((real_number + 1) * grid_thickness)) != 0
        && max_number_pixel / (real_number + ((real_number + 1) * grid_thickness)) >= number_wanted
    {
        real_number += 1;
    }
    real_number
}
fn fill_grid(
    pixel: &Pixel,
    number_grid: u16,
    grid_thickness: u16,
    coordonate_to_check: u16,
    fill_thickness: u16,
) -> (Pixel, u16) {
    let mut fill_thickness = fill_thickness;
    let mut my_new_pixel = Pixel::new(
        pixel.id,
        pixel.coordonate.clone(),
        pixel.r,
        pixel.g,
        pixel.b,
        pixel.a,
    );
    if coordonate_to_check % (number_grid + ((number_grid + 1) * grid_thickness)) == 0 {
        my_new_pixel.r = 255;
        my_new_pixel.g = 255;
        my_new_pixel.b = 255;
        my_new_pixel.a = 80;
        fill_thickness = 1;
    } else if fill_thickness > 0 {
        my_new_pixel.r = 255;
        my_new_pixel.g = 255;
        my_new_pixel.b = 255;
        my_new_pixel.a = 80;
        fill_thickness += 1;
    }
    if fill_thickness == grid_thickness {
        fill_thickness = 0
    }
    (my_new_pixel, fill_thickness)
}
impl<'a> Drawing for GridCreation<'a> {
    fn draw(&self, all_pixels: &Vec<Pixel>, _loop_iteration: u32) -> Vec<Pixel> {
        let mut column_number = self.column_number_appoximation;

        column_number = determine_line_column_grid(
            column_number,
            self.box_coordonate.number_pixel_width,
            self.grid_thickness,
        );

        let mut line_number = self.line_number_appoximation;
        line_number = determine_line_column_grid(
            line_number,
            self.box_coordonate.number_pixel_height,
            self.grid_thickness,
        );
        let mut all_pixels = all_pixels.clone();
        let mut fill_thickness_x: u16 = 0;
        let mut fill_thickness_y: u16 = 0;
        for pixel in &mut all_pixels {
            (*pixel, fill_thickness_x) = fill_grid(
                pixel,
                column_number,
                self.grid_thickness,
                pixel.coordonate.x,
                fill_thickness_x,
            );
            (*pixel, fill_thickness_y) = fill_grid(
                pixel,
                line_number,
                self.grid_thickness,
                pixel.coordonate.y,
                fill_thickness_y,
            );
        }
        all_pixels
    }
}
