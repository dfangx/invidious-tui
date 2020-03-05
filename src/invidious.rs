use failure::Error as AnyError;
use reqwest::{
    Client,
    Error,
};
use crate::{
    media::{
        video::Video,
        channel::Channel,
        playlist::Playlist,
    },
    data::Search,
};
use std::sync::{
    Arc,
    RwLock,
};

const SEARCH_URL: &str = "https://www.invidio.us/api/v1/search";
const TRENDING_URL: &str = "https://www.invidio.us/api/v1/trending";
const POPULAR_URL: &str = "https://www.invidio.us/api/v1/popular";
//const TOP_URL: &str = "https://www.invidio.us/api/v1/top";


pub async fn load_home(client: &Client) -> Result<(Vec<Video>, Vec<Video>), reqwest::Error> {
    let trending_fut = invidious_videos(vec![], client, TRENDING_URL);
    let popular_fut = invidious_videos(vec![], client, POPULAR_URL);
    //let top_fut = home_videos(client, TOP_URL);

    futures::try_join!(trending_fut, popular_fut)//, top_fut)
}

pub async fn search(query: String, client: &Client) -> Result<Search, AnyError> {
    let params = vec![
        ("q", query.as_str()),
        ("page", "1"),
        ("type", "video"),
        ("sort_by", "relevance"),
    ];
    let videos_fut = invidious_videos(params, &client, SEARCH_URL);
    
    let params = vec![
        ("q", query.as_str()),
        ("page", "1"),
        ("type", "playlist"),
        ("sort_by", "relevance"),
    ];
    let playlists_fut = invidious_playlists(params, &client, SEARCH_URL);
    
    let params = vec![
        ("q", query.as_str()),
        ("page", "1"),
        ("type", "channel"),
        ("sort_by", "relevance"),
    ];
    let channels_fut = invidious_channels(params, &client, SEARCH_URL);

    let (videos, playlists, channels) = futures::try_join!(videos_fut, playlists_fut, channels_fut)?;
    log::debug!("VIDEOS: {:#?}", videos);
    
    Ok(Search {
        query,
        videos: (Arc::new(RwLock::new(videos)), 1),
        playlists: (Arc::new(RwLock::new(playlists)), 1),
        channels: (Arc::new(RwLock::new(channels)), 1),
    })
}

pub async fn invidious_videos(params: Vec<(&str, &str)>, client: &Client, url: &str) -> Result<Vec<Video>, Error> {
    let rsp = client.get(url)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}

pub async fn invidious_playlists(params: Vec<(&str, &str)>, client: &Client, url: &str) -> Result<Vec<Playlist>, Error> {
    let rsp = client.get(url)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}

pub async fn invidious_channels(params: Vec<(&str, &str)>, client: &Client, url: &str) -> Result<Vec<Channel>, Error> {

    let rsp = client.get(url)
        .query(&params)
        .send()
        .await?;
    rsp.json().await
}

