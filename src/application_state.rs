pub struct ApplicationState {
    palette_on: bool,
}

impl ApplicationState {
    pub fn new(palette_on: bool) -> Self {
        ApplicationState {
            palette_on: palette_on,
        }
    }
    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn toggle_palette(&mut self) {
        self.palette_on = !self.palette_on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_palette_toggle_palette_on_is_inverted() {
        let mut state = ApplicationState { palette_on: false };
        state.toggle_palette();
        assert_eq!(true, state.palette_on());
    }
}
