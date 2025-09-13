use image::DynamicImage;
use palette_extract::get_palette_rgb;

pub type Palette = [u32; 9];

pub fn get_palette(image: &DynamicImage) -> Palette {
    let mut palette: Palette = [0; 9];
    let pixels: &[u8] = image.as_bytes();
    let colors = get_palette_rgb(pixels);
    colors.iter().enumerate().for_each(|(i, c)| {
        palette[i] = ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32);
    });
    palette.sort();
    palette
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen_image::{NINE_COLORS, gen_nine_colors};

    #[test]
    fn extract_a_palette_of_9_most_used_colors() {
        let image: DynamicImage = gen_nine_colors();
        let palette = get_palette(&image);
        assert_eq!(263172, palette[0]);
        assert_eq!(263420, palette[1]);
        assert_eq!(296068, palette[2]);
    }
}
