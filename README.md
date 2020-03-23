# invidious-tui [![Build Status](https://travis-ci.com/dfangx/invidious-tui.svg?branch=master)](https://travis-ci.com/dfangx/invidious-tui)
invidious-tui is a TUI interface for Invidious. It aims to allow users to browse
and play videos found on YouTube/Invidious through interfacing with the
Invidious API.

## Status
The project is in its early development. Bugs will be present.

## Dependencies
* mpv (for playing videos directly from the application)
* youtube-dl (for interacting with invidio.us)

## Installation
Run `cargo build --release`. The resulting executable can be found in
`target/release/invidious-tui`.

## Usage
To run, run the executable found in the build direcotry. Invidious-tui uses the
following default keybindings:

### Global Keybindings
Key | Function
--- | --------
q | Quit
Esc | Back 
/ | Activate search
h | Previous tab
l | Next tab
j | Move selection down
k | Move selection up
z | View Invidious home
x | View search view
Space | Toggle play/pause (for audio player only)
Enter | Play selection with video (for videos and playlists)
v | Queue a selection with video (for videos and playlists)
a | Play selection audio only (for videos and playlists)
A | Queue a selection audio only (for videos and playlists)
o | Open selection
L | Loop current audio

## Configuration
Configuration is done in a file config.toml. Most configuration options are in
the works, but keybinds should work properly. Your configuration file should be
placed in `$HOME/.config/`. This feature is pretty preliminary and lots of
progress still needs to be made.

## License
This crate is licensed under the MIT/Apache license
