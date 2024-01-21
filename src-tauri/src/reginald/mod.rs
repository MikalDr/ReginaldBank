use self::{coin::Denomination, item::RegiItem, regi_cmd::RegiCmd};

pub mod coin;
pub mod item;
pub mod regi_cmd;

#[derive(Debug)]
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

    pub fn load(items: Vec<RegiItem>, coins: [Denomination; 5]) -> Self {
        Reginald { items, coins }
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
