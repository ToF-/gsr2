use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Change {
    AddCategory,
    AddTag,
    Catalog,
    Category,
    Cover,
    Label,
    MoveCategory,
    Name,
    RemoveCategory,
    RemoveTag,
    Unlabel,
}

impl FromStr for Change {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AddCategory" => Ok(Change::AddCategory),
            "AddTag" => Ok(Change::AddTag),
            "Catalog" => Ok(Change::Catalog),
            "Category" => Ok(Change::Category),
            "Cover" => Ok(Change::Cover),
            "MoveCategory" => Ok(Change::MoveCategory),
            "Name" => Ok(Change::Name),
            "Label" => Ok(Change::Label),
            "RemoveCategory" => Ok(Change::RemoveCategory),
            "RemoveTag" => Ok(Change::RemoveTag),
            "Unlabel" => Ok(Change::Unlabel),

            _ => Err(format!("unknown change: {s}")),
        }
    }
}

impl Display for Change {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Change::AddTag => "add-tag",
                Change::Catalog => "catalog",
                Change::Category => "category",
                Change::Cover => "cover",
                Change::Name => "name",
                Change::Label => "label",
                Change::RemoveTag => "remove-tag",
                Change::Unlabel => "unlabel",
                _ => "else",
            }
        )
    }
}
