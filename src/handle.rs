use std::time::Duration;

use crate::app::MainSubScene;
use crate::app::PlayState;
use crate::app::Scene;
use crate::player::PlayerEvent;
use crate::player::PlayerHandler;
use crate::{app::App, event::Event};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle_terminal_event(app: &mut App, event: Event, play_handler: &mut PlayerHandler) {
    match event {
        Event::Tick => {}
        Event::Key(key_event) => key::handle(app, key_event, play_handler),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
    };
}

mod key {

    use ratatui::crossterm::event::KeyModifiers;

    use crate::player::PlayerCommand;

    use super::*;
    pub fn handle(app: &mut App, key_event: KeyEvent, player_handler: &mut PlayerHandler) {
        match &app.scene {
            Scene::Main(main_sub_scene) => match main_sub_scene {
                MainSubScene::SideBar => handle_sidebar(app, key_event, player_handler),
                MainSubScene::Player => {
                    handle_player(app, key_event, player_handler);
                }
            },
            Scene::Detail => todo!(),
        }
    }

    fn handle_sidebar(app: &mut App, key_event: KeyEvent, player_handler: &mut PlayerHandler) {
        match key_event.code {
            KeyCode::Char('j') => app.music_lib.select_next(),
            KeyCode::Char('k') => app.music_lib.select_prev(),
            KeyCode::Char('q') => app.quit(),
            KeyCode::Tab => app.scene = Scene::Main(MainSubScene::Player),
            KeyCode::Enter => {
                if let Some(idx) = app.music_lib.sel_song.selected() {
                    let song_path = app.music_lib.songs[idx].path.clone();

                    // NOTE: 这里有一个设计上的取舍：我们提前设置了 play_index，因为它属于播放调度状态；但是没有提前设置 cur_song，因为还不知道播放是否成功。
                    app.music_player_state
                        .init_play_list(app.music_lib.songs.len(), idx);
                    player_handler.send(PlayerCommand::Play {
                        index: idx,
                        path: song_path,
                    });
                }
            }
            _ => {}
        }
    }

    fn handle_player(app: &mut App, key_event: KeyEvent, player_handler: &mut PlayerHandler) {
        match key_event.code {
            KeyCode::Char(' ') => {
                if app.music_player_state.cur_song.is_some() {
                    match app.music_player_state.play_state {
                        PlayState::Idle => {}
                        PlayState::Playing => {
                            player_handler.send(PlayerCommand::Pause);
                        }
                        PlayState::Pause => {
                            player_handler.send(PlayerCommand::Resume);
                        }
                    }
                }
            }
            KeyCode::Up if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                let new_vol = app.music_player_state.volume + 0.1;
                let new_vol = new_vol.clamp(0.0, 1.0);
                player_handler.send(PlayerCommand::SetVolume(new_vol));
            }
            KeyCode::Down if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                let new_vol = app.music_player_state.volume - 0.1;
                let new_vol = new_vol.clamp(0.0, 1.0);
                player_handler.send(PlayerCommand::SetVolume(new_vol));
            }
            KeyCode::Char('m') => {
                player_handler.send(PlayerCommand::ToggleMute);
            }
            KeyCode::Tab => app.scene = Scene::Main(MainSubScene::SideBar),
            _ => {}
        }
    }
}

pub fn handle_player_event(app: &mut App, player_event: PlayerEvent) {
    match player_event {
        PlayerEvent::Started(index) => {
            app.music_player_state.play_state = PlayState::Playing;
            app.music_player_state.cur_song = Some(index);
            app.music_player_state.position = Duration::ZERO;
        }
        PlayerEvent::Paused => {
            app.music_player_state.play_state = PlayState::Pause;
        }
        PlayerEvent::Resumed => {
            app.music_player_state.play_state = PlayState::Playing;
        }
        PlayerEvent::Stopped => {
            app.music_player_state.play_state = PlayState::Idle;
            app.music_player_state.cur_song = None;
            app.music_player_state.position = Duration::ZERO;
        }
        PlayerEvent::MuteChanged(is_muted) => {
            app.music_player_state.is_mute = is_muted;
        }
        PlayerEvent::VolumeChanged(vol) => {
            app.music_player_state.volume = vol;
        }
        PlayerEvent::Finished => {}
        PlayerEvent::PositionUpdated(duration) => {}
        PlayerEvent::Error(_) => {}
    }
}
