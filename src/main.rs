use crate::adapter::Adapter;

mod adapter;

fn app() -> anyhow::Result<()> {
    let adapter = Adapter::new("myproject")?;
    adapter.close();
    Ok(())
}

fn main() {
    if let Err(err) = app() {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}
