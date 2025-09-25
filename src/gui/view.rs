use gtk::glib::clone;

pub struct View {
    aaplication: gtk::Application,
    application_window: gtk::ApplicationWindow,
}

impl View {
    pub fn make_view(width: usize, height: usize, cells_per_row: usize) -> Self {
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
        
        let aplication: gtk::Application  = make_application("example.org.gsr2");
        let application_window: gtk::ApplicationWindow = make_application_window(application);
        View {
            application,
            application_window,
        }
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
}

