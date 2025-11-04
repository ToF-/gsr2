#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    AddTag,
    RemoveTag,
    Number,
    Order,
    DeleteConfirmation,
    MoveConfirmation,
    Find,
    FindLabel,
    Information,
    Help,
    SetRestriction,
    SetSelection,
}
