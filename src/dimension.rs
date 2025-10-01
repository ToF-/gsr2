use crate::env::default_values;
use std::env;

pub fn dimension(
    source: Option<i32>,
    var_name: &str,
    dimension_name: &str,
    default: i32,
) -> Option<i32> {
    let value = match source {
        Some(n) => n,
        None => match env::var(var_name) {
            Ok(s) => match s.parse::<i32>() {
                Ok(n) => n,
                _ => {
                    println!(
                        "illegal {} value: {}, setting to default",
                        dimension_name, s
                    );
                    default
                }
            },
            _ => default,
        },
    };
    if (value >= default_values::DIMENSION_MIN) && (value <= default_values::DIMENSION_MAX) {
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
    let value = match source {
        Some(n) => n,
        None => return None,
    };
    if (value >= default_values::SLIDESHOW_DELAY_MIN)
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
