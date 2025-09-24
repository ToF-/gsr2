use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::gui::view::View;

pub struct Controller {
    args:  CommandLineInterface,
    state: ApplicationState,
    view:  View,
}

