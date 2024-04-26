use crate::cmd::ItemComponent;
use crate::cmd::{Currency, Money};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    desc: Option<String>,
    pub count: isize,
    pub cost: Money,
    tag: Vec<String>,
    weight: isize,
}

impl Item {
    pub fn to_key(&self) -> Key {
        (
            self.name.clone(),
            self.desc.clone().unwrap_or_default(),
            self.count,
            self.cost.clone().convert(Currency::Gold).to_value() as isize,
            self.tag.clone(),
        )
    }
}

pub type Key = (
    // Name
    String,
    // Desc
    String,
    // Count
    isize,
    // Cost
    isize,
    // Tags
    Vec<String>,
);

pub struct Bag {
    items: HashMap<Key, Item>,
}

impl Bag {
    /// Uses a list of `ItemComponent`s to search for an Item, everything except the Name is
    /// optional
    pub fn lookup(&self, ic: Vec<ItemComponent>) -> Option<&Item> {
        let key = self.create_key(ic);
        self.items.get(&key)
    }

    /// Puts the given item in the bag, optionally returning the old-item
    pub fn put(&mut self, mut item: Item) -> Option<Item> {
        let key = item.to_key();
        let mut res: Option<Item> = None;
        if let Some(i) = self.items.get(&key) {
            item.count += i.count;
            res = Some(i.clone());
        }

        self.items.insert(key, item);
        return res;
    }

    fn create_key(&self, mut ic: Vec<ItemComponent>) -> Key {
        ic.sort();
        let name = match ic.pop().unwrap() {
            ItemComponent::Name(s) => s,
            _ => String::new(),
        };

        let mut desc = String::new();
        let mut count: isize = 1;
        let mut cost: isize = 0;
        let mut tags: Vec<String> = Vec::new();

        for i in ic {
            match i {
                ItemComponent::Desc(d) => desc = d,
                ItemComponent::Count(c) => count = c,
                ItemComponent::Cost(c) => cost = c,
                ItemComponent::Tag(t) => tags.push(t),
                _ => continue,
            }
        }

        (name, desc, count, cost, tags)
    }

    pub fn total_cost(&self, currency: Currency) -> Money {
        let mut money = Money::new(0f64, currency);

        for item in self.items.values() {
            money.add(item.cost.clone());
        }

        return money;
    }
}
