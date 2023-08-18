use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Property {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl ToString for Property {
    fn to_string(&self) -> String {
        match self {
            Property::Integer(n) => n.to_string(),
            Property::Float(f) => f.to_string(),
            Property::Bool(b) => b.to_string(),
            Property::String(s) => s.to_string(),
        }
    }
}
