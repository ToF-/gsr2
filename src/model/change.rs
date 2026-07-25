use std::str::FromStr;
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Change {
    AddTag,
    Category,
    Cover,
    Name,
    Label,
    RemoveTag,
    Unlabel,
}

impl FromStr for Change {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AddTag" => Ok(Change::AddTag),
            "Category" => Ok(Change::Category),
            "Cover" => Ok(Change::Cover),
            "Name" => Ok(Change::Name),
            "Label" => Ok(Change::Label),
            "RemoveTag" => Ok(Change::RemoveTag),
            "Unlabel" => Ok(Change::Unlabel),

            _ => Err(format!("unknown change: {s}")),
        }
    }
}
