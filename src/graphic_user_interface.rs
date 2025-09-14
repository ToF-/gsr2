use crate::application_state::ApplicationState;
use crate::command_line_interface::Command::File;
use crate::command_line_interface::CommandLineInterface;
use crate::default_values::{
    DEFAULT_HEIGHT, DEFAULT_WIDTH, PALETTE_AREA_HEIGHT, PALETTE_AREA_WIDTH,
};
use crate::image_data::{Palette, get_palette_from_picture_file};
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Orientation, Picture, ScrolledWindow, gdk};
use std::cell::RefCell;
use std::rc::Rc;

struct GraphicalUserInterface {
    command_line_interface: CommandLineInterface,
    application_state: ApplicationState,
    application_window: gtk::ApplicationWindow,
    single_view_picture: gtk::Picture,
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
fn set_picture_for_file_view(gui: &GraphicalUserInterface, cli: &CommandLineInterface) {
    let picture = &gui.single_view_picture;
    let view_box = &gui.single_view_box;
    picture.set_valign(Align::Center);
    picture.set_halign(Align::Center);
    picture.set_opacity(1.00);
    if let Some(File { file_name }) = &cli.command {
        println!("{}", file_name);
        picture.set_filename(Some(file_name));
        if gui.application_state.palette_on()
            && let Ok(colors) = get_palette_from_picture_file(file_name)
        {
            let palette_area = make_palette_area(colors);
            view_box.insert_child_after(&palette_area, Some(picture));
        };
    } else {
        println!("no picture file to display")
    };
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
    };
    let evk = gtk::EventControllerKey::new();
    let gui_rc = Rc::new(RefCell::new(gui));
    evk.connect_key_pressed(clone!(@strong gui_rc => move |_, key, _, _| {
        process_key(&gui_rc, key)
    }));
    if let Ok(gui) = gui_rc.try_borrow() {
        set_picture_for_file_view(&gui, cli);
        gui.application_window.add_controller(evk);
        gui.application_window.present()
    }
}

fn process_key(gui_rc: &RcRefCellGui, key: Key) -> gtk::Inhibit {
    if let Ok(gui) = gui_rc.try_borrow_mut()
        && let Some(key_name) = key.name()
        && key_name.as_str() == "q"
    {
        gui.application_window.close()
    };
    if let Ok(mut gui) = gui_rc.try_borrow_mut()
        && let Some(key_name) = key.name()
        && key_name.as_str() == "x"
    {
        gui.application_state.toggle_palette();
        let cli = gui.command_line_interface.clone();
        set_picture_for_file_view(&gui, &cli);
    };
    gtk::Inhibit(false)
}
pub fn launch_application(cli: CommandLineInterface) {
    println!("launching app…");
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    application
        .connect_activate(move |application: &gtk::Application| build_gui(application, &cli));
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
