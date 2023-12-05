// bin/day04.rs

use aoc2023::*;

// use anyhow::anyhow;
use clap::Parser;
// use log::*;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut n = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();
        let _linesz = line.len();

        n += 1;
    }

    Ok(())
}

// EOF
