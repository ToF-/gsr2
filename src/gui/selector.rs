use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::control::Controls;
use gtk::gdk::Key;
use crate::gui::mode::Mode;
use crate::MainWindow;
use crate::gui::view::treelist_window::TreeListWindow;

#[derive(Clone, Debug)]
pub struct Selector {
    prompt: String,
    selected: String,
    controls: Controls,
    treelist_window_opt: Option<TreeListWindow>,
}

impl Selector {
    pub fn new() -> Selector {
        Selector {
            prompt: "".to_string(),
            controls: default_controls(),
            selected: "".to_string(),
            treelist_window_opt: None,
        }
    }

    pub fn begin(&mut self, main_window: &MainWindow) {
        let prompt = "select a category";
        self.prompt = prompt.to_string();
        self.treelist_window_opt = Some(main_window.popup_treelist_window(&self.prompt));
    }
    pub fn process(&mut self, key: Key) {
        match key.name() {
            None => {}
            Some(key_name) => match self.controls.get(&(key_name.to_string(), Mode::Editing)) {
                Some(Control::CancelEdition) => self.cancel(),
                Some(Control::ConfirmEdition) => self.enter(),
                Some(_) | None => {},
            },
        }
    }

    pub fn cancel(&mut self) {
        self.selected = String::from("");
        self.treelist_window_opt.clone().unwrap().close();
    }

    pub fn enter(&mut self) {
        self.selected = String::from("foo");
        self.treelist_window_opt.clone().unwrap().close();
    }

}
