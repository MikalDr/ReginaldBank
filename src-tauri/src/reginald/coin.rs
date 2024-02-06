use std::ops::{Add, AddAssign};

use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Denomination {
    Copper(i32),
    Silver(i32),
    Gold(i32),
    Electrum(i32),
    Platinum(i32),
}

impl Serialize for Denomination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("coin", 2)?;
        state.serialize_field("denomination", &self.to_string())?;
        state.serialize_field("val", &self.val())?;
        state.end()
    }
}

impl Denomination {
    pub fn to_string(&self) -> String {
        match self {
            Denomination::Copper(_) => "Copper".to_string(),
            Denomination::Silver(_) => "Silver".to_string(),
            Denomination::Gold(_) => "Gold".to_string(),
            Denomination::Electrum(_) => "Electrum".to_string(),
            Denomination::Platinum(_) => "Platinum".to_string(),
        }
    }

    pub fn new(coin: &Denomination, val: i32) -> Self {
        match coin {
            Denomination::Copper(_) => Denomination::Copper(val),
            Denomination::Silver(_) => Denomination::Silver(val),
            Denomination::Gold(_) => Denomination::Gold(val),
            Denomination::Electrum(_) => Denomination::Electrum(val),
            Denomination::Platinum(_) => Denomination::Platinum(val),
        }
    }

    pub fn val(&self) -> i32 {
        match self {
            Denomination::Copper(i) => i.clone(),
            Denomination::Silver(i) => i.clone(),
            Denomination::Gold(i) => i.clone(),
            Denomination::Electrum(i) => i.clone(),
            Denomination::Platinum(i) => i.clone(),
        }
    }

    pub fn convert(&self, other: Self) -> Self {
        match (self, other) {
            (Denomination::Copper(i), Denomination::Silver(_)) => Denomination::Silver(i / 10),
            (Denomination::Copper(i), Denomination::Gold(_)) => Denomination::Gold(i / 100),
            (Denomination::Copper(i), Denomination::Electrum(_)) => Denomination::Electrum(i / 500),
            (Denomination::Copper(i), Denomination::Platinum(_)) => {
                Denomination::Platinum(i / 1000)
            }
            (Denomination::Silver(i), Denomination::Copper(_)) => Denomination::Copper(i * 10),
            (Denomination::Silver(i), Denomination::Gold(_)) => Denomination::Gold(i / 10),
            (Denomination::Silver(i), Denomination::Electrum(_)) => Denomination::Electrum(i / 50),
            (Denomination::Silver(i), Denomination::Platinum(_)) => Denomination::Platinum(i / 100),
            (Denomination::Gold(i), Denomination::Copper(_)) => Denomination::Copper(i * 100),
            (Denomination::Gold(i), Denomination::Silver(_)) => Denomination::Silver(i * 10),
            (Denomination::Gold(i), Denomination::Electrum(_)) => Denomination::Electrum(i * 5),
            (Denomination::Gold(i), Denomination::Platinum(_)) => Denomination::Platinum(i * 10),
            (Denomination::Electrum(i), Denomination::Copper(_)) => Denomination::Copper(i * 500),
            (Denomination::Electrum(i), Denomination::Silver(_)) => Denomination::Silver(i * 50),
            (Denomination::Electrum(i), Denomination::Gold(_)) => Denomination::Gold(i * 5),
            (Denomination::Electrum(i), Denomination::Platinum(_)) => {
                Denomination::Platinum(i / 50)
            }
            (Denomination::Platinum(i), Denomination::Copper(_)) => {
                Denomination::Platinum(i * 1000)
            }
            (Denomination::Platinum(i), Denomination::Silver(_)) => Denomination::Silver(i * 100),
            (Denomination::Platinum(i), Denomination::Gold(_)) => Denomination::Gold(i * 10),
            (Denomination::Platinum(i), Denomination::Electrum(_)) => Denomination::Electrum(i * 5),
            _ => self.clone(),
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let mut raw_num = String::new();
        let mut coin = Self::Gold(0);
        for c in input.chars() {
            match c {
                'g' => continue,
                'c' => coin = Self::Copper(0),
                's' => coin = Self::Silver(0),
                'e' => coin = Self::Electrum(0),
                'p' => coin = Self::Platinum(0),
                _ => raw_num.push(c),
            }
        }

        match raw_num.parse::<i32>() {
            Ok(i) => Some(Self::new(&coin, i)),
            Err(_) => None,
        }
    }
}

impl Add for Denomination {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Denomination::new(&self, self.val() + self.convert(rhs).val())
    }
}

impl Add<i32> for Denomination {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Denomination::new(&self, self.val() + rhs)
    }
}
