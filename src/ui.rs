use std::io::Split;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Gauge, LineGauge, List, ListItem, Paragraph, Wrap,
    },
};

use crate::{
    app::{
        App,
        MainSubScene::{self, SideBar},
        PlayMode,
        Scene::Main,
    },
    song::{self, Song, format_duration},
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
    let songs = app.music_lib.songs.iter().enumerate().map(|(i, song)| {
        let is_playing = app.music_player_state.cur_song == Some(i);
        if is_playing {
            ListItem::new(format!("{:<3} ♫ {}", i, &song.name[..]))
                .style(Color::LightGreen)
                .bold()
        } else {
            ListItem::new(format!("{:<3} {}", i, &song.name[..]))
        }
    });
    let list = List::new(songs)
        // .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol(">")
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
    let playing_song = match app.music_player_state.cur_song {
        Some(i) => app.get_song_detail_by_index(i).unwrap(),
        None => &Song::placeholder(),
    };
    let mut text = Text::default();
    text.push_span(Span::from(&playing_song.name).bold());
    if playing_song.author.is_some() {
        text.push_line(playing_song.author.clone().unwrap());
    }
    if playing_song.album.is_some() {
        text.push_line(playing_song.album.clone().unwrap());
    }
    let ratio = if playing_song.duration.is_zero() || app.music_player_state.position.is_zero() {
        0.0
    } else {
        (app.music_player_state.position.as_secs_f64() / playing_song.duration.as_secs_f64())
            .clamp(0.0, 1.0)
    };

    text.push_line(format_duration(&app.music_player_state.position));
    text.push_span("/");
    text.push_span(format_duration(&playing_song.duration));
    let volume_ratio = app.music_player_state.volume as f64;
    let play_icon = match app.music_player_state.play_state {
        crate::app::PlayState::Idle => Text::from("Pauused"),
        crate::app::PlayState::Playing => Text::from("Playing"),
        crate::app::PlayState::Pause => Text::from("Pauused"),
    }
    .bold();
    let play_mode = match app.music_player_state.play_mode {
        PlayMode::Circle => Text::from("O"),
        PlayMode::One => Text::from("·"),
        PlayMode::Random => Text::from("r"),
    }
    .bold()
    .centered();
    let volume_gauge = LineGauge::default()
        .style(Style::default().bg(Color::Black))
        .filled_style(Style::default().fg(Color::White))
        .filled_symbol("━")
        .unfilled_symbol("─")
        .label(Span::styled(
            if app.music_player_state.is_mute {
                " Meewte"
            } else {
                " Unmute"
            },
            Style::default().fg(Color::White),
        ))
        .ratio(volume_ratio);
    let player_line = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Color::Cyan);

    let inner = player_block.inner(player);
    frame.render_widget(player_block, player);

    let player_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Max(1), Constraint::Max(1)])
        .split(inner);
    let player_bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Min(8),
        ])
        .split(player_layout[2]);
    let gauge = LineGauge::default()
        .style(Style::default().bg(Color::Black))
        .filled_style(Style::default().fg(Color::LightGreen))
        .filled_symbol("━")
        .unfilled_symbol("─")
        .label(Span::styled(
            format!(
                "{} / {}",
                format_duration(&app.music_player_state.position),
                format_duration(&playing_song.duration)
            ),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .ratio(ratio);
    frame.render_widget(player_line, player_layout[0]);
    frame.render_widget(gauge, player_layout[1]);
    frame.render_widget(play_icon, player_bottom_layout[0]);
    frame.render_widget(play_mode, player_bottom_layout[1]);
    frame.render_widget(volume_gauge, player_bottom_layout[2]);
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
