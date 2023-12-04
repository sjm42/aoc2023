// bin/sjmb.rs

use aoc2023::*;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    Ok(())
}
// EOF
