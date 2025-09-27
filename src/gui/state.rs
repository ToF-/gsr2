#[derive(Clone, Debug)]
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

    pub fn pictures_per_row(&self) -> usize {
        self.pictures_per_row
    }

    pub fn single_view(&self) -> bool {
        self.pictures_per_row == 1
    }

    pub fn full_size_on(&self) -> bool {
        self.full_size_on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
