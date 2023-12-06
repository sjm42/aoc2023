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

    let mut cards = Vec::new();
    let mut card_wins = Vec::new();
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
        trace!("Winning[#{n}]: {win_v:?}");
        let mut w_hash = HashSet::new();
        for v in win_v.iter() {
            w_hash.insert(*v);
        }

        let have_v = have
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<_>>();
        trace!("Have[#{n}]: {have_v:?}");

        let (mut win_cnt, mut score) = (0, 0);
        for h in have_v.iter() {
            if w_hash.contains(h) {
                win_cnt += 1;
                score = match score {
                    0 => 1,
                    n => n * 2,
                };
            }
        }

        cards.push(n as i32);
        card_wins.push(win_cnt);

        info!("Card#{n} wins: {win_cnt} score: {score}");
        score_sum += score;

        n += 1;
        if let Some(max) = opts.max_iter {
            if n >= max {
                break;
            }
        }
    }
    println!("Score: {score_sum}");

    let cards = calc_cards(&card_wins, cards, 0);
    println!("Cards# {num}", num = cards.len());

    Ok(())
}

fn calc_cards(wins: &[i32], mut cards: Vec<i32>, r: usize) -> Vec<i32> {
    let mut copies = Vec::new();
    for c in cards.iter() {
        let win_count = wins[*c as usize];
        trace!("Calc{r} Card#{c} wins {win_count}");
        (*c..*c + win_count).for_each(|n| copies.push(n + 1));
        trace!(" calc{r} copies: {copies:?}");
    }
    trace!(" calc[{r}] all copies: {copies:?}");
    if !copies.is_empty() {
        let mut new_copies = calc_cards(wins, copies, r + 1);
        trace!("  calc[{r}] new copies: {new_copies:?}");
        cards.append(&mut new_copies);
    }

    cards
}
// EOF
