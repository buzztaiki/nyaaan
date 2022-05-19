use std::{
    fs::File,
    io::{stdin, stdout, BufReader},
};

use anyhow::Context;
use clap::Parser;

use crate::{nyaaan::Nyaaan, nyaaanize::nyaanize};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(long, default_value_t = String::from("nya"))]
    nya: String,
    #[clap(long, default_value_t = String::from("n"))]
    n: String,
    #[clap(help = "input files")]
    files: Vec<String>,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let nyaaan = Nyaaan::from_nyan(self.nya.as_ref(), self.n.as_ref())?;

        if self.files.is_empty() {
            nyaanize(&nyaaan, stdin().lock(), stdout().lock()).context("failed to nyaaan")?;
        } else {
            for file in &self.files {
                let input = File::open(file).context(format!("failed to open: {}", file))?;
                nyaanize(&nyaaan, BufReader::new(input), stdout().lock())
                    .context(format!("failed to nyaaan: {}", file))?;
            }
        }

        Ok(())
    }
}
