pub mod views;
pub mod table_info;

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
        Tabs,
    },
    style::{
        Style,
        Color,
        Modifier,
    },
    symbols::DOT,
};
use failure::Error;
use crate::{
    ui::views::{
        Window,
        TabState,
        ContentType,
    },
    app::App,
    player::Player,
};
use std::{
    convert::AsRef,
    collections::VecDeque,
};

pub fn draw<B>(terminal: &mut Terminal<B>,
               mut app: &mut App
              ) -> Result<(), Error> 
where
B: Backend,
{
    terminal.draw(|mut f| {
        let layout = fixed_layout(&f);
        if let Some(root_view) = app.view_list.get(&app.focused_view) {
            if let Some(view) = root_view.get_current_view() {
                draw_tabs(&mut f, layout[0], &view.tabs);
                if let Some(window) = view.root_windows.get(view.tabs.selected) {
                    draw_table(&mut f, layout[1], window);
                }
            }
        }

        draw_player(&mut f, layout[2], &mut app);
        draw_cmdline(&mut f, layout[3], &app.input);
    })?;
    Ok(())
}


fn fixed_layout<B>(f: &Frame<B>) -> Vec<Rect> 
where
B: Backend,
{
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                     Constraint::Length(3),
                     Constraint::Min(1),
                     Constraint::Length(5),
                     Constraint::Length(2),
        ].as_ref())
        .split(f.size())
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

fn draw_player<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
    where
    B: Backend,
{
    let state = app.player.get_status();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                     Constraint::Length(4),
                     Constraint::Length(1),
        ].as_ref())
        .split(area);

    if app.player.audio_changed() {
        app.current_audio = get_current_media_text(&mut app.audio_queue);
    }
    if app.player.video_changed() {
        app.current_video = get_current_media_text(&mut app.video_queue);
    }

    app.next_audio = get_next_media_text(&app.audio_queue);
    app.next_video = get_next_media_text(&app.video_queue);

    let audio_text = [
        Text::styled("Current Track: ", Style::default().modifier(Modifier::BOLD)),
        Text::raw(&app.current_audio),
        Text::styled("Next Track: ", Style::default().modifier(Modifier::BOLD)),
        Text::raw(&app.next_audio),
    ];

    let video_text = [
        Text::styled("Current Video: ", Style::default().modifier(Modifier::BOLD)),
        Text::raw(&app.current_video),
        Text::styled("Next Video: ", Style::default().modifier(Modifier::BOLD)),
        Text::raw(&app.next_video),
    ];

    let player_status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
                     Constraint::Percentage(50),
                     Constraint::Percentage(50),
        ].as_ref())
        .split(chunks[0]);

    let audio_title = format!("Audio Player: {}", state);
    Paragraph::new(audio_text.iter())
        .block(
            Block::default()
            .title(&audio_title)
            .borders(Borders::ALL)
            )
        //.wrap(true)
        .render(f, player_status_layout[0]);
    Paragraph::new(video_text.iter())
        .block(
            Block::default()
            .title("Video Player")
            .borders(Borders::ALL)
            )
        //.wrap(true)
        .render(f, player_status_layout[1]);

    draw_progress_bar(f, chunks[1], &app.player);
}

fn get_current_media_text(queue: &mut VecDeque<(String, String, Option<String>)>) -> String {
    match queue.pop_front() {
        Some((title, author, playlist)) => {
            match playlist {
                Some(pl) => format!("[{}] {} by {}\n", pl, title, author),
                None => format!("{} by {}\n", title, author)
            }
        },
        None => String::from("None\n"),
    }
}

fn get_next_media_text(queue: &VecDeque<(String, String, Option<String>)>) -> String {
    match queue.front() {
        Some((title, author, playlist)) => {
            match playlist {
                Some(pl) => format!("[{}] {} by {}\n", pl, title, author),
                None => format!("{} by {}\n", title, author)
            }
        },
        None => String::from("None\n"),
    }
}

fn draw_progress_bar<B>(f: &mut Frame<B>, area: Rect, player: &Player) 
    where
    B: Backend,
{
    let percent = player.get_percent_pos();
    let time = player.get_time();

    Gauge::default()
        .label(&time)
        .percent(percent)
        .style(Style::default().fg(Color::Red).bg(Color::Gray))
        .render(f, area)
}

fn _draw_selectable_list<B>(f: &mut Frame<B>, 
                           area: Rect, 
                           pane: &Window, 
                          )
    where
    B: Backend,
{
    let content = match pane.content {
        ContentType::ListContent(ref text) => text.clone(),
        _ => vec![]
    };

    SelectableList::default()
        .block(Block::default()
               .title(&pane.title)
               .border_style(Style::default().fg(Color::White))
               .borders(Borders::ALL))
        .items(&content)
        .select(Some(pane.selected))
        .highlight_style(Style::default()
                         .fg(Color::Red))
        .render(f, area);
}

fn draw_table<B>(f: &mut Frame<B>, 
                 area: Rect, 
                 pane: &Window, 
                )
    where
    B: Backend,
{
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
    let headers = match &pane.headers {
        Some(ref headers) => headers.clone(),
        None => Box::new([]),
    };

    Table::new(
        headers.iter(),
        content,
        )
        .block(Block::default()
               .title(&pane.title)
               .border_style(Style::default().fg(Color::White))
               .borders(Borders::ALL)
              )
        .widths(&pane.column_widths)
        .style(Style::default().fg(Color::White))
        .column_spacing(10)
        .render(f, area);
}

fn draw_tabs<B>(f: &mut Frame<B>, area: Rect, tabs: &TabState) 
    where
    B: Backend,
{

    Tabs::default()
        .block(Block::default()
               .title(&tabs.title)
               .borders(Borders::ALL))
        .titles(&tabs.items)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(tabs.selected)
        .divider(DOT)
        .render(f, area);
}
