pub mod gsr_application;
pub mod gsr_application_window;
pub mod gsr_entry_window;
pub mod gsr_main_window;
pub mod gsr_palette_area;
pub mod gsr_picture_cell_box;
pub mod gsr_picture_frame;
pub mod gsr_picture_grid;
pub mod gsr_treelist_window;
pub mod gsr_window;
pub mod my_window;

use std::path::Path;

pub fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
    gtk::Picture::builder()
        .file(&gtk::gio::File::for_path(file_path))
        .hexpand(true)
        .vexpand(true)
        .build()
}
