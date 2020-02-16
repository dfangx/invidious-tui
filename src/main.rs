mod player;
mod events;
mod app;
mod config;
mod media;
mod utils;
mod handlers;
mod ui;
mod search;

use std::{
    io::{
        stdout,
        Stdout,
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

type Backend = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;

fn main() -> Result<(), Error> {
    setup_logger()?;
	let config = config::Config::load_config()?;
	let events = Events::default();
	let mut terminal = init_term()?;
	let mut app = app::App::new(config)?;

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
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
