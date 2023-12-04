// bin/sjmb.rs

use anyhow::anyhow;
use aoc2023::*;
use clap::Parser;
use log::*;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut n = 0;
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        n += 1;
        let line = line?;
        let lt = line.trim();
        let d1 = lt
            .chars()
            .find_map(|c| if c.is_numeric() { c.to_digit(10) } else { None })
            .ok_or_else(|| anyhow!("No first digit"))?;
        let d2 = lt
            .chars()
            .rev()
            .find_map(|c| if c.is_numeric() { c.to_digit(10) } else { None })
            .ok_or_else(|| anyhow!("No last digit"))?;

        info!("#{n:0>4} {d1}{d2} <-- {lt}");
        sum += (d1 as u64) * 10 + d2 as u64;

        if let Some(max) = opts.max_iter {
            if n >= max {
                break;
            }
        }
    }
    println!("Sum: {sum}");

    Ok(())
}
// EOF
