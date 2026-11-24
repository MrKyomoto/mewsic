use color_eyre::eyre::Result;
use rodio::MixerDeviceSink;
use rodio::Player;
use rodio::{Decoder, DeviceSinkBuilder};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::{path::PathBuf, time::Duration};

use crate::app::AppEvent;

pub enum PlayerCommand {
    Play { index: usize, path: PathBuf },
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    ToggleMute,
    ShutDown,
}

pub enum PlayerEvent {
    Started(usize),
    Paused,
    Resumed,
    Stopped,
    MuteChanged(bool),
    VolumeChanged(f32),
    Finished,
    PositionUpdated(Duration),
    Error(String),
}

pub struct PlayerEngine {
    _sink: MixerDeviceSink,
    player: Player,
    is_muted: bool,
    volume: f32,
}

impl PlayerEngine {
    pub fn new() -> Result<Self> {
        let sink = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(sink.mixer());
        Ok(Self {
            _sink: sink,
            player,
            is_muted: false,
            volume: 0.0,
        })
    }

    pub fn play(&self, path: &PathBuf) -> Result<()> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::try_from(file)?;

        self.player.stop();
        self.player.append(source);
        self.player.play();

        Ok(())
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn stop(&self) {
        self.player.stop();
    }

    pub fn get_pos(&self) -> Duration {
        self.player.get_pos()
    }
    pub fn set_volume(&mut self, v: f32) {
        self.volume = v.clamp(0.0, 1.0);
        if !self.is_muted {
            self.player.set_volume(self.volume);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn mute(&mut self) {
        if self.is_muted {
            return;
        }

        self.is_muted = true;
        self.player.set_volume(0.0);
    }

    pub fn unmute(&mut self) {
        if !self.is_muted {
            return;
        }

        self.is_muted = false;
        self.player.set_volume(self.volume);
    }
}

pub struct PlayerHandler {
    sender: Sender<PlayerCommand>,
    _handle: JoinHandle<()>,
}

impl PlayerHandler {
    pub fn new(event_sender: Sender<AppEvent>, tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel::<PlayerCommand>();

        let _handle = {
            thread::spawn(move || {
                let mut engine = match PlayerEngine::new() {
                    Ok(engine) => engine,
                    Err(err) => {
                        let _ = event_sender
                            .send(AppEvent::Player(PlayerEvent::Error(err.to_string())));
                        return;
                    }
                };

                let mut has_active_track = false;
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    match receiver.recv_timeout(timeout) {
                        Ok(player_command) => match player_command {
                            PlayerCommand::Play { index, path } => match engine.play(&path) {
                                Ok(()) => {
                                    has_active_track = true;
                                    let _ = event_sender
                                        .send(AppEvent::Player(PlayerEvent::Started(index)));
                                }
                                Err(err) => {
                                    let _ = event_sender.send(AppEvent::Player(
                                        PlayerEvent::Error(err.to_string()),
                                    ));
                                }
                            },
                            PlayerCommand::Pause => {
                                engine.pause();
                                let _ = event_sender.send(AppEvent::Player(PlayerEvent::Paused));
                            }
                            PlayerCommand::Resume => {
                                engine.resume();
                                let _ = event_sender.send(AppEvent::Player(PlayerEvent::Resumed));
                            }
                            PlayerCommand::Stop => {
                                engine.stop();
                                has_active_track = false;
                                let _ = event_sender.send(AppEvent::Player(PlayerEvent::Stopped));
                            }
                            PlayerCommand::SetVolume(v) => {
                                engine.set_volume(v);
                                let _ = event_sender.send(AppEvent::Player(
                                    PlayerEvent::VolumeChanged(engine.volume),
                                ));
                            }
                            PlayerCommand::ToggleMute => {
                                if engine.is_muted {
                                    engine.unmute();
                                } else {
                                    engine.mute();
                                }
                                let _ = event_sender.send(AppEvent::Player(
                                    PlayerEvent::MuteChanged(engine.is_muted),
                                ));
                            }
                            PlayerCommand::ShutDown => {
                                break;
                            }
                        },
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            break;
                        }
                    };

                    if last_tick.elapsed() >= tick_rate {
                        if has_active_track && engine.is_empty() {
                            has_active_track = false;
                            let _ = event_sender.send(AppEvent::Player(PlayerEvent::Finished));
                        } else if has_active_track {
                            let _ = event_sender.send(AppEvent::Player(
                                PlayerEvent::PositionUpdated(engine.get_pos()),
                            ));
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self { sender, _handle }
    }

    pub fn send(&self, command: PlayerCommand) {
        let _ = self.sender.send(command);
    }
}
