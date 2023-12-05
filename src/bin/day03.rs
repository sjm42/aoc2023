// bin/sjmb.rs

use anyhow::anyhow;
use aoc2023::*;
use clap::Parser;
use colored::*;
use log::*;
use std::collections::HashMap;
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
    let mut found_gears = Vec::new();

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
                if chr == '*' {
                    found_gears.push((n + 1, idx));
                }
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
    debug!("Found nums (#{n}): {found_nums:?}", n = found_nums.len());
    debug!("Found gears (#{n}): {found_gears:?}", n = found_gears.len());

    // Assumption: all input lines are equal length
    // We are adding "empty" lines as the first and last one to avoid index overruns
    let line_sz = ctype_map[0].len();

    let mut pad = Vec::with_capacity(line_sz);
    (0..line_sz).for_each(|_| pad.push(CharType::Dot));
    trace!("Pad: {pad:?}");
    ctype_map.insert(0, pad.clone());
    ctype_map.push(pad);

    let mut lpad = Vec::with_capacity(line_sz);
    (0..line_sz).for_each(|_| lpad.push(b'.'));
    let lpad = str::from_utf8(lpad.as_slice())?;
    trace!("Lpad: {lpad}");
    lines.insert(0, lpad.to_string());
    lines.push(lpad.to_string());

    let map_sz = ctype_map.len();
    trace!("ctype_map[{map_sz}]:\n{ctype_map:?}");

    let mut sum = 0;
    let mut num_pos = Vec::with_capacity(map_sz);
    (0..map_sz).for_each(|_| {
        let mut v = Vec::with_capacity(line_sz);
        (0..line_sz).for_each(|_| v.push(None));
        num_pos.push(v);
    });

    let mut valid_nums = Vec::new();
    for (num, row, start, size) in found_nums {
        let numid = format!("{row}/{start}");
        (start..start + size).for_each(|i| {
            num_pos[row][i] = Some((numid.clone(), num));
        });

        trace!("Checking {num} at ({row}, {start}) len {size}");
        if ctype_map[row][start - 1] == CharType::Sym
            || ctype_map[row][start + size] == CharType::Sym
        {
            trace!("PASS at own row{row}");
            valid_nums.push((num, row, start, size));
            sum += num;
            continue;
        }
        'outer: for r in [row - 1, row + 1] {
            for i in (start - 1)..(start + size + 1) {
                trace!("Checking row{r}[{i}]");
                if ctype_map[r][i] == CharType::Sym {
                    trace!("PASS at row{r}[{i}]");
                    valid_nums.push((num, row, start, size));
                    sum += num;
                    break 'outer;
                }
            }
        }
    }
    info!("Valid nums (#{n}): {valid_nums:?}", n = valid_nums.len());

    let mut gear_sum = 0;
    let mut valid_gears = Vec::new();
    for (row, idx) in found_gears.iter() {
        let mut nums = HashMap::new();
        for (r, i) in [
            (*row - 1, *idx - 1),
            (*row - 1, *idx),
            (*row - 1, *idx + 1),
            (*row, *idx - 1),
            (*row, *idx + 1),
            (*row + 1, *idx - 1),
            (*row + 1, *idx),
            (*row + 1, *idx + 1),
        ] {
            if let Some((numid, num)) = &num_pos[r][i] {
                nums.insert(numid.clone(), *num);
            }
        }
        if nums.len() == 2 {
            valid_gears.push((*row, *idx));
            let gear_nums = nums.values().map(|n| *n).collect::<Vec<_>>();
            let (a, b) = (gear_nums[0], gear_nums[1]);
            let mul = a * b;
            debug!("BOOM found gear[{row},{idx}]: nums = {a} * {b} = {mul}");
            gear_sum += mul;
        } else {
            debug!("Invalid gear at [{row},{idx}]");
        }
    }
    info!("Valid gears (#{n}): {valid_gears:?}", n = valid_gears.len());

    if opts.debug {
        // populate a hilite + gears matrices
        let mut hilite = Vec::with_capacity(lines.len());
        let mut gears = Vec::with_capacity(lines.len());
        let mut gears_v = Vec::with_capacity(lines.len());
        for l in lines.iter() {
            let sz = l.len();
            let mut h = Vec::with_capacity(sz);
            (0..sz).for_each(|_| h.push(false));
            hilite.push(h.clone());
            gears.push(h.clone());
            gears_v.push(h);
        }
        for (_num, row, start, size) in valid_nums {
            (start..start + size).for_each(|i| hilite[row][i] = true);
        }
        found_gears.iter().for_each(|(r, i)| gears[*r][*i] = true);
        valid_gears.iter().for_each(|(r, i)| gears_v[*r][*i] = true);

        // print each line with hilite/color
        for (i, l) in lines.iter().enumerate() {
            l.chars().enumerate().for_each(|(j, c)| {
                if hilite[i][j] {
                    print!("{}", c.to_string().as_str().red());
                } else if gears_v[i][j] {
                    print!("{}", c.to_string().as_str().red());
                } else if gears[i][j] {
                    print!("{}", c.to_string().as_str().green());
                } else {
                    print!("{c}");
                }
            });
            println!();
        }
    }

    println!("Sum: {sum}");
    println!("Gear sum: {gear_sum}");

    Ok(())
}

// EOF
