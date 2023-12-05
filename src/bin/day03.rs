// bin/sjmb.rs

use anyhow::anyhow;
use aoc2023::*;
use clap::Parser;
use colored::*;
use log::*;
use std::io::{self, BufRead};
use std::str;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CharType {
    Dot = b'.',
    Num = b'N',
    Sym = b'#',
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut ctype_map = Vec::new();
    let mut found_nums = Vec::new();
    let mut lines = Vec::new();

    let mut n = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();
        let line = format!(".{line}."); // nasty trick to avoid index overruns
        let linesz = line.len();

        let mut num_v = Vec::new();
        let mut ctype_line = Vec::with_capacity(linesz);

        for (idx, chr) in line.chars().enumerate() {
            let ctype = if chr == '.' {
                CharType::Dot
            } else if chr.is_ascii_digit() {
                CharType::Num
            } else if chr.is_ascii_punctuation() {
                CharType::Sym
            } else {
                return Err(anyhow!("Illegal char {chr} at [{n}][{idx}]"));
            };

            if ctype == CharType::Num {
                // keep building a number
                num_v.push((idx, chr));
            } else if !num_v.is_empty() {
                // we had a number building up, save it
                let start_idx = num_v[0].0;
                let num_len = num_v.len();
                let num = num_v
                    .iter()
                    .map(|i| i.1)
                    .collect::<Vec<char>>()
                    .into_iter()
                    .collect::<String>()
                    .as_str()
                    .parse::<i32>()?;

                // +1 is because we will be adding one row at front
                found_nums.push((num, n + 1, start_idx, num_len));
                num_v.clear();
            }

            ctype_line.push(ctype);
        }

        let ctype_u8 = ctype_line.iter().map(|c| *c as u8).collect::<Vec<u8>>();
        let ctype_s = str::from_utf8(ctype_u8.as_slice())?;
        debug!("Line: {line}");
        trace!("tmap: {ctype_s}");

        lines.push(line);
        ctype_map.push(ctype_line);

        n += 1;
        if let Some(max) = opts.max_iter {
            if n >= max {
                break;
            }
        }
    }
    debug!("Found nums: {found_nums:?}");

    // Assumption: all input lines are equal length
    // We are adding "empty" lines as the first and last one to avoid index overruns
    let sz = ctype_map[0].len();

    let mut pad = Vec::with_capacity(sz);
    (0..sz).for_each(|_| pad.push(CharType::Dot));
    trace!("Pad: {pad:?}");
    ctype_map.insert(0, pad.clone());
    ctype_map.push(pad);

    let mut lpad = Vec::with_capacity(sz);
    (0..sz).for_each(|_| lpad.push(b'.'));
    let lpad = str::from_utf8(lpad.as_slice())?;
    trace!("Lpad: {lpad}");
    lines.insert(0, lpad.to_string());
    lines.push(lpad.to_string());

    let map_sz = ctype_map.len();
    trace!("ctype_map[{map_sz}]:\n{ctype_map:?}");

    let mut sum = 0;
    let mut valid_nums = Vec::new();
    for (num, row, start, size) in found_nums {
        debug!("Checking {num} at ({row}, {start}) len {size}");
        if ctype_map[row][start - 1] == CharType::Sym
            || ctype_map[row][start + size] == CharType::Sym
        {
            debug!("PASS at own row{row}");
            valid_nums.push((num, row, start, size));
            sum += num;
            continue;
        }
        'outer: for r in [row - 1, row + 1] {
            for i in (start - 1)..(start + size + 1) {
                debug!("Checking row{r}[{i}]");
                if ctype_map[r][i] == CharType::Sym {
                    debug!("PASS at row{r}[{i}]");
                    valid_nums.push((num, row, start, size));
                    sum += num;
                    break 'outer;
                }
            }
        }
    }
    info!("Valid nums: {valid_nums:?}");

    if opts.debug {
        // populate a hilite matrix
        let mut hilite = Vec::with_capacity(lines.len());
        for l in lines.iter() {
            let sz = l.len();
            let mut h = Vec::with_capacity(sz);
            (0..sz).for_each(|_| h.push(false));
            hilite.push(h);
        }

        for (_num, row, start, size) in valid_nums {
            (start..start + size).for_each(|i| hilite[row][i] = true);
        }
        // print each line with hilite
        for (i, l) in lines.iter().enumerate() {
            l.chars().enumerate().for_each(|(j, c)| {
                if hilite[i][j] {
                    print!("{}", c.to_string().as_str().red());
                } else {
                    print!("{c}");
                }
            });
            println!();
        }
    }

    println!("Sum: {sum}");

    Ok(())
}

// EOF
