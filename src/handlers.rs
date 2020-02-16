use termion::event::Key;
use crate::app::{
    App,
    Mode,
    PaneType,
    ContentType,
};
use crate::search;
use crate::utils;
use failure::Error;
use std::sync::{
    Arc,
    RwLock,
};

fn global_handler(key: Key, mut app: &mut App) -> Option<Key> {
    if key == app.config.keys.quit {
        app.quit = true;
    }
    else if key == app.config.keys.enter_normal_mode {
        app.mode = Mode::Normal;
    }
    else if key == app.config.keys.search {
        app.mode = Mode::Input;
        if let Key::Char(c) = key {
            app.input.push(c);
        }
    }
    else if key == app.config.keys.play_pause {
        app.player.toggle_playback();
    }
    else {
        return Some(key)
    }
    None
}

fn media_pane_handler(key: Key, mut app: &mut App) {
    if key == app.config.keys.move_down {
        if let Some(mut pane) = app.panes.get_mut(&app.focused) {
            let content_len = match &pane.content {
                ContentType::ListContent(vec) => vec.len(),
                ContentType::MediaContent(vec) => vec.read().unwrap().len(),
            };
            utils::next_selection(&mut pane, content_len);
            if pane.selected == content_len.saturating_sub(4) {
                match utils::fetch_next_page(&mut app) {
                    Ok(_) => log::info!("Fetched next page of focused pane"),
                    Err(e) => log::error!("Error fetching next page: {}", e),
                }
            }
        }
    }
    else if key == app.config.keys.move_up {
        utils::prev_selection(&mut app);
    }
    else if key == app.config.keys.submit_entry {
        let media = utils::get_media(&mut app);
        app.current_title = media.title();
        app.current_author = media.author();
        media.play_video(&mut app.player);
    }
    else if key == app.config.keys.audio_only {
        let media = utils::get_media(&mut app);
        app.current_title = media.title();
        app.current_author = media.author();
        media.play_audio(&mut app.player);
    }
}

fn normal_mode_handler(key: Key, mut app: &mut App) {
    if key == app.config.keys.move_down {
        app.move_focus_down();
    }
    else if key == app.config.keys.move_up {
        app.move_focus_up();
    }
    else if key == app.config.keys.move_left {
        app.move_focus_left();
    }
    else if key == app.config.keys.move_right {
        app.move_focus_right();
    }
    else if key == app.config.keys.enter_selection_mode {
        app.mode = Mode::Selection;
    }
}

fn selection_mode_handler(key: Key, app: &mut App) {
    match app.focused {
        PaneType::Search => search_list_handler(key, app),
        _ => media_pane_handler(key, app),
    }
}

fn search_list_handler(key: Key, mut app: &mut App) {
    if key == app.config.keys.move_down {
        if let Some(mut pane) = app.panes.get_mut(&app.focused) {
            let content_len = match &pane.content {
                ContentType::ListContent(vec) => vec.len(),
                ContentType::MediaContent(vec) => vec.read().unwrap().len(),
            };
            utils::next_selection(&mut pane, content_len);
        }
    }
    else if key == app.config.keys.move_up {
        utils::prev_selection(&mut app);
    }
    else if key == app.config.keys.submit_entry {
        if let Some(pane) = app.panes.get(&app.focused) {
            match pane.selected {
                0 => utils::swap_focus(&mut app, PaneType::Videos),
                1 => utils::swap_focus(&mut app, PaneType::Playlists),
                2 => utils::swap_focus(&mut app, PaneType::Channels),
                _ => {},
            }
        }
    }
}

fn cmdline_handler(key: Key, mut app: &mut App) -> Result<(), Error> {
    if key == app.config.keys.submit_entry {
        let input = app.input.clone();
        let client = app.client.clone();
        let search_data = app.runtime.block_on(search::search(input, client))?;
        
        let video_text = search::video_to_text(search_data.videos.0.read().unwrap().clone());
        let playlist_text = search::playlist_to_text(search_data.playlists.0.read().unwrap().clone());
        let channel_text = search::channel_to_text(search_data.channels.0.read().unwrap().clone());
        if let Some(mut pane) = app.panes.get_mut(&PaneType::Videos) {
            pane.content = ContentType::MediaContent(Arc::new(RwLock::new(video_text)));
            pane.selected = 0;
            //pane.length = search_data.videos.0.read().unwrap().len();
        }
        if let Some(mut pane) = app.panes.get_mut(&PaneType::Playlists) {
            pane.content = ContentType::MediaContent(Arc::new(RwLock::new(playlist_text)));
            pane.selected = 0;
            //pane.length = search_data.playlists.0.read().unwrap().len();
        }

        if let Some(mut pane) = app.panes.get_mut(&PaneType::Channels) {
            pane.content = ContentType::MediaContent(Arc::new(RwLock::new(channel_text)));
            pane.selected = 0;
            //pane.length = search_data.channels.0.read().unwrap().len();
        }

        app.loaded_data.search_data = search_data;

        app.main_pane = PaneType::SearchResults;
        utils::swap_focus(&mut app, PaneType::Search);
        log::debug!("SWITCHED TO SEARCH FOCUS");

        app.input = String::new();
        app.mode = Mode::Selection;
    }
    else if let Key::Char(c) = key {
        app.input.push(c);
    }
    else if key == Key::Backspace {
        app.input.pop();
        if app.input.is_empty() {
            app.mode = Mode::Normal;
        }
    }
    Ok(())
}

pub fn event_handler(key: Key, app: &mut App) -> Result<(), Error> {
    if global_handler(key, app).is_some() {
        match app.mode {
            Mode::Normal => normal_mode_handler(key, app),
            Mode::Selection => selection_mode_handler(key, app),
            Mode::Input => cmdline_handler(key, app)?,
        }
    }
    Ok(())
}
