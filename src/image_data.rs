use crate::default_values::MAX_PALETTE_COLORS;
use image::{DynamicImage, Rgb};
use palette_extract::{MaxColors, PixelEncoding, PixelFilter, Quality, get_palette_with_options};
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
    let colors = get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::new(5),
        MaxColors::new(MAX_PALETTE_COLORS + 1),
        PixelFilter::None,
    );
    eprint!("{} {:?}", colors.len(), colors);
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
        assert_eq!(Rgb([4, 4, 252]), palette[1]);
        assert_eq!(Rgb([4, 132, 132]), palette[2]);
        assert_eq!(Rgb([136, 100, 76]), palette[3]);
        assert_eq!(Rgb([156, 204, 52]), palette[4]);
    }
}
// [Color { r: 252, g: 4, b: 4, hex: "#FC0404" }, , Color { r: 236, g: 132, b: 236, hex: "#EC84EC" }, Color { r: 252, g: 140, b: 4, hex: "#FC8C04" }, Color { r: 252, g: 252, b: 4, hex: "#FCFC04" }, }]
