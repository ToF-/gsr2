use crate::Command::{Dir, File};
use crate::application_state::ApplicationState;
use crate::command_line_interface::CommandLineInterface;
use crate::control::Control;
use crate::default_values::ONE_CELL_PER_ROW;
use crate::default_values::{
    DEFAULT_HEIGHT, DEFAULT_WIDTH, FOCUS_SYMBOL, PALETTE_AREA_HEIGHT, PALETTE_AREA_WIDTH,
    SCROLL_STEP,
};
use crate::direction::Direction;
use crate::display::title_display;
use crate::editor::{Editor, InputKind};
use crate::gallery::Gallery;
use crate::gui::components::*;
use crate::image_data::{Palette, get_palette_from_picture_file};
use crate::order::Order;
use crate::picture;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{self};
use gtk::{
    Align, Application, ApplicationWindow, CssProvider, Label, Orientation, Picture,
    ScrolledWindow, Text, gdk,
};
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

struct GraphicalUserInterface {
    command_line_interface: CommandLineInterface,
    application_state: ApplicationState,
    application_window: gtk::ApplicationWindow,
    single_view_picture: gtk::Picture,
    single_view_scrolled_window: gtk::ScrolledWindow,
    single_view_box: gtk::Box,
    multiple_view_scrolled_window: gtk::ScrolledWindow,
    multiple_view_grid: gtk::Grid,
    view_stack: gtk::Stack,
}

type RcRefCellGui = Rc<RefCell<GraphicalUserInterface>>;


fn make_palette_area(palette: Palette) -> gtk::DrawingArea {
    let palette_area = gtk::DrawingArea::new();
    palette_area.set_valign(Align::Center);
    palette_area.set_halign(Align::Center);
    palette_area.set_content_width(PALETTE_AREA_WIDTH);
    palette_area.set_content_height(PALETTE_AREA_HEIGHT);
    palette_area.set_draw_func(move |_, ctx, _, _| {
        draw_palette(ctx, PALETTE_AREA_WIDTH, PALETTE_AREA_HEIGHT, &palette)
    });
    palette_area
}

fn draw_palette(ctx: &Context, width: i32, height: i32, palette: &Palette) {
    const COLOR_MAX: f64 = 9.0;
    let square_size: f64 = height as f64;
    let offset: f64 = (width as f64 - (COLOR_MAX * square_size)) / 2.0;
    let surface =
        ImageSurface::create(Format::ARgb32, width, height).expect("can't create surface");
    let context = Context::new(&surface).expect("can't create context");
    for (i, color) in palette.iter().enumerate() {
        let r = color[0];
        let g = color[1];
        let b = color[2];
        context.set_source_rgb(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
        let x = i as f64 * square_size;
        context.rectangle(offset + x, 0.0, square_size, square_size);
        context.fill().expect("can't fill rectangle");
    }
    ctx.set_source_surface(&surface, 0.0, 0.0)
        .expect("can't set source surface");
    ctx.paint().expect("can't paint surface")
}

fn make_label_for_picture(gui: &GraphicalUserInterface, index: usize) -> gtk::Label {
    let focus = if index == gui.application_state.navigator().position() {
        FOCUS_SYMBOL
    } else {
        ""
    };
    let picture = gui.application_state.gallery().picture(index);
    let picture_label = match picture.image_data() {
        Some(image_data) => image_data.label(),
        None => String::from(""),
    };
    let content = format!("{}{}", focus, picture_label);
    let label = gtk::Label::new(Some(&content));
    label.set_valign(Align::Center);
    label.set_halign(Align::Center);
    label.set_widget_name("picture_label");
    label
}

fn set_label_at(col: i32, row: i32, label_content: &str, gui: &GraphicalUserInterface) {
    let widget = gui
        .multiple_view_grid
        .child_at(col, row)
        .expect("cannot find cell box in multiple view grid");
    let cell_box = widget
        .downcast::<gtk::Box>()
        .expect("cannot downcast widget to Box");
    let picture_child = cell_box.first_child();
    let label_child = match picture_child {
        Some(ref widget) => widget.next_sibling(),
        None => None,
    };
    if picture_child.is_some() {
        if let Some(widget) = label_child {
            cell_box.remove(&widget);
        };
        let label = gtk::Label::new(Some(label_content));
        label.set_valign(Align::Center);
        label.set_halign(Align::Center);
        label.set_widget_name("picture_label");
        cell_box.append(&label)
    }
}

fn set_picture_at(col: i32, row: i32, gui: &GraphicalUserInterface) {
    let coords = (row as usize, col as usize);
    let widget = gui
        .multiple_view_grid
        .child_at(col, row)
        .expect("cannot find cell box in multiple view grid");
    let cell_box = widget
        .downcast::<gtk::Box>()
        .expect("cannot downcast widget to Box");
    while let Some(child) = cell_box.first_child() {
        cell_box.remove(&child)
    }
    if let Some(index) = gui
        .application_state
        .navigator()
        .position_from_coords(coords.0, coords.1)
    {
        let picture = make_gtk_picture_from_picture(&gui.application_state, index);
        cell_box.append(&picture);
        let label = make_label_for_picture(gui, index);
        label.set_widget_name("picture_label");
        cell_box.append(&label);
    }
}

fn set_picture_for_file_view(gui: &GraphicalUserInterface, picture: &picture::Picture) {
    let single_view_picture = &gui.single_view_picture;
    let view_box = &gui.single_view_box;
    if gui.application_state.expand_on() {
        single_view_picture.set_valign(Align::Fill);
        single_view_picture.set_halign(Align::Fill);
    } else {
        single_view_picture.set_valign(Align::Center);
        single_view_picture.set_halign(Align::Center);
    }
    single_view_picture.set_opacity(1.00);
    single_view_picture.set_can_shrink(!&gui.application_state.full_size_on());
    if let Some(widget) = view_box.last_child()
        && widget != *single_view_picture
    {
        view_box.remove(&widget)
    };
    single_view_picture.set_filename(Some(picture.file_path()));
    if gui.application_state.palette_on()
        && let Ok(colors) = get_palette_from_picture_file(&picture.file_path())
    {
        let palette_area = make_palette_area(colors);
        view_box.insert_child_after(&palette_area, Some(single_view_picture));
    }
    gui.application_window
        .set_title(Some(&title_display(&gui.application_state)))
}

fn process_key(gui_rc: &RcRefCellGui, key: Key) -> gtk::Inhibit {
    let mut refresh_view_required: bool = false;
    if let Ok(mut gui) = gui_rc.try_borrow_mut() {
        if gui.application_state.editor().editing() {
            refresh_view_required = process_edition(&mut gui, key);
        } else if let Some(key_name) = key.name()
            && let Some(control) = gui.application_state.get_control(key_name.as_str())
        {
            let refresh_view_required: bool = process_control(&mut gui, control);
        }
        if refresh_view_required {
            set_view(&gui, false);
        }
    }
    gtk::Inhibit(false)
}

fn process_edition(mut gui: &mut GraphicalUserInterface, key: Key) -> bool {
    let mut refresh_view_required: bool = false;
    let mut editor: Editor = gui.application_state.editor().clone();
    if let Some(key_name) = key.name() {
        println!("{}", key_name.as_str());
        match key_name.as_str() {
            "Escape" => {
                editor.cancel_input();
            }
            "Return" => {
                let content = editor.confirm_input();
                refresh_view_required = true;
            }
            "BackSpace" => {
                editor.delete();
            }
            _ => {
                if let Some(ch) = key.to_unicode() {
                    editor.append(ch);
                }
            }
        }
    }
    gui.application_state.set_editor(editor.clone());
    if editor.editing() {
        gui.application_window.set_title(Some(&editor.input()))
    };
    refresh_view_required
}

fn process_control(gui: &mut GraphicalUserInterface, control: Control) -> bool {
    let mut refresh_view_required: bool = true;
    match control {
        Control::MoveNext if !gui.application_state.full_size_on() => {
            if gui.application_state.pictures_per_row() == 1 {
                if gui.application_state.can_move(Direction::Right) {
                    gui.application_state.move_towards(Direction::Right)
                }
            } else {
                let next_page_start = gui.application_state.navigator().next_page_start();
                if gui.application_state.can_move(Direction::Index {
                    value: next_page_start,
                }) {
                    gui.application_state.move_towards(Direction::Index {
                        value: next_page_start,
                    });
                }
            }
        }
        Control::Right if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Right) {
                gui.application_state.move_towards(Direction::Right)
            }
        }
        Control::MovePrev if !gui.application_state.full_size_on() => {
            if gui.application_state.pictures_per_row() == 1 {
                if gui.application_state.can_move(Direction::Left) {
                    gui.application_state.move_towards(Direction::Left)
                }
            } else {
                let prev_page_start = gui.application_state.navigator().prev_page_start();
                if gui.application_state.can_move(Direction::Index {
                    value: prev_page_start,
                }) {
                    gui.application_state.move_towards(Direction::Index {
                        value: prev_page_start,
                    });
                }
            }
        }
        Control::Left if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Left) {
                gui.application_state.move_towards(Direction::Left)
            }
        }
        Control::Down if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Down) {
                gui.application_state.move_towards(Direction::Down)
            }
        }
        Control::Up if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Up) {
                gui.application_state.move_towards(Direction::Up)
            }
        }
        Control::MoveEndPage => gui.application_state.move_towards(Direction::PageEnd),
        Control::MoveStartPage => gui.application_state.move_towards(Direction::PageStart),
        Control::MoveLast => gui.application_state.move_towards(Direction::Last),
        Control::MoveFirst => gui.application_state.move_towards(Direction::First),
        Control::Quit => {
            gui.application_window.close();
            refresh_view_required = false
        }
        Control::TogglePalette => {
            gui.application_state.toggle_palette();
        }
        Control::ToggleExpand => {
            gui.application_state.toggle_expand();
        }
        Control::ToggleFullSize => {
            gui.application_state.toggle_full_size();
        }
        Control::ToggleSingleView => {
            gui.application_state.toggle_single_view();
        }
        Control::Label => {
            gui.application_state.set_editor({
                let mut editor = gui.application_state.editor().clone();
                editor.begin_input(InputKind::Label);
                editor
            });
        }
        direction @ Control::Left
        | direction @ Control::Right
        | direction @ Control::Up
        | direction @ Control::Down => {
            if gui.application_state.full_size_on() {
                process_arrow_key_in_fullsize(direction, gui);
            }
            refresh_view_required = false
        }
        _ => refresh_view_required = false,
    };
    refresh_view_required
}

fn set_picture_for_single_view(gui: &GraphicalUserInterface) {
    set_picture_for_file_view(gui, &gui.application_state.current_picture());
}

fn set_label_for_picture_at_new_coords(gui: &GraphicalUserInterface) {
    let navigator = gui.application_state.navigator();
    let old_position = navigator.old_position();
    let new_position = navigator.position();
    let old_coords = navigator.coords_from_position(old_position).unwrap();
    let new_coords = navigator.coords_from_position(new_position).unwrap();
    let old_label = gui
        .application_state
        .gallery()
        .picture(old_position)
        .label();
    let new_label = format!(
        "{} {}",
        FOCUS_SYMBOL,
        gui.application_state
            .gallery()
            .picture(new_position)
            .label()
    );
    set_label_at(old_coords.1 as i32, old_coords.0 as i32, &old_label, gui);
    set_label_at(new_coords.1 as i32, new_coords.0 as i32, &new_label, gui);
}
fn set_picture_for_multiple_view(gui: &GraphicalUserInterface, pictures_per_row: i32) {
    for col in 0..pictures_per_row {
        for row in 0..pictures_per_row {
            set_picture_at(col, row, gui)
        }
    }
}

fn single_view_mode(gui: &GraphicalUserInterface) -> bool {
    let child = gui
        .view_stack
        .visible_child()
        .expect("view stack has no child");
    child == gui.single_view_scrolled_window
}

fn set_view(gui: &GraphicalUserInterface, initial: bool) {
    let cells_per_row = gui.application_state.pictures_per_row();
    let view_has_changed: bool = (cells_per_row == ONE_CELL_PER_ROW) != single_view_mode(gui);

    if initial || view_has_changed {
        if cells_per_row == ONE_CELL_PER_ROW {
            gui.multiple_view_scrolled_window.set_visible(false);
            gui.single_view_scrolled_window.set_visible(true);
            gui.view_stack
                .set_visible_child(&gui.single_view_scrolled_window);
        } else {
            gui.single_view_scrolled_window.set_visible(false);
            gui.multiple_view_scrolled_window.set_visible(true);
            gui.view_stack
                .set_visible_child(&gui.multiple_view_scrolled_window);
            set_picture_for_multiple_view(gui, cells_per_row as i32)
        }
    }
    if cells_per_row == ONE_CELL_PER_ROW {
        set_picture_for_single_view(gui)
    } else if gui.application_state.navigator().page_changed() {
        set_picture_for_multiple_view(gui, cells_per_row as i32)
    } else if gui.application_state.navigator().has_moved() {
        set_label_for_picture_at_new_coords(gui)
    };
    gui.application_window
        .set_title(Some(&title_display(&gui.application_state)));
}

fn load_and_launch(gui_rc: RcRefCellGui) {
    if let Ok(mut gui) = gui_rc.try_borrow_mut() {
        let mut gallery = Gallery::new();
        let result = match &gui.command_line_interface.command {
            Some(File { file_path }) => gallery.load_from_file_path(file_path),
            Some(Dir { directory }) => gallery.load_from_directory(directory),
            None => gallery.load_from_database(gui.application_state.database()),
        };
        if gui.command_line_interface.random {
            gallery.sort_by(Order::Random)
        } else {
            gallery.sort_by(Order::Name)
        };
        match result {
            Ok(0) => {}
            Ok(_) => {
                let cells_per_row: usize = (gui.command_line_interface).cells_per_row() as usize;
                gui.application_state.set_gallery(gallery, cells_per_row);
                set_view(&gui, true);
                gui.application_window.present()
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

fn process_arrow_key_in_fullsize(direction: Control, gui: &GraphicalUserInterface) -> bool {
    let (picture_adjustment, step) = match direction {
        Control::Right => (gui.single_view_scrolled_window.hadjustment(), SCROLL_STEP),
        Control::Left => (gui.single_view_scrolled_window.hadjustment(), -SCROLL_STEP),
        Control::Down => (gui.single_view_scrolled_window.vadjustment(), SCROLL_STEP),
        Control::Up => (gui.single_view_scrolled_window.vadjustment(), -SCROLL_STEP),
        _ => return false,
    };
    picture_adjustment.set_value(picture_adjustment.value() + step);
    false
}

pub fn startup_gui(_application: &gtk::Application) {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(
        "window { background-color:black;} image { margin:1em ; } label { color:white; }",
    );
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        1000,
    );
}

fn make_gtk_picture_from_picture(
    application_state: &ApplicationState,
    index: usize,
) -> gtk::Picture {
    let gtk_picture = gtk::Picture::new();
    gtk_picture.set_halign(Align::Center);
    gtk_picture.set_valign(Align::Center);
    gtk_picture.set_opacity(1.00);
    gtk_picture.set_can_shrink(!application_state.full_size_on());
    let file_path = if application_state.thumbnails_on() {
        application_state
            .gallery()
            .picture(index)
            .thumbnail_file_path()
    } else {
        application_state.gallery().picture(index).file_path()
    };
    gtk_picture.set_filename(Some(file_path));
    gtk_picture.set_visible(true);
    gtk_picture
}

fn make_application_window(application: &gtk::Application) -> gtk::ApplicationWindow {
    ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build()
}

fn make_text() -> gtk::Text {
    Text::builder().build()
}

pub fn activate(application: &gtk::Application, cli_rc: &Rc<RefCell<CommandLineInterface>>) {
    let command_line_interface = match cli_rc.try_borrow() {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let application_window = make_application_window(application);
    let single_view_scrolled_window = make_single_view_scrolled_window();
    let view_box = make_view_box();
    let picture = make_picture();
    view_box.append(&picture);
    single_view_scrolled_window.set_child(Some(&view_box));

    let multiple_view_scrolled_window = make_multiple_view_scrolled_window();
    let multiple_view_grid = make_multiple_view_grid();

    let multiple_view_panel = make_multiple_view_panel();

    multiple_view_scrolled_window.set_child(Some(&multiple_view_panel));

    let left_button = make_label("←");
    let right_button = make_label("→");

    multiple_view_panel.attach(&left_button, 0, 0, 1, 1);
    multiple_view_panel.attach(&multiple_view_grid, 1, 0, 1, 1);
    multiple_view_panel.attach(&right_button, 2, 0, 1, 1);

    let view_stack = make_view_stack();
    let _ = view_stack.add_child(&single_view_scrolled_window);
    let _ = view_stack.add_child(&multiple_view_scrolled_window);
    if command_line_interface.cells_per_row() == 1 {
        view_stack.set_visible_child(&single_view_scrolled_window);
    } else {
        view_stack.set_visible_child(&multiple_view_scrolled_window);
    }
    let cells_per_row: i32 = command_line_interface.cells_per_row();
    for col in 0..cells_per_row {
        for row in 0..cells_per_row {
            let cell_box = make_cell_box();
            multiple_view_grid.attach(&cell_box, col, row, 1, 1);
        }
    }
    application_window.set_child(Some(&view_stack));
    let gui_rc = match ApplicationState::new() {
        Ok(application_state) => Rc::new(RefCell::new(GraphicalUserInterface {
            command_line_interface: command_line_interface.clone(),
            application_state,
            application_window,
            single_view_picture: picture,
            single_view_box: view_box,
            single_view_scrolled_window,
            multiple_view_scrolled_window,
            multiple_view_grid,
            view_stack,
        })),
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    let evk = gtk::EventControllerKey::new();
    evk.connect_key_pressed(clone!(@strong gui_rc => move |_, key, _, _| {
        process_key(&gui_rc, key)
    }));
    if let Ok(gui) = gui_rc.try_borrow() {
        gui.application_window.add_controller(evk)
    };
    let left_gesture = gtk::GestureClick::new();
    left_gesture.set_button(1);
    left_gesture.connect_pressed(clone!(@strong gui_rc => move |_,_,_,_| {
        {
            if let Ok(mut gui) = gui_rc.try_borrow_mut() {
                let prev_page_start = gui.application_state.navigator().prev_page_start();
                if gui.application_state.can_move(Direction::Index {
                    value: prev_page_start,
                }) {
                    gui.application_state.move_towards(Direction::Index {
                        value: prev_page_start,
                    });
                };
                set_view(&gui, false)
            }
        }
    }));
    left_button.add_controller(left_gesture);
    let right_gesture = gtk::GestureClick::new();
    right_gesture.set_button(1);
    right_gesture.connect_pressed(clone!(@strong gui_rc => move |_,_,_,_| {
        {
            if let Ok(mut gui) = gui_rc.try_borrow_mut() {
                let next_page_start = gui.application_state.navigator().next_page_start();
                if gui.application_state.can_move(Direction::Index {
                    value: next_page_start,
                }) {
                    gui.application_state.move_towards(Direction::Index {
                        value: next_page_start,
                    });
                };
                set_view(&gui, false)
            }
        }
    }));
    right_button.add_controller(right_gesture);
    if let Ok(gui) = gui_rc.try_borrow() {
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let widget = gui
                    .multiple_view_grid
                    .child_at(col, row)
                    .expect("can't locate cell box");
                let cell_box = widget
                    .downcast::<gtk::Box>()
                    .expect("cannot downcast widget to Box");
                let gesture_left = gtk::GestureClick::new();
                gesture_left.set_button(1);
                gesture_left.connect_pressed(clone!(@strong gui_rc => move |_,n_pressed,_,_| {
                if let Ok(mut gui) = gui_rc.try_borrow_mut()
                    && let Some(index) = gui.application_state.navigator().position_from_coords(row as usize, col as usize) {
                        match n_pressed {
                            1 => {
                                gui.application_state.move_towards(Direction::Index {
                                    value: index,
                                });
                                set_view(&gui, false)
                            },
                            2 => {
                                gui.application_state.move_towards(Direction::Index {
                                    value: index,
                                });
                                gui.application_state.toggle_single_view();
                                set_view(&gui, true)
                            }
                            _ => {}
                        }
                    }
            }));
                cell_box.add_controller(gesture_left);
            }
        }
    };
    if let Ok(gui) = gui_rc.try_borrow() {
        let gesture_left = gtk::GestureClick::new();
        gesture_left.set_button(1);
        gesture_left.connect_pressed(clone!(@strong gui_rc => move |_,_,_,_| {
            if let Ok(mut gui) = gui_rc.try_borrow_mut() {
                gui.application_state.toggle_single_view();
                set_view(&gui, true)
            }
        }));
        gui.single_view_picture.add_controller(gesture_left);
    };
    load_and_launch(gui_rc);
}

pub fn build_and_run_application(cli: CommandLineInterface) {
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    let cli_rc = Rc::new(RefCell::new(cli));
    // clone! passes a strong reference to a variable in the closure that activates the application
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    application.connect_activate(
        clone!(@strong cli_rc, => move |application: &gtk::Application| {
        activate(application, &cli_rc); }),
    );
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
