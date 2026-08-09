

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


