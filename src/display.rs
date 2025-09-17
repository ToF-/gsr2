use crate::application_state::ApplicationState;
use crate::default_values::{EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL};
use crate::gallery::Gallery;

fn expand_display(on: bool) -> String {
    match on {
        false => String::from(""),
        true => String::from(EXPAND_ON_SYMBOL),
    }
}

fn full_size_display(on: bool) -> String {
    match on {
        false => String::from(""),
        true => String::from(FULL_SIZE_ON_SYMBOL),
    }
}

pub fn title_display(application_state: &ApplicationState) -> String {
    format!(
        "#{} {}{}{}",
        application_state.navigator().position(),
        application_state.current_picture().file_name(),
        expand_display(application_state.expand_on()),
        full_size_display(application_state.full_size_on())
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    fn an_application_state() -> ApplicationState {
        let mut application_state = ApplicationState::new(false);
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        application_state.set_gallery(gallery);
        application_state
    }

    #[test]
    fn display_title_for_application_state() {
        assert_eq!("#0 nine_colors.png", title_display(&an_application_state()));
    }

    #[test]
    fn display_position_for_application_state() {
        let mut application_state = an_application_state();
        application_state.move_towards(Direction::Right);
        assert_eq!("#1 single_dot.png", title_display(&application_state));
    }

    #[test]
    fn display_title_for_application_state_with_expand_on() {
        let mut application_state = an_application_state();
        application_state.toggle_expand();
        assert_eq!(
            "#0 nine_colors.png".to_owned() + EXPAND_ON_SYMBOL,
            title_display(&application_state)
        )
    }

    #[test]
    fn display_title_for_application_state_with_full_size_on() {
        let mut application_state = an_application_state();
        application_state.toggle_full_size();
        assert_eq!(
            "#0 nine_colors.png".to_owned() + FULL_SIZE_ON_SYMBOL,
            title_display(&application_state)
        )
    }
}
