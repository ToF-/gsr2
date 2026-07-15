use crate::model::category::Category;
use crate::model::label::Label;
use crate::model::rank::Rank;

#[derive(Debug, Clone)]
pub enum Action {
    Nothing,
    Rank(Rank),
    Label(Label),
    Unlabel,
    AddTag(Label),
    RemoveTag(Label),
    Categorize(Category),
}
