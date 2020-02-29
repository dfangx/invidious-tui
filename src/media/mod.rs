use crate::{
    player::Player,
    ui::views::View,
    data::LoadedData,
};
use failure::Error;
use reqwest::Client;
use tokio::runtime::Runtime;
use std::sync::{
    Arc,
    RwLock,
};

pub mod video;
pub mod playlist;
pub mod channel;

pub trait Media {
    fn open(&self, _: &Client, _: Arc<RwLock<Runtime>>, _: &mut LoadedData) -> Result<View, Error> {
        Err(failure::format_err!("")) 
    }

    fn play_video(&self, _: &mut Player) {}

    fn play_audio(&self, _: &mut Player) {}

    fn title(&self) -> String {
        String::new()
    }

    fn author(&self) -> String {
        String::new()
    }

    fn queue(&self, _: &mut Player) {}
}



pub trait ListItem {
    fn into_text(&self) -> Vec<String>;
}

