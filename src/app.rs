use std::{
    collections::HashMap,
    fs::{self, Metadata},
    path::{Path, PathBuf},
    time::Duration,
};

use color_eyre::eyre::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    read_from_path,
    tag::Accessor,
};
use ratatui::widgets::ListState;

use crate::{app::Scene::Main, event::Event, player::PlayerEvent};

pub enum Scene {
    Main(MainSubScene),
    Detail,
}
pub enum MainSubScene {
    SideBar,
    Player,
}
pub struct Song {
    pub name: String,
    pub author: Option<String>,
    pub album: Option<String>,
    pub duration: Duration,
    pub path: PathBuf,
}

impl Song {
    pub fn from_path(path: PathBuf) -> Self {
        let file = read_from_path(&path).unwrap();
        let duration = file.properties().duration();

        let mut name_ = path.file_stem().unwrap().to_string_lossy().into_owned();
        let mut author_: Option<String> = None;
        let mut album_: Option<String> = None;
        if let Some(tag) = file.primary_tag() {
            if let Some(name) = tag.title() {
                name_ = String::from(name);
                author_ = tag.artist().map(|s| s.into_owned());
                album_ = tag.album().map(|s| s.into_owned());
            }
        }

        Self {
            name: name_,
            author: author_,
            album: album_,
            duration: duration,
            path: path,
        }
    }
}

pub struct MusicPlayerState {
    pub cur_song: Option<usize>,
    pub play_state: PlayState,
    pub play_list: Vec<usize>,
    pub play_index: Option<usize>,
    pub play_mode: PlayMode,
    pub position: Duration,
    pub volume: f32,
    pub is_mute: bool,
}

impl MusicPlayerState {
    pub fn new() -> Self {
        Self {
            cur_song: None,
            play_state: PlayState::Idle,
            play_list: Vec::new(),
            play_index: None,
            play_mode: PlayMode::Circle,
            position: Duration::ZERO,
            volume: 1.0,
            is_mute: false,
        }
    }

    pub fn init_play_list(&mut self, song_count: usize, selected: usize) {
        self.play_list.clear();
        match self.play_mode {
            PlayMode::Circle => {
                self.play_list = (0..song_count).collect();
                self.play_index = Some(selected);
            }
            PlayMode::One => {
                self.play_list.push(selected);
                self.play_index = Some(0);
            }
            PlayMode::Random => {
                // TODO: impl with rand
                self.play_list = (0..song_count).collect();
                self.play_index = self.play_list.iter().position(|&idx| idx == selected);
            }
        }
    }
}

pub enum PlayState {
    Idle,
    Playing,
    Pause,
}
pub enum PlayMode {
    Circle,
    One,
    Random,
}

pub struct MusicLib {
    pub songs: Vec<Song>,
    pub sel_song: ListState,
    pub path: String,
}
impl MusicLib {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            sel_song: ListState::default().with_selected(None),
            path: String::new(),
        }
    }
    pub fn load_music_lib(&mut self, path: String) -> Result<()> {
        self.path = path;
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let is_audio = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    matches!(
                        ext.to_ascii_lowercase().as_str(),
                        "mp3" | "flac" | "m4a" | "wav" | "ogg"
                    )
                });

            if !is_audio {
                continue;
            }

            let song = Song::from_path(path);

            self.songs.push(song);
        }

        self.songs.sort_by(|a, b| a.name.cmp(&b.name));

        if !self.songs.is_empty() {
            self.sel_song = ListState::default().with_selected(Some(0));
        }

        Ok(())
    }

    pub fn select_next(&mut self) {
        if self.songs.is_empty() {
            return;
        }

        if self.sel_song.selected().unwrap() == self.songs.len() - 1 {
            self.sel_song.select_first();
        } else {
            self.sel_song.select_next();
        }
    }
    pub fn select_prev(&mut self) {
        if self.songs.is_empty() {
            return;
        }
        if self.sel_song.selected().unwrap() == 0 {
            self.sel_song.select_last();
        } else {
            self.sel_song.select_previous();
        }
    }
}
pub struct App {
    pub running: bool,
    pub scene: Scene,
    pub music_player_state: MusicPlayerState,
    pub music_lib: MusicLib,
}

pub enum AppEvent {
    Terminal(Event),
    Player(PlayerEvent),
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            scene: Main(MainSubScene::SideBar),
            music_player_state: MusicPlayerState::new(),
            music_lib: MusicLib::new(),
        }
    }

    pub fn init(&mut self, path: String) -> Result<()> {
        self.music_lib.load_music_lib(path)?;
        self.music_player_state = MusicPlayerState::new();
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn play_sel_music(&self) {
        todo!()
    }
}
