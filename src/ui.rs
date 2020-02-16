use tui::{
    backend::Backend,
    Terminal,
    Frame,
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    widgets::{
        SelectableList,
        Block,
        Borders,
        Widget,
        Gauge,
        Paragraph,
        Text,
        Row,
        Table,
    },
    style::{
        Style,
        Color,
    },
};
use failure::Error;
use crate::app::{
    App,
    Pane,
    PaneType,
    ContentType,
};
use std::convert::TryInto;
use std::convert::AsRef;
use std::fmt::Display;

const PLAYLIST_HEADERS: [&str; 3]= ["Name", "Author", "# of Videos"];
const CHANNEL_HEADERS: [&str; 3] = ["Name", "# of Subs", "# of Videos"];
const VIDEO_HEADERS: [&str; 3] = ["Title", "Author", "Duration"];

pub fn draw<B>(terminal: &mut Terminal<B>, mut app: &mut App) -> Result<(), Error> 
where
B: Backend,
{
    terminal.draw(|mut f| {
        let layout = fixed_layout(&f);
        let settings = app.panes.get(&PaneType::Settings).unwrap();
        let library = app.panes.get(&PaneType::Frontpage).unwrap();
        let playlist = app.panes.get(&PaneType::Search).unwrap();
        
        draw_selectable_list(&mut f, layout[0], settings);
        draw_selectable_list(&mut f, layout[1], library);
        
        //draw_selectable_list(&mut f, layout[2], playlist, &vec![""]);
        
        draw_selectable_list(&mut f, layout[2], playlist);
       
        draw_player(&mut f, layout[3], &app);
        draw_cmdline(&mut f, layout[4], &app.input);
        /*
        match app.main_pane {
            PaneType::SearchResults => draw_search_view(&mut f, layout[5], &mut app),
            _ => draw_table(&mut f, layout[5], main, &vec![], &vec![""]),
        }
        */
        draw_main_pane(&mut f, layout[5], &mut app);
        
    })?;
    Ok(())
}

fn draw_main_pane<B>(mut f: &mut Frame<B>, area: Rect, mut app: &mut App) 
    where
    B: Backend,
{
    match app.main_pane {
        PaneType::SearchResults => draw_search_view(&mut f, area, &mut app),
        //PaneType::Videos => draw_table(&mut f, area, main, &vec![], &VIDEO_HEADERS),
        _ => {},
    }
}

fn draw_search_view<B>(f: &mut Frame<B>, area: Rect, app: &mut App) 
    where
    B: Backend,
{
    if let Some(pane) = app.panes.get(&PaneType::Search) {
        let selected = pane.selected;
        match selected {
            0 => {
                if let Some(pane) = app.panes.get_mut(&PaneType::Videos) {
                    draw_table(f, area, pane, &VIDEO_HEADERS);
                }
            },
            1 => {
                if let Some(pane) = app.panes.get_mut(&PaneType::Playlists) {
                    draw_table(f, area, pane, &PLAYLIST_HEADERS);
                }
            },
            2 => {
                if let Some(pane) = app.panes.get_mut(&PaneType::Channels) {
                    draw_table(f, area, pane, &CHANNEL_HEADERS);
                }
            },
            _ => {},
        }
    }
}

fn fixed_layout<B>(f: &Frame<B>) -> Vec<Rect> 
where
B: Backend,
{
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                     Constraint::Min(1),
                     Constraint::Length(6),
                     Constraint::Length(2),
        ].as_ref())
        .split(f.size());

    let hamburger_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
                     Constraint::Length(35),
                     Constraint::Min(1),
        ].as_ref())
        .split(layout[0]);
    
    let mut chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                     Constraint::Percentage(15),
                     Constraint::Percentage(25),
                     Constraint::Percentage(60),
        ].as_ref())
        .split(hamburger_split[0]);

    chunks.push(layout[1]);
    chunks.push(layout[2]);
    chunks.push(hamburger_split[1]);
    chunks
}

fn draw_cmdline<B>(f: &mut Frame<B>, area: Rect, input: &str)
    where
    B: Backend
{
    let text = [Text::raw(input)];
    Paragraph::new(text.iter())
        .block(Block::default())
        .wrap(false)
        .render(f, area);
}

fn draw_player<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
    B: Backend,
{
    Block::default()
        .title("Playing | Shuffle")
        .borders(Borders::ALL)
        .render(f, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
                     Constraint::Length(3),
                     Constraint::Length(1),
        ].as_ref())
        .split(area);
    
    let text = {
        /*
        let label = match app.active {
            ActivePlayer::AudioOnly => app.audio_player.get_property("media-title").unwrap_or(""),
            ActivePlayer::VideoAudio => app.full_player.get_property("media-title").unwrap_or(""),
            _ => "",
        };
        */
        [
            Text::raw(app.current_title.clone() + "\n"),
            Text::raw(app.current_author.clone() + "\n"),
        ]
    };
    Paragraph::new(text.iter())
        .render(f, chunks[0]);
    
    draw_progress_bar(f, chunks[1], app);
}

fn draw_progress_bar<B>(f: &mut Frame<B>, area: Rect, _app: &App) 
    where
    B: Backend,
{
    /*
    let percent = match app.active {
        ActivePlayer::AudioOnly => app.audio_player.get_property("percent-pos").unwrap_or(0),
        ActivePlayer::VideoAudio => app.full_player.get_property("percent-pos").unwrap_or(0),
        _ => 0,
    };
    */
    let percent = 20;
    
    Gauge::default()
        .label("")
        .percent(percent.try_into().unwrap_or(0))
        .style(Style::default().fg(Color::Red).bg(Color::Gray))
        .render(f, area)
}

fn draw_selectable_list<B>(f: &mut Frame<B>, 
                              area: Rect, 
                              pane: &Pane, 
                              )
    where
    B: Backend,
{

    let border_color = if pane.has_focus {
        Color::Red
    }
    else {
        Color::White
    };

    let content = match pane.content {
        ContentType::ListContent(ref text) => text.clone(),
        _ => vec![]
    };
    
    SelectableList::default()
        .block(Block::default()
               .title(&pane.title)
               .border_style(Style::default().fg(border_color))
               .borders(Borders::ALL))
        .items(&content)
        .select(Some(pane.selected))
        .highlight_style(Style::default()
                         .fg(Color::Red))
        .render(f, area);
}

fn draw_table<B, S>(f: &mut Frame<B>, 
                        area: Rect, 
                        pane: &Pane, 
                        headers: &[S]
                        )
    where
    B: Backend,
    S: AsRef<str> + Display,
{
    let border_color = if pane.has_focus {
        Color::Red
    }
    else {
        Color::White
    };

    let offset = area.height
        .checked_sub(5)
        .and_then(|height| {
            pane.selected.checked_sub(height as usize)
        }).unwrap_or(0);

    let content = match &pane.content {
        ContentType::MediaContent(content) => content.clone(),
        _ => std::sync::Arc::new(std::sync::RwLock::new(vec![])),
    };
    let content = content.read().unwrap();
    let content = content.iter().skip(offset).enumerate().map(|(i, row)| {
                let color = if i == pane.selected.saturating_sub(offset) {
                    Color::Red
                }
                else {
                    Color::White
                };

                Row::StyledData(row.iter(), Style::default().fg(color))
            });

    Table::new(
        headers.iter(),
        content,
        )
        .block(Block::default()
               .title(&pane.title)
               .border_style(Style::default().fg(border_color))
               .borders(Borders::ALL)
              )
        .widths(&[
                Constraint::Length(50), 
                Constraint::Length(30),
                Constraint::Length(10),
        ])
        .style(Style::default().fg(Color::White))
        .column_spacing(10)
        .render(f, area);
}


