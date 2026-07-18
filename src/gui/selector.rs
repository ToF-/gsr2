use crate::model::catalog::Catalog;
use crate::MainWindow;
use crate::gui::control::Control;
use crate::gui::control::Controls;
use crate::gui::control::default_controls;
use crate::gui::mode::Mode;
use crate::gui::view::treelist_window::TreeListWindow;
use gtk::gdk::Key;

#[derive(Clone, Debug)]
pub struct Selector {
    prompt: String,
    selecting: bool,
    selected: String,
    controls: Controls,
    treelist_window_opt: Option<TreeListWindow>,
    catalog: Catalog,
}

impl Selector {
    pub fn new(catalog: &Catalog) -> Selector {
        Selector {
            prompt: "".to_string(),
            selecting: false,
            controls: default_controls(),
            selected: "".to_string(),
            treelist_window_opt: None,
            catalog: catalog.clone(),
        }
    }

    pub fn begin(&mut self, main_window: &MainWindow) {
        let prompt = "select a category";
        self.prompt = prompt.to_string();
        self.treelist_window_opt = Some(main_window.popup_treelist_window(&self.prompt, &self.catalog));
        self.selecting = true;
    }

    pub fn catalog(&self) -> Catalog {
        self.catalog.clone()
    }

    pub fn process(&mut self, key: Key) {
        match key.name() {
            None => {}
            Some(key_name) => match self.controls.get(&(key_name.to_string(), Mode::Selecting)) {
                Some(Control::CancelSelection) => self.cancel(),
                Some(Control::ConfirmSelection) => self.enter(),
                Some(_) | None => {}
            },
        }
    }

    pub fn selected(&self) -> String {
        self.selected.clone()
    }

    pub fn set_selected(&mut self, selected: &str) {
        self.selected = selected.to_string();
    }

    pub fn cancel(&mut self) {
        self.selected = String::from("");
        self.treelist_window_opt.clone().unwrap().close();
        self.selecting = false;
    }

    pub fn enter(&mut self) {
        self.treelist_window_opt.clone().unwrap().close();
        self.selecting = false;
    }

    pub fn selecting(&self) -> bool {
        self.selecting
    }
}
