use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use gtk::glib::VariantTy;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq)]
pub enum GioActionParameterType {
    None,
    Char,
    Int32,
    String,
    StringPair,
}

impl GioActionParameterType {
    pub fn variant_ty(&self) -> Option<&'static VariantTy> {
        match self {
            GioActionParameterType::None => None,
            GioActionParameterType::Char => Some(VariantTy::STRING),
            GioActionParameterType::Int32 => Some(VariantTy::INT32),
            GioActionParameterType::String => Some(VariantTy::STRING),
            GioActionParameterType::StringPair => {
                static STRING_PAIR: OnceLock<&'static VariantTy> = OnceLock::new();
                Some(STRING_PAIR.get_or_init(|| {
                    Box::leak(VariantTy::new("(ss)").unwrap().into())
                }))
            }
        }
    }
}

#[derive(Debug)]
pub struct GioAction {
    name: String,
    action_entry_name: String,
    parameter_type: GioActionParameterType,
}

impl GioAction {
    pub fn new(name: &str, action_parameter_type: GioActionParameterType) -> Self {
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
    pub fn parameter_type(&self) -> GioActionParameterType {
        self.parameter_type.clone()
    }
}
