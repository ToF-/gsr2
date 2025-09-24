use crate::default_values::{ DEFAULT_HEIGHT, DEFAULT_WIDTH };

pub struct View {
    application_window: gtk::ApplicationWindow,
}

impl View {
    pub fn make_view(application: &gtk::Application, width: usize, height: usize, cells_per_row: usize) -> Self {
        // make grid
        // for 0..cells_per_row, 0..cells_per_row
        //      make cell_box
        //      attach cell_box to grid at col,row

        // make left_button
        // make right_button
        // make panel
        // attach left_button, grid, to panel
        //
        // make multiple_view_scrolled_window
        //      set child panel
        //
        // make single_view_scrolled_window
        // make single_view_box
        // make single_view_picture
        // append single_picture to single_view_box
        // set child single_view_box single_view_scrolled_window
        //
        // make view_stack         
        //   add child single_view_scrolled_window
        //   add child multiple_view_scrolled_window
        //   set_visible_child on one or the other according to cells_per_row
        //
        //  make application_window
        //  set child view_stack application_window


        // single_picture (if cells_per_row == 1
        //   = application_window
        //      .first_child
        //          .visible_child
        //              .first_child
        //                  .first_child
        //
        View {
            application_window: gtk::ApplicationWindow::builder()
                .application(application)
                .title("gsr2")
                .default_width(width as i32)
                .default_height(height as i32)
                .build()
        }
    }
}

