use image::{DynamicImage, Rgb};
use palette_extract::get_palette_rgb;
use std::cmp::Ordering;

pub type Rgb8 = Rgb<u8>;
pub type Palette = [Rgb8; 9];

fn compare_rgb(color: &Rgb8, other: &Rgb8) -> Ordering {
    match color[0].cmp(&other[0]) {
        Ordering::Equal => match color[1].cmp(&other[1]) {
            Ordering::Equal => color[2].cmp(&other[2]),
            res => res,
        },
        res => res,
    }
}

pub fn get_palette(image: &DynamicImage) -> Palette {
    let mut palette: Palette = [Rgb([0, 0, 0]); 9];
    let pixels: &[u8] = image.as_bytes();
    let colors = get_palette_rgb(pixels);
    colors.iter().enumerate().for_each(|(i, c)| {
        palette[i] = Rgb([c.r, c.g, c.b]);
    });
    palette.sort_by(|a, b| compare_rgb(&a, &b));
    palette
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen_image::gen_nine_colors;

    #[test]
    fn extract_a_palette_of_9_most_used_colors() {
        let image: DynamicImage = gen_nine_colors();
        let palette = get_palette(&image);
        assert_eq!(Rgb([4, 4, 4]), palette[0]);
    }
}
