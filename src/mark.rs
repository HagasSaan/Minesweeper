use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Empty,
    Mine,
    Unknown,
}
impl Mark {
    pub fn to_string(self) -> String {
        match self {
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
fn mark_from_string() {
    assert_eq!(Mark::Empty.to_string(), "*");
    assert_eq!(Mark::Mine.to_string(), "M");
    assert_eq!(Mark::Unknown.to_string(), "?");
}

#[test]
fn mark_to_string() {
    assert_eq!(Mark::from_string("*"), Mark::Empty);
    assert_eq!(Mark::from_string("M"), Mark::Mine);
    assert_eq!(Mark::from_string("?"), Mark::Unknown);
    assert_eq!(Mark::from_string("wrong value"), Mark::Empty);
}
