//use crate::traits::{ListItem, ViewExt};
//use std::process::Command;
//use serde_json::Value;
use serde::Deserialize;
use crate::{
    player::Player,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), default)]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub length_seconds: u64,
    pub live_now: bool,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), default)]
pub struct Playlist {
    pub title: String,
    pub playlist_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), default)]
pub struct Channel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: u64,
    pub sub_count: u64,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            title: String::new(),
            video_id: String::new(),
            length_seconds: 0,
            live_now: false,
            author: String::new(),
            author_id: String::new(),
            author_url: String::new(),
        }
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            title: String::new(),
            playlist_id: String::new(),
            author: String::new(),
            author_id: String::new(),
            author_url: String::new(),
            video_count: 0,
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            author: String::new(),
            author_id: String::new(),
            author_url: String::new(),
            video_count: 0,
            sub_count: 0,
        }
    }
}

pub trait Media {
    fn open(&self) {

    }

    fn play_video(&self, _: &mut Player) {

    }

    fn play_audio(&self, _: &mut Player) {

    }

    fn title(&self) -> String {
        String::new()
    }

    fn author(&self) -> String {
        String::new()
    }
}

impl Media for Video {
    fn play_video(&self, player: &mut Player) {
        let id = &self.video_id;
        let url = format!("https://invidio.us/watch?v={}", id);
        player.play(url, true);
    }
    
    fn play_audio(&self, player: &mut Player) {
        let id = &self.video_id;
        let url = format!("https://invidio.us/watch?v={}", id);
        player.play(url, false);
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Media for Playlist {
    fn play_video(&self, _player: &mut Player) {
    }
    
    fn play_audio(&self, _player: &mut Player) {
    }
}

pub trait ListItem {
    fn into_text(&self) -> Vec<String> {
        vec![]
    }
}

impl ListItem for Video {
    fn into_text(&self) -> Vec<String> {
        let title = self.title.clone();
        let author = self.author.clone();
        let duration = if self.live_now {
            String::from("Live Now")
        }
        else {
            let seconds = self.length_seconds;
            let hours = seconds / 3600;
            let seconds = seconds % 3600;
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
        };
        vec![title, author, duration]
    }
}

impl ListItem for Playlist {
    fn into_text(&self) -> Vec<String> {
        let title = self.title.clone();
        let author = self.author.clone();
        let video_count = format!("{} videos", self.video_count);
        vec![title, author, video_count]
    }
}

impl ListItem for Channel {
    fn into_text(&self) -> Vec<String> {
        let author = self.author.clone();
        let subs = format!("{} subscribers", self.sub_count);
        let video_count = format!("{} videos", self.video_count);
        vec![author, subs, video_count]
    }
}
