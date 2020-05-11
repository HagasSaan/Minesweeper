use crate::mark::Mark;

#[derive(Debug)]
pub enum CellStatus {
    Mine,
    Safe,
    Marked,
}

#[derive(Debug, Clone, Copy)]
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
