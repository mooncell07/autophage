mod adapter;
mod models;
mod ui;
use adapter::Adapter;
use ui::UserInterface;

pub fn run() -> anyhow::Result<()> {
    let adapter = Adapter::new("myproject")?;
    let mut ui = UserInterface::new(adapter);

    ratatui::run(|terminal| ui.run(terminal))?;

    Ok(())
}
