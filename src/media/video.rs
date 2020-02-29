use serde::Deserialize;
use crate::{
    player::Player,
    media::{
        Media,
        ListItem,
    },
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

    fn queue(&self, player: &mut Player){
        let id = &self.video_id;
        let url = format!("https://invidio.us/watch?v={}", id);
        player.queue(url);
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
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
