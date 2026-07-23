
use std::str::FromStr;
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Find {
    Category,
    Name,
    Label,
    SubCategory,
    Tags,
}

impl FromStr for Find {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Category" => Ok(Find::Category),
            "Label" => Ok(Find::Label),
            "Name" => Ok(Find::Name),
            "SubCategory" => Ok(Find::SubCategory),
            "Tags" => Ok(Find::Tags),
            _ => Err(format!("unknown find: {s}")),
        }
    }
}
