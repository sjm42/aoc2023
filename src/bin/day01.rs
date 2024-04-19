// bin/day01.rs

use std::io::{self, BufRead};

use anyhow::anyhow;

use aoc2023::*;

const NUMS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let (mut n, mut sum) = (0, 0);
    for line in io::stdin().lock().lines() {
        n += 1;
        let line = line?;
        let line = line.trim();

        let c_vec = NUMS
            .iter()
            .filter_map(|n| Some((line.find(n.0)?, n.1)))
            .chain(NUMS.iter().filter_map(|n| Some((line.rfind(n.0)?, n.1))))
            .collect::<Vec<(usize, u32)>>();

        let d_vec = line
            .chars()
            .enumerate()
            .filter_map(|c| match c.1.is_numeric() {
                true => Some((c.0, c.1.to_digit(10)?)),
                _ => None,
            })
            .collect::<Vec<(usize, u32)>>();
        debug!("c_vec {c_vec:?} d_vec {d_vec:?}");

        let n1 = c_vec
            .iter()
            .chain(d_vec.iter())
            .min_by_key(|i| i.0)
            .ok_or_else(|| anyhow!("no num"))?
            .1;

        let n2 = c_vec
            .iter()
            .chain(d_vec.iter())
            .max_by_key(|i| i.0)
            .ok_or_else(|| anyhow!("no num"))?
            .1;

        let num = 10 * (n1 as u64) + n2 as u64;
        info!("#{n:0>4} {num} <-- {line}");
        sum += num;

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
