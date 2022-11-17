use clap::Parser;
use selectors::{run, Config, Result};

fn main() -> Result<()> {
    let config = Config::parse();

    run(config)?;

    Ok(())
}
