mod player;
mod events;
mod app;
mod config;
mod media;
mod utils;
mod handlers;
mod ui;
mod invidious;
mod data;

use std::{
    io::{
        stdout,
        Stdout,
    },
    sync::{
        Arc,
        RwLock,
    },
};
use tui::{
    backend::TermionBackend,
    Terminal,
};
use termion::{
    raw::{
     IntoRawMode,
     RawTerminal,
    },
    screen::AlternateScreen,
};
use failure::{
    Error,
};
use events::{
    Events,
    Event,
};
use app::App;
use ui::{
    table_info,
    views::{
        ViewType,
        View,
        Window,
        WindowType,
        ContentType,
    },
};

type Backend = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;

fn main() -> Result<(), Error> {
    setup_logger()?;
    let config = config::Config::load_config()?;
    let events = Events::default();
    let mut terminal = init_term()?;
    let (search_view_type, search_view) = init_search_view();
    let (home_view_type, home_view) = init_home_view();
    let mut app = App::new(config)
        .view(search_view_type, search_view)
        .view(home_view_type, home_view);
    app.run_setup();

    while !app.quit {
        ui::draw(&mut terminal, &mut app)?;
        if let Event::Input(key) = events.next()? {
            handlers::event_handler(key, &mut app)?;
        }
    }

    Ok(())
}

fn init_term() -> Result<Terminal<Backend>, Error> {
    let screen = AlternateScreen::from(stdout().into_raw_mode()?);
    let backend = TermionBackend::new(screen);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("/tmp/output.log")?)
        .apply()?;
    Ok(())
}

fn init_search_view() -> (ViewType, View) {
    let media_list = vec!["Videos".to_owned(), "Playlists".to_owned(), "Channels".to_owned()];
    let search_windows = vec![
        Window::new("Videos".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::VIDEO_HEADERS)), WindowType::SearchVideos, Box::new(table_info::VIDEO_COLUMN_CONSTRAINTS)),
        Window::new("Playlists".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::PLAYLIST_HEADERS)), WindowType::SearchPlaylists, Box::new(table_info::DEFAULT_COLUMN_CONSTRAINTS)),
        Window::new("Channels".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::CHANNEL_HEADERS)), WindowType::SearchChannels, Box::new(table_info::DEFAULT_COLUMN_CONSTRAINTS)),
    ];

    (ViewType::Search, View::new(search_windows, media_list))
}

fn init_home_view() -> (ViewType, View) {
    let home_list = vec!["Trending".to_owned(), "Popular".to_owned(), "Top".to_owned()];

    let home_windows = vec![
        Window::new("Trending".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::VIDEO_HEADERS)), WindowType::TrendingVideos, Box::new(table_info::VIDEO_COLUMN_CONSTRAINTS)),
        Window::new("Popular".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::VIDEO_HEADERS)), WindowType::PopularVideos, Box::new(table_info::VIDEO_COLUMN_CONSTRAINTS)),
        Window::new("Top".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(Box::new(table_info::VIDEO_HEADERS)), WindowType::TopVideos, Box::new(table_info::VIDEO_COLUMN_CONSTRAINTS)),
    ];

    (ViewType::Home, View::new(home_windows, home_list))
}
