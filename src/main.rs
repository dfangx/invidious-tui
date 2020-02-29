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
use ui::views::{
    ViewType,
    View,
    Window,
    WindowType,
    ContentType,
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
    let playlist_headers = vec!["Name".to_owned(), "Author".to_owned(), "# of Videos".to_owned()];
    let channel_headers = vec!["Name".to_owned(), "# of Subs".to_owned(), "# of Videos".to_owned()];
    let video_headers = vec!["Title".to_owned(), "Author".to_owned(), "Duration".to_owned()];
    let media_list = vec!["Videos".to_owned(), "Playlists".to_owned(), "Channels".to_owned()];
    let search_windows = vec![
        Window::new("Videos".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(video_headers), WindowType::SearchVideos),
        Window::new("Playlists".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(channel_headers), WindowType::SearchPlaylists),
        Window::new("Channels".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(playlist_headers), WindowType::SearchChannels),
    ];

    (ViewType::Search, View::new(search_windows, media_list))
}

fn init_home_view() -> (ViewType, View) {
    let video_headers = vec!["Title".to_owned(), "Author".to_owned(), "Duration".to_owned()];
    let home_list = vec!["Trending".to_owned(), "Popular".to_owned(), "Top".to_owned()];

    let home_windows = vec![
        Window::new("Trending".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(video_headers.clone()), WindowType::TrendingVideos),
        Window::new("Popular".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(video_headers.clone()), WindowType::PopularVideos),
        Window::new("Top".to_owned(), 0, ContentType::MediaContent(Arc::new(RwLock::new(vec![]))), Some(video_headers), WindowType::TopVideos),
    ];

    (ViewType::Home, View::new(home_windows, home_list))
}
