mod app;
mod event;
mod macros;
mod msg;
mod pages;
mod panes;
mod util;
mod widget;

use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let (_, rx) = event::new();
    let ret = App::new().start(&mut terminal, rx);
    ratatui::try_restore()?;
    ret
}
