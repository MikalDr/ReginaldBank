use crate::ast::{base::money::Denomination, typechecker::typed_ast::Value};

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
    pub fn output(&self) -> Vec<String> {
        self.output.clone()
    }

    pub fn add(self, funds: Value) -> Self {
        match funds {
            Value::Piece(val, denomination) => match denomination {
                Denomination::Platinum => Self {
                    pp: self.pp + val,
                    ..self
                },
                Denomination::Electrum => Self {
                    ep: self.ep + val,
                    ..self
                },
                Denomination::Gold => Self {
                    gp: self.gp + val,
                    ..self
                },
                Denomination::Silver => Self {
                    sp: self.sp + val,
                    ..self
                },
                Denomination::Copper => Self {
                    cp: self.cp + val,
                    ..self
                },
            },
            Value::Int(val) => Self {
                gp: self.gp + val,
                ..self
            },
            Value::Float(val) => Self {
                gp: self.gp + val.round() as i64,
                ..self
            },
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
