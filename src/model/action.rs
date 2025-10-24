use crate::model::rank::Rank;
use crate::model::label::Label;

#[derive(Debug, Clone)]
pub enum Action {
    NoAction,
    Rank(Rank),
    Label(Label),
    Unlabel,
    AddTag(Label),
    RemoveTag(Label),
}
