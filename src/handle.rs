use crate::app::MainSubScene;
use crate::app::Scene;
use crate::{app::App, event::Event};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle(app: &mut App, event: Event) {
    match event {
        Event::Tick => {}
        Event::Key(key_event) => key::handle(app, key_event),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
    };
}

mod key {

    use super::*;
    pub fn handle(app: &mut App, key_event: KeyEvent) {
        match &app.scene {
            Scene::Main(main_sub_scene) => match main_sub_scene {
                MainSubScene::SideBar => handle_sidebar(app, key_event),
                MainSubScene::Player => {
                    todo!()
                }
            },
            Scene::Detail => todo!(),
        }
    }

    fn handle_sidebar(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') => app.music_lib.select_next(),
            KeyCode::Char('k') => app.music_lib.select_prev(),
            KeyCode::Char('q') => app.quit(),
            _ => {}
        }
    }
}
