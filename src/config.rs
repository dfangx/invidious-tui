use termion::event::Key;
use serde::Deserialize;
use failure::Error;
use std::{
    path::PathBuf,
};

#[derive(Debug)]
pub struct Keybinds {
    pub quit: Key,
    pub move_left: Key,
    pub move_right: Key,
    pub move_down: Key,
    pub move_up: Key,
    pub enter_selection_mode: Key,
    pub submit_entry: Key,
    pub enter_normal_mode: Key,
    pub open_selection: Key,
    pub search: Key,
    pub play_pause: Key,
    pub audio_only: Key,
}

impl Default for Keybinds {
    fn default() -> Self {
        Keybinds {
            quit: Key::Char('q'),
            move_left: Key::Char('h'),
            move_right: Key::Char('l'),
            move_down: Key::Char('j'),
            move_up: Key::Char('k'),
            enter_selection_mode: Key::Char('\n'),
            submit_entry: Key::Char('\n'),
            enter_normal_mode: Key::Esc,
            open_selection: Key::Char('o'),
            search: Key::Char('/'),
            play_pause: Key::Char(' '),
            audio_only: Key::Char('a'),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct KeybindsAsStr {
    pub quit: String,
    pub move_left: String,
    pub move_right: String,
    pub move_down: String,
    pub move_up: String,
    pub enter_selection_mode: String,
    pub submit_entry: String,
    pub enter_normal_mode: String,
    pub open_selection: String,
    pub search: String,
    pub play_pause: String,
    pub audio_only: String,
}

impl Default for KeybindsAsStr {
    fn default() -> Self {
        KeybindsAsStr {
            quit: String::from("q"),
            move_left: String::from("h"),
            move_right: String::from("l"),
            move_down: String::from("j"),
            move_up: String::from("k"),
            enter_selection_mode: String::from("enter"),
            submit_entry: String::from("enter"),
            enter_normal_mode: String::from("esc"),
            open_selection: String::from("o"),
            search: String::from("/"),
            play_pause: String::from(" "),
            audio_only: String::from("a"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    username: String,
    password: String,
    pub keybinds: KeybindsAsStr,

    #[serde(skip_deserializing)]
    pub keys: Keybinds,
}

impl Config {
    pub fn load_config() -> Result<Config, Error>{
        let config = Self::find_file();
        match config {
            Some(config_file) => {
                let file_content = std::fs::read_to_string(&config_file)?;
                let mut config: Config = toml::from_str(&file_content)?;
                config.keys = Self::de_keybinds(&mut config.keybinds);
                Ok(config)
            },
            None => Ok(Config::default())
        }
    }

    fn de_keybinds(keybinds_as_str: &mut KeybindsAsStr) -> Keybinds {
        Keybinds {
            quit: Self::str_to_key(&mut keybinds_as_str.quit),
            move_left: Self::str_to_key(&mut keybinds_as_str.move_left),
            move_right: Self::str_to_key(&mut keybinds_as_str.move_right),
            move_down: Self::str_to_key(&mut keybinds_as_str.move_down),
            move_up: Self::str_to_key(&mut keybinds_as_str.move_up),
            enter_selection_mode: Self::str_to_key(&mut keybinds_as_str.enter_selection_mode),
            submit_entry: Self::str_to_key(&mut keybinds_as_str.submit_entry),
            enter_normal_mode: Self::str_to_key(&mut keybinds_as_str.enter_normal_mode),
            open_selection: Self::str_to_key(&mut keybinds_as_str.open_selection),
            search: Self::str_to_key(&mut keybinds_as_str.search),
            play_pause: Self::str_to_key(&mut keybinds_as_str.play_pause),
            audio_only: Self::str_to_key(&mut keybinds_as_str.audio_only),
        }
    }

    fn str_to_key(key: &mut String) -> Key {
        match key.to_ascii_lowercase().as_str() {
            "enter" => Key::Char('\n'),
            "backspace" => Key::Backspace,
            "left" => Key::Left,
            "right" => Key::Right,
            "up" => Key::Up,
            "down" => Key::Down,
            "home" => Key::Home,
            "end" => Key::End,
            "page-up" => Key::PageUp,
            "page-down" => Key::PageDown,
            "delete" => Key::Delete,
            "insert" => Key::Insert,
            "esc" => Key::Esc,
            "space" => Key::Char(' '),
            _ => Self::modified_keys(key)
        }
    }

    fn modified_keys(key: &mut String) -> Key {
        let c = key.pop();
        let key = key.to_ascii_lowercase();
        match c {
            Some(c) => {
                if key.is_empty() {
                    Key::Char(c)
                }
                else if key == "ctrl-" {
                    Key::Ctrl(c)
                }
                else if key == "alt-" {
                    Key::Alt(c)
                }
                else if c.is_numeric() && key.len() == 1 && key == "f" {
                    Key::F(c as u8)
                }
                else {
                    Key::Null
                }
            },
            None => Key::Null,
        }
    }

    fn find_file() -> Option<PathBuf>{
        let config = dirs::config_dir();
        if let Some(mut cfg) = config {
            cfg.push("config.toml");
            return Some(cfg)
        }
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            client_id: String::new(),
            client_secret: String::new(),
            username: String::new(),
            password: String::new(),
            keys: Keybinds::default(),
            keybinds: KeybindsAsStr::default(),
        }
    }
}