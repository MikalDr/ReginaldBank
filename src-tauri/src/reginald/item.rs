use serde::{Deserialize, Serialize};

use super::coin::Denomination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegiItem {
    name: String,
    desc: Option<String>,
    cost: Option<Denomination>,
    amount: u32,
}

impl RegiItem {
    // !item <item-name> <cost> <amount> <note> <- optional
    pub fn parse(input: &str) -> Option<Self> {
        match input.split_whitespace().collect::<Vec<_>>()[..] {
            [name, cost, amount, ref desc @ ..] => {
                if let Ok(a) = amount.parse::<u32>()  {
                    Some(RegiItem { name: name.to_string(), desc: Some(desc.join(" ")), cost: Denomination::parse(cost), amount: a })
                } else {
                    None
                }
            },
            [name] => Some(RegiItem { name: name.to_string(), desc: None, cost: None, amount: 1 }),
            _ => todo!(),
        }
    }
}
