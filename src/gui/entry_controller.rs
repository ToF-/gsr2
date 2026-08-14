pub const ENTRY_CONTROLLER_GROU_NAME: &str = "entry-controller";
pub type RcEntryController = Rc<RefCell<EntryController>>;

#[derive(Debug, Clone)]
pub struct EntryController {
    pub gio_action_group: gtk::gio::SimpleActionGroup,
    controller_opt_rc: RefCell<Option<RcController>>,
    entry_view_rc: RefCell<Option<EntryView>>,
}

impl Default for EntryController {
    fn default() -> Self {
        Self {
            gio_action_group: gtk::gio::SimpleActionGroup::new(),
            controller_opt_rc: RefCell::new(None),
            entry_view_rc: RefCell::new(None),
        }
    }
}

impl EntryController {
    pub fn new(entry_view_opt: Option<EntryView>, controller_opt: Option<RcController>) -> Self {
        let obj = Self.default();
        obj.initialize(entry_view, controller_opt);
        obj
    }

    pub fn gio_action_group(&self) -> gtk::gio::SimpleActionGroup() {
        self.gio_action_group.clone()
    }

    pub fn initialize(&self, entry_view_opt: Option<EntryView>, controller_opt: Option<RcController>) {
        *self.controller_opt_rc.borrow_mut() = controller_opt.clone();
        *self.entry_view_rc.borrow_mut() = entry_view_opt;
        let entry_view_rc: RefCell<EntryView> = self.entry_view_rc.clone();

        let mut entries = vec![];

        let activate = clone!(
            #[strong]
            entry_view_rc,
            #[strong]
            controller_opt,
            move | _group: &gtk::gio::SimpleActionGroup,
                _object: &gtk::gio::SimpleAction,
                variant: Option<&gtk::glib::Variant>| {
                    if let Some(controller_rc) = &controller_opt {
                        let controller = controller_rc.borrow_mut();
                        controller.process_entry_action(entry_view_rc, object, variant);
                    } else {
                        println!("controller not set");
                    }
                }
        );
    }


}
