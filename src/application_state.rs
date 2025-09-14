
#[cfg(test)]
mod tests {
    #[test]
    fn after_palette_toggle_palette_on_is_inverted() {
        let mut state = ApplicationState {
            palette_on: false,
        };
        state.toggle_palette();
        assert_eq!(true, state.palette_on());
    }
}
