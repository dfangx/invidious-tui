//use crate::traits::{ListItem, ViewExt};
//use std::process::Command;
//use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub length_seconds: u64,
    pub live_now: bool,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
}

/*
impl Video {
    pub fn new(video: Value) -> Self {
        let live_now = if let Some(v) = video["liveNow"].as_bool() {
            v
        }
        else {
            false
        };

        let length_seconds = if live_now {
            0
        }
        else {
            video["lengthSeconds"].as_u64().unwrap()
        };
        
        Video {
            title: video["title"].as_str().unwrap().to_string(),
            video_id: video["videoId"].as_str().unwrap().to_string(),
            length_seconds,
            live_now,
            author: video["author"].as_str().unwrap().to_string(),
            author_id: video["authorId"].as_str().unwrap().to_string(),
            author_url: video["authorUrl"].as_str().unwrap().to_string(),
        }
    }
}
impl ListItem for Video {
    fn right_text(&self) -> String {
        if self.live_now {
            "Live".to_string()
        }
        else {
            let hours = self.length_seconds / 3600;
            let minutes = self.length_seconds % 3600 /  60;
            let seconds = self.length_seconds % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
    }

    fn left_text(&self) -> String {
        let text = format!("[{}] {}", self.author, self.title);
        text
    }

    fn play(&self) {
        let url = "https://www.invidio.us/watch?v=".to_owned() + &self.video_id;
        std::thread::spawn(move || {
            Command::new("mpv")
                .arg(&url)
                .output()
                .expect("Failed to play video");
        });
    }

    fn open(&self) -> Option<Box<dyn ViewExt>>{
        None
    }

    fn download(&self) {
        let url = "https://www.invidio.us/watch?v=".to_owned() + &self.video_id;
        std::thread::spawn(move || {
            Command::new("youtube-dl")
                .arg("-o")
                .arg("~/downloads/%(title)s.%(ext)s")
                .arg(&url)
                .output()
                .expect("Failed to download video");
        });
    }

    fn download_audio_only(&self) {
        let url = "https://www.invidio.us/watch?v=".to_owned() + &self.video_id;
        std::thread::spawn(move || {
            Command::new("youtube-dl")
                .arg("-o")
                .arg("~/downloads/%(title)s.%(ext)s")
                .arg("-f140")
                .arg(&url)
                .output()
                .expect("Failed to download video");
        });
    }
}
*/
