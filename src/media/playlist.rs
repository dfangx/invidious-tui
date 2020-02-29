use serde::Deserialize;
use tokio::runtime::Runtime;
use failure::Error;
use serde_json::Value;
use reqwest::Client;
use crate::{
    player::Player,
    ui::views::{
        Window,
        WindowType,
        ContentType,
        View,
    },
    data::LoadedData,
    media::{
        Media,
        ListItem,
        video::Video,
    },
    utils,
};
use std::sync::{
    Arc,
    RwLock,
};

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

const PLAYLIST_URL: &str = "https://www.invidio.us/api/v1/playlists";
impl Playlist {
    pub async fn get_videos(&self, client: &Client, total_pages: u32) -> Result<Vec<Video>, Error> {
        let id = &self.playlist_id;
        let url = format!("{}/{}", PLAYLIST_URL, id);
        let mut playlist_videos = vec![];
        log::debug!("PLAYLIST_VIDEO_URL: {:?}", url);
        
        for page in 0..total_pages {
            let params = [
                ("page", page + 1),
            ];

            let rsp = client.get(&url)
                .query(&params)
                .send()
                .await?;
            let json: Value = rsp.json().await?;
            if json["videos"].is_array() {
                match serde_json::from_value::<Vec<Video>>(json["videos"].clone()) {
                    Ok(mut videos) => {
                        playlist_videos.append(&mut videos);
                    },
                    Err(e) => {
                        log::error!("Unable to convert to type Video: {}", e);
                        return Err(failure::format_err!("{}", e))
                    }
                }
            }
            else {
                break;
            }
        }
        Ok(playlist_videos)
    }
}

impl Media for Playlist {
    fn open(&self, client: &Client, runtime: Arc<RwLock<Runtime>>, loaded_data: &mut LoadedData) -> Result<View, Error> {
        log::debug!("OPENING PL_VIDEOS");
        let videos = runtime.write().unwrap().block_on(self.get_videos(client, 1))?;
        log::debug!("PL VIDEOS: {:#?}", videos);
        let video_text = utils::video_to_text(videos.clone());
        let video_headers = vec!["Title".to_owned(), "Author".to_owned(), "Duration".to_owned()];
        loaded_data.playlist_videos = videos;
        let window = Window::new(self.title.clone(),
                  0,
                  ContentType::MediaContent(Arc::new(RwLock::new(video_text))),
                  Some(video_headers),
                  WindowType::PlaylistVideos,
                  );
        let view = View::new(vec![window], vec!["Videos".to_owned()]);
        Ok(view)
    }
    
    fn play_video(&self, player: &mut Player) {
        let id = &self.playlist_id;
        let url = format!("https://invidio.us/playlist?list={}", id);
        player.play(url, true);
    }
    
    fn play_audio(&self, player: &mut Player) {
        let id = &self.playlist_id;
        let url = format!("https://invidio.us/playlist?list={}", id);
        player.play(url, false);
    }
    
    fn queue(&self, player: &mut Player){
        let id = &self.playlist_id;
        let url = format!("https://invidio.us/playlist?list={}", id);
        player.queue(url);
    }
    
    fn title(&self) -> String {
        self.title.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
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
