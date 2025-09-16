use crate::application_state::ApplicationState;
use crate::gallery::Gallery;

pub fn title_display(application_state: &ApplicationState) -> String {
    format!("{}", application_state.current_picture().file_name())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_title_for_application_state() {
        let mut application_state = ApplicationState::new(false);
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        application_state.set_gallery(gallery);
        assert_eq!("nine_colors.png", title_display(&application_state));
    }
}
