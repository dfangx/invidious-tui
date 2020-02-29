# invidious-tui [![Build Status](https://travis-ci.com/dfangx/invidious-tui.svg?branch=master)](https://travis-ci.com/dfangx/invidious-tui)
invidious-tui is a TUI interface for Invidious.

## Dependencies
* mpv (for playing videos directly from the application)

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
x | View search results
Space | Toggle play/pause
Enter | Play selection with video (for videos and playlists)
a | Play selection audio only (for videos and playlists)
p | Queue selection
o | Open selection

## Configuration
Configuration is done in a file config.toml. Most configuration options are in
the works, but keybinds should work properly. Your configuration file should be
placed in `$HOME/.config/`. This feature is pretty preliminary and lots of
progress still needs to be made.

## Planned Features
Feature | Priority | Status
------- | -------- | --------
Video view for playlists | High | Complete
Video view for channels | High | Complete
Playlist view for channels | High | Complete
Front page (popular and trending videos) | Medium | Complete
User profiles | Medium | Not started
Color configuration | Low | Not started
Command-line options | Low | Not started
Video thumbnails | Low | Not started

