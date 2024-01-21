use serde::{Deserialize, Serialize};

use super::coin::Denomination;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegiItem {
    name: String,
    desc: Option<String>,
    cost: Option<Denomination>,
}

impl RegiItem {
    pub fn parse(input: &str) -> Option<Self> {
        todo!()
    }
}
