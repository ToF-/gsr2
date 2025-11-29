use crate::model::color::Color;

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

    pub fn from_string(s: &str) -> Self {
        Self::default()
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
        let color_range = ColorRange::from_string("#0A20ff #FA4010 0.8");
        assert_eq!(
            "ColorRange { color_min: Color { r: 10, g: 32, b: 255 }, color_max: Color { r: 240, g: 64, b: 16 }, ratio: 0.8 }",
            format!("{:?}", color_range));
    }


}
