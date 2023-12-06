// bin/day05.rs

use aoc2023::*;

use anyhow::anyhow;
use clap::Parser;
use itertools::Itertools;
use log::*;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
pub enum State {
    Seeds,
    ToSoil,
    ToFertilizer,
    ToWater,
    ToLight,
    ToTemperature,
    ToHumidity,
    ToLocation,
}

#[derive(Clone, Debug)]
pub struct MapItem {
    pub src: i64,
    pub len: i64,
    pub dst: i64,
}

pub fn map_add<'a>(
    map: &'a mut Vec<MapItem>,
    values: &str,
) -> anyhow::Result<&'a mut Vec<MapItem>> {
    let (dst, src, len) = values
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap_or(-1))
        .collect_tuple()
        .ok_or_else(|| anyhow!("map values error: {values:?}"))?;
    map.push(MapItem { src, len, dst });
    Ok(map)
}

pub fn map_value(map: &[MapItem], v: i64) -> i64 {
    for m in map {
        if v >= m.src && v < m.src + m.len {
            return m.dst + (v - m.src);
        }
    }
    v
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut n = 0;
    let mut state = State::Seeds;
    let mut seeds = Vec::new();

    let mut to_soil = Vec::new();
    let mut to_fertilizer = Vec::new();
    let mut to_water = Vec::new();
    let mut to_light = Vec::new();
    let mut to_temperature = Vec::new();
    let mut to_humidity = Vec::new();
    let mut to_location = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match state {
            State::Seeds => {
                if line.starts_with("seeds:") {
                    let (_, nums) = line
                        .split_once(':')
                        .ok_or_else(|| anyhow!("parse error: line#{n} {line:?}"))?;
                    let nums = nums.trim();
                    seeds = nums
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap_or(-1))
                        .collect::<Vec<_>>();
                }
                if line.starts_with("seed-to-") {
                    state = State::ToSoil;
                }
            }
            State::ToSoil => {
                if line.starts_with("soil-to-") {
                    state = State::ToFertilizer;
                    continue;
                }
                map_add(&mut to_soil, line)?;
            }
            State::ToFertilizer => {
                if line.starts_with("fertilizer-to-") {
                    state = State::ToWater;
                    continue;
                }
                map_add(&mut to_fertilizer, line)?;
            }
            State::ToWater => {
                if line.starts_with("water-to-") {
                    state = State::ToLight;
                    continue;
                }
                map_add(&mut to_water, line)?;
            }
            State::ToLight => {
                if line.starts_with("light-to-") {
                    state = State::ToTemperature;
                    continue;
                }
                map_add(&mut to_light, line)?;
            }
            State::ToTemperature => {
                if line.starts_with("temperature-to-") {
                    state = State::ToHumidity;
                    continue;
                }
                map_add(&mut to_temperature, line)?;
            }
            State::ToHumidity => {
                if line.starts_with("humidity-to-") {
                    state = State::ToLocation;
                    continue;
                }
                map_add(&mut to_humidity, line)?;
            }
            State::ToLocation => {
                map_add(&mut to_location, line)?;
            }
        }

        n += 1;
    }

    debug!("seeds: {seeds:?}");
    debug!("to_soil: {to_soil:?}");
    debug!("to_fertilizer: {to_fertilizer:?}");
    debug!("to_water: {to_water:?}");
    debug!("to_light: {to_light:?}");
    debug!("to_temperature: {to_temperature:?}");
    debug!("to_humidity: {to_humidity:?}");
    debug!("to_location: {to_location:?}");

    let mut low_loc = i64::MAX;
    for sd in seeds.iter() {
        let s = map_value(&to_soil, *sd);
        let f = map_value(&to_fertilizer, s);
        let w = map_value(&to_water, f);
        let l = map_value(&to_light, w);
        let t = map_value(&to_temperature, l);
        let h = map_value(&to_humidity, t);
        let loc = map_value(&to_location, h);
        low_loc = std::cmp::min(low_loc, loc);
        debug!("seed {sd} soil {s} fert {f} wat {w} lt {l} temp {t} hum {h} loc {loc}");
    }
    println!("Lowest location PART 1: {low_loc}");

    low_loc = i64::MAX;
    for v in seeds.clone().chunks_exact(2) {
        for sd in v[0]..v[0] + v[1] {
            let s = map_value(&to_soil, sd);
            let f = map_value(&to_fertilizer, s);
            let w = map_value(&to_water, f);
            let l = map_value(&to_light, w);
            let t = map_value(&to_temperature, l);
            let h = map_value(&to_humidity, t);
            let loc = map_value(&to_location, h);
            low_loc = std::cmp::min(low_loc, loc);
            debug!("seed {sd} soil {s} fert {f} wat {w} lt {l} temp {t} hum {h} loc {loc}");
        }
    }
    println!("Lowest location PART 2: {low_loc}");

    Ok(())
}
// EOF
