use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use gtk::glib::VariantTy;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionParameterType {
    None,
    Int32,
    String,
}

impl ActionParameterType {
    pub fn variant_ty(&self) -> Option<&'static VariantTy> {
        match self {
            ActionParameterType::None => None,
            ActionParameterType::Int32 => Some(VariantTy::INT32),
            ActionParameterType::String => Some(VariantTy::STRING),
        }
    }
}

#[derive(Debug)]
pub struct MainControllerAction {
    name: String,
    action_entry_name: String,
    parameter_type: ActionParameterType,
}

impl MainControllerAction {
    pub fn new(name: &str, action_parameter_type: ActionParameterType) -> Self {
        Self {
            name: name.to_string(),
            action_entry_name: format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, name),
            parameter_type: action_parameter_type,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }
    pub fn parameter_type(&self) -> ActionParameterType {
        self.parameter_type.clone()
    }
}
