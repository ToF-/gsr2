use gtk::glib::prelude::ToVariant;

#[derive(PartialEq, Clone, Debug)]
pub struct GioActionParameter {
    variant: gtk::glib::Variant,
}


impl From<i64> for GioActionParameter {
    fn from(n: i64) -> Self {
        Self {
            variant: n.to_variant(),
        }
    }
}

impl From<i32> for GioActionParameter {
    fn from(n: i32) -> Self {
        Self {
            variant: n.to_variant(),
        }
    }
}

impl From<&str> for GioActionParameter {
    fn from(s: &str) -> Self {
        Self {
            variant: s.to_string().to_variant(),
        }
    }
}

impl GioActionParameter {
    pub fn from_string_pair(pair: (&str, &str)) -> Self {
        Self {
            variant: pair.to_variant(),
        }
    }
}

