use clap::Parser;
use cli::Cli;

use crate::{adapter::Adapter, app::App};

mod action;
mod adapter;
mod app;
mod cli;
mod components;
mod errors;
mod logging;
mod models;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let adapter = Adapter::new("myproject").unwrap();
    let mut app = App::new(args.tick_rate, args.frame_rate, adapter)?;
    app.run().await?;
    Ok(())
}
