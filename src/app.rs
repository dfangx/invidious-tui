use tokio::runtime::Runtime;
use reqwest::Client;
use failure::Error;
use crate::{
    player::Player,
    config::Config,
    utils,
    search::{
        Search,
    }
};
use std::collections::HashMap;
use std::sync::{
    Arc,
    RwLock,
};

#[derive(Clone, Debug)]
pub enum ContentType {
    MediaContent(Arc<RwLock<Vec<Vec<String>>>>),
    ListContent(Vec<String>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PaneType {
    Settings,
    Frontpage,
    Search,
    Videos,
    Channels,
    Playlists,
    SearchResults,
}

#[derive(Clone, Debug)]
pub struct Pane {
    pub title: String,
    pub selected: usize,
    pub has_focus: bool,
    pub content: ContentType,
}

impl Pane {
    fn new(title: String, selected: usize, has_focus: bool, content: ContentType) -> Self {
        Pane {
            title,
            selected,
            has_focus,
            content,
        }
    }
}

impl Default for Pane {
    fn default() -> Self {
        Pane {
            title: String::new(),
            selected: 0,
            has_focus: false,
            content: ContentType::ListContent(vec![]),
        }
    }
}

pub enum Mode {
    Normal,
    Selection,
    Input,
}

pub struct App {
    pub focused: PaneType,
    pub mode: Mode,
    pub panes: HashMap<PaneType, Pane>,
    pub quit: bool,
    pub config: Config,
    pub loaded_data: LoadedData,
    pub main_pane: PaneType,
    pub prev_focused: PaneType,
    pub input: String,
    pub client: Client,
    pub runtime: Runtime,
    pub player: Player,
    pub current_title: String,
    pub current_author: String,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        let main_pane = PaneType::Videos;
        
        Ok(App {
            input: String::new(),
            current_title: String::new(),
            current_author: String::new(),
            mode: Mode::Normal,
            focused: PaneType::Settings,
            panes: Self::init_pane_data(),
            loaded_data: LoadedData::default(),
            client: Client::new(),
            runtime: Runtime::new().unwrap(),
            player: Player::default(),
            prev_focused: main_pane,
            quit: false,
            main_pane,
            config,
        })
    }


    fn init_pane_data() -> HashMap<PaneType, Pane> {
        let settings_list= vec!["Home".to_owned(), "Help".to_owned(), "Settings".to_owned(), "User Profile".to_owned()];
        let frontpage_list = vec!["Trending".to_owned(), "Top Videos".to_owned(), "Popular".to_owned()];
        let media_list= vec!["Videos".to_owned(), "Playlists".to_owned(), "Channels".to_owned()];
        [
            (PaneType::Settings, Pane::new("Settings".to_owned(), 
                                           0, 
                                           true, 
                                           ContentType::ListContent(settings_list),
                                           )),
            (PaneType::Frontpage, Pane::new("Main Menu".to_owned(), 
                                          0, 
                                          false, 
                                          ContentType::ListContent(frontpage_list),
                                          )),
            (PaneType::Search, Pane::new("Search".to_owned(), 
                                           0, 
                                           false, 
                                           ContentType::ListContent(media_list),
                                           )),
            (PaneType::Videos, Pane::new("Videos".to_owned(), 
                                         0, 
                                         false, 
                                         ContentType::MediaContent(Arc::new(RwLock::new(vec![]))),
                                         )),
            (PaneType::Channels, Pane::new("Channels".to_owned(), 
                                           0, 
                                           false, 
                                           ContentType::MediaContent(Arc::new(RwLock::new(vec![]))),
                                           )),
            (PaneType::Playlists, Pane::new("Playlists".to_owned(), 
                                            0, 
                                            false, 
                                            ContentType::MediaContent(Arc::new(RwLock::new(vec![]))),
                                            )),
        ].iter().cloned().collect::<HashMap<PaneType, Pane>>()
    }

    pub fn move_focus_down(&mut self) {
        match self.focused {
            PaneType::Settings => {
                utils::swap_focus(self, PaneType::Frontpage);
            },
            PaneType::Frontpage => {
                utils::swap_focus(self, PaneType::Search);
            },
            _ => {},
        }
    }

    pub fn move_focus_right(&mut self) {
        match self.focused {
            PaneType::Videos 
                | PaneType::Channels 
                | PaneType::Playlists => {},
            _ => utils::swap_focus(self, self.main_pane),
        }
    }

    pub fn move_focus_left(&mut self) {
        match self.focused {
            PaneType::Videos 
                | PaneType::Playlists 
                | PaneType::Channels => utils::swap_focus(self, self.prev_focused),
            _ => {}
        }
    }

    pub fn move_focus_up(&mut self) {
        match self.focused {
            PaneType::Search => {
                utils::swap_focus(self, PaneType::Frontpage);
            },
            PaneType::Frontpage => {
                utils::swap_focus(self, PaneType::Settings);
            },
            _ => {},
        }
    }


}

use crate::media::Video;
pub struct LoadedData {
    pub search_data: Search,
    pub videos: Vec<Video>,
}

impl Default for LoadedData {
    fn default() -> Self {
        LoadedData {
            search_data: Search::default(),
            videos: vec![],
        }
    }
}
