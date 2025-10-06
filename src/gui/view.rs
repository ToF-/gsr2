use crate::file::paths::check_path_exists;
use crate::gui::controller::RcController;
use crate::gui::direction::Direction;
use crate::gui::display::picture_label_display;
use crate::gui::display::title_display;
use crate::gui::view::components::main_window::MainWindow;
use crate::gui::view::components::picture_cell_box::make_picture_cell_box;
use crate::gui::view::components::picture_frame::PictureFrame;
use crate::gui::view::components::picture_grid::PictureGrid;
use gtk::glib::timeout_add_local;
use gtk::{self};
use gtk::{Orientation, gdk};

pub mod components;

