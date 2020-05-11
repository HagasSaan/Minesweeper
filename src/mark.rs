use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Empty,
    Mine,
    Unknown,
}
impl Mark {
    pub fn value(&self) -> String {
        match *self {
            Mark::Empty => "*",
            Mark::Mine => "M",
            Mark::Unknown => "?",
        }
        .to_string()
    }
}
