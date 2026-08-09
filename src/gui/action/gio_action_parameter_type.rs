use gtk::glib::VariantTy;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq)]
pub enum GioActionParameterType {
    None,
    Char,
    Int32,
    Int64,
    String,
    StringPair,
    Usize,
}

impl GioActionParameterType {
    pub fn variant_ty(&self) -> Option<&'static VariantTy> {
        match self {
            GioActionParameterType::None => None,
            GioActionParameterType::Char => Some(VariantTy::STRING),
            GioActionParameterType::Int32 => Some(VariantTy::INT32),
            GioActionParameterType::Int64 => Some(VariantTy::INT64),
            GioActionParameterType::String => Some(VariantTy::STRING),
            GioActionParameterType::StringPair => {
                static STRING_PAIR: OnceLock<&'static VariantTy> = OnceLock::new();
                Some(STRING_PAIR.get_or_init(|| Box::leak(VariantTy::new("(ss)").unwrap().into())))
            }
            GioActionParameterType::Usize => Some(VariantTy::UINT64),
        }
    }
}
