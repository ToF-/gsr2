use image::DynamicImage;
use image::GenericImageView;
use crate::file::picture_file::get_image_from_picture_file;
use std::num::ParseIntError;
use crate::model::color::Color;
use std::fmt;

#[derive(Debug)]
pub struct ParseColorRangeError;

impl fmt::Display for ParseColorRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing color range")
    }
}

impl std::error::Error for ParseColorRangeError {}

#[derive(Debug, Clone, Default)]
pub struct ColorRange {
    pub color_min: Color,
    pub color_max: Color,
    pub ratio: f64,
}

impl ColorRange {
    pub fn default() -> Self {
        ColorRange {
            color_min: Color::default(),
            color_max: Color {
                r: 255,
                g: 255,
                b: 255,
            },
            ratio: 1.0,
        }
    }

    pub fn from_string(s: &str) -> Result<Self, ParseColorRangeError> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.len() {
            3 => match Color::from_string(parts[0]) {
                Ok(color_min) => match Color::from_string(parts[1]) {
                    Ok(color_max) => match parts[2].parse::<f64> () {
                        Ok(ratio) => Ok(ColorRange {
                            color_min,
                            color_max,
                            ratio,
                        }),
                        Err(_) => Err(ParseColorRangeError),
                    },
                    Err(_) => Err(ParseColorRangeError),
                },
                Err(_) => Err(ParseColorRangeError),
            },
            _ => Err(ParseColorRangeError),
        }
    }

    pub fn matches(&self, file_path: &str) -> bool {
        let red_range = self.color_min.r..=self.color_max.r;
        let green_range = self.color_min.g..=self.color_max.g;
        let blue_range = self.color_min.b..=self.color_max.b;
        match get_image_from_picture_file(file_path) {
            Ok(image) => {
                let mut count: u32 = 0;
                let total: u32 = image.width() * image.height();
                for y in 0..image.height() {
                    for x in 0..image.width() {
                        let pixel = image.get_pixel(x, y);
                        let r = pixel[0];
                        let g = pixel[1];
                        let b = pixel[2];
                        if red_range.contains(&r)
                            && green_range.contains(&g)
                                && blue_range.contains(&b) {
                                    count+= 1
                        };
                    }
                }
                let ratio: f64 = (count as f64) / (total as f64);
                if ratio >= self.ratio {
                    println!("{:.1}>={:.1} : {}", ratio, self.ratio, file_path);
                };
                ratio >= self.ratio
            },
            Err(e) => {
                eprintln!("{}",e);
                false
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_range_is_all_colors_and_a_ratio_of_1() {
        let color_range = ColorRange::default();
        assert_eq!(
            "ColorRange { color_min: Color { r: 0, g: 0, b: 0 }, color_max: Color { r: 255, g: 255, b: 255 }, ratio: 1.0 }",
            format!("{:?}", color_range));
    }

    #[test]
    fn color_range_can_parse_a_string_to_initialize() {
        let color_range_opt = ColorRange::from_string("0A20ff F04010 0.8");
        assert!(color_range_opt.is_ok());
        assert_eq!(
            "ColorRange { color_min: Color { r: 10, g: 32, b: 255 }, color_max: Color { r: 240, g: 64, b: 16 }, ratio: 0.8 }",
            format!("{:?}", color_range_opt.unwrap()));
    }


}
