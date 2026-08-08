use gtk::glib::prelude::ToVariant;

#[derive(PartialEq, Clone, Debug)]
pub struct GioActionParameter {
    variant: gtk::glib::Variant,
}

impl GioActionParameter {
    pub fn variant(&self) -> gtk::glib::Variant {
        self.variant.clone()
    }
}

impl From<i64> for GioActionParameter {
    fn from(n: i64) -> Self {
        Self {
            variant: n.to_variant(),
        }
    }
}

impl From<GioActionParameter> for i64 {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter.variant().get::<i64>().unwrap()
    }
}

impl From<i32> for GioActionParameter {
    fn from(n: i32) -> Self {
        Self {
            variant: n.to_variant(),
        }
    }
}

impl From<GioActionParameter> for i32 {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter.variant().get::<i32>().unwrap()
    }
}

impl From<String> for GioActionParameter {
    fn from(s: String) -> Self {
        Self {
            variant: s.to_variant(),
        }
    }
}

impl From<GioActionParameter> for String {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter.variant().get::<String>().unwrap()
    }
}

impl From<(String, String)> for GioActionParameter {
    fn from(pair: (String, String)) -> Self {
        Self {
            variant: (pair.0, pair.1).to_variant(),
        }
    }
}

impl From<GioActionParameter> for (String,String) {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter.variant().get::<(String,String)>().unwrap()
    }
}
