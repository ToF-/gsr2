#[derive(Debug)]
pub struct State {
    pub pictures_per_row: usize,
    pub old_pictures_per_row: usize,
    pub expand_on: bool,
    pub full_size_on: bool,
    pub palette_on: bool,
}

impl State {
    pub fn new(pictures_per_row: usize) -> Self {
        State {
            pictures_per_row,
            old_pictures_per_row: pictures_per_row,
            expand_on: false,
            full_size_on: false,
            palette_on: false,
        }
    }
}
