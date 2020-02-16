use crate::app::{
    App,
    PaneType,
    Pane,
    ContentType,
};
use crate::media::Video;
use crate::media::Media;
use failure::Error;
use crate::search;
use tokio::runtime::Runtime;

pub fn swap_focus(app: &mut App, new_focus: PaneType) {
    app.prev_focused = app.focused;
    if let Some(pane) = app.panes.get_mut(&app.focused) {
        pane.has_focus = false;
    }
    app.focused = new_focus;
    if let Some(pane) = app.panes.get_mut(&app.focused) {
        pane.has_focus = true;
    }
    
    /*
    app.panes[app.focused.value()].has_focus = false;
    app.focused = new_focus;
    app.panes[app.focused.value()].has_focus = true;
    */
}

pub fn get_media(app: &mut App) -> Box<dyn Media>{
    match app.main_pane {
        PaneType::SearchResults => {
            match app.focused {
                PaneType::Videos => {
                    if let Some(pane) = app.panes.get(&PaneType::Videos) {
                        return Box::new(app.loaded_data.search_data.videos.0.read().unwrap()[pane.selected].clone());
                    }
                },
                PaneType::Playlists => {
                    if let Some(pane) = app.panes.get(&PaneType::Videos) {
                        return Box::new(app.loaded_data.search_data.playlists.0.read().unwrap()[pane.selected].clone());
                    }
                },
                _ => log::info!("Not implemented yet"),
            }
        }
        _ => log::info!("Not implemented yet"),
    }
    Box::new(Video::default())
}
pub fn fetch_next_page(app: &mut App) -> Result<(), Error> {
    let query = app.loaded_data.search_data.query.clone();
    let client = app.client.clone();
    match app.focused {
        PaneType::Videos => {
            app.loaded_data.search_data.videos.1 += 1;
            let page = app.loaded_data.search_data.videos.1;
            let current_data = app.loaded_data.search_data.videos.0.clone();
            if let Some(pane) = app.panes.get(&app.focused) {
                if let ContentType::MediaContent(content) = &pane.content {
                    let content = content.clone();
                    log::debug!("spawning thread");
                    std::thread::spawn(move || {
                        Runtime::new().unwrap().block_on(
                            async move {
                                log::debug!("Beginning search: {} {} ", query, page);
                                let videos = search::search_videos(&query, &client, page).await;
                                log::debug!("Done search: {:#?}", videos);
                                match videos {
                                    Ok(mut videos) => {
                                        log::debug!("Writing data");
                                        current_data.write().unwrap().append(&mut videos);
                                        log::debug!("Writing content");
                                        *content.write().unwrap() = search::video_to_text(current_data.read().unwrap().to_vec());
                                        log::debug!("Done");
                                    },
                                    Err(e) => log::error!("{}", e),
                                }
                            });
                    });
                }
            }
        }
        _ => {},
    }
    Ok(())
}
    
pub fn next_selection(mut pane: &mut Pane, len: usize) {
    if pane.selected + 1 < len {
        pane.selected+=1;
    }
}

pub fn prev_selection(app: &mut App) {
    if let Some(pane) = app.panes.get_mut(&app.focused) {
        if pane.selected > 0 {
            pane.selected-=1;
        }
    }
}
