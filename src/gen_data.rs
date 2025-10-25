extern crate image;
use crate::test_data::*;

mod test_data;

pub fn save_large_picture() {
    let image = large_picture();
    image.save(large_picture_file_path()).unwrap();
}

pub fn save_nine_colors() {
    let image = gen_nine_colors();
    image.save(nine_colors_file_path()).unwrap();
}

pub fn save_single_dot() {
    let image = gen_single_dot();
    image.save(single_dot_file_path()).unwrap();
}

pub fn save_white_square() {
    let image = gen_white_square();
    image.save(white_square_file_path()).unwrap();
}

pub fn main() {
    save_large_picture();
    save_nine_colors();
    save_single_dot();
    save_white_square();
}
