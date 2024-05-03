use crate::item::{BoxGame, Pixel};

use super::Drawing;

pub struct GridCreation {
    box_coordonate: BoxGame,
    pub column_number: u16,
    pub line_number: u16,
    pub grid_thickness: u16,
    pub size_cell_column: Option<u16>,
    pub size_cell_line: Option<u16>,
}
impl GridCreation {
    pub fn new(
        box_coordonate: BoxGame,
        column_number_appoximation: u16,
        line_number_appoximation: u16,
        grid_thickness: u16,
    ) -> Self {
        GridCreation {
            box_coordonate,
            column_number: column_number_appoximation,
            line_number: line_number_appoximation,
            grid_thickness,
            size_cell_column: None,
            size_cell_line: None,
        }
    }
}

fn determine_line_column_grid(
    wanted_number: u16,
    max_number_pixel: u16,
    grid_thickness: u16,
) -> Option<u16> {
    let mut real_number = 1;

    let real_number_of_pixel = max_number_pixel;
    while (real_number_of_pixel % (real_number + ((real_number + 1) * grid_thickness))) > 10
        || real_number_of_pixel / (real_number + ((real_number + 1) * grid_thickness))
            > wanted_number
    {
        real_number += 1;
    }
    Some(real_number + ((real_number + 1) * grid_thickness))
}

fn fill_grid(
    pixel: &Pixel,
    modulo: u16,
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
    if coordonate_to_check % modulo == 0 {
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
impl Drawing for GridCreation {
    fn draw(&mut self, all_pixels: &mut Vec<Pixel>) {
        self.size_cell_column = determine_line_column_grid(
            self.column_number,
            self.box_coordonate.number_pixel_width,
            self.grid_thickness,
        );
        self.column_number =
            (self.box_coordonate.number_pixel_width + 1) / self.size_cell_column.unwrap();

        self.size_cell_line = determine_line_column_grid(
            self.line_number,
            self.box_coordonate.number_pixel_height,
            self.grid_thickness,
        );

        self.line_number =
            (self.box_coordonate.number_pixel_height + 1) / self.size_cell_line.unwrap();

        let mut fill_thickness_x: u16 = 0;
        let mut fill_thickness_y: u16 = 0;

        for pixel in &mut *all_pixels {
            (*pixel, fill_thickness_x) = fill_grid(
                pixel,
                self.size_cell_column.unwrap(),
                self.grid_thickness,
                pixel.coordonate.x,
                fill_thickness_x,
            );
            (*pixel, fill_thickness_y) = fill_grid(
                pixel,
                self.size_cell_line.unwrap(),
                self.grid_thickness,
                pixel.coordonate.y,
                fill_thickness_y,
            );
        }
        println!("number of column --> {}", self.column_number);
        println!("number of line --> {}", self.line_number);
    }
}
