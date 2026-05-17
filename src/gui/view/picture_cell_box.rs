use crate::gui::controller::RcController;
use crate::gui::event::Event::{PictureClicked, PictureDoubleClicked};
use gtk::Align;
use gtk::Orientation;
use gtk::glib::clone;
use gtk::prelude::{GestureSingleExt, WidgetExt};

pub fn make_picture_cell_box(col: i32, row: i32, controller_rc: &RcController) -> gtk::Box {
    let cell_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .valign(Align::Center)
        .halign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    cell_box.add_controller(make_gesture_click(1, col, row, controller_rc));
    cell_box.add_controller(make_gesture_click(3, col, row, controller_rc));
    cell_box
}

fn make_gesture_click(
    button: u32,
    col: i32,
    row: i32,
    controller_rc: &RcController,
) -> gtk::GestureClick {
    let gesture_click = gtk::GestureClick::new();
    gesture_click.set_button(button);
    gesture_click.connect_pressed(clone!(
        #[strong]
        col,
        #[strong]
        row,
        #[strong]
        controller_rc,
        move |_, n_pressed, _, _| {
            if n_pressed == 1 {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(PictureClicked { button, col, row }, &controller_rc);
                }
            } else if n_pressed == 2 {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller
                        .process_event(PictureDoubleClicked { button, col, row }, &controller_rc);
                }
            }
        }
    ));
    gesture_click
}
