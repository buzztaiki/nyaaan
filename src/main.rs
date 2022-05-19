use clap::Parser;
use cli::Cli;

mod cli;
mod errors;
mod nyaaan;
mod nyaaanize;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
