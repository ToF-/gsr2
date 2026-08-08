use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;
#[repr(i32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Find {
    AllTags = 0,
    Category = 1,
    Label = 2,
    Name = 3,
    SomeTags = 4,
    SubCategory = 5,
}

impl FromStr for Find {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Category" => Ok(Find::Category),
            "Label" => Ok(Find::Label),
            "Name" => Ok(Find::Name),
            "SubCategory" => Ok(Find::SubCategory),
            "SomeTags" => Ok(Find::SomeTags),
            "AllTags" => Ok(Find::AllTags),
            _ => Err(format!("unknown find: {s}")),
        }
    }
}

impl From<i32> for Find {
    fn from(n: i32) -> Self {
        match n {
            0 => Find::AllTags,
            1 => Find::Category,
            2 => Find::Label,
            3 => Find::Name,
            4 => Find::SomeTags,
            5 => Find::SubCategory,
            _ => todo!()
        }
    }
}

impl Display for Find {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Find::Category => "category",
                Find::Label => "label",
                Find::Name => "name",
                Find::SubCategory => "subcategory",
                Find::SomeTags => "tags",
                Find::AllTags => "tags",
            }
        )
    }
}
