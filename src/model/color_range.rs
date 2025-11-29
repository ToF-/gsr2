use std::fmt::Display;
use palette_extract::Color;

#[derive(Debug, Clone, Default)]
pub struct ColorRange {
    pub color_min: Color,
    pub color_max: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_range_is_all_colors_and_a_ratio_of_1() {
        let color_range = ColorRange::default();
        assert_eq!("#000000;#ffffff;1.0", color_range.to_string());

    }


}
