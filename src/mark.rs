use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Empty,
    Mine,
    Unknown,
}
impl Mark {
    pub fn to_string(&self) -> String {
        match *self {
            Mark::Empty => "*",
            Mark::Mine => "M",
            Mark::Unknown => "?",
        }
        .to_string()
    }
    pub fn from_string(value: &str) -> Mark {
        match value {
            "*" => Mark::Empty,
            "M" => Mark::Mine,
            "?" => Mark::Unknown,
            _ => {
                error!("Unknown mark type, marked as empty");
                Mark::Empty
            }
        }
    }
}

#[test]
fn string_to_mark() {
    unimplemented!();
}

#[test]
fn mark_to_string() {
    unimplemented!();
}
