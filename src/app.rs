use tokio::runtime::Runtime;
use reqwest::Client;
use clipboard::{
    ClipboardContext,
    ClipboardProvider,
};
use crate::{
    player::Player,
    config::Config,
    data:: LoadedData,
    ui::views::{
        ViewType,
        View,
        ContentType,
    },
    invidious,
    utils,
};
use std::{
    collections::{
        HashMap,
        VecDeque,
    },
    sync::{
        Arc,
        RwLock,
    },
};

pub struct App {
    pub quit: bool,
    pub config: Config,
    pub loaded_data: LoadedData,
   
    pub cmdline_focused: bool,
    pub input: String,
    
    pub client: Client,
    pub runtime: Arc<RwLock<Runtime>>,
    
    pub player: Player,
    pub current_audio: String,
    pub next_audio: String,
    pub current_video: String,
    pub next_video: String,
    pub video_queue: VecDeque<(String, String, Option<String>)>,
    pub audio_queue: VecDeque<(String, String, Option<String>)>,
    
    pub focused_view: ViewType,
    pub view_list: HashMap<ViewType, View>,

    pub clipboard: ClipboardContext,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            cmdline_focused: false,
            input: String::new(),
            current_audio: String::from("None\n"),
            next_audio: String::from("None\n"),
            current_video: String::from("None\n"),
            next_video: String::from("None\n"),
            loaded_data: LoadedData::default(),
            client: Client::new(),
            runtime: Arc::new(RwLock::new(Runtime::new().unwrap())),
            player: Player::default(),
            quit: false,
            audio_queue: VecDeque::new(),
            video_queue: VecDeque::new(),
            
            focused_view: ViewType::Home,
            view_list: HashMap::new(),
            clipboard: ClipboardProvider::new().unwrap(),
            config,
        }
    }

    pub fn run_setup(&mut self) {
        let client = &self.client;
        let (trending, popular) = self.runtime.write().unwrap().block_on(invidious::load_home(&client)).unwrap();

        let trending_text = utils::video_to_text(trending.clone());
        let popular_text = utils::video_to_text(popular.clone());
        //let top_text = search::video_to_text(top.clone());

        if let Some(view) = self.view_list.get_mut(&ViewType::Home) {
            if let Some(mut window) = view.root_windows.get_mut(0) {
                window.content = ContentType::MediaContent(Arc::new(RwLock::new(trending_text)));
                window.selected = 0;
            }
            if let Some(mut window) = view.root_windows.get_mut(1) {
                window.content = ContentType::MediaContent(Arc::new(RwLock::new(popular_text)));
                window.selected = 0;
            }
        }
        self.loaded_data.trending_videos = trending;
        self.loaded_data.popular_videos = popular;
        //self.loaded_data.top_videos = top;
    }

    pub fn view(mut self, view_type: ViewType, view: View) -> Self {
        self.view_list.insert(view_type, view);
        self
    }
}

