use crate::ast::{base::money::Denomination, typechecker::typed_ast::Value};
use anyhow::{Result, anyhow};

#[derive(Debug, Default, Clone)]
pub struct ReginaldEnv {
    pub pp: i64,
    pub ep: i64,
    pub gp: i64,
    pub sp: i64,
    pub cp: i64,
    pub output: Vec<String>,
}

impl ReginaldEnv {
    pub fn print_out(&mut self) {
        for l in &self.output {
            println!("{l}");
        }
        self.output.clear();
    }

    pub fn output(&self) -> Vec<String> {
        self.output.clone()
    }

    pub fn checked_add(self, funds: Value) -> Result<Self> {
        let (val, den) = funds.unmake();
        if val.fract() != 0f32 {
            return Err(anyhow!("Cannot have fractional values: {val:?}"));
        }
        let val = val as i64;
        match den {
            Denomination::Platinum => {
                let pp = self.pp + val;
                if pp < 0 {
                    Err(anyhow!("Cannot have negative money"))
                } else {
                    Ok(Self { pp, ..self })
                }
            }
            Denomination::Electrum => {
                let ep = self.ep + val;

                if ep < 0 {
                    Err(anyhow!("Cannot have negative money"))
                } else {
                    Ok(Self { ep, ..self })
                }
            }
            Denomination::Gold => {
                let gp = self.gp + val;

                if gp < 0 {
                    Err(anyhow!("Cannot have negative money"))
                } else {
                    Ok(Self { gp, ..self })
                }
            }
            Denomination::Silver => {
                let sp = self.sp + val;

                if sp < 0 {
                    Err(anyhow!("Cannot have negative money"))
                } else {
                    Ok(Self { sp, ..self })
                }
            }
            Denomination::Copper => {
                let cp = self.cp + val;

                if cp < 0 {
                    Err(anyhow!("Cannot have negative money"))
                } else {
                    Ok(Self { cp, ..self })
                }
            }
        }
    }

    pub fn print<S: ToString>(self, out: S) -> Self {
        let mut output = self.output;
        output.push(out.to_string());
        Self { output, ..self }
    }

    pub fn funds(&self) -> String {
        let mut output = Vec::new();
        if self.pp != 0 {
            output.push(format!("{}pp", self.pp));
        }

        if self.ep != 0 {
            output.push(format!("{}ep", self.ep));
        }

        if self.gp != 0 {
            output.push(format!("{}gp", self.gp));
        }

        if self.sp != 0 {
            output.push(format!("{}sp", self.sp));
        }

        if self.cp != 0 {
            output.push(format!("{}cp", self.cp));
        }

        output.join(", ")
    }
}
