use crate::adapter::Adapter;
use crate::ui::ui_main;
mod adapter;
mod ui;

fn app() -> anyhow::Result<()> {
    let adapter = Adapter::new("myproject")?;
    ui_main(adapter);

    Ok(())
}

fn main() {
    if let Err(err) = app() {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}
