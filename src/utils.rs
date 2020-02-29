use crate::{
    ui::views::{
        WindowType,
        Window,
        ContentType,
    },
    data::LoadedData,
    media::{
        video::Video,
        channel::Channel,
        playlist::Playlist,
        Media,
        ListItem,
    },
    invidious
};
use failure::Error;
use tokio::runtime::Runtime;
use reqwest::Client;
use std::sync::{
    Arc,
    RwLock,
};

const SEARCH_URL: &str = "https://www.invidio.us/api/v1/search";

pub fn get_media(window: &Window, data: &LoadedData) -> Box<dyn Media>{
    match window.window_type {
        WindowType::SearchVideos => Box::new(data.search_data.videos.0.read().unwrap()[window.selected].clone()),
        WindowType::PlaylistVideos => Box::new(data.playlist_videos[window.selected].clone()),
        WindowType::SearchPlaylists => Box::new(data.search_data.playlists.0.read().unwrap()[window.selected].clone()),
        WindowType::TrendingVideos => Box::new(data.trending_videos[window.selected].clone()),
        WindowType::PopularVideos => Box::new(data.popular_videos[window.selected].clone()),
        WindowType::TopVideos => Box::new(data.top_videos[window.selected].clone()),
        WindowType::SearchChannels => Box::new(data.search_data.channels.0.read().unwrap()[window.selected].clone()),
        WindowType::ChannelVideos => Box::new(data.channel_videos[window.selected].clone()),
        WindowType::ChannelPlaylists => Box::new(data.channel_playlists[window.selected].clone()),
    }
}

pub fn fetch_next_page(client: Client, runtime: Arc<RwLock<Runtime>>, data: &LoadedData, window: &Window) -> Result<(), Error> {
    //let client = app.client.clone();
    match window.window_type {
        WindowType::SearchVideos => {
            let query = data.search_data.query.clone();
            let page = data.search_data.videos.1 + 1;
            let current_data = data.search_data.videos.0.clone();
            if let ContentType::MediaContent(ref content) = window.content {
                let content = content.clone();
                std::thread::spawn(move || {
                    runtime.write().unwrap().block_on(
                        async move {
                            let page_str = page.to_string();
                            let params = vec![
                                ("q", query.as_str()),
                                ("page", page_str.as_str()),
                                ("type", "video"),
                                ("sort_by", "relevance"),
                            ];
                            let videos = invidious::invidious_videos(params, &client, SEARCH_URL).await;
                            match videos {
                                Ok(mut videos) => {
                                    current_data.write().unwrap().append(&mut videos);
                                    *content.write().unwrap() = video_to_text(current_data.read().unwrap().to_vec());
                                },
                                Err(e) => log::error!("{}", e),
                            }
                        });
                });
            }
        },
        WindowType::SearchPlaylists => {
            let query = data.search_data.query.clone();
            let page = data.search_data.playlists.1 + 1;
            let current_data = data.search_data.playlists.0.clone();
            if let ContentType::MediaContent(ref content) = window.content {
                let content = content.clone();
                std::thread::spawn(move || {
                    runtime.write().unwrap().block_on(
                        async move {
                            let page_str = page.to_string();
                            let params = vec![
                                ("q", query.as_str()),
                                ("page", page_str.as_str()),
                                ("type", "playlist"),
                                ("sort_by", "relevance"),
                            ];
                            let playlists = invidious::invidious_playlists(params, &client, SEARCH_URL).await;
                            match playlists {
                                Ok(mut playlists) => {
                                    current_data.write().unwrap().append(&mut playlists);
                                    *content.write().unwrap() = playlist_to_text(current_data.read().unwrap().to_vec());
                                },
                                Err(e) => log::error!("{}", e),
                            }
                        });
                });
            }
        },
        WindowType::SearchChannels => {
            let query = data.search_data.query.clone();
            let page = data.search_data.channels.1 + 1;
            let current_data = data.search_data.channels.0.clone();
            if let ContentType::MediaContent(ref content) = window.content {
                let content = content.clone();
                std::thread::spawn(move || {
                    runtime.write().unwrap().block_on(
                        async move {
                            let page_str = page.to_string();
                            let params = vec![
                                ("q", query.as_str()),
                                ("page", page_str.as_str()),
                                ("type", "channel"),
                                ("sort_by", "relevance"),
                            ];
                            let channels = invidious::invidious_channels(params, &client, SEARCH_URL).await;
                            match channels {
                                Ok(mut channels) => {
                                    current_data.write().unwrap().append(&mut channels);
                                    *content.write().unwrap() = channel_to_text(current_data.read().unwrap().to_vec());
                                },
                                Err(e) => log::error!("{}", e),
                            }
                        });
                });
            }
        },
        _ => {},
    }
    Ok(())
}
    
pub fn next_selection(mut window: &mut Window, len: usize) {
    if window.selected + 1 < len {
        window.selected+=1;
    }
}

pub fn prev_selection(mut window: &mut Window) {
    if window.selected > 0 {
        window.selected-=1;
    }
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
