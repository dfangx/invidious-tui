/*
use crate::traits::ListItem;
use crate::video::Video;
use crate::traits::ViewExt;
use crate::ui::playlist_view::PlaylistView;
use crate::ui::search_view::SearchView;
use serde_json::Value;

use reqwest::blocking::Client;
*/

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Playlist {
    pub title: String,
    pub playlist_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: u64,
    pub client: Client,
}

/*
impl Playlist {
    pub fn new(playlist: Value, client: Client) -> Self {
        Playlist {
            title: playlist["title"].as_str().unwrap().to_string(),
            playlist_id: playlist["playlistId"].as_str().unwrap().to_string(),
            video_count: playlist["videoCount"].as_u64().unwrap(),
            author: playlist["author"].as_str().unwrap().to_string(),
            author_id: playlist["authorId"].as_str().unwrap().to_string(),
            author_url: playlist["authorUrl"].as_str().unwrap().to_string(),
            client,
        }
    }
    
    pub fn fetch_videos(&self) -> Vec<Video> {
        let url = "https://www.invidio.us/api/v1/playlists/".to_owned() + &self.playlist_id;
        let mut total_pages = self.video_count / 100;
        if total_pages == 0 {
            total_pages = 1;
        }
        SearchView::playlist_video_search(&self.client, &url, total_pages)
    }
}

impl ListItem for Playlist {
    fn right_text(&self) -> String {
        format!("{} videos", self.video_count)
    }

    fn left_text(&self) -> String {
        format!("[{}] {}", self.author, self.title)
    }

    fn play(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("mpv")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to start playlist");
        //});
    }

    fn open(&self) -> Option<Box<dyn ViewExt>> {
        Some(Box::new(PlaylistView::new(self.clone())))
    }

    fn download(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("youtube-dl")
        //        .arg("-o")
        //        .arg("~/downloads/%(playlist_title)s_%(playlist_uploader)s/(title)s_%(channel)s.%(ext)s")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to download video");
        //});
    }

    fn download_audio_only(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("youtube-dl")
        //        .arg("-o")
        //        .arg("~/downloads/%(playlist_title)s_%(playlist_uploader)s/%(title)s_%(channel)s.%(ext)s")
        //        .arg("-f140")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to download video");
        //});
    }
}
*/
