use crate::command_line_interface::CommandLineInterface;
use crate::direction::Direction;
use crate::display::title_display;
use crate::editor::{Editor,InputKind};
use crate::gui::components::*;
use crate::order;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, ApplicationWindow, CssProvider, Grid, gdk, Label, Orientation, Picture, ScrolledWindow};
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::time::Duration;

pub struct View {
    width: i32,
    height: i32,
    cells_per_row: i32,
    application: gtk::Application,
    application_window: gtk::ApplicationWindow,
}

impl View {
    pub fn make_view(width: i32, height: i32, cells_per_row: i32) -> Self {

        let application: gtk::Application  = make_application("example.org.gsr2");
        let application_window: gtk::ApplicationWindow = make_application_window(&application);
        View {
            width,
            height,
            cells_per_row,
            application: application,
            application_window,
        }
    }

    pub fn setup_components(&mut self) {
        let grid = grid(self.cells_per_row);

         let left_pane = make_pane_with_label("←");
         let right_pane = make_pane_with_label("→");

         let panel = make_panel();
         panel.attach(&left_pane, 0, 0, 1, 1);
         panel.attach(&grid, 1, 0, 1, 1);
         panel.attach(&right_pane, 2, 0, 1, 1);

         let multiple_view_scrolled_window = make_multiple_view_scrolled_window();

         multiple_view_scrolled_window.set_child(Some(&panel));

         let frame = make_frame();
         let picture = make_picture();
         frame.append(&picture);
         let single_view_scrolled_window = make_single_view_scrolled_window();
         single_view_scrolled_window.set_child(Some(&frame));

         let view_stack = make_stack();
         let _ = view_stack.add_child(&single_view_scrolled_window);
         let _ = view_stack.add_child(&multiple_view_scrolled_window);
         if self.cells_per_row == 1 {
             view_stack.set_visible_child(&single_view_scrolled_window);
         } else {
             view_stack.set_visible_child(&multiple_view_scrolled_window);
         }
         self.application_window.set_child(Some(&view_stack));
    }
}
