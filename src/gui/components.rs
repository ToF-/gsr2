use gtk::ScrolledWindow;

pub fn make_single_view_scrolled_window() -> gtk::ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .name("view")
        .build()
}

