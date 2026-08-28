use crate::gui::control::Control;
use crate::gui::control::Controls;
use crate::gui::control::default_controls;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use gtk::gdk::Key;

#[derive(Clone, Debug)]
pub struct Selector {
    prompt: String,
    selecting: bool,
    selected: String,
    prev_selected: String,
    controls: Controls,
    treelist_view_opt: Option<TreeListView>,
    catalog: Catalog,
}

impl Selector {
    pub fn new(catalog: &Catalog) -> Selector {
        Selector {
            prompt: "".to_string(),
            selecting: false,
            controls: default_controls(),
            selected: "".to_string(),
            prev_selected: "".to_string(),
            treelist_view_opt: None,
            catalog: catalog.clone(),
        }
    }

    pub fn begin(&mut self, _main_view: &GsrApplicationWindow, prompt: &str, catalog: &Catalog) {
        self.catalog = catalog.clone();
        self.prompt = prompt.to_string();
        // self.treelist_view_opt = Some(main_view.popup_treelist_view(&self.prompt, &self.catalog));
        self.selecting = true;
    }

    pub fn catalog(&self) -> Catalog {
        self.catalog.clone()
    }

    pub fn process(&mut self, key: Key) {
        match key.name() {
            None => {}
            Some(key_name) => match self
                .controls
                .get(&(key_name.to_string(), Mode::Categorizing))
            {
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
        self.selected = selected.to_string()
    }

    pub fn prev_selected(&self) -> String {
        self.prev_selected.clone()
    }

    pub fn set_prev_selected(&mut self, selected: &str) {
        self.prev_selected = selected.to_string()
    }

    pub fn cancel(&mut self) {
        self.selected = String::from("");
        self.treelist_view_opt.clone().unwrap().close();
        self.selecting = false;
    }

    pub fn enter(&mut self) {
        self.treelist_view_opt.clone().unwrap().close();
        self.selecting = false;
    }

    pub fn selecting(&self) -> bool {
        self.selecting
    }
}
