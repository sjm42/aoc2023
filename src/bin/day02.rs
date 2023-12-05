// bin/day02.rs

use aoc2023::*;

use anyhow::anyhow;
use clap::Parser;
use log::*;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let limits = HashMap::from([("blue", 14), ("green", 13), ("red", 12)]);
    debug!("Limits: {limits:?}");

    let (mut n, mut sum, mut sum_p) = (0, 0, 0);
    for line in io::stdin().lock().lines() {
        n += 1;
        let line = line?;
        let line = line.trim();

        let mut pass = true;
        let (g_id, rounds) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("parse error: line"))?;
        let (g_id, rounds) = (g_id.trim(), rounds.trim());
        let id = g_id
            .split_once(' ')
            .ok_or_else(|| anyhow!("parse error: game num"))?
            .1
            .parse::<i32>()?;

        let (mut p, mut max) = (1, HashMap::new());
        for r in rounds.split(';') {
            let r = r.trim();
            for n_c in r.split(',') {
                let n_c = n_c.trim();
                let (n, c) = n_c
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("parse error: {n_c}"))?;
                let n = n.parse::<i32>()?;
                if n > *limits.get(c).ok_or_else(|| anyhow!("Unknown color {c}"))? {
                    pass = false;
                }

                if n > *max.get(c).unwrap_or(&0) {
                    max.insert(c, n);
                }
            }
        }
        max.values().for_each(|v| p *= *v);

        info!("id {id} {pass} {p}");
        sum_p += p;
        if pass {
            sum += id;
        }

        if let Some(max) = opts.max_iter {
            if n >= max {
                break;
            }
        }
    }

    println!("Sum: {sum} Sum-p: {sum_p}");
    Ok(())
}
// EOF
