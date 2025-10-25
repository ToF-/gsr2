use crate::env::default_values;

pub fn dimension(value: i32, dimension_name: &str, default: i32) -> Option<i32> {
    if (default_values::DIMENSION_MIN..=default_values::DIMENSION_MAX).contains(&value) {
        Some(value)
    } else {
        println!(
            "illegal {} value: {}, setting to default",
            dimension_name, value
        );
        Some(default)
    }
}

pub fn slideshow_delay(source: Option<i32>, dimension_name: &str, default: i32) -> Option<i32> {
    let value = source?;
    if (default_values::SLIDESHOW_DELAY_MIN..=default_values::SLIDESHOW_DELAY_MAX).contains(&value)
        && (value <= default_values::SLIDESHOW_DELAY_MAX)
    {
        Some(value)
    } else {
        println!(
            "illegal {} value: {}, setting to default",
            dimension_name, value
        );
        Some(default)
    }
}
