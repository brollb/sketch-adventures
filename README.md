# Sketch Adventures

This is a game made for Global Game Jam 2018. One of the main goals was to learn rust so it is a little less than polished ;)

Regardless, it is a game that allows users to move with the arrow keys and draw with the mouse. The drawing is then classified using [this model](https://github.com/neungkl/quickdraw-10-CNN-classifier) and then the given entity is created in the game world. Clocks are pretty much the only one with any sort of meaningful behavior but it was still an interesting idea imo.

## Installation
First, you need to install the python dependencies from `requirements.txt`. Then, you can compile this using rust (we recommend nightly-2017-12-21):

    pip install -r requirements.txt
    cargo run --release

## To Do:
- [x] Draw on the screen
- [x] Move the stick figure with arrow keys
- [x] Game update loop
- [x] Draw the ground
- [x] drawing on the screen
- [x] Allow the user to draw on the screen

- [ ] Draw a stick figure (use an image?)
- [ ] Add physics (should be provided by piston, I think)
    - not really necessary

- [x] Get the user-drawn image
- [x] Call python classifier on the image
- How can we get the dirty part of the screen?

- Draw character on screen. Awesome idea

- [ ] collision detection
    - hacked something rough in...
- [x] text on the screen?
    - What types are missing?


- [x] add the intro scene
- [x] fix the font

- add walking animation?
    - lower priority...

## Images to detect:
- [x] lightning?
    - collision detection??
- [x] clock
    - [x] freeze time

