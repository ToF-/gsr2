use crate::Command::{Dir, File};
use crate::application_state::ApplicationState;
use crate::command::Command;
use crate::command_line_interface::CommandLineInterface;
use crate::control::Control;
use crate::default_values::{
    DEFAULT_HEIGHT, DEFAULT_WIDTH, PALETTE_AREA_HEIGHT, PALETTE_AREA_WIDTH, SCROLL_STEP,
};
use crate::direction::Direction;
use crate::display::title_display;
use crate::gallery::Gallery;
use crate::image_data::{Palette, get_palette_from_picture_file};
use crate::navigator;
use crate::picture;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Orientation, Picture, ScrolledWindow, gdk};
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
}

type RcRefCellGui = Rc<RefCell<GraphicalUserInterface>>;

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
fn set_picture_for_file_view(
    gui: &GraphicalUserInterface,
    picture: &picture::Picture,
    cli: &CommandLineInterface,
) {
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
pub fn build_gui(application: &gtk::Application, cli: &CommandLineInterface) {
    let application_window = ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build();
    let single_view_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .name("view")
        .build();
    let view_box = gtk::Box::new(Orientation::Vertical, 0);
    view_box.set_valign(Align::Fill);
    view_box.set_halign(Align::Fill);
    view_box.set_hexpand(true);
    view_box.set_vexpand(true);
    view_box.set_homogeneous(false);

    let picture = Picture::new();
    picture.set_hexpand(true);
    picture.set_vexpand(true);

    view_box.append(&picture);
    single_view_scrolled_window.set_child(Some(&view_box));

    let view_stack = gtk::Stack::new();
    view_stack.set_hexpand(true);
    view_stack.set_vexpand(true);
    let _ = view_stack.add_child(&single_view_scrolled_window);
    view_stack.set_visible_child(&single_view_scrolled_window);
    application_window.set_child(Some(&view_stack));

    let gui = GraphicalUserInterface {
        command_line_interface: cli.clone(),
        application_state: ApplicationState::new(false),
        application_window,
        single_view_picture: picture,
        single_view_box: view_box,
        single_view_scrolled_window,
    };
    let evk = gtk::EventControllerKey::new();
    let gui_rc = Rc::new(RefCell::new(gui));
    evk.connect_key_pressed(clone!(@strong gui_rc => move |_, key, _, _| {
        process_key(&gui_rc, key)
    }));
    if let Ok(mut gui) = gui_rc.try_borrow_mut() {
        gui.application_window.add_controller(evk);
    }
    load_and_launch(gui_rc, cli);
}

fn load_and_launch(gui_rc: RcRefCellGui, cli: &CommandLineInterface) {
    if let Ok(mut gui) = gui_rc.try_borrow_mut() {
        let mut gallery = Gallery::new();
        if let Some(File { file_path }) = &gui.command_line_interface.command {
            match gallery.load_from_file_path(file_path) {
                Ok(count) => {
                    println!("{} picture file paths loaded", count);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        } else if let Some(Dir { directory }) = &gui.command_line_interface.command {
            println!("loading…");
            match gallery.load_from_directory(directory) {
                Ok(count) => {
                    println!("{} picture file path loaded", count);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        gui.application_state.set_gallery(gallery);
        set_picture_for_file_view(&gui, &gui.application_state.gallery().picture(0), cli);
        gui.application_window.present()
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

fn process_key(gui_rc: &RcRefCellGui, key: Key) -> gtk::Inhibit {
    if let Ok(mut gui) = gui_rc.try_borrow_mut()
        && let Some(key_name) = key.name()
    {
        let mut picture: picture::Picture = gui.application_state.gallery().picture(0);
        let cli = gui.command_line_interface.clone();
        let mut refresh: bool = true;
        match gui.application_state.get_control(key_name.as_str()) {
            Some(Control::MoveNext) | Some(Control::Right)
                if !gui.application_state.full_size_on() =>
            {
                if gui.application_state.navigator().can_move(Direction::Right) {
                    gui.application_state.move_towards(Direction::Right)
                } else {
                    println!("bump")
                }
            }
            Some(Control::MovePrev) | Some(Control::Left)
                if !gui.application_state.full_size_on() =>
            {
                if gui.application_state.navigator().can_move(Direction::Left) {
                    gui.application_state.move_towards(Direction::Left)
                } else {
                    println!("bump")
                }
            }
            Some(Control::MoveLast) => gui.application_state.move_last(),
            Some(Control::MoveFirst) => gui.application_state.move_first(),
            Some(Control::Quit) => gui.application_window.close(),
            Some(Control::TogglePalette) => {
                gui.application_state.toggle_palette();
            }
            Some(Control::ToggleExpand) => {
                gui.application_state.toggle_expand();
            }
            Some(Control::ToggleFullSize) => {
                gui.application_state.toggle_full_size();
            }

            Some(direction @ Control::Left)
            | Some(direction @ Control::Right)
            | Some(direction @ Control::Up)
            | Some(direction @ Control::Down) => {
                if gui.application_state.full_size_on() {
                    process_arrow_key_in_fullsize(direction, &gui);
                }
                refresh = false
            }
            _ => {
                println!("{:?}", key_name);
                refresh = false
            }
        };
        if refresh {
            let position = gui.application_state.navigator().position();
            picture = gui.application_state.gallery().picture(position);
            set_picture_for_file_view(&gui, &gui.application_state.current_picture(), &cli)
        }
    };
    gtk::Inhibit(false)
}

pub fn build_application(cli: CommandLineInterface) -> gtk::Application {
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    application
        .connect_activate(move |application: &gtk::Application| build_gui(application, &cli));
    application
}

pub fn build_and_run_application(cli: CommandLineInterface) {
    let application = build_application(cli);
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
