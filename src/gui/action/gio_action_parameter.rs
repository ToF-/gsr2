use crate::gui::direction::Direction;
use crate::model::category::Category;
use crate::model::category::category_from_string;
use crate::model::category::string_from_category;
use crate::model::find::Find;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;
use gtk::glib::Variant;
use gtk::glib::prelude::ToVariant;

#[derive(PartialEq, Clone, Debug)]
pub struct GioActionParameter {
    variant: gtk::glib::Variant,
}

impl GioActionParameter {
    pub fn variant(&self) -> &gtk::glib::Variant {
        &self.variant
    }
}

impl From<Variant> for GioActionParameter {
    fn from(variant: Variant) -> Self {
        Self { variant: variant }
    }
}

impl From<usize> for GioActionParameter {
    fn from(n: usize) -> Self {
        Self {
            variant: (n as u64).to_variant(),
        }
    }
}

impl From<GioActionParameter> for usize {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter.variant().get::<u64>().unwrap() as usize
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

impl From<GioActionParameter> for char {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter = gio_action_parameter.variant().get::<i32>().unwrap() as u32;
        char::from_u32(parameter).unwrap()
    }
}

impl From<char> for GioActionParameter {
    fn from(c: char) -> Self {
        Self {
            variant: (c as i32).to_variant(),
        }
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

impl From<GioActionParameter> for (String, String) {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        gio_action_parameter
            .variant()
            .get::<(String, String)>()
            .unwrap()
    }
}

impl From<Order> for GioActionParameter {
    fn from(order: Order) -> Self {
        Self {
            variant: (order as i32).to_variant(),
        }
    }
}

impl From<GioActionParameter> for Order {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: i32 = gio_action_parameter.variant().get::<i32>().unwrap();
        Order::from(parameter_value)
    }
}

impl From<ViewOption> for GioActionParameter {
    fn from(view_option: ViewOption) -> Self {
        Self {
            variant: (view_option as i32).to_variant(),
        }
    }
}

impl From<GioActionParameter> for ViewOption {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: i32 = gio_action_parameter.variant().get::<i32>().unwrap();
        ViewOption::from(parameter_value)
    }
}

impl From<Category> for GioActionParameter {
    fn from(category: Category) -> Self {
        Self {
            variant: string_from_category(category).to_variant(),
        }
    }
}

impl From<GioActionParameter> for Category {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: String = gio_action_parameter.variant().get::<String>().unwrap();
        category_from_string(&parameter_value)
    }
}

impl From<Find> for GioActionParameter {
    fn from(find: Find) -> Self {
        Self {
            variant: (find as i32).to_variant(),
        }
    }
}

impl From<GioActionParameter> for Find {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: i32 = gio_action_parameter.variant().get::<i32>().unwrap();
        Find::from(parameter_value)
    }
}

impl From<Direction> for GioActionParameter {
    fn from(direction: Direction) -> Self {
        Self {
            variant: (i32::from(direction)).to_variant(),
        }
    }
}

impl From<GioActionParameter> for Direction {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: i32 = gio_action_parameter.variant().get::<i32>().unwrap();
        Direction::from(parameter_value)
    }
}

impl From<Rank> for GioActionParameter {
    fn from(rank: Rank) -> Self {
        Self {
            variant: (i64::from(rank)).to_variant(),
        }
    }
}

impl From<GioActionParameter> for Rank {
    fn from(gio_action_parameter: GioActionParameter) -> Self {
        let parameter_value: i64 = gio_action_parameter.variant().get::<i64>().unwrap();
        Rank::from(parameter_value)
    }
}
