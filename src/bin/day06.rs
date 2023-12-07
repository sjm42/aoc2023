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

    let mut times = Vec::new();
    let mut dists = Vec::new();
    let mut mul = 1i64;

    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (key, nums_s) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("parse error: line#{n} {line:?}"))?;
        let nums_s = nums_s.trim();
        let mut nums = nums_s
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap_or(-1))
            .collect::<Vec<_>>();

        if key == "Time" {
            times.append(&mut nums);
            continue;
        }
        if key == "Distance" {
            dists.append(&mut nums);
            continue;
        }
        error!("Cannot parse line: {line}");
    }

    info!("Times: {times:?}");
    info!("Dists: {dists:?}");

    if times.len() != dists.len() {
        return Err(anyhow!("Unbalanced numbers!"));
    }

    // ref. https://en.wikipedia.org/wiki/Quadratic_equation
    // ax^2 + bx + c = 0
    // where
    // a=1, x = charge time
    // -b = race time
    // c = distance
    // thus, we get
    //
    // x = (-b +/- sqrt(b^2 - 4ac)) / 2*a

    for i in 0..times.len() {
        let t = times[i] as f64;
        let d = dists[i] as f64;

        let discr = t * t - 4.0 * d;
        if discr < 0.0 {
            return Err(anyhow!(
                "Race [{i}] negative discriminant {discr}, no real roots."
            ));
        }
        let sqrt_discr = discr.sqrt();
        let x1 = 0.5 * (t - sqrt_discr);
        let x2 = 0.5 * (t + sqrt_discr);
        info!("Race#{i} x1 {x1} x2 {x2}");

        let i1 = (x1 as i64) + 1;
        let i2 = (x2 - 0.000000001) as i64;
        let beats = i2 - i1 + 1;
        mul *= beats;
        info!("Race#{i} i1 {i1} i2 {i2} #ways {beats}");
    }
    println!("Mul: {mul}");

    Ok(())
}
// EOF
