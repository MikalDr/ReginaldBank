use anyhow::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use serenity::model::channel::Message;
use serenity::prelude;
use time::Date;

pub enum Flag {
    Help,
    DryRun,
    Silent,
}

impl Flag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "h" | "help" => Some(Self::Help),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemComponent {
    Name(String),
    Desc(String),
    Count(isize),
    Cost(isize),
    Weight(isize),
    Tag(String),
}

// TODO: Add a way to convert some Money, into a list of Monies, such that we can have change,
// instead of 11.5 GP, we have 11 GP and 5 SP
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Money {
    value: f64,
    currency: Currency,
}

const COPPER_TO_SILVER: f64 = 0.1;
const COPPER_TO_GOLD: f64 = 0.01;
const COPPER_TO_ELECTRUM: f64 = 0.005;
const COPPER_TO_PLATINUM: f64 = 0.001;

const SILVER_TO_COPPER: f64 = 1.0 / COPPER_TO_SILVER;
const SILVER_TO_GOLD: f64 = COPPER_TO_GOLD * SILVER_TO_COPPER;
const SILVER_TO_ELECTRUM: f64 = COPPER_TO_ELECTRUM * SILVER_TO_COPPER;
const SILVER_TO_PLATINUM: f64 = COPPER_TO_PLATINUM * SILVER_TO_COPPER;

const GOLD_TO_COPPER: f64 = 1.0 / COPPER_TO_GOLD;
const GOLD_TO_SILVER: f64 = 1.0 / SILVER_TO_GOLD;
const GOLD_TO_ELECTRUM: f64 = SILVER_TO_ELECTRUM * GOLD_TO_SILVER;
const GOLD_TO_PLATINUM: f64 = SILVER_TO_PLATINUM * GOLD_TO_SILVER;

const ELECTRUM_TO_COPPER: f64 = 1.0 / COPPER_TO_ELECTRUM;
const ELECTRUM_TO_SILVER: f64 = 1.0 / SILVER_TO_ELECTRUM;
const ELECTRUM_TO_GOLD: f64 = 1.0 / GOLD_TO_ELECTRUM;
const ELECTRUM_TO_PLATINUM: f64 = GOLD_TO_PLATINUM * ELECTRUM_TO_GOLD;

const PLATINUM_TO_COPPER: f64 = 1.0 / COPPER_TO_PLATINUM;
const PLATINUM_TO_SILVER: f64 = 1.0 / SILVER_TO_PLATINUM;
const PLATINUM_TO_GOLD: f64 = 1.0 / GOLD_TO_PLATINUM;
const PLATINUM_TO_ELECTRUM: f64 = 1.0 / ELECTRUM_TO_PLATINUM;

impl Money {
    pub fn to_value(&self) -> f64 {
        self.value.clone()
    }

    pub fn new(value: f64, currency: Currency) -> Self {
        Self { value, currency }
    }

    pub fn add(&mut self, other: Money) {
        self.value += other.convert(self.currency).to_value();
    }

    pub fn convert(self, currency: Currency) -> Self {
        let mut money = self;
        // From -> To
        money.value *= match (money.currency, currency) {
            (Currency::Copper, Currency::Silver) => COPPER_TO_SILVER,
            (Currency::Copper, Currency::Gold) => COPPER_TO_GOLD,
            (Currency::Copper, Currency::Electrum) => COPPER_TO_ELECTRUM,
            (Currency::Copper, Currency::Platinum) => COPPER_TO_PLATINUM,
            (Currency::Silver, Currency::Copper) => SILVER_TO_COPPER,
            (Currency::Silver, Currency::Gold) => SILVER_TO_GOLD,
            (Currency::Silver, Currency::Electrum) => SILVER_TO_ELECTRUM,
            (Currency::Silver, Currency::Platinum) => SILVER_TO_PLATINUM,
            (Currency::Gold, Currency::Copper) => GOLD_TO_COPPER,
            (Currency::Gold, Currency::Silver) => GOLD_TO_SILVER,
            (Currency::Gold, Currency::Electrum) => GOLD_TO_ELECTRUM,
            (Currency::Gold, Currency::Platinum) => GOLD_TO_PLATINUM,
            (Currency::Electrum, Currency::Copper) => ELECTRUM_TO_COPPER,
            (Currency::Electrum, Currency::Silver) => ELECTRUM_TO_SILVER,
            (Currency::Electrum, Currency::Gold) => ELECTRUM_TO_GOLD,
            (Currency::Electrum, Currency::Platinum) => ELECTRUM_TO_PLATINUM,
            (Currency::Platinum, Currency::Copper) => PLATINUM_TO_COPPER,
            (Currency::Platinum, Currency::Silver) => PLATINUM_TO_SILVER,
            (Currency::Platinum, Currency::Gold) => PLATINUM_TO_GOLD,
            (Currency::Platinum, Currency::Electrum) => PLATINUM_TO_ELECTRUM,
            _ => 1f64,
        };

        return money;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Currency {
    Copper,
    Silver,
    Gold,
    Electrum,
    Platinum,
}

pub struct Session {
    id: usize,
    date: Date,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CmdParser;

/// Parses commands, will assume that any message given should be a valid command.
/// TODO: Add good explanations and diagrams
pub async fn command_parser(_ctx: prelude::Context, msg: Message) -> Result<()> {
    // TODO: Find out why the parser returns an iterator, does it mean a user can send int multiple
    // commands?
    let input = CmdParser::parse(Rule::input, &msg.content)?
        .into_iter()
        .next()
        .ok_or(Error::msg("Expected `input`, got None"))?;

    // We will just assume that `input` is valid, and get the inner-pairs
    // Since `input` is `bot` - `command`, the inner of the input will be:
    // bot and then command
    let mut inner_iter = input.into_inner();
    // We don't really need the `bot` pair, unless we want to do different things
    // based on what the user called regi
    let _ = inner_iter.next();

    // We allow flags to become before or after a `command`
    let (cmd, flg) = {
        let fst = inner_iter
            .next()
            .ok_or(Error::msg("Expected a `command` or `flag`, got None"))?;

        let snd = inner_iter.next();

        match (fst.as_rule(), snd.clone().and_then(|e| Some(e.as_rule()))) {
            (Rule::command, Some(Rule::flag)) => Ok((fst, snd)),
            (Rule::flag, Some(Rule::command)) => Ok((snd.unwrap(), Some(fst))),
            (Rule::command, _) => Ok((fst, None)),
            (f, s) => Err(Error::msg(format!(
                "Expected a `command` or `flag` got: {:?} and {:?}",
                f, s
            ))),
        }
    }?;

    // We will just assume it's a `command`-pair
    let mut command_iter = cmd.into_inner();

    // Collect flags
    let mut flags = Vec::new();
    if let Some(f) = flg {
        flags = f
            .into_inner()
            .filter_map(|f| Flag::from_str(f.as_span().as_str()))
            .collect();
    }

    let cmd = command_iter
        .next()
        .ok_or(Error::msg("Expected a `*_cmd`, got None"))?;

    match cmd.as_rule() {
        Rule::log_cmd => {
            for c in cmd.into_inner() {
                match c.as_rule() {
                    Rule::log_all => todo!(),
                    Rule::date => todo!(),
                    Rule::session_nr => todo!(),
                    _ => unreachable!("Only `log_all`, `date`, or `session_nr` in log_cmd"),
                }
            }
        }

        Rule::rm_cmd => {
            for c in cmd.into_inner() {
                match c.as_rule() {
                    Rule::item => rm_item(
                        // Gets the item components
                        parse_items(c),
                        &flags,
                    ),
                    Rule::money => rm_money(parse_money(c), &flags),
                    _ => unreachable!("Only item or money should be here"),
                }
            }
        }

        Rule::add_cmd => {
            for c in cmd.into_inner() {
                match c.as_rule() {
                    Rule::item => add_item(
                        // Gets the item components
                        parse_items(c),
                        &flags,
                    ),
                    Rule::money => add_money(parse_money(c), &flags),
                    _ => unreachable!("Only item or money should be here"),
                }
            }
        }

        Rule::funds_cmd => {
            display_monies(_ctx, &flags);
        }

        Rule::bag_cmd => {
            let bag_cmds = cmd.into_inner().collect::<Vec<_>>().pop();

            if bag_cmds.is_none() {
                display_bag(_ctx, &flags);
                return Ok(());
            }

            match bag_cmds.unwrap().as_rule() {
                Rule::bag_cost => bag_cost(_ctx, &flags),
                Rule::bag_weight => bag_weight(_ctx, &flags),
                _ => unreachable!("Only `bag_cost` and `bag_weight`"),
            }
        }

        Rule::find_cmd => find_items(
            cmd.into_inner().map(|c| parse_items(c)).collect::<Vec<_>>(),
            &flags,
        ),

        Rule::take_cmd => {
            for c in cmd.into_inner() {
                match c.as_rule() {
                    Rule::money => take_money(parse_money(c), &flags),
                    Rule::item => take_item(parse_items(c), &flags),
                    _ => unreachable!("Only money or item in take"),
                }
            }
        }

        _ => unreachable!("Only a command rule should occur"),
    }

    Ok(())
}

fn parse_items(c: Pair<Rule>) -> Vec<ItemComponent> {
    c.into_inner()
        .filter_map(|ic| match ic.as_rule() {
            Rule::item_name => Some(ItemComponent::Name(ic.as_span().as_str().to_string())),
            Rule::item_description => Some(ItemComponent::Desc(ic.as_span().as_str().to_string())),
            Rule::item_cost => match ic.as_span().as_str().to_string().parse::<isize>() {
                Ok(i) => Some(ItemComponent::Cost(i)),
                Err(_) => None,
            },
            Rule::item_weight => match ic.as_span().as_str().to_string().parse::<isize>() {
                Ok(i) => Some(ItemComponent::Weight(i)),
                Err(_) => None,
            },
            Rule::item_count => match ic.as_span().as_str().to_string().parse::<isize>() {
                Ok(i) => Some(ItemComponent::Count(i)),
                Err(_) => None,
            },
            Rule::item_tag => Some(ItemComponent::Tag(ic.as_span().as_str().to_string())),
            _ => unreachable!("Only item_components here"),
        })
        .collect::<Vec<ItemComponent>>()
}

fn parse_money(c: Pair<Rule>) -> Money {
    let mut money_iter = c.into_inner();
    Money {
        value: money_iter
            .next()
            .unwrap()
            .as_span()
            .as_str()
            .parse::<f64>()
            .unwrap(),
        currency: money_iter
            .next()
            .and_then(|e| match e.as_rule() {
                Rule::copper_ident => Some(Currency::Copper),
                Rule::silver_ident => Some(Currency::Silver),
                Rule::electrum_ident => Some(Currency::Electrum),
                Rule::platinum_ident => Some(Currency::Platinum),
                _ => Some(Currency::Gold),
            })
            .unwrap_or(Currency::Gold),
    }
}

// TODO: Implement
fn add_item(_item_components: Vec<ItemComponent>, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn add_money(_money: Money, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn find_items(_items: Vec<Vec<ItemComponent>>, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn take_item(_item: Vec<ItemComponent>, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn take_money(_money: Money, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn display_bag(_ctx: prelude::Context, _flgs: &Vec<Flag>) {
    unimplemented!()
}

//TODO: Implement
fn display_monies(_ctx: prelude::Context, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn rm_item(_item_components: Vec<ItemComponent>, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn rm_money(_money: Money, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn bag_weight(_ctx: prelude::Context, _flgs: &Vec<Flag>) {
    unimplemented!()
}

// TODO: Implement
fn bag_cost(_ctx: prelude::Context, _flgs: &Vec<Flag>) {
    unimplemented!()
}
