// bin/day05.rs

use aoc2023::*;

use anyhow::anyhow;
use clap::Parser;
use log::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            _ => Card::Two,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Card2 {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl From<char> for Card2 {
    fn from(c: char) -> Self {
        match c {
            'A' => Card2::A,
            'K' => Card2::K,
            'Q' => Card2::Q,
            'J' => Card2::J,
            'T' => Card2::T,
            '9' => Card2::Nine,
            '8' => Card2::Eight,
            '7' => Card2::Seven,
            '6' => Card2::Six,
            '5' => Card2::Five,
            '4' => Card2::Four,
            '3' => Card2::Three,
            _ => Card2::Two,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Kind {
    Five = 32,
    Four = 28,
    FullHouse = 24,
    Three = 20,
    TwoPair = 16,
    OnePair = 12,
    High = 8,
    None = 4,
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        let mut f: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            *f.entry(c).or_insert(0) += 1;
        }
        trace!("Card {s} report: {f:?}");
        let mut g = f.values().copied().collect::<Vec<_>>();
        g.sort_by(|a, b| b.partial_cmp(a).unwrap());
        trace!("Card {s} sort: {g:?}");

        if g[0] == 5 {
            return Kind::Five;
        }
        if g[0] == 4 {
            return Kind::Four;
        }
        if g[0] == 3 && g[1] == 2 {
            return Kind::FullHouse;
        }
        if g[0] == 3 {
            return Kind::Three;
        }
        if g[0] == 2 && g[1] == 2 {
            return Kind::TwoPair;
        }
        if g[0] == 2 {
            return Kind::OnePair;
        }
        if g.len() == 5 {
            // every card is different
            return Kind::High;
        }

        Kind::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Kind2 {
    Five = 32,
    Four = 28,
    FullHouse = 24,
    Three = 20,
    TwoPair = 16,
    OnePair = 12,
    High = 8,
    None = 4,
}

impl From<&str> for Kind2 {
    fn from(s: &str) -> Self {
        let mut f: HashMap<char, u32> = HashMap::new();
        f.insert('J', 0);
        let mut n_j = 0u32;
        for c in s.chars() {
            if c == 'J' {
                n_j += 1;
            } else {
                *f.entry(c).or_insert(0) += 1;
            }
        }
        trace!("Card {s} report: {f:?}");
        let mut g = f.values().copied().collect::<Vec<_>>();
        g.sort_by(|a, b| b.partial_cmp(a).unwrap());
        g[0] += n_j;
        trace!("Card {s} sort: {g:?}");

        if g[0] == 5 {
            return Kind2::Five;
        }
        if g[0] == 4 {
            return Kind2::Four;
        }
        if g[0] == 3 && g[1] == 2 {
            return Kind2::FullHouse;
        }
        if g[0] == 3 {
            return Kind2::Three;
        }
        if g[0] == 2 && g[1] == 2 {
            return Kind2::TwoPair;
        }
        if g[0] == 2 {
            return Kind2::OnePair;
        }
        if g.len() == 5 {
            // every card is different
            return Kind2::High;
        }

        Kind2::None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hand {
    pub chars: String,
    pub cards: Vec<Card>,
    pub cards2: Vec<Card2>,
    pub kind: Kind,
    pub kind2: Kind2,
    pub bid: usize,
}

pub fn hand_cmp(a: &Hand, b: &Hand) -> Ordering {
    if a.kind != b.kind {
        return (a.kind as u8).cmp(&(b.kind as u8));
    }
    // Kind is equal, compare card by card in order
    for (i, ac) in a.cards.iter().enumerate() {
        let bc = &b.cards[i];
        if *ac != *bc {
            return (*ac as u8).cmp(&(*bc as u8));
        }
    }
    Ordering::Equal
}

pub fn hand_cmp2(a: &Hand, b: &Hand) -> Ordering {
    if a.kind2 != b.kind2 {
        return (a.kind2 as u8).cmp(&(b.kind2 as u8));
    }
    // Kind2 is equal, compare card by card in order
    for (i, ac) in a.cards2.iter().enumerate() {
        let bc = &b.cards2[i];
        if *ac != *bc {
            return (*ac as u8).cmp(&(*bc as u8));
        }
    }
    Ordering::Equal
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finish()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut hands = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let items = line.split_whitespace().collect::<Vec<_>>();
        let cards = items[0].chars().map(Card::from).collect::<Vec<_>>();
        let cards2 = items[0].chars().map(Card2::from).collect::<Vec<_>>();
        if cards.len() != 5 {
            return Err(anyhow!("Line #{n} error: Hand must be exactly 5 cards."));
        }
        let hand = Hand {
            chars: items[0].to_string(),
            cards,
            cards2,
            kind: Kind::from(items[0]),
            kind2: Kind2::from(items[0]),
            bid: items[1].parse()?,
        };
        debug!("New hand: {hand:?}");
        hands.push(hand);
    }
    let mut hands2 = hands.clone();
    hands.sort_by(hand_cmp);
    hands2.sort_by(hand_cmp2);

    debug!("hands:\n{hands:#?}");
    debug!("hands2:\n{hands2:#?}");

    let mut total = 0;
    for (i, h) in hands.iter().enumerate() {
        let mul = (i + 1) * h.bid;
        total += mul;
    }

    let mut total2 = 0;
    for (i, h) in hands2.iter().enumerate() {
        let mul = (i + 1) * h.bid;
        total2 += mul;
    }

    println!("Total: {total}");
    println!("Total2: {total2}");

    Ok(())
}
// EOF
