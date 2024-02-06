use serde::{Deserialize, Serialize};

use self::{coin::Denomination, item::RegiItem, regi_cmd::RegiCmd};

pub mod coin;
pub mod item;
pub mod regi_cmd;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reginald {
    items: Vec<RegiItem>,
    coins: [Denomination; 5],
}

impl Reginald {
    pub fn new() -> Self {
        Reginald {
            items: Vec::new(),
            coins: [
                Denomination::Copper(0),
                Denomination::Silver(0),
                Denomination::Gold(0),
                Denomination::Electrum(0),
                Denomination::Platinum(0),
            ],
        }
    }

    pub fn add_item(&mut self, item: RegiItem) {
        self.items.push(item);
    }

    pub fn add_coin(&mut self, coin: Denomination) {
        match coin {
            Denomination::Copper(i) => self.coins[0] = self.coins[0] + i,
            Denomination::Silver(i) => self.coins[1] = self.coins[1] + i,
            Denomination::Gold(i) => self.coins[2] = self.coins[2] + i,
            Denomination::Electrum(i) => self.coins[3] = self.coins[3] + i,
            Denomination::Platinum(i) => self.coins[4] = self.coins[4] + i,
        }
    }

    pub fn from(&mut self, other: Self) {
        self.items = other.items;
        self.coins = other.coins;
    }

    pub fn items(&self) -> &Vec<RegiItem> {
        &self.items
    }

    pub fn exec_cmd(&mut self, cmd: RegiCmd) {
        match cmd {
            RegiCmd::Help(_) => todo!(),
            RegiCmd::LogDate(_, _) => todo!(),
            RegiCmd::LogSession(_, _) => todo!(),
            RegiCmd::Add(_, _) => todo!(),
            RegiCmd::Funds(_) => todo!(),
            RegiCmd::Bag(_) => todo!(),
            RegiCmd::Find(_, _) => todo!(),
            RegiCmd::Take(_, _) => todo!(),
            RegiCmd::RollDice(_, _) => todo!(),
            RegiCmd::LogAll => todo!(),
            RegiCmd::None => todo!(),
        }
    }
}
