

MainView : manage objects that make things visible and interactive

    |-> GridView : manage objects that make the picture grid visible and interactive

            |-> gtk::Grid
            |-> RcController

    |-> PictureFrame : manage objects that make a single picture visible and interactive
        |-> gtk::Box

    |-> gtk::ApplicationWindow : has the action group of MainController
    |-> gtk::Stack
    |-> gtk::ScrolledWindow
    |-> RcController
            ^Controller : manage all interactions between parts of the application

                

archetypes

    - gtk Objects:
        - display things and interact with the user
        - key press and other events are dispatched to Controller
        - some events like blinking or slide-show delay are managed internally

            
    - View:
        - manage objects that make things visible and interactive e.g the pictures, the input field, the title
        - has gtk objects and sets them

    - State:
        - knows and updates the current state of what is under the view, e.g view settings, picture index position, configuration…

    - Controller:
        - manage interactions between view and state
        - consult and update the repository

    - Navigator:
        - allow for navigating through the pictures

    - Editor:
        - allow for input entry or picking a choice

    - Selection:
        - allow for applying an action on several pictures

simplify this.

    GsrApplicationWindow: 
        - display things and interact with the user at the application level
        - has properties that can be bound to labels or other widget and that can be set or get
        - has a (rc)State that it can consult and update
        - is inserted the action group of a controller so that it can activate these actions on certain events
        - has other gtkObject references that it gives access to
        - can popup a GsrEntryWindow or a GsrTreeListWindow

        no more view, the GsrApplicationWindow is the view

    GsrEntryWindow:
        - prompt the user for an input and interact with them at the level of an entry
        - has a prompt propery that it displays
        - has an entry that changes according to user key presses
        - is inserted the action group of a controller so that it can activate these actions on certain events

    Controller:
        - manage interactions between GsrApplication, State and the rest of the app
        - define actions that are activable by GsrObjects
        - lauch actions which themselves update the State and the rest of the app

    State:
        - knows and updates the current state of the word as seen by GsrObjects : view options, picture index position, etc.
        - has a (rc)Gallery that have all the pictures in the current list of pictures
        - has a (rc)Navigator that knows where the user is the list of pictures and where it can move
        - has a (rc)Selection that allows applying an action on several pictures
        
examples of user interaction effects

- move left on a grid
    - the picture grid is showing 100 pictures with their labels
    - the first picture is having focus: every second a symbol just under the picture changes from ⭓ to ⭔ 
    - the window title displays some picture information about the picture having focus
    - the user presses the Left key
    - the second picture is having focus, with the blinking shape
    - the first picture is doesn't have a blinking shape anymore


- move next page on a grid
    - the picture grid is showing 100 pictures with their labels
    - the first picture is having focus: every second a symbol just under the picture changes from ⭓ to ⭔ 
    - the window title displays some picture information about the picture having focus
    - the user presses the Next Page key
    - the picture grid is showing 100 new pictures
    - the focus is on the first picture

- GsrApplicationWindow
    - can show a single picture frame when in single view mode
    - can show a picture grid with left and right panes buttons when in multiple view mode
    - can change its title
    - can popup an information window, an entry window, a tree list window, all these windows are modal

- GsrPictureFrame
    - can show a picture
    - can show the picture palette if asked to

- Navigator
    - knows which position in the gallery is the current position

- SelectionRange
    - is either unset (no start nor end position), half set (start position, no end position) or set (start position and end position)

- GsrPictureGrid 
    - knows how to set its pictures using the Navigator's position and the Gallery
    - knows which picture file and thumbnail size -- should be set 
    - knows not to create picture cells where there should not be picture (e.g last 25 of 125 pictures on a 10x10 grid)
    - knows to draw a special picture for cells where picture file could not be set
    - knows which cell should receive or leave the focus
    - knows how to ask each picture cell to display it's palette
    - knows how to ask each picture to decrease or increase opacity according if its in the current selection range
        - unset : all picture cell should be full opacity
        - half set : the start position, or end postion should be half opacity, the rest full
        - set : all the picture cell within the range should be half opacity

- GsrPictureCellBox 
    - knows how to change its label with a shape every second if it receives the focus, stop this blinking if leaves the focus
    - knows how to display or hide its palette when asked to
    - knows how to decrease or increase its opacity when asked to


entry interaction

- Controller launches a GsrEntryWindow
  with actions from controller
  with EntryEditor itself equipped with Validator and CompletionDispenser
    - GsrEntryWindow starts
        - sets its prompt and empty its input
        -
        - key_pressed follow this logic
            if Escape or Confirm : activate the matching action
            otherwise ask EntryEditor for the new content of input (and optionally prompt)
            update its input (and optionnaly prompt)

  on action Escape, controller simply closes the GsrEntryWindow
  on action Enter, controller:
    - clone the GsrEntryWindow's input value
    - close the GsrEntryWindow

Editor
    - knows its entry kind, its validator and its completion dispenser
    - validate keys entered in the GsrEntryWindow that owns it
    - given an input, and a key, returns what should be the updated input, and optionnally which action to activate
        - e.g. for entry kind = View, given input = "" and key = t, then resulting input = "Thumbs" and action is ApplyViewSetting(ViewOption::Thumbnails)
        on its key_pressed event, GsrEntryWindow
            - updates its input with the result input
            - since an action is result rather than none, activate an action, then close itself

        - e.g. for entry kind = FindLabel, if key is escape then resulting input = "" and action is Cancel 

        on its key_pressed event GsrEntryWindow
            - udptaes its input
            - since action Cancel is result, close itself
    - has a specific behavior on certain keys:
        - Escape : return empty input and launch a dismiss action
        - Return : 
            - if the given input matches the validation criteria defined by the view then return the input and the action to launch
            - otherwise if the given input can be univocally completed by the CompletionDispenser then return the completion and the action to launch
        - Tab :
            - if the given input can be univocally completed by CompletionDispenser then return the completion
            - if the given input can be completed by several candidates then return the same imput, and the candidates
            - if the given input cannot be completet then return the same input and no canditates
        
        
Note on control / action / gioaction

# Action::from_control(control: &Control) -> Self  // FOO

maps a Control to an Action, meaning an input token (a key, or a click) to an action, used by widgets in connect_key_pressed closures

# Controller::process_action(&self, simple_action: &gtk::gio::SimpleAction, variant_opt: Option<&gtk::glib::Variant>) // BAR

receive any gio simple action and its parameter activated by gtk widgets, and execute this action

# MainController::initialize(&self, controller_opt: Option<RcController>)   // LAW

creates the list of gio simple actions that can be activated by widgets adding these action entries to thir list

# Action::from(gio_action: gioaction) -> Self // QUX

maps a GioAction and its parameter to an Action

# GioAction::from(action: Action) -> Self  // GUS

maps an action and its parameter to a GioAction

to make gsr recognize a new action :
FOO : can I map a control to this action ?
GUS : is this action correctly mapped to a GioAction and its variant parameter ?
QUX : is the gio_action correctly mapped back to an Action and its parameter ?
LAW : is the main controller registering this action entry ?
FOO : is the controller acting on this action ?

what does a GsrWindow do ?

- build its own structure:
    
    ApplicationWindow   (set_view single/multiple, set_fullsize on/off set_pictures set_focus_at, set_picture_at, set_label_at, set_palette/on off)
                             [activate_action] [timeout_local (slide_show)]
        -> Stack                                            (set_visible_child)
            -> ScrolledWindow named "single_view"           (set full size on/off) [on_key_pressed (arrow keys in full size)]
                -> Box                                      (set_picture, set palette/on off)
                    -> Picture
                    -> DrawingArea
            -> ScrolledWindow name "multiple_view"
                -> Grid                                    
                    -> Label                                [on_click (prev_page)]
                    -> GsrPictureGrid                       (set_focus_at, set_picture_at, set_label_at set_palette/on off) [on_click] [on_key_pressed]

                        -> … GsrPictureCell                 (enter_focus, leave_focus) [timeout_local (focus_blink)]
                            -> Picture
                            -> Label
                            -> DrawingArea
                    -> Label                                [on_click (next_page)]
                    

- GsrPictureCell subclass gtk::Box
- GsrPictureGrid subclass gtk::Grid

- GsrApplicationWindow wrap gtk::ApplicationWindow
            

