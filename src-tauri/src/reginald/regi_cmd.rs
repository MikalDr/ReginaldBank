use serde::{Deserialize, Serialize};

use super::{coin::Denomination, item::RegiItem};

#[derive(Debug, Serialize, Deserialize)]
pub enum RegiCmd {
    Help(Option<Box<RegiCmd>>),
    LogDate(Option<i32>, Vec<RegiFlag>),
    LogSession(u32, Vec<RegiFlag>),
    LogAll,
    Add(Result<Denomination, RegiItem>, Vec<RegiFlag>),
    Funds(Vec<RegiFlag>),
    Bag(Vec<RegiFlag>),
    Find(RegiItem, Vec<RegiFlag>),
    Take(Result<Denomination, RegiItem>, Vec<RegiFlag>),
    RollDice(Vec<Dice>, Option<u32>),
    None,
}

impl RegiCmd {
    pub fn parse(input: String) -> Option<Self> {
        match input.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            // Help all
            ["help"] => Some(Self::Help(Some(Box::new(Self::None)))),
            // Help about the given command
            ["help", ref strs] => match Self::parse(strs.to_string()) {
                Some(cmd) => Some(Self::Help(Some(Box::new(cmd)))),
                None => Some(Self::Help(None)),
            },
            // List logs
            ["log"] => Some(Self::LogAll),
            // Log, Date || Session, Flags
            ["log", ref strs @ ..] => {
                let (flgs, rem) = RegiFlag::parse(strs.join(""));
                match rem.parse::<u32>() {
                    Ok(i) => Some(Self::LogSession(i, flgs)),
                    Err(_) => todo!(), // It's probably a date
                }
            }
            // Add, Item || Coins, Flags
            ["add", ref strs @ ..] => {
                let (flgs, rem) = RegiFlag::parse(strs.join(""));

                if let Some(c) = Denomination::parse(&rem) {
                    return Some(Self::Add(Ok(c), flgs));
                }

                if let Some(item) = RegiItem::parse(&rem) {
                    return Some(Self::Add(Err(item), flgs));
                }

                None
            }
            // Show funds in a given currency, Flags
            // Should probably not ignore the remainder of strings
            ["funds", ref strs @ ..] => Some(Self::Funds(RegiFlag::parse(strs.join("")).0)),
            // Take, Item || Coins, Flags
            // No way to take N items, atm.
            ["take", ref strs @ ..] => {
                let (flgs, rem) = RegiFlag::parse(strs.join(""));

                if let Some(c) = Denomination::parse(&rem) {
                    return Some(Self::Take(Ok(c), flgs));
                }

                if let Some(item) = RegiItem::parse(&rem) {
                    return Some(Self::Take(Err(item), flgs));
                }
                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegiFlag {
    NoLog,
    NoEffect,
    Explain,
}

impl RegiFlag {
    pub fn parse(input: String) -> (Vec<Self>, String) {
        let mut flags: Vec<Self> = Vec::new();
        let mut rem = input.clone();
        let mut res = None;
        for _ in 0..input.split_ascii_whitespace().count() {
            (res, rem) = Self::parse_helper(&rem);

            if let Some(flg) = res {
                flags.push(flg);
            }

            if rem.len() == 0 {
                return (flags, rem);
            }
        }

        return (flags, rem);
    }

    fn parse_helper(input: &str) -> (Option<Self>, String) {
        match input.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            ["-i", ref strs @ ..] => (Some(Self::NoLog), strs.join("")),
            ["-t", ref strs @ ..] => (Some(Self::NoEffect), strs.join("")),
            ["-h", ref strs @ ..] => (Some(Self::Explain), strs.join("")),
            [_, ref strs @ ..] => (None, strs.join("")),
            _ => (None, "".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dice {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}
