use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{
    App,
    MainSubScene::{self, SideBar},
    Scene::Main,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // NOTE: always render Main, hightlight the focued one
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let active_style = Style::default().fg(Color::LightGreen);
    let inactive_style = Style::default().fg(Color::Gray);
    let sidebar = main_layout[0];
    let sidebar_title = Line::from("Mew Lib");
    let sidebar_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if matches!(app.scene, Main(MainSubScene::SideBar)) {
            active_style
        } else {
            inactive_style
        })
        .title(sidebar_title);
    let songs = app
        .music_lib
        .songs
        .iter()
        .enumerate()
        .map(|(i, song)| ListItem::new(format!("{:<3} {}", i, &song.name[..])));
    let list = List::new(songs)
        // .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(sidebar_block);

    frame.render_stateful_widget(list, sidebar, &mut app.music_lib.sel_song);

    let player = main_layout[1];
    let player_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if matches!(app.scene, Main(MainSubScene::Player)) {
            active_style
        } else {
            inactive_style
        })
        .title(Line::from("Playing Something...").centered());
    frame.render_widget(player_block, player);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
