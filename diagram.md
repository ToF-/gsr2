

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
        - has properties that are we can bind to labels or other widget and that can be set or get
        - has a (rc)State that it can consult and update
        - is inserted the action group of a controller so that it can activate these actions on certain events
        - has other gtkObject references that it gives access to
        - can popup a GsrEntryWindow or a GsrTreeListWindow

        no more view, the GsrApplicationWindow is the view

    Controller:
        - manage interactions between GsrApplication, State and the rest of the app
        - define actions that are activable by GsrObjects
        - lauch actions which themselves update the State and the rest of the app

    State:
        - knows and updates the current state of the word as seen by GsrObjects : view options, picture index position, etc.
        - has a (rc)Gallery that have all the pictures in the current list of pictures
        - has a (rc)Navigator that knows where the user is the list of pictures and where it can move
        - has a (rc)Selection that allows applying an action on several pictures
        

