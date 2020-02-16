# invidious-tui [![Build Status](https://travis-ci.com/dfangx/invidious-tui.svg?branch=develop)](https://travis-ci.com/dfangx/invidious-tui)
invidious-tui is a TUI interface for Invidious.

## Dependencies
* mpv (for playing videos directly from the application)

## Installation
Run `cargo build --release`. The resulting executable can be found in
`target/release/invidious-tui`.

## Usage
To run, run the executable found in the build direcotry. Invidious-tui utilizes
a modal navigation system, with a set of global keybinds that can be used across
all modes. Keybindings for each mode can be found as follows:

### Global Keybindings
Key | Function
--- | --------
/ | Activate search
q | Quit the application
Esc | Enter Normal mode if not in Normal mode

### Normal Mode
Key | Function
--- | --------
h | Move focus left
l | Move focus right
j | Move focus down
k | Move focus up
/ | Activate search
Enter | Enter Selection Mode for focused pane

### Selection Mode
Key | Function
--- | --------
j | Move selection down
k | Move selection up
Enter | Play selection with video (for videos and playlists)
a | Play selection audio only (for videos and playlists)

## Configuration
Configuration is done in a file config.toml. Most configuration options are in
the works, but keybinds should work properly. Your configuration file should be
placed in `$HOME/.config/`. This feature is pretty preliminary and lots of
progress still needs to be made.

## Planned Features
Feature | Priority | Status
------- | -------- | --------
Video view for playlists | High | In progress
Video view for channels | High | Not started
Playlist view for channels | High | Not started
Front page (popular and top videos) | Medium | Not started
User profiles | Medium | Not started
Color configuration | Low | Not started
Command-line options | Low | Not started

