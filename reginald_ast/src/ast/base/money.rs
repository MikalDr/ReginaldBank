use anyhow::{Result, anyhow};
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Money {
    pub(crate) pp: i64,
    pub(crate) ep: i64,
    pub(crate) gp: i64,
    pub(crate) sp: i64,
    pub(crate) cp: i64,
}

impl Money {
    pub fn from_val(val: i64, denomination: Denomination) -> Self {
        let default = Self::default();
        match denomination {
            Denomination::Platinum => Self { pp: val, ..default },
            Denomination::Electrum => Self { ep: val, ..default },
            Denomination::Gold => Self { gp: val, ..default },
            Denomination::Silver => Self { sp: val, ..default },
            Denomination::Copper => Self { cp: val, ..default },
        }
    }

    pub fn convert(self, denomination: Denomination) -> Self {
        let mut pp = self.pp;
        let mut ep = self.ep;
        let mut gp = self.gp;
        let mut sp = self.sp;
        let mut cp = self.cp;

        if Denomination::Platinum > denomination {
            ep += Denomination::Platinum.exchange(denomination)(pp) as i64;
            pp = 0;
        }

        if Denomination::Electrum > denomination {
            gp += Denomination::Electrum.exchange(denomination)(ep) as i64;
            ep = 0;
        }

        if Denomination::Gold > denomination {
            sp += Denomination::Gold.exchange(denomination)(gp) as i64;
            gp = 0;
        }

        if Denomination::Silver > denomination {
            cp += Denomination::Silver.exchange(denomination)(sp) as i64;
            sp = 0;
        }

        Self { pp, ep, gp, sp, cp }
    }

    fn exchange(c: i64, d: i64, i: i64, t: i64) -> (i64, i64) {
        let mut new_count = c;
        let mut new_t = t;
        while new_count >= d {
            new_t += i;
            new_count -= d;
        }
        (new_count, new_t)
    }

    pub fn least_denomination(self) -> Self {
        let pp = self.pp;
        let ep = self.ep;
        let gp = self.gp;
        let sp = self.sp;
        let cp = self.cp;

        let (cp, sp) = Self::exchange(cp, SILVER_COPPER as i64, 1, sp);
        let (sp, gp) = Self::exchange(sp, GOLD_SILVER as i64, 1, gp);
        let (gp, ep) = Self::exchange(gp, ELECTRUM_GOLD as i64, 1, ep);
        let (ep, pp) = Self::exchange(ep, PLATINUM_ELECTRUM as i64, 1, pp);

        Self { pp, ep, gp, sp, cp }
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pp: self.pp + rhs.pp,
            ep: self.ep + rhs.ep,
            gp: self.gp + rhs.gp,
            sp: self.sp + rhs.sp,
            cp: self.cp + rhs.cp,
        }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            pp: self.pp - rhs.pp,
            ep: self.ep - rhs.ep,
            gp: self.gp - rhs.gp,
            sp: self.sp - rhs.sp,
            cp: self.cp - rhs.cp,
        }
    }
}

impl Mul for Money {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

pub const PLATINUM_ELECTRUM: f32 = 5f32;

pub const PLATINUM_GOLD: f32 = 10f32;

pub const PLATINUM_SILVER: f32 = 100f32;

pub const PLATINUM_COPPER: f32 = 1000f32;

pub const ELECTRUM_GOLD: f32 = 5f32;

pub const ELECTRUM_COPPER: f32 = 500f32;

pub const GOLD_ELECTRUM: f32 = 0.5f32;

pub const GOLD_COPPER: f32 = 100f32;

pub const SILVER_ELECTRUM: f32 = 0.05f32;

pub const SILVER_COPPER: f32 = 10f32;

pub const COPPER_ELECTRUM: f32 = 0.005f32;

pub const COPPER_SILVER: f32 = 0.1f32;

pub const ELECTRUM_PLATINUM: f32 = 0.5f32;

pub const ELECTRUM_SILVER: f32 = 50f32;

pub const GOLD_SILVER: f32 = 10f32;

pub const GOLD_PLATINUM: f32 = 0.1f32;

pub const SILVER_PLATINUM: f32 = 0.01f32;

pub const SILVER_GOLD: f32 = 0.1f32;

pub const COPPER_PLATINUM: f32 = 0.001f32;

pub const COPPER_GOLD: f32 = 0.01f32;

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Copy, Default)]
pub enum Denomination {
    Platinum,
    Electrum,
    #[default]
    Gold,
    Silver,
    Copper,
}

impl Denomination {
    pub fn decr(self) -> Self {
        match self {
            Self::Platinum => Self::Electrum,
            Self::Electrum => Self::Gold,
            Self::Gold => Self::Silver,
            Self::Silver => Self::Copper,
            Self::Copper => Self::Copper,
        }
    }

    pub const fn exchange_const(&self, target: Denomination) -> f32 {
        match (self, target) {
            (Denomination::Platinum, Denomination::Electrum) => PLATINUM_ELECTRUM,
            (Denomination::Platinum, Denomination::Gold) => PLATINUM_GOLD,
            (Denomination::Platinum, Denomination::Silver) => PLATINUM_SILVER,
            (Denomination::Platinum, Denomination::Copper) => PLATINUM_COPPER,
            (Denomination::Electrum, Denomination::Platinum) => ELECTRUM_PLATINUM,
            (Denomination::Electrum, Denomination::Gold) => ELECTRUM_GOLD,
            (Denomination::Electrum, Denomination::Silver) => ELECTRUM_SILVER,
            (Denomination::Electrum, Denomination::Copper) => ELECTRUM_COPPER,
            (Denomination::Gold, Denomination::Platinum) => GOLD_PLATINUM,
            (Denomination::Gold, Denomination::Electrum) => GOLD_ELECTRUM,
            (Denomination::Gold, Denomination::Silver) => GOLD_SILVER,
            (Denomination::Gold, Denomination::Copper) => GOLD_COPPER,
            (Denomination::Silver, Denomination::Platinum) => SILVER_PLATINUM,
            (Denomination::Silver, Denomination::Electrum) => SILVER_ELECTRUM,
            (Denomination::Silver, Denomination::Gold) => SILVER_GOLD,
            (Denomination::Silver, Denomination::Copper) => SILVER_COPPER,
            (Denomination::Copper, Denomination::Platinum) => COPPER_PLATINUM,
            (Denomination::Copper, Denomination::Electrum) => COPPER_ELECTRUM,
            (Denomination::Copper, Denomination::Gold) => COPPER_GOLD,
            (Denomination::Copper, Denomination::Silver) => COPPER_SILVER,
            (l, r) if l.comp(&r) => 1f32,
            _ => panic!("Invalid exchange"),
        }
    }

    pub const fn comp(&self, other: &Self) -> bool {
        match (self, other) {
            (Denomination::Platinum, Denomination::Platinum)
            | (Denomination::Electrum, Denomination::Electrum)
            | (Denomination::Gold, Denomination::Gold)
            | (Denomination::Silver, Denomination::Silver)
            | (Denomination::Copper, Denomination::Copper) => true,
            _ => false,
        }
    }

    pub fn exchange(&self, target: Denomination) -> impl Fn(i64) -> f32 {
        match (self, target) {
            (Denomination::Platinum, Denomination::Electrum) => {
                |i: i64| i as f32 * PLATINUM_ELECTRUM
            }
            (Denomination::Platinum, Denomination::Gold) => |i: i64| i as f32 * PLATINUM_GOLD,
            (Denomination::Platinum, Denomination::Silver) => |i: i64| i as f32 * PLATINUM_SILVER,
            (Denomination::Platinum, Denomination::Copper) => |i: i64| i as f32 * PLATINUM_COPPER,
            (Denomination::Electrum, Denomination::Platinum) => {
                |i: i64| i as f32 * ELECTRUM_PLATINUM
            }
            (Denomination::Electrum, Denomination::Gold) => |i: i64| i as f32 * ELECTRUM_GOLD,
            (Denomination::Electrum, Denomination::Silver) => |i: i64| i as f32 * ELECTRUM_SILVER,
            (Denomination::Electrum, Denomination::Copper) => |i: i64| i as f32 * ELECTRUM_COPPER,
            (Denomination::Gold, Denomination::Platinum) => |i: i64| i as f32 * GOLD_PLATINUM,
            (Denomination::Gold, Denomination::Electrum) => |i: i64| i as f32 * GOLD_ELECTRUM,
            (Denomination::Gold, Denomination::Silver) => |i: i64| i as f32 * GOLD_SILVER,
            (Denomination::Gold, Denomination::Copper) => |i: i64| i as f32 * GOLD_COPPER,
            (Denomination::Silver, Denomination::Platinum) => |i: i64| i as f32 * SILVER_PLATINUM,
            (Denomination::Silver, Denomination::Electrum) => |i: i64| i as f32 * SILVER_ELECTRUM,
            (Denomination::Silver, Denomination::Gold) => |i: i64| i as f32 * SILVER_GOLD,
            (Denomination::Silver, Denomination::Copper) => |i: i64| i as f32 * SILVER_COPPER,
            (Denomination::Copper, Denomination::Platinum) => |i: i64| i as f32 * COPPER_PLATINUM,
            (Denomination::Copper, Denomination::Electrum) => |i: i64| i as f32 * COPPER_ELECTRUM,
            (Denomination::Copper, Denomination::Gold) => |i: i64| i as f32 * COPPER_GOLD,
            (Denomination::Copper, Denomination::Silver) => |i: i64| i as f32 * COPPER_SILVER,
            (l, r) if l == &r => |i: i64| i as f32,
            _ => panic!("Cant exchange: {self:?} to {target:?}"),
        }
    }
}

impl FromStr for Denomination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pp" => Ok(Self::Platinum),
            "ep" => Ok(Self::Electrum),
            "gp" => Ok(Self::Gold),
            "sp" => Ok(Self::Silver),
            "cp" => Ok(Self::Copper),
            _ => Err(anyhow!("Cant convert: {s}")),
        }
    }
}
