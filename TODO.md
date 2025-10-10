# GSR2
## Gallery Show in Rust

## Todo

- [x] upon launching with argument --directory DIRECTORY, displays the directory (full) name
- [x] upon launching with inexistent directory argument, exit with error message
- [x] upon launching with an argument that is not a directory, exit with error message
- [x] upon launching with the command: File followed by a file, displays the file name
- [x] upon launching with the command: File followed by a file that doesn't exist exit with error message
- [x] upon launching with the command: File followed by an object that is not a file exit with error message
- [x] upon launching with the command: File followed by a file that doesn't have extension jpg,jpeg or png, exit with error message
- [x] upon launching with the command: File display a single view of the picture
- [x] key q quits the application
- [x] key x toggle showing palette under the picture
- [x] key e expand the picture to the surface of the view
- [x] key f expand full view with arrow key to navigate the view
- [x] upon launching with the command: Directory followed by a directory, load pictures from that directory for display
- [x] option --thumbnails show the gallery in a 10x10 grid, looking for the picture's THUMB
- [x] option --random show the gallery in random order
- [x] in grid mode, show the picture which is the current entry
- [x] in grid mode, up and down arrow move the current entry
- [x] key a goes to beginning of page, key z to end of page
- [x] insert and retrieve Pictures in a sqlite database
- [x] when loading pictures from a directory, check the database and load image data if found
- [x] dot key put the current picture in single view and back
- [x] left click on the picture in grid view set the current position
- [x] load picture file paths and data from the database
- [x] switch from grid view to single view with controls
- [x] view in full size with key movements
- [x] width and heigth arguments
- [x] slideshow mode
- [x] any other event stops the slide show
- [x] S command put the slide show back
- [ ] edit and add a label, save it in the database and retrieve it
- [X] solve the dilemna:
    
    Controller has a View
    View holds the graphic Components
    Components respond to signal, transmitting information to Controller

    thus creating the components requires a reference to the controller
    creating the view requires the components
    creating the controller requires the view

    complete mess
    the application must be the main process, prior to anything
    the activate closure should
        build the components and attach them
        encapsulate references to these components into a MainWindow value
        initialize a Controller with this MainWindow as a refcell
        attach all event handlers with a Controller refcell
- [X] commands with two letters: 
       DD display date
       DS display size
       OD order by date
       OS order by size
       ON order by name
       OR randomize
- [X] entry window with cursor for label and jump
- [ ] make the focus character used in grid blink :
    - replace with_focus: bool by with_focus: Option<char>
    - store a RefCell of this symbol in the controlle struc
    - add timer event to alternate this symbol every second ▚ ▞
