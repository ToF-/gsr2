use crate::env::default_values::{THREE_STARS_SYMBOL, TWO_STARS_SYMBOL, ONE_STAR_SYMBOL, NO_STAR};
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Rank {
   ThreeStars, TwoStars, OneStar, NoStar,
}

impl Rank {
    pub fn show(&self) -> String {
        let limit = 3 - *self as usize;
        if limit > 0 {
            "☆".repeat(limit)
        } else {
            "".to_string()
        }
    }
}

impl From<i64> for Rank {
    fn from(n: i64) -> Self {
        match n {
            0 => Rank::ThreeStars,
            1 => Rank::TwoStars,
            2 => Rank::OneStar,
            _ => Rank::NoStar,
        }
    }
}

impl Into<i64> for Rank {
    fn into(self) -> i64 {
        match self {
            Rank::ThreeStars => 0,
            Rank::TwoStars => 1,
            Rank::OneStar => 2,
            Rank::NoStar => 4,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Rank::ThreeStars => THREE_STARS_SYMBOL,
            Rank::TwoStars => TWO_STARS_SYMBOL,
            Rank::OneStar => ONE_STAR_SYMBOL,
            Rank::NoStar => NO_STAR,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_and_to_i64() {
        let result: i64 = Rank::OneStar.into();
        assert_eq!(2, result); 
        let rank: Rank = Rank::from(1);
        assert_eq!(Rank::TwoStars, rank);
    }
}
