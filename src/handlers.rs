use termion::event::Key;
use tui::{
    Terminal,
    backend::Backend,
};
use crate::{
    app::{
        App,
    },
    ui::views::{
        ContentType,
        ViewType,
        WindowType,
    },
    utils,
    invidious,
};
use failure::Error;
use std::sync::{
    Arc,
    RwLock,
};

fn cmdline_handler<B: Backend>(key: Key, mut app: &mut App, terminal: &mut Terminal<B>) -> Result<(), Error> {
    if key == app.config.keys.submit_entry {
        terminal.hide_cursor()?;
        if let Some(view) = app.view_list.get_mut(&ViewType::Search) {
            let input = app.input.clone();
            let client = &app.client;
            let search_data = app.runtime.write().unwrap().block_on(invidious::search(input, &client));
            let search_data = search_data.unwrap();

            let video_text = utils::video_to_text(search_data.videos.0.read().unwrap().clone());
            let playlist_text = utils::playlist_to_text(search_data.playlists.0.read().unwrap().clone());
            let channel_text = utils::channel_to_text(search_data.channels.0.read().unwrap().clone());
            
            if let Some(mut window) = view.root_windows.get_mut(0) {
                window.content = ContentType::MediaContent(Arc::new(RwLock::new(video_text)));
                window.selected = 0;
            }
            if let Some(mut window) = view.root_windows.get_mut(1) {
                window.content = ContentType::MediaContent(Arc::new(RwLock::new(playlist_text)));
                window.selected = 0;
            }

            if let Some(mut window) = view.root_windows.get_mut(2) {
                window.content = ContentType::MediaContent(Arc::new(RwLock::new(channel_text)));
                window.selected = 0;
            }
        
            view.view_stack = vec![];
            app.focused_view = ViewType::Search;
            app.loaded_data.search_data = search_data;
        }


        app.input = String::new();
        app.cmdline_focused = false;
    }
    else if let Key::Char(c) = key {
        app.input.push(c);
    }
    else if key == Key::Backspace {
        app.input.pop();
        if app.input.is_empty() {
            terminal.hide_cursor()?;
            app.cmdline_focused = false;
        }
    }
    Ok(())
}

pub fn event_handler<B: Backend>(key: Key, mut app: &mut App, terminal: &mut Terminal<B>) -> Result<(), Error> {
    if app.cmdline_focused {
        cmdline_handler(key, app, terminal)?;
    }
    else if key == app.config.keys.quit {
        app.quit = true;
        app.player.stop_all();
    }
    else if key == app.config.keys.home_view {
        app.focused_view = ViewType::Home;
    }
    else if key == app.config.keys.search_view {
        app.focused_view = ViewType::Search;
    }
    else if key == app.config.keys.search {
        app.cmdline_focused = true;
        //terminal.show_cursor()?;
        if let Key::Char(c) = key {
            app.input.push(c);
        }
    }
    else if key == app.config.keys.back {
        if let Some(view) = app.view_list.get_mut(&app.focused_view) {
            view.pop_stack();
        }
    }
    else if key == app.config.keys.move_down {
        if let Some(root_view) = app.view_list.get_mut(&app.focused_view) {
            if let Some(view) = root_view.get_current_view_mut() { 
                if let Some(mut window) = view.root_windows.get_mut(view.tabs.selected) {
                    let content_len = match &window.content {
                        ContentType::ListContent(vec) => vec.len(),
                        ContentType::MediaContent(vec) => vec.read().unwrap().len(),
                    };
                    utils::next_selection(&mut window, content_len);
                    if window.selected == content_len.saturating_sub(4) {
                        match utils::fetch_next_page(app.client.clone(), app.runtime.clone(), &app.loaded_data, window) {
                            Ok(_) => {
                                app.loaded_data.search_data.videos.1 += 1;
                                log::info!("Fetched next page of focused window");
                            },
                            Err(e) => log::error!("Error fetching next page: {}", e),
                        }
                    }
                }
            }
        }
    }
    else if key == app.config.keys.move_up {
        if let Some(view) = app.view_list.get_mut(&app.focused_view) {
            if let Some(view) = view.get_current_view_mut() {
                if let Some(mut window) = view.root_windows.get_mut(view.tabs.selected) {
                    utils::prev_selection(&mut window);
                }
            }
        }
    }
    else if key == app.config.keys.move_left {
        if let Some(root_view) = app.view_list.get_mut(&app.focused_view) {
            if let Some(view) = root_view.get_current_view_mut() {
                view.tabs.selected = view.tabs.selected.saturating_sub(1);
            }
        }
    }
    else if key == app.config.keys.move_right {
        if let Some(root_view) = app.view_list.get_mut(&app.focused_view) {
            if let Some(view) = root_view.get_current_view_mut() {
                let len = view.tabs.items.len();
                if view.tabs.selected + 1 < len {
                    view.tabs.selected+=1;
                }
            }
        }
    }
    else if key == app.config.keys.submit_entry {
        if let Some(root_view) = app.view_list.get(&app.focused_view) {
            if let Some(view) = root_view.get_current_view() {
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    let media = utils::get_media(&window, &app.loaded_data);
                    if !app.video_queue.is_empty() {
                        app.video_queue.clear();
                    }
                    match window.window_type {
                        WindowType::SearchPlaylists | WindowType::ChannelPlaylists => {
                            let client = &app.client;
                            let runtime = &mut app.runtime;
                            let view = media.open(client, runtime.clone(), &mut app.loaded_data).unwrap();
                            if let Some(window) = view.root_windows.get(0) {
                                if let ContentType::MediaContent(ref content) = window.content {
                                    let mut text = content.read().unwrap().iter().map(|text| {
                                        (text[0].clone(), text[1].clone(), Some(window.title.clone()))
                                    }).collect();
                                    app.video_queue.push_back((media.title(), media.author(), None));
                                    app.video_queue.append(&mut text);
                                }
                            }
                        },
                        _ => app.video_queue.push_back((media.title(), media.author(), None)),

                    }
                    media.play_video(&mut app.player);
                }
            }
        }
    }
    else if key == app.config.keys.play_pause {
        app.player.toggle_audio_playback();
    }
    else if key == app.config.keys.audio_only {
        if let Some(root_view) = app.view_list.get(&app.focused_view) {
            if let Some(view) = root_view.get_current_view() {
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    let media = utils::get_media(&window, &app.loaded_data);
                    if !app.audio_queue.is_empty() {
                        app.audio_queue.clear();
                    }
                    match window.window_type {
                        WindowType::SearchPlaylists | WindowType::ChannelPlaylists => {
                            let client = &app.client;
                            let runtime = &mut app.runtime;
                            let view = media.open(client, runtime.clone(), &mut app.loaded_data).unwrap();
                            if let Some(window) = view.root_windows.get(0) {
                                if let ContentType::MediaContent(ref content) = window.content {
                                    let mut text = content.read().unwrap().iter().map(|text| {
                                        (text[0].clone(), text[1].clone(), Some(window.title.clone()))
                                    }).collect();
                                    app.audio_queue.push_back((media.title(), media.author(), None));
                                    app.audio_queue.append(&mut text);
                                }
                            }
                        },
                        _ => app.audio_queue.push_back((media.title(), media.author(), None)),

                    }
                    media.play_audio(&mut app.player);
                }
            }
        }
    }
    else if key == app.config.keys.queue_audio {
        if let Some(root_view) = app.view_list.get(&app.focused_view) {
            if let Some(view) = root_view.get_current_view() {
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    let media = utils::get_media(&window, &app.loaded_data);
                    match window.window_type {
                        WindowType::SearchPlaylists | WindowType::ChannelPlaylists=> app.audio_queue.push_back((media.title(), media.author(), Some(window.title.clone()))),
                        _ => app.audio_queue.push_back((media.title(), media.author(), None)),

                    }
                    
                    app.player.queue_audio(media.url());
                }
            }
        }
    }
    else if key == app.config.keys.queue_video {
        if let Some(root_view) = app.view_list.get(&app.focused_view) {
            if let Some(view) = root_view.get_current_view() {
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    let media = utils::get_media(&window, &app.loaded_data);
                    match window.window_type {
                        WindowType::SearchPlaylists | WindowType::ChannelPlaylists=> app.video_queue.push_back((media.title(), media.author(), Some(window.title.clone()))),
                        _ => app.video_queue.push_back((media.title(), media.author(), None)),

                    }
                    
                    app.player.queue_video(media.url());
                }
            }
        }
    }
    else if key == app.config.keys.open_selection {
        if let Some(root_view) = app.view_list.get_mut(&app.focused_view) {
            if let Some(view) = root_view.get_current_view_mut() {
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    let media = utils::get_media(&window, &app.loaded_data);
                    let client = &app.client;
                    let runtime = &mut app.runtime;
                    let new_view = media.open(client, runtime.clone(), &mut app.loaded_data).unwrap();
                    root_view.view_stack.push(new_view);
                }
            }
        }
    }
    else if key == app.config.keys.loop_audio {
        app.player.toggle_loop_audio();
    }
    Ok(())
}
