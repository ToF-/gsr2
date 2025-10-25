use crate::model::label::Label;
use crate::model::rank::Rank;

#[derive(Debug, Clone)]
pub enum Action {
    NoAction,
    Rank(Rank),
    Label(Label),
    Unlabel,
    AddTag(Label),
    RemoveTag(Label),
}
