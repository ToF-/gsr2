use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;
#[repr(i32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Change {
    AddCategory = 0,
    AddTag = 1,
    Catalog = 2,
    Category = 3,
    Cover = 4,
    Label = 5,
    MoveCategory = 6,
    Name = 7,
    RemoveCategory = 8,
    RemoveTag = 9,
    Unlabel = 10,
}

impl From<i32> for Change {
    fn from(n: i32) -> Self {
        match n {
            0 => Change::AddCategory,
            1 => Change::AddTag,
            2 => Change::Catalog,
            3 => Change::Category,
            4 => Change::Cover,
            5 => Change::Label,
            6 => Change::MoveCategory,
            7 => Change::Name,
            8 => Change::RemoveCategory,
            9 => Change::RemoveTag,
            10 => Change::Unlabel,
            _ => todo!(),
        }
    }
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
