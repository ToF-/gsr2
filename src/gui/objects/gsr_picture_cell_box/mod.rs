use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::action::gio_action::SimpleActionCall;
use crate::gui::main_controller::MainController;
use glib::Variant;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Align, Orientation};

mod imp;

glib::wrapper! {
    pub struct GsrPictureCellBox(ObjectSubclass<imp::GsrPictureCellBox>)
        @extends gtk::Widget, gtk::Box,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureCellBox {
    pub fn new(col: i32, row: i32) -> Self {
        let obj: Self = glib::Object::new();
        obj.initialize();
        obj.imp().col.set(col);
        obj.imp().row.set(row);
        obj
    }
}
impl GsrPictureCellBox {
    pub fn initialize(&self) {
        self.set_orientation(Orientation::Vertical);
        self.set_spacing(0);
        self.set_valign(Align::Center);
        self.set_halign(Align::Center);
        self.set_hexpand(true);
        self.set_vexpand(true);
    }

    pub fn receive_focus(&self) {
        self.imp().has_focus.set(true);
    }

    pub fn leave_focus(&self) {
        self.imp().has_focus.set(false);
    }

    pub fn connect_main_controller(&self, main_controller: &MainController) {
        self.insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
        let col = self.imp().col.get();
        let row = self.imp().row.get();
        let left_click_action_call =
            GioAction::from(Action::FocusAt(col, row)).to_simple_action_call();
        let right_click_action_call =
            GioAction::from(Action::ToggleSelectedAt(col, row)).to_simple_action_call();
        self.add_controller(self.make_gesture_click(1, left_click_action_call));
        self.add_controller(self.make_gesture_click(3, right_click_action_call));
    }

    fn make_gesture_click(
        &self,
        button: u32,
        action_call: SimpleActionCall
    ) -> gtk::GestureClick {
        let gesture_click = gtk::GestureClick::new();
        gesture_click.set_button(button);
        gesture_click.connect_pressed(clone!(
            #[strong(rename_to = this)]
            self,
            #[strong]
            action_call,
            move |_, n_pressed, _, _| {
                let name = action_call.0.clone();
                let variant = action_call.1.clone();
                let variant_ref: Option<&Variant> = match &variant {
                    None => None,
                    Some(v) => Some(v.as_ref()),
                };
                if n_pressed == 1 {
                    match this.activate_action(&name, variant_ref) {
                        Ok(_) => {}
                        Err(e) => panic!("{e}"),
                    }
                } else {
                    println!("double click not yet implemented")
                }
            }
        ));
        gesture_click
    }
}
