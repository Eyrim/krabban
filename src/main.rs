use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};

mod ui;
mod ticket;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = ui::App::new();

    let result = app.run(&mut terminal);

    ratatui::restore();
    Ok(())
    //result?
}

