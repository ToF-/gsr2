use std::num::ParseIntError;
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_string(s: &str) -> Result<Self, ParseIntError> {
        u8::from_str_radix(&s[0..2], 16).and_then(|r| {
            u8::from_str_radix(&s[2..4], 16)
                .and_then(|g| u8::from_str_radix(&s[4..6], 16).map(|b| Color { r, g, b }))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_can_parse_a_string_to_initialize() {
        let color_opt = Color::from_string("0a20ff");
        println!("{:?}", color_opt);
        assert!(color_opt.is_ok());
        assert_eq!(
            Color {
                r: 10,
                g: 32,
                b: 255,
            },
            color_opt.unwrap()
        );
    }
}
