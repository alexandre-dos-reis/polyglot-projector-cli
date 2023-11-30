use anyhow::Result;
use clap::Parser;
use projector_cli::{config::Config, opts::Opts};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    println!("{:?}", config);

    return Ok(());
}
