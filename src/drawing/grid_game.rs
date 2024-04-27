use crate::item::{BoxGame, Cell, Pixel};

use super::Drawing;

#[derive(Copy, Clone)]
pub struct GridCreation<'a> {
    box_coordonate: &'a BoxGame,
    pub column_number: u16,
    pub line_number: u16,
    pub grid_thickness: u16,
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
            column_number: column_number_appoximation,
            line_number: line_number_appoximation,
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
    let modulo = max_number_pixel - 1;
    while modulo % (real_number + ((real_number + 1) * grid_thickness)) != 0
        && modulo / (real_number + ((real_number + 1) * grid_thickness)) >= number_wanted
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
    if fill_thickness == grid_thickness {
        fill_thickness = 0
    }
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

    (my_new_pixel, fill_thickness)
}
impl<'a> Drawing for GridCreation<'a> {
    fn draw(mut self, all_pixels: &mut Vec<Pixel>) -> &mut Vec<Pixel> {
        self.column_number = determine_line_column_grid(
            self.column_number,
            self.box_coordonate.number_pixel_width,
            self.grid_thickness,
        );
        self.line_number = determine_line_column_grid(
            self.line_number,
            self.box_coordonate.number_pixel_height,
            self.grid_thickness,
        );

        let mut fill_thickness_x: u16 = 0;
        let mut fill_thickness_y: u16 = 0;

        for pixel in &mut *all_pixels {
            (*pixel, fill_thickness_x) = fill_grid(
                pixel,
                self.column_number,
                self.grid_thickness,
                pixel.coordonate.x,
                fill_thickness_x,
            );
            (*pixel, fill_thickness_y) = fill_grid(
                pixel,
                self.line_number,
                self.grid_thickness,
                pixel.coordonate.y,
                fill_thickness_y,
            );
        }
        all_pixels
    }
}
