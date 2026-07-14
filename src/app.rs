mod adapter;
mod ui;
use adapter::Adapter;
use ui::ui_main;

pub fn run() -> anyhow::Result<()> {
    let adapter = Adapter::new("myproject")?;
    ui_main(adapter);

    Ok(())
}
