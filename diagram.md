

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

