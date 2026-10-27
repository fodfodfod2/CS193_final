# CS193_final - Balatre-systemone
Final for CS193, a program to use systemone ML models to play balatro

**This project is a work in progress**

# Planned format
## Reading from Balatro
create a small mod to dump current game state. for singleplayer games it would
likely be possible to do this using just save.jkr, however for the multiplayer
mod it's easier to just create our own state dump.
## Using the model
once we have the game state, we can assemble a list of possible actions, from
where we can query a systemone API, then perform it's action in game.
## Inputing to the game
use a rust library that supports mouse control (like enigo) to perform the
models decision in the game
##
rinse and repeat and you have succesfully forced a computer to play a full game
of Balatro!

# Planned features
- multiplayer mod support
- training custom models
- pit fight between multiple different models
