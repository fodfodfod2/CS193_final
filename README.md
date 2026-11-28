# CS193_final - Balatre-systemone
Final for CS193, a program to use systemone ML models to play balatro

**This project is a work in progress**
# Problem Statement
The goal of this project is simply to get a systemone balatro model to play
through a few ante's of Balatro. I may have initially aimed for creating a
worthy opponent, however this has proven extremely difficult. When in doubt,
lower your standards for success.

# Program Format
## Reading from Balatro
create a small mod to dump current game state. for singleplayer games it would
likely be possible to do this using just save.jkr, however for the multiplayer
mod it's easier to just create our own state dump.
## Using the model
once we have the game state, we can assemble a list of possible actions, from
where we can query a systemone API, then perform it's action in game.
## Inputing to the game
use the BalatroBot framework to input our actions back into the game. You may be
asking why we didn't use this for the game state, the answer is, I didn't
realize this existed at that time, and at a point I'm in too deep to rewrite the
entire balatro_state crate to work with it. For now this is a future goal due to
not being able to complete this in the time remaining.

# Planned features
- [x] Support multiple systemone Api's
- [ ] Multiplayer mod support
- [ ] Training custom models
- [ ] Pit fight between multiple different models
- [ ] Rewrite balatro_state to work all through BalatroBot instead of custom mod

# A note on AI use
You may notice that this project is ~4500 lines of code, this was 95% hand
coded, The only functional rust code that was AI generated was the enums for
consumables and vouchers, where I provided the format I wanted and asked it to
copy paste from the balatro wiki, as I didn't feel like doing it my self.
\
\
For the Lua side, I understood very little about Lua before starting, and the
balatro docs are basically nonexistent and not very self explanatory. (The
reason consumable is spelled as consumeable in half the project is because it's
mispelled in the balatro source code). The vast majority of the code was still
hand written and understood by me, however I'm not going to pretend that I
didn't use claude to find where everything was in the balatro source. All system
design and organization was done by me.
\
\
I had also AI generated an explanation of the game for the system prompt as a
placeholder, however this has now been replaced with a handwritten system
prompt.
\
\
This was my first project in both Rust and Lua, rest assured that the ancient
art of writing slop without LLMs has not been lost.

# Apologies to the poor TA responsible for grading this
I may have overestimated the expected scope of this project. I realized after I
submitted the proposal that perhaps my project may have been too big, but its a
little late for that now. This project requires quite a bit of setup and
requires you to own Balatro (which if you don't its a great game) as well as set
up a ton of other stuff. I am truly sorry for the pain this will cause you
assuming you are required to get this running yourself. I severly overestimated
the required complexity of our final project and while I am proud of what I
accomplished, it probably would have been easier on all of us to just write a
100 line bash script. I have spent easily over 100 hours on this and I hope you
enjoy looking over it!

# Install Guide
## Disclaimer
This project was only tested on Arch Linux (btw) using a legally owned copy of
Balatro running through Proton using Steam, Not to say it won't work on other platforms, but that is
outside the scope of this project

## Prereqs
- A copy of Balatro (Tested with a copy owned on steam)
- [Lovely Injector](github.com/ethangreen-dev/lovely-injector), [SteamModded](https://github.com/Steamodded/smods) and [BalatroBot](https://github.com/coder/balatrobot) installed
- Cargo (rust build system/package manager)
- A api key for Typesafe.ai (Jev) with some credits loaded. OR a local model
  setup (after traditional install guide)

## How to find your Balatro Appdata folder
You will need this when installing SMODS, however make sure to write it down, on
Windows, it should be in ```~\AppData\Roaming\Balatro.``` On linux, if your run it
through proton like I do it will be at ```~/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/Mods```. Make  sure to copy down the full path (with your username) as Rust file I/O didn't work with shortcuts like ~.

## Before we start
Due to some weird reason, BalatroBot does not allow you to sell
Jokers/Consumables. Due to this we must modify 2 lines in the source code of the
mod. In your Balatro mods folder, open the folder for balatrobot, then go to ```$BALATROBOT_FOLDER/src/lua/endpoints/sell.lua``` and change:
- ```requires_state = { G.STATES.SELECTING_HAND, G.STATES.SHOP},```\
to ```  requires_state = { G.STATES.SELECTING_HAND, G.STATES.SHOP,
G.STATES.SMODS_BOOSTER_OPENED},```
- And ```local valid_state = (G.STATE == G.STATES.SHOP or G.STATE == G.STATES.SELECTING_HAND```\
to ```local valid_state = (G.STATE == G.STATES.SHOP or G.STATE == G.STATES.SELECTING_HAND or G.STATE == G.STATES.SMODS_BOOSTER_OPENED)```

## Actual Install,
- ```git clone git clone https://github.com/fodfodfod2/CS193_final.git```
- Add a symlink between your mods folder and ```CS193_final/lua/src/game_dump```
  - Your Mods folder should contain ```game_dump -> /$YOUR_INSTALL_FOLDER/CS193_final/lua/src/game_dump```
- In your project root, create a copy of .envrc.example, rename it to .envrc,
  and fill in your API key and path to your dump file(```Appdata/Roaming/Balatro/DUMP.jkr```). This file is generated by the game_dump mod and basically stores your game state. I use a utility called direnv that automatically runs this .envrc when I go into a directory, it can be installed with ```sudo pacman -S direnv```

## Getting started
Once you have everything installed, start Balatro, you should see an additional
window open containing debug logs for each mod. Before you start a run, you may
see some errors in the debug logs **This is almost certainly normal** and you
should ignore it. Now hit play and start a new run, in the debug output
terminal, you should now see log messages that say ```DUMP SUCCEEDED```. This is
the game_dump mod working! If you don't see this, double check you added the
symlink correctly. Now
go to ```$YOUR_INSTALL_FOLDER/CS193_final/rust/balatro-systemone``` and run
```cargo run```(or ```./balatro-systemone```/executable name if you downloaded from releases). The game should start playing itself! It will play until a run
wins or dies (almost definitely dies). It's not that good at the game. This was
primarily tested on lower stakes.

## How to use a local model
In order to start the server for a local model, in another terminal
```
cd local-model-laya # This should be from project root
chmod +x ./run.sh
./run.sh
```
- This uses [Laya](https://github.com/nandhakishorm/laya), A local systemone
  model.
- In order to use Laya over Jev, run with ```USE_LAYA=1 cargo run``` or ```USE_LAYA=1 ./balatro-systemone```


## Some troubleshooting
- I just get a countdown about waiting for a decision state
  - Double check you have a symlink in your mods folder, your mods folder should
    look something like this.
```
Roaming/Balatro/Mods ll
total 36
drwxr-xr-x  8 $USER $USER 4096 Nov 22 11:22 .
drwxr-xr-x  5 $USER $USER 4096 Nov 27 20:45 ..
drwxr-xr-x  8 $USER $USER 4096 Jun 11 13:21 balatrobot-1.5.2
lrwxrwxrwx  1 $USER $USER   79 Oct 27 18:38 game_dump -> /home/$USER/Documents/Classes/Fall-2026/CS193/final-project/lua/src/game_dump
drwxr-xr-x  3 $USER $USER 4096 Nov 22 11:22 logs
drwxr-xr-x  5 $USER $USER 4096 Nov 27 20:45 lovely
drwxr-xr-x  9 $USER $USER 4096 Oct  2 16:28 smods-26.1002.0
```
