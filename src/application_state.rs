#[derive(Debug)]
pub struct ApplicationState {
    expand_on: bool,
    palette_on: bool,
}

impl ApplicationState {
    pub fn new(palette_on: bool) -> Self {
        ApplicationState {
            expand_on: false,
            palette_on: palette_on,
        }
    }
    pub fn expand_on(&self) -> bool {
        self.expand_on
    }

    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn toggle_expand(&mut self) {
        self.expand_on = !self.expand_on
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
        let mut state = ApplicationState::new(false);
        state.toggle_palette();
        assert_eq!(true, state.palette_on());
    }

    #[test]
    fn after_expand_toggle_expand_on_is_inverted() {
        let mut state = ApplicationState::new(false);
        state.toggle_expand();
        assert_eq!(true, state.expand_on());
        state.toggle_expand();
        assert_eq!(false, state.expand_on());
    }
}
