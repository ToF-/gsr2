use crate::model::label::Label;
use crate::model::rank::Rank;
use crate::model::category::Category;

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
