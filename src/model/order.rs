use clap::builder::PossibleValue;
use serde::{Deserialize, Serialize};

#[repr(i32)]
#[derive(PartialEq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Order {
    Category = 0,
    ColorCount = 1,
    Cover = 2,
    Date = 3,
    Label = 4,
    Name = 5,
    Palette = 6,
    Random = 7,
    Score = 8,
    Size = 9,
    Value = 10,
}

impl From<i32> for Order {
    fn from(n: i32) -> Self {
        match n {
            0 => Order::Category,
            1 => Order::ColorCount,
            2 => Order::Cover,
            3 => Order::Date,
            4 => Order::Label,
            5 => Order::Name,
            6 => Order::Palette,
            7 => Order::Random,
            8 => Order::Score,
            9 => Order::Size,
            10 => Order::Value,
            _ => todo!(),
        }
    }
}
#[allow(dead_code)]
pub fn from(s: &str) -> Option<Order> {
    match s {
        "a" => Some(Order::Category),
        "c" => Some(Order::ColorCount),
        "d" => Some(Order::Date),
        "l" => Some(Order::Label),
        "m" => Some(Order::Score),
        "n" => Some(Order::Name),
        "o" => Some(Order::Cover),
        "p" => Some(Order::Palette),
        "r" => Some(Order::Random),
        "s" => Some(Order::Size),
        "v" => Some(Order::Value),
        _ => None,
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl clap::ValueEnum for Order {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Order::Category,
            Order::ColorCount,
            Order::Cover,
            Order::Date,
            Order::Name,
            Order::Random,
            Order::Score,
            Order::Size,
            Order::Value,
            Order::Palette,
            Order::Label,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Order::Category => PossibleValue::new("Category"),
            Order::ColorCount => PossibleValue::new("Colors"),
            Order::Cover => PossibleValue::new("Cover"),
            Order::Date => PossibleValue::new("Date"),
            Order::Name => PossibleValue::new("Name"),
            Order::Random => PossibleValue::new("Random"),
            Order::Value => PossibleValue::new("Value"),
            Order::Score => PossibleValue::new("Score"),
            Order::Size => PossibleValue::new("Size"),
            Order::Palette => PossibleValue::new("Palette"),
            Order::Label => PossibleValue::new("Label"),
        })
    }
}
