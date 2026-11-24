pub mod app;
pub mod event;
pub mod handle;
pub mod tui;
pub mod ui;

use app::App;
use color_eyre::Result;
use event::EventHandler;
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;

fn main() -> Result<()> {
    let mut app = App::new();
    app.init("asset/".to_string())?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    tui.draw(&mut app)?;

    while app.running {
        let event = tui.events.next()?;
        handle::handle(&mut app, event);

        tui.draw(&mut app)?;
    }

    tui.exit()?;

    Ok(())
}
