#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    AddTag,
    RemoveTag,
    Number,
    Order,
    DeleteConfirmation,
    MoveConfirmation,
    MoveToLabelConfirmation(String),
    Find,
    FindLabel,
    Information,
    Help,
    Rename,
    SetRestriction,
    SetSelection,
}
