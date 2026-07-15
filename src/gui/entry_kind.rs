#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    AddTag,
    Categorize,
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
