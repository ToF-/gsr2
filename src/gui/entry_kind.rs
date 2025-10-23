#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    AddTag,
    RemoveTag,
    Number,
    DeleteConfirmation,
    Find,
    SetRestriction,
    SetSelection,
}
