use crate::mark::Mark;

#[derive(Debug, PartialEq)]
pub enum CellStatus {
    Mine,
    Safe,
    Marked,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub is_mine: bool,
    pub opened: bool,
    pub mines_neighbors_count: i8,
    pub marked_as: Mark,
}

impl Cell {
    pub fn new(is_mine: bool) -> Self {
        Self {
            is_mine: is_mine,
            opened: false,
            mines_neighbors_count: -1,
            marked_as: Mark::Empty,
        }
    }
    pub fn open(&mut self) -> CellStatus {
        self.opened = true;
        if self.is_mine {
            CellStatus::Mine
        } else {
            CellStatus::Safe
        }
    }
    pub fn mark(&mut self, state: Mark) -> CellStatus {
        if !self.opened {
            self.marked_as = state;
        }
        CellStatus::Marked
    }
}

#[test]
fn cell_creation() {
    let cell = Cell::new(true);
    assert_eq!(cell.is_mine, true);
    assert_eq!(cell.opened, false);
    assert_eq!(cell.mines_neighbors_count, -1);
    assert_eq!(cell.marked_as, Mark::Empty);
}

#[test]
fn cell_opening_mine() {
    let mut cell = Cell::new(true);
    assert_eq!(cell.is_mine, true);
    assert_eq!(cell.opened, false);
    assert_eq!(cell.open(), CellStatus::Mine);
    assert_eq!(cell.opened, true);
}

#[test]
fn cell_opening_safe() {
    let mut cell = Cell::new(false);
    assert_eq!(cell.is_mine, false);
    assert_eq!(cell.opened, false);
    assert_eq!(cell.open(), CellStatus::Safe);
    assert_eq!(cell.opened, true);
}

#[test]
fn cell_mark() {
    let mut cell = Cell::new(false);
    assert_eq!(cell.marked_as, Mark::Empty);
    assert_eq!(cell.mark(Mark::Mine), CellStatus::Marked);
    assert_eq!(cell.marked_as, Mark::Mine);
}
