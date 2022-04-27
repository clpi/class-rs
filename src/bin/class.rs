use class::*;
use owo_colors::{Color, OwoColorize, DynColor, colors::*};

fn main() -> anyhow::Result<()> {
    let cmd = class::Args::parse();
    println!("GOT ARGS: {:#?}", cmd.fg::<Red>());
    Ok(())
}
