use crate::model::gen_image::*;

mod cli;
mod env;
mod file;
mod model;

pub fn main() {
    save_nine_colors();
    save_large_picture();
    save_single_dot();
    save_white_square();
}
