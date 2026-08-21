use crate::env::default_values::FOCUS_BLINKING_DURATION;
use crate::env::default_values::FOCUS_SYMBOL_1;
use crate::env::default_values::FOCUS_SYMBOL_2;
use crate::env::default_values::GRID_PALETTE_AREA_HEIGHT;
use crate::env::default_values::GRID_PALETTE_AREA_WIDTH;
use crate::file::paths::check_path_exists;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::action::gio_action::SimpleActionCall;
use crate::gui::display::picture_label_display;
use crate::gui::main_controller::MainController;
use crate::gui::view::palette_area::make_palette_area;
use crate::model::palette::Palette;
use crate::model::picture::Picture;
use crate::model::thumbnail::no_thumbnail_picture;
use glib::Variant;
use glib::clone;
use gtk::Box as GtkBox;
use gtk::Label as GtkLabel;
use gtk::Picture as GtkPicture;
use gtk::gio::File as GtkFile;
use gtk::glib;
use gtk::glib::ControlFlow;
use gtk::glib::subclass::prelude::*;
use gtk::glib::timeout_add_local;
use gtk::prelude::*;
use gtk::{Align, Orientation};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

mod imp;

glib::wrapper! {
    pub struct GsrPictureCellBox(ObjectSubclass<imp::GsrPictureCellBox>)
        @extends gtk::Widget, GtkBox,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureCellBox {
    pub fn new(
        col: i32,
        row: i32,
        picture_index: usize,
        pictures_per_row: i32,
        palette_on: bool,
    ) -> Self {
        let obj: Self = glib::Object::new();
        obj.initialize();
        obj.imp().col.set(col);
        obj.imp().row.set(row);
        obj.imp().picture_index.set(picture_index);
        obj.imp().pictures_per_row.set(pictures_per_row);
        obj.imp().palette_on.set(palette_on);
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

    pub fn enter_focus(&self) {
        self.imp().has_focus.set(true);
        let label_rc = self.imp().label.clone();
        label_rc.borrow().as_ref().map(flip_focus_symbol_on_label);
        self.attach_focus_blink_event();
    }

    fn attach_focus_blink_event(&self) {
        let label_rc = self.imp().label.clone();
        *self.imp().timeout_rc.borrow_mut() = Some(timeout_add_local(
            Duration::from_millis(FOCUS_BLINKING_DURATION),
            clone!(
                #[strong]
                label_rc,
                move || {
                    label_rc.borrow().as_ref().map(flip_focus_symbol_on_label);
                    ControlFlow::Continue
                }
            ),
        ));
    }

    pub fn leave_focus(&self) {
        self.imp().has_focus.set(false);
        self.detach_focus_blink_event();
        let label_rc = self.imp().label.clone();
        label_rc
            .borrow()
            .as_ref()
            .map(remove_focus_symbol_from_label);
    }

    fn detach_focus_blink_event(&self) {
        if let Some(id) = self.imp().timeout_rc.borrow_mut().take() {
            id.remove();
        }
    }

    fn remove_children(&self) {
        while let Some(child) = self.first_child() {
            self.remove(&child)
        }
        *self.imp().label.borrow_mut() = None;
    }

    pub fn set_label(&self, text: &str) {
        let label_opt = self.imp().label.borrow().clone();
        if let Some(label) = label_opt {
            label.set_text(text);
        }
    }

    pub fn set_label_from_picture(&self, picture: &Picture) {
        let text = picture_label_display(
            &picture.label(),
            picture.rank(),
            picture.cover(),
            None, // focus will be inserted / flipped / removed directly on the GtkLabel
            picture.file_size(),
        );
        self.set_label(&text);
    }

    fn append_palette(&self, palette_opt: Option<Palette>) {
        if let Some(palette) = palette_opt {
            self.append(&make_palette_area(
                palette.sample(),
                GRID_PALETTE_AREA_WIDTH,
                GRID_PALETTE_AREA_HEIGHT,
            ))
        }
    }
    pub fn attach_picture(&self, picture: &Picture, picture_index: usize) {
        self.remove_children();
        let picture_file_path = picture.view_file_path(self.imp().pictures_per_row.get() as usize);
        self.append(&make_picture(&picture_file_path));
        let label = make_label(&picture_label_display(
            &picture.label(),
            picture.rank(),
            picture.cover(),
            None, // focus will be inserted / flipped / removed directly on the GtkLabel
            picture.file_size(),
        ));
        self.append(&label);
        *self.imp().label.borrow_mut() = Some(label);
        if self.imp().palette_on.get() {
            self.append_palette(picture.palette())
        };
        self.imp().picture_index.set(picture_index);
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

    fn make_gesture_click(&self, button: u32, action_call: SimpleActionCall) -> gtk::GestureClick {
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
fn make_label(text: &str) -> GtkLabel {
    GtkLabel::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .label(text)
        .build()
}

fn make_picture(picture_file_path: &str) -> GtkPicture {
    if let Ok(file_path) = check_path_exists(&PathBuf::from(picture_file_path)) {
        gtk_picture_from_file_path(file_path)
    } else {
        no_thumbnail_picture()
    }
}

// flip or insert the focus symbol on the label
// empty -> ⭓
// ⭓ foo -> ⭔ foo
// ⭔ foo -> ⭓ foo
// bar -> ⭓ bar

fn flip_focus_symbol_on_label(label: &GtkLabel) {
    label.set_text(&flip_focus_symbol(&label.text().to_string()));
}

fn flip_focus_symbol(label_text: &str) -> String {
    let mut text = label_text.to_string();
    if !text.is_empty() {
        let first_char = text.remove(0);
        match first_char {
            FOCUS_SYMBOL_1 => text.insert(0, FOCUS_SYMBOL_2),
            FOCUS_SYMBOL_2 => text.insert(0, FOCUS_SYMBOL_1),
            first_label_char => {
                text.insert(0, first_label_char);
                text.insert(0, ' ');
                text.insert(0, FOCUS_SYMBOL_1);
            }
        }
    } else {
        text.insert(0, FOCUS_SYMBOL_1);
    }
    text.to_string()
}

// remove the focus symbol from the label
// empty -> empty
// ⭓ foo -> foo
// ⭔ foo -> foo
// bar -> bar
fn remove_focus_symbol_from_label(label: &GtkLabel) {
    label.set_text(&remove_focus_symbol(&label.text().to_string()));
}

fn remove_focus_symbol(label_text: &str) -> String {
    let mut text = label_text.to_string();
    if !text.is_empty() {
        let first_char = text.remove(0);
        match first_char {
            FOCUS_SYMBOL_1 | FOCUS_SYMBOL_2 => {
                if !text.is_empty() {
                    let first_char = text.remove(0);
                    match first_char {
                        ' ' => {}
                        first_label_char => {
                            text.insert(0, first_label_char);
                        }
                    }
                }
            }
            first_label_char => text.insert(0, first_label_char),
        };
    }
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flipping_or_inserting_the_focus_symbol() {
        assert_eq!("⭓", flip_focus_symbol(""));
        assert_eq!("⭓ foo", flip_focus_symbol("foo"));
        assert_eq!("⭔ foo", flip_focus_symbol("⭓ foo"));
        assert_eq!("⭓ foo", flip_focus_symbol("⭔ foo"));
    }
    #[test]
    fn removing_the_focus_symbol() {
        assert_eq!("foo", remove_focus_symbol("⭓ foo"));
        assert_eq!("foo", remove_focus_symbol("⭔ foo"));
        assert_eq!("foo", remove_focus_symbol("foo"));
        assert_eq!("", remove_focus_symbol(""));
    }
}

fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
    GtkPicture::builder()
        .file(&GtkFile::for_path(file_path))
        .hexpand(true)
        .vexpand(true)
        .build()
}
