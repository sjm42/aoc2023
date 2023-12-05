// bin/day04.rs

use aoc2023::*;

use anyhow::anyhow;
use clap::Parser;
use log::*;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut score_sum = 0;
    let mut n = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();

        let (_id, nums) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("parse error: line#{n} {line:?}"))?;
        let nums = nums.trim();

        let (winning, have) = nums
            .split_once('|')
            .ok_or_else(|| anyhow!("parse error: line#{n} nums: {nums:?}"))?;
        let (winning, have) = (winning.trim(), have.trim());

        let win_v = winning
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<_>>();
        debug!("Winning[#{n}]: {win_v:?}");
        let mut w_hash = HashSet::new();
        for v in win_v.iter() {
            w_hash.insert(*v);
        }

        let have_v = have
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<_>>();
        debug!("Have[#{n}]: {have_v:?}");

        let mut score = 0;
        for h in have_v.iter() {
            if w_hash.contains(h) {
                score = match score {
                    0 => 1,
                    n => n * 2,
                };
            }
        }

        info!("Score #{n} {score}");
        score_sum += score;

        n += 1;
        if let Some(max) = opts.max_iter {
            if n >= max {
                break;
            }
        }
    }

    println!("Score: {score_sum}");
    Ok(())
}

// EOF
