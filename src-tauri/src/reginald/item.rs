use serde::{Deserialize, Serialize};

use super::coin::Denomination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegiItem {
    name: String,
    desc: Option<String>,
    cost: Option<Denomination>,
}

impl RegiItem {
    pub fn new(name: String) -> Self {
        RegiItem {
            name,
            desc: None,
            cost: None,
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        todo!()
    }
}
