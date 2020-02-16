use failure::Error as AnyError;
use reqwest::{
    Client,
    Error,
};
use crate::{
    media::{
        ListItem,
        Video,
        Channel,
        Playlist,
    },
};
use std::sync::{
    Arc,
    RwLock,
};

const SEARCH_URL: &str = "https://www.invidio.us/api/v1/search";

pub struct Search {
    pub query: String,
    pub videos: (Arc<RwLock<Vec<Video>>>, u32),
    pub playlists: (Arc<RwLock<Vec<Playlist>>>, u32),
    pub channels: (Arc<RwLock<Vec<Channel>>>, u32),
}

impl Default for Search {
    fn default() -> Self {
        Search {
            query: String::new(),
            videos: (Arc::new(RwLock::new(vec![])), 1),
            playlists: (Arc::new(RwLock::new(vec![])), 1),
            channels: (Arc::new(RwLock::new(vec![])), 1),
        }
    }
}

pub async fn search(query: String, client: Client) -> Result<Search, AnyError> {
    let videos_fut = search_videos(&query, &client, 1);
    let playlists_fut = search_playlists(&query, &client, 1);
    let channels_fut = search_channels(&query, &client, 1);

    let (videos, playlists, channels) = futures::try_join!(videos_fut, playlists_fut, channels_fut)?;
    
    Ok(Search {
        query,
        videos: (Arc::new(RwLock::new(videos)), 1),
        playlists: (Arc::new(RwLock::new(playlists)), 1),
        channels: (Arc::new(RwLock::new(channels)), 1),
    })
}

pub async fn search_videos(query: &str, client: &Client, page: u32) -> Result<Vec<Video>, Error> {
    let params = [
        ("q", query),
        ("page", &page.to_string()),
        ("type", "video"),
        ("sort_by", "relevance"),
    ];

    let rsp = client.get(SEARCH_URL)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}

async fn search_playlists(query: &str, client: &Client, page: u32) -> Result<Vec<Playlist>, Error> {
    let params = [
        ("q", query),
        ("page", &page.to_string()),
        ("type", "playlist"),
        ("sort_by", "relevance"),
    ];

    let rsp = client.get(SEARCH_URL)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}

async fn search_channels(query: &str, client: &Client, page: u32) -> Result<Vec<Channel>, Error> {
    let params = [
        ("q", query),
        ("page", &page.to_string()),
        ("type", "channel"),
        ("sort_by", "relevance"),
    ];

    let rsp = client.get(SEARCH_URL)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}


pub fn video_to_text(videos: Vec<Video>) -> Vec<Vec<String>> {
    videos.into_iter().map(|item| {
        item.into_text()
    }).collect()
}

pub fn playlist_to_text(playlists: Vec<Playlist>) -> Vec<Vec<String>> {
    playlists.into_iter().map(|item| {
        item.into_text()        
    }).collect()
}

pub fn channel_to_text(channels: Vec<Channel>) -> Vec<Vec<String>> {
    channels.into_iter().map(|item| {
        item.into_text()
    }).collect()
}
