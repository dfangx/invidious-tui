use std::sync::{
    Arc,
    RwLock,
};
use crate::{
    media::{
        video::Video,
        playlist::Playlist,
        channel::Channel,
    },
};

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

pub struct LoadedData {
    pub search_data: Search,
    pub playlist_videos: Vec<Video>,
    pub trending_videos: Vec<Video>,
    pub popular_videos: Vec<Video>,
    pub top_videos: Vec<Video>,
    pub channel_videos: Vec<Video>,
    pub channel_playlists: Vec<Playlist>,
}

impl Default for LoadedData {
    fn default() -> Self {
        LoadedData {
            search_data: Search::default(),
            playlist_videos: vec![],
            channel_videos: vec![],
            channel_playlists: vec![],
            trending_videos: vec![],
            popular_videos: vec![],
            top_videos: vec![],
        }
    }
}
