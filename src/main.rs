pub mod app;
pub mod event;
pub mod handle;
pub mod player;
pub mod song;
pub mod tui;
pub mod ui;

use std::sync::mpsc;

use app::App;
use color_eyre::Result;
use event::EventHandler;
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;

use crate::{app::AppEvent, player::PlayerHandler};

fn main() -> Result<()> {
    let mut app = App::new();
    app.init("asset/".to_string())?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let (sender, receiver) = mpsc::channel::<AppEvent>();

    let _terminal_handler = EventHandler::new(sender.clone(), 250);
    let mut player_hanlder = PlayerHandler::new(sender.clone(), 1000);

    let mut tui = Tui::new(terminal);
    tui.enter()?;

    tui.draw(&mut app)?;

    while app.running {
        match receiver.recv()? {
            AppEvent::Terminal(event) => {
                handle::handle_terminal_event(&mut app, event, &mut player_hanlder);
            }
            AppEvent::Player(player_event) => {
                handle::handle_player_event(&mut app, player_event, &mut player_hanlder);
            }
        }

        tui.draw(&mut app)?;
    }

    tui.exit()?;

    Ok(())
}
