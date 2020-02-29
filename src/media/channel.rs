use serde::Deserialize;
use tokio::runtime::Runtime;
use failure::Error;
use serde_json::Value;
use reqwest::Client;
use crate::{
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
        playlist::Playlist,
        video::Video,
    },
    utils,
    invidious,
};
use std::sync::{
    Arc,
    RwLock,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"), default)]
pub struct Channel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: u64,
    pub sub_count: u64,
}

const CHANNEL_VIDEOS_URL: &str = "https://www.invidio.us/api/v1/channels/videos";
const CHANNEL_PLAYLISTS_URL: &str = "https://www.invidio.us/api/v1/channels/playlists";
impl Channel {
    async fn get_playlists(&self, params: Vec<(&str, &str)>, client: &Client, url: &str) -> Result<Vec<Playlist>, reqwest::Error> {
        let rsp = client.get(url)
            .query(&params)
            .send()
            .await?;
        
        let json: Value = rsp.json().await?;
        if json["playlists"].is_array() {
            match serde_json::from_value::<Vec<Playlist>>(json["playlists"].clone()) {
                Ok(playlists) => return Ok(playlists),
                Err(e) => {
                    log::error!("Unable to convert to type Video: {}", e);
                    return Ok(vec![])
                }
            }
        }
        Ok(vec![])
    }
    
    pub async fn get_channel_media(&self, client: &Client) -> Result<(Vec<Video>, Vec<Playlist>), reqwest::Error> {
        let id = &self.author_id;
        let video_url = format!("{}/{}", CHANNEL_VIDEOS_URL, id);
        let playlist_url = format!("{}/{}", CHANNEL_PLAYLISTS_URL, id);
        log::debug!("{:#}", video_url);
        let params = vec![
            ("page", "1"),
            ("sort_by", "newest"),
        ];
        let videos_fut = invidious::invidious_videos(params, client, &video_url);

        let params = vec![
            ("sort_by", "newest"),
        ];
        //let playlists_fut = invidious::invidious_playlists(params, client, &playlist_url);
        let playlists_fut = self.get_playlists(params, client, &playlist_url);
        futures::try_join!(videos_fut, playlists_fut)
    }
}

impl Media for Channel {
    fn open(&self, client: &Client, runtime: Arc<RwLock<Runtime>>, loaded_data: &mut LoadedData) -> Result<View, Error> {
        log::debug!("OPENING CHANNELS");
        let (videos, playlists) = runtime.write().unwrap().block_on(self.get_channel_media(client))?;
        log::debug!("{:#?}", videos);
       
        let video_title = format!("{}'s Videos", self.author());
        let video_text = utils::video_to_text(videos.clone());
        let video_headers = vec!["Title".to_owned(), "Author".to_owned(), "Duration".to_owned()];
        
        let playlist_title = format!("{}'s Playlists", self.author());
        let playlist_headers = vec!["Name".to_owned(), "Author".to_owned(), "# of Videos".to_owned()];
        let playlist_text = utils::playlist_to_text(playlists.clone());

        loaded_data.channel_videos = videos;
        loaded_data.channel_playlists = playlists;

        let tabs = vec!["Videos".to_owned(), "Playlists".to_owned()];
        let root_windows = vec![
            Window::new(
                video_title,
                0,
                ContentType::MediaContent(Arc::new(RwLock::new(video_text))),
                Some(video_headers),
                WindowType::ChannelVideos,
                ),
                Window::new(
                    playlist_title,
                    0,
                    ContentType::MediaContent(Arc::new(RwLock::new(playlist_text))),
                    Some(playlist_headers),
                    WindowType::ChannelPlaylists,
                    ),
        ];
        let view = View::new(root_windows, tabs);
        Ok(view)
    }
    
    fn author(&self) -> String {
        self.author.clone()
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
