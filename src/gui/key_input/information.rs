use crate::gui::action::Action;
use crate::env::default_values::INFORMATION_SYMBOL;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::KeyInputMode;
use crate::gui::key_input::KeyInputRules;

pub fn information_key_input() -> KeyInput {
    KeyInput::new(
        INFORMATION_SYMBOL,
        None,
        KeyInputMode::Information,
        |_, _| true,
        |s, _| s,
        |_| Action::Dismiss,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use gtk::gdk::Key;

    #[test]
    fn given_any_key_does_not_change_the_input() {
        let key_input = information_key_input();
        let status = key_input.edit("foo", Key::from_name("b").unwrap());
        assert_eq!("foo", &status.input());
    }
    #[test]
    fn given_any_key_lauche_the_dismiss_action() {
        let key_input = information_key_input();
        let status = key_input.edit("fib", Key::from_name("a").unwrap());
        assert_eq!(Some(Action::Dismiss), status.result_action());
    }
}
