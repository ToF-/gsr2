use image::DynamicImage;
use image::Rgb;
use image::RgbImage;

pub const SINGLE_DOT: &str = "testdata/single_dot.png";
pub const NINE_COLORS: &str = "testdata/nine_colors.png";
pub const WHITE_SQUARE: &str = "testdata/white_square.png";
pub const LARGE_PICTURE: &str = "testdata/large_picture.png";

pub fn large_picture() -> DynamicImage {
    let mut image = RgbImage::new(2250, 2250);
    for cx in 0..2250 {
        for cy in 0..2250 {
            let color = Rgb([
                (cx % 256) as u8,
                (255 - (cy % 256)) as u8,
                ((cx * 7 + cy * 13) % 256) as u8,
            ]);
            image.put_pixel(cx, cy, color)
        }
    }
    DynamicImage::ImageRgb8(image)
}

pub fn gen_nine_colors() -> DynamicImage {
    let mut image = RgbImage::new(900, 900);

    for cx in 0..90 {
        for cy in 0..90 {
            let color = match (cx * cy) % 10 {
                0 => Rgb([0, 0, 0]),
                1 => Rgb([238, 130, 238]),
                2 => Rgb([154, 205, 50]),
                3 => Rgb([0, 0, 255]),
                4 => Rgb([255, 0, 0]),
                5 => Rgb([255, 140, 0]),
                6 => Rgb([0, 128, 128]),
                7 => Rgb([139, 69, 19]),
                8 => Rgb([255, 255, 0]),
                _ => Rgb([128, 128, 128]),
            };
            for x in 0..10 {
                for y in 0..10 {
                    image.put_pixel(cx * 10 + x, cy * 10 + y, color)
                }
            }
        }
    }
    DynamicImage::ImageRgb8(image)
}

pub fn gen_single_dot() -> DynamicImage {
    let mut image = RgbImage::new(10, 10);
    image.put_pixel(5, 5, Rgb([255, 255, 255]));
    image::DynamicImage::ImageRgb8(image)
}

pub fn gen_white_square() -> DynamicImage {
    let range = 100..900;
    let mut image = RgbImage::new(1000, 1000);

    for x in 0..900 {
        for y in 0..900 {
            let color = if range.contains(&x) && range.contains(&y) {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            };
            image.put_pixel(x, y, color)
        }
    }
    image::DynamicImage::ImageRgb8(image)
}
