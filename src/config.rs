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
    pub submit_entry: Key,
    pub back: Key,
    pub open_selection: Key,
    pub search: Key,
    pub play_pause: Key,
    pub audio_only: Key,
    pub queue_video: Key,
    pub queue_audio: Key,
    pub home_view: Key,
    pub search_view: Key,
    pub loop_audio: Key,
    pub copy_url: Key,
    pub seek_audio_forward: Key,
    pub seek_audio_backward: Key,
}

impl Default for Keybinds {
    fn default() -> Self {
        Keybinds {
            move_left: Key::Char('h'),
            move_right: Key::Char('l'),
            move_down: Key::Char('j'),
            move_up: Key::Char('k'),
            
            quit: Key::Char('q'),
            back: Key::Esc,
           
            home_view: Key::F(1),
            search_view: Key::F(2),
            search: Key::Char('/'),
            
            play_pause: Key::Char(' '),
            
            audio_only: Key::Char('a'),
            queue_audio: Key::Char('A'),
            loop_audio: Key::Char('L'),
            seek_audio_forward: Key::Right,
            seek_audio_backward: Key::Left,
            
            open_selection: Key::Char('o'),
            copy_url: Key::Char('y'),
            submit_entry: Key::Char('\n'),
            
            queue_video: Key::Char('v'),
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
    //pub enter_selection_mode: String,
    pub submit_entry: String,
    pub back: String,
    pub open_selection: String,
    pub search: String,
    pub play_pause: String,
    pub audio_only: String,
    pub queue_video: String,
    pub queue_audio: String,
    pub home_view: String,
    pub search_view: String,
    pub loop_audio: String,
    pub copy_url: String,
    pub seek_audio_forward: String,
    pub seek_audio_backward: String,
}

impl Default for KeybindsAsStr {
    fn default() -> Self {
        KeybindsAsStr {
            move_left: String::from("h"),
            move_right: String::from("l"),
            move_down: String::from("j"),
            move_up: String::from("k"),
            
            quit: String::from("q"),
            back: String::from("esc"),
            
            search: String::from("/"),
            play_pause: String::from(" "),
           
            home_view: String::from("f1"),
            search_view: String::from("f2"),
            
            audio_only: String::from("a"),
            queue_audio: String::from("A"),
            loop_audio: String::from("L"),
            
            open_selection: String::from("o"),
            submit_entry: String::from("enter"),
            copy_url: String::from("y"),
            seek_audio_forward: String::from("right"),
            seek_audio_backward: String::from("left"),
            
            queue_video: String::from("v"),
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
            //enter_selection_mode: Self::str_to_key(&mut keybinds_as_str.enter_selection_mode),
            submit_entry: Self::str_to_key(&mut keybinds_as_str.submit_entry),
            back: Self::str_to_key(&mut keybinds_as_str.back),
            open_selection: Self::str_to_key(&mut keybinds_as_str.open_selection),
            search: Self::str_to_key(&mut keybinds_as_str.search),
            play_pause: Self::str_to_key(&mut keybinds_as_str.play_pause),
            audio_only: Self::str_to_key(&mut keybinds_as_str.audio_only),
            queue_video: Self::str_to_key(&mut keybinds_as_str.queue_video),
            queue_audio: Self::str_to_key(&mut keybinds_as_str.queue_audio),
            home_view: Self::str_to_key(&mut keybinds_as_str.home_view),
            search_view: Self::str_to_key(&mut keybinds_as_str.search_view),
            loop_audio: Self::str_to_key(&mut keybinds_as_str.loop_audio),
            copy_url: Self::str_to_key(&mut keybinds_as_str.copy_url),
            seek_audio_forward: Self::str_to_key(&mut keybinds_as_str.seek_audio_forward),
            seek_audio_backward: Self::str_to_key(&mut keybinds_as_str.seek_audio_backward),
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
