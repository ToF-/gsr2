pub struct EntryView {
    gtk_window_opt: Option<gtk::Window>,
}

impl EntryView {
    pub fn new() -> Self {
        Self {
            gtk_window_opt: None,
        }
    }

    pub fn input(&self) -> String {
        if let Some(gtk_window) = self.gtk_window_opt.clone() {
            String::new()
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }

    pub fn set_input(&self,text: &str) {
        if let Some(gtk_window) = &self.gtk_window_opt {
            todo!("set the gtk window input")
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }

}
