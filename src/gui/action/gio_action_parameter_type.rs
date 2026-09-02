use gtk::glib::VariantTy;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq)]
pub enum GioActionParameterType {
    None,
    Char,
    Int32,
    Int32Pair,
    Int32String,
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
            GioActionParameterType::Int32Pair => {
                static INT32_PAIR: OnceLock<&'static VariantTy> = OnceLock::new();
                Some(INT32_PAIR.get_or_init(|| Box::leak(VariantTy::new("(ii)").unwrap().into())))
            }
            GioActionParameterType::Int32String => {
                static INT32_STRING: OnceLock<&'static VariantTy> = OnceLock::new();
                Some(INT32_STRING.get_or_init(|| Box::leak(VariantTy::new("(is)").unwrap().into())))
            }
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
