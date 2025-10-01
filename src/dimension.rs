use crate::default_values;
use std::env;

pub fn dimension(source: Option<i32>, var_name: &str, dimension_name: &str, default: i32) -> i32 {
    let value = match source {
        Some(n) => n,
        None => match env::var(var_name) {
            Ok(s) => match s.parse::<i32>() {
                Ok(n) => n,
                _ => {
                    println!("illegal {} value: {}, setting to default", dimension_name, s);
                    default
                }
            },
            _ => {
                default
            }
        }
    };
    if (value >= default_values::DIMENSION_MIN)
        && (value <= default_values::DIMENSION_MAX) {
        value
    } else {
        println!("illegal {} value: {}, setting to default", dimension_name, value);
        default
    }
}

