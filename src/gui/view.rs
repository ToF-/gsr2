use std::path::Path;
pub mod palette_area;
pub mod treelist_view;
pub mod treelist_window;

pub fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
    gtk::Picture::builder()
        .file(&gtk::gio::File::for_path(file_path))
        .hexpand(true)
        .vexpand(true)
        .build()
}
