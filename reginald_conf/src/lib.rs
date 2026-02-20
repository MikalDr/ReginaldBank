use anyhow::{Result, anyhow};
use toml::Table;

const RAW_CONFIG: &str = include_str!("../../Reginald.toml");

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct ReginaldConfig {
    pub dm: String,
    pub players: Vec<Player>,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    class: String,
}

impl Player {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> &str {
        &self.class
    }
}

impl ReginaldConfig {
    pub fn get_conf() -> Result<Self> {
        let toml: Table = toml::from_str(RAW_CONFIG)?;

        let dm = toml
            .get("dm")
            .ok_or(anyhow!("Missing dm"))
            .and_then(|val| val.as_table().ok_or(anyhow!("dm is not table")))?
            .get("user")
            .ok_or(anyhow!("Missing user from dm table"))
            .and_then(|val| val.as_str().ok_or(anyhow!("dm.user is not string")))?
            .to_string();

        let mut players = Vec::new();

        for (name, tbl) in toml
            .get("players")
            .ok_or(anyhow!("Missing players"))
            .and_then(|val| val.as_table().ok_or(anyhow!("players is not table")))?
        {
            let class = tbl
                .get("class")
                .ok_or(anyhow!("missing class from users.{name} table"))
                .and_then(|val| {
                    val.as_str()
                        .ok_or(anyhow!("users.{name}.class is not string"))
                })?
                .to_string();

            players.push(Player {
                name: name.to_string(),
                class,
            });
        }

        Ok(ReginaldConfig { players, dm })
    }
}
