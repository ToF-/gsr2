# GSR2
## Gallery Show in Rust

## Todo
- [ ] use + and - to descend / ascend the file paths ?
- [X] find fist position for label or part of a label 
- [X] do not push args on saved args if entering the same subdir again
- [X] back from a subdir to the general level, at the picture index that was left
- [X] enter into a subdir via one its picture: select the parent in the path
- [X] display label on title
- [X] option label select only pictures with LABEL
- [X] after deletion or move, reload the gallery
- [X] option --move-selected <DIRECTORY> interactively moves the selected file on confirm
- [X] i toggles information on the title
- [X] I shows information about the current picture
- [X] move takes selection / restriction into account
- [X] move all pictures starting with a a file path to a new file path (the new file path must exist, must be absolute or home based)
- [X] Tab in view mode repeat the last command for selection
- [ ] convert database of gsr V1 into gsr V2 -- not really important
- [X] list should be a command
- [X] create missing thumbnails should be a command
- [X] restrict option picture on list of tags picture selected only if having all the tags
- [X] select option pictures on list of tags : any picture having one of these tags is selected
- [X] initialize should be a command
- [X] collect should be a commandi with a directory argument, not an option, and not possible with a directory that is not starting with home dir or /
- [X] cancel the current selection
- [X] add a select command in view mode that sorts the gallery by the current sort criteria + selected tag in major. e.g sort by size becomes sort by [excluded + size], the picture not excluded will all appear first
- [X] completion when entering a label or a tag
- [X] find a picture by a pattern in the file path
- [X] show only cover pictures
- [X] mark a picture to be a cover
- [X] retrieving tags from a picture, retrieving all tags for all pictures in a big hashmap, then sets the tags in each picture image_data
- [X] adding and removing tags in grid view
- [X] if a new database file is given, and no option --initialize, print message and exit. if --initialize, create database and exit.
- [X] an environment variable defines where to find .gsr.toml file, then this toml defines all interesting variables
- [X] toggle show palette
- [X] order by palette
- [X] order by colour count
- [X] order by label
- [X] order by rank
- [X] set rank on a picture with 1,2,3 or 0
- [X] collect color information about picture files
- [X] range affects the selected flag
- [X] space selects/unselects a picture
- [X] get the number of colors in an image
- [X] confirmation box for deletion
- [X] deletion of picture data and file
- [X] apply label on a range
- [X] a control depends on key and mode
- [X] navigator can define a range
- [X] picture grid can mark pictures  50% opaque
- [X] refactor : transfer editing features to Editor
- [X] --print option only prints the picture file names and quit
- [X] the gallery loads picture file with names like pierre-soulages-peinture-19-juin-1963-1963.jpg!Large.jpg, that's a bug
    - add timer event to alternate this symbol every second ▚ ▞
    - store a RefCell of this symbol in the controlle struc
    - replace with_focus: bool by with_focus: Option<char>
- [X] make the focus character used in grid blink :
- [X] entry window with cursor for label and jump
- [X] commands with two letters: 
       ON order by name
       OD order by date
       OS order by size
       OR randomize
       DD display date
       DS display size
- [x] edit and add a label, save it in the database and retrieve it
- [x] S command put the slide show back
- [x] any other event stops the slide show
- [x] slideshow mode
- [x] width and heigth arguments
- [x] view in full size with key movements
- [x] switch from grid view to single view with controls
- [x] load picture file paths and data from the database
- [x] left click on the picture in grid view set the current position
- [x] dot key put the current picture in single view and back
- [x] when loading pictures from a directory, check the database and load image data if found
- [x] insert and retrieve Pictures in a sqlite database
- [x] key a goes to beginning of page, key z to end of page
- [x] in grid mode, up and down arrow move the current entry
- [x] in grid mode, show the picture which is the current entry
- [x] option --random show the gallery in random order
- [x] option --thumbnails show the gallery in a 10x10 grid, looking for the picture's THUMB
- [x] upon launching with the command: Directory followed by a directory, load pictures from that directory for display
- [x] key f expand full view with arrow key to navigate the view
- [x] key e expand the picture to the surface of the view
- [x] key x toggle showing palette under the picture
- [x] key q quits the application
- [x] upon launching with the command: File display a single view of the picture
- [x] upon launching with the command: File followed by a file that doesn't have extension jpg,jpeg or png, exit with error message
- [x] upon launching with the command: File followed by an object that is not a file exit with error message
- [x] upon launching with the command: File followed by a file that doesn't exist exit with error message
- [x] upon launching with the command: File followed by a file, displays the file name
- [x] upon launching with an argument that is not a directory, exit with error message
- [x] upon launching with inexistent directory argument, exit with error message
- [x] upon launching with argument --directory DIRECTORY, displays the directory (full) name

