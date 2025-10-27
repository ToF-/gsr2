#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    AddTag,
    RemoveTag,
    Number,
    Order,
    DeleteConfirmation,
    Find,
    Information,
    SetRestriction,
    SetSelection,
}
