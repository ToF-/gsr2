use crate::application_state::ApplicationState;
use crate::gallery::Gallery;

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
    }
}
