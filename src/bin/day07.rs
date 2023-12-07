// bin/day05.rs

use aoc2023::*;

use anyhow::anyhow;
use clap::Parser;
use log::*;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
    }

    Ok(())
}
// EOF
