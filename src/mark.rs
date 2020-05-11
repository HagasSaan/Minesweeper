#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Empty,
    Mine,
    Unknown,
}
impl Mark {
    pub fn value(&self) -> String {
        match *self {
            Mark::Empty => "*".to_string(),
            Mark::Mine => "M".to_string(),
            Mark::Unknown => "?".to_string(),
        }
    }
}
