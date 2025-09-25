use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::gui::view::View;
use gtk::glib::clone;

pub struct Controller {
    args:  CommandLineInterface,
    state: ApplicationState,
    view:  View,
}

impl Controller {

pub fn build_and_run_application(&self, cli: CommandLineInterface) {

    application.connect_startup(|application| {
        startup_gui(application);
    });
    let cli_rc = Rc::new(RefCell::new(cli));
    // clone! passes a strong reference to a variable in the closure that activates the application
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    application.connect_activate(
        clone!(@strong cli_rc, => move |application: &gtk::Application| {
        activate(application, &cli_rc); }),
    );
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
}
