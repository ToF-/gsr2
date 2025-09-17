use crate::Command::{Dir, File};
use crate::application_state::ApplicationState;
use crate::command_line_interface::CommandLineInterface;
use crate::control::Control;
use crate::default_values::{
    DEFAULT_HEIGHT, DEFAULT_WIDTH, PALETTE_AREA_HEIGHT, PALETTE_AREA_WIDTH, SCROLL_STEP,
};
use crate::direction::Direction;
use crate::display::title_display;
use crate::gallery::Gallery;
use crate::image_data::{Palette, get_palette_from_picture_file};
use crate::picture;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, CssProvider, Label, Orientation, Picture,
    ScrolledWindow, gdk,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GraphicalUserInterface {
    command_line_interface: CommandLineInterface,
    application_state: ApplicationState,
    application_window: gtk::ApplicationWindow,
    single_view_picture: gtk::Picture,
    single_view_scrolled_window: gtk::ScrolledWindow,
    single_view_box: gtk::Box,
    multiple_view_scrolled_window: gtk::ScrolledWindow,
    multiple_view_grid: gtk::Grid,
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
    if let Ok(mut gui) = gui_rc.try_borrow_mut()
        && let Some(key_name) = key.name()
        && let Some(control) = gui.application_state.get_control(key_name.as_str())
    {
        let refresh: bool = process_control(&mut gui, control);
        if refresh {
            set_picture_for_file_view(&gui, &gui.application_state.current_picture())
        }
    };
    gtk::Inhibit(false)
}

fn process_control(gui: &mut GraphicalUserInterface, control: Control) -> bool {
    let mut refresh: bool = true;
    match control {
        Control::MoveNext | Control::Right if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Right) {
                gui.application_state.move_towards(Direction::Right)
            } else {
                println!("bump")
            }
        }
        Control::MovePrev | Control::Left if !gui.application_state.full_size_on() => {
            if gui.application_state.can_move(Direction::Left) {
                gui.application_state.move_towards(Direction::Left)
            } else {
                println!("bump")
            }
        }
        Control::MoveLast => gui.application_state.move_towards(Direction::Last),
        Control::MoveFirst => gui.application_state.move_towards(Direction::First),
        Control::Quit => gui.application_window.close(),
        Control::TogglePalette => {
            gui.application_state.toggle_palette();
        }
        Control::ToggleExpand => {
            gui.application_state.toggle_expand();
        }
        Control::ToggleFullSize => {
            gui.application_state.toggle_full_size();
        }

        direction @ Control::Left
        | direction @ Control::Right
        | direction @ Control::Up
        | direction @ Control::Down => {
            if gui.application_state.full_size_on() {
                process_arrow_key_in_fullsize(direction, gui);
            }
            refresh = false
        }
        _ => refresh = false,
    };
    refresh
}

fn load_and_launch(gui_rc: RcRefCellGui) {
    if let Ok(mut gui) = gui_rc.try_borrow_mut() {
        let mut gallery = Gallery::new();
        let result = match &gui.command_line_interface.command {
            Some(File { file_path }) => gallery.load_from_file_path(file_path),
            Some(Dir { directory }) => gallery.load_from_directory(directory),
            None => Ok(0),
        };
        match result {
            Ok(0) => {}
            Ok(count) => {
                println!("{} picture file paths loaded", count);
                gui.application_state.set_gallery(gallery);
                set_picture_for_file_view(&gui, &gui.application_state.gallery().picture(0));
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

fn make_application_window(application: &gtk::Application) -> gtk::ApplicationWindow {
    ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build()
}

fn make_single_view_scrolled_window() -> gtk::ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .name("view")
        .build()
}

fn make_view_box() -> gtk::Box {
    gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .homogeneous(false)
        .build()
}

fn make_picture() -> gtk::Picture {
    Picture::builder().hexpand(true).vexpand(true).build()
}

fn make_view_stack() -> gtk::Stack {
    gtk::Stack::builder().hexpand(true).vexpand(true).build()
}

fn make_multiple_view_scrolled_window() -> gtk::ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .name("grid")
        .build()
}

fn make_multiple_view_grid() -> gtk::Grid {
    let grid = gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .hexpand(true)
        .vexpand(true)
        .build();
    grid.set_widget_name("multiple_view_grid");
    grid
}

fn make_multiple_view_panel() -> gtk::Grid {
    gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .hexpand(true)
        .vexpand(true)
        .build()
}

fn make_label(symbol: &str) -> gtk::Label {
    let buttons_css_provider = CssProvider::new();
    buttons_css_provider.load_from_data(
        "
            label {
                color: gray;
                font-size: 12px;
                }
            text-button {
                background-color: black;
                }
        ",
    );
    let label = Label::new(Some(symbol));
    label.set_width_chars(10);
    label.style_context().add_provider(
        &buttons_css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    label
}

pub fn activate(application: &gtk::Application, cli: &CommandLineInterface) {
    let application_window = make_application_window(application);
    let single_view_scrolled_window = make_single_view_scrolled_window();
    let view_box = make_view_box();
    let picture = make_picture();
    view_box.append(&picture);
    single_view_scrolled_window.set_child(Some(&view_box));
    let view_stack = make_view_stack();

    let _ = view_stack.add_child(&single_view_scrolled_window);
    view_stack.set_visible_child(&single_view_scrolled_window);
    application_window.set_child(Some(&view_stack));

    let multiple_view_scrolled_window = make_multiple_view_scrolled_window();
    let multiple_view_grid = make_multiple_view_grid();

    let multiple_view_panel = make_multiple_view_panel();

    let left_button = make_label("←");
    let right_button = make_label("→");

    multiple_view_panel.attach(&left_button, 0, 0, 1, 1);
    multiple_view_panel.attach(&multiple_view_grid, 1, 0, 1, 1);
    multiple_view_panel.attach(&right_button, 2, 0, 1, 1);

    let gui_rc = Rc::new(RefCell::new(GraphicalUserInterface {
        command_line_interface: cli.clone(),
        application_state: ApplicationState::new(false),
        application_window,
        single_view_picture: picture,
        single_view_box: view_box,
        single_view_scrolled_window,
        multiple_view_scrolled_window,
        multiple_view_grid,
    }));

    let evk = gtk::EventControllerKey::new();
    evk.connect_key_pressed(clone!(@strong gui_rc => move |_, key, _, _| {
        process_key(&gui_rc, key)
    }));
    if let Ok(gui) = gui_rc.try_borrow_mut() {
        gui.application_window.add_controller(evk);
    }
    load_and_launch(gui_rc);
}

pub fn build_application(cli: CommandLineInterface) -> gtk::Application {
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    application.connect_activate(move |application: &gtk::Application| activate(application, &cli));
    application
}

pub fn build_and_run_application(cli: CommandLineInterface) {
    let application = build_application(cli);
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
