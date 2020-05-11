use crate::mark::Mark;
use rand::prelude::*;
use std::vec::Vec;

use crate::cell::{Cell, CellStatus};

#[derive(Debug)]
pub enum GameResult {
    Win,
    Play,
    Lose,
    Stop,
    Info,
    Error,
}

pub const FIELD_DEFAULT_SIZE: usize = 10;

const NEIGHBORS_SHIFTS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
pub struct Field {
    size: usize,
    cells: Vec<Vec<Cell>>,
    mines_count: i16,
    closed_safe_cells_count: i16,
}

impl Field {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut mines_count = 0;
        let mut closed_safe_cells_count = 0;
        let mut cells = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| {
                        let random_value: f32 = rng.gen();
                        let is_mine = random_value >= 0.8;
                        if is_mine {
                            mines_count += 1;
                        } else {
                            closed_safe_cells_count += 1;
                        }
                        Cell::new(is_mine)
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        for row_idx in 0..size {
            for col_idx in 0..size {
                let mut mines_neighbors_count = 0;
                for (row_shift, col_shift) in NEIGHBORS_SHIFTS.iter() {
                    let row_idx_shifted = row_idx as i8 + row_shift;
                    let col_idx_shifted = col_idx as i8 + col_shift;
                    if row_idx_shifted < 0
                        || row_idx_shifted >= size as i8
                        || col_idx_shifted < 0
                        || col_idx_shifted >= size as i8
                    {
                        continue;
                    }
                    let neighbor = cells[row_idx_shifted as usize][col_idx_shifted as usize];
                    if neighbor.is_mine {
                        mines_neighbors_count += 1;
                    }
                }

                cells[row_idx][col_idx].mines_neighbors_count = mines_neighbors_count;
            }
        }
        Self {
            size: size,
            cells: cells,
            mines_count: mines_count,
            closed_safe_cells_count: closed_safe_cells_count,
        }
    }

    pub fn draw(&self, opened: bool) -> String {
        let mut result: String = "".to_string();
        for row_idx in 0..self.size {
            for col_idx in 0..self.size {
                let cell = {
                    if self.cells[row_idx][col_idx].opened || opened {
                        if self.cells[row_idx][col_idx].is_mine {
                            Mark::Mine.value()
                        } else {
                            self.cells[row_idx][col_idx]
                                .mines_neighbors_count
                                .to_string()
                        }
                    } else {
                        self.cells[row_idx][col_idx].marked_as.value()
                    }
                };
                result.push_str(&cell);
            }
            result.push('\n');
        }
        result
    }

    pub fn open_cell(&mut self, x: usize, y: usize) -> GameResult {
        let result = self.cells[x][y].open();
        let game_result = self.game_status(result);
        match game_result {
            GameResult::Lose => (),
            _ => {
                if self.cells[x][y].mines_neighbors_count == 0 {
                    for (row_shift, col_shift) in NEIGHBORS_SHIFTS.iter() {
                        let row_idx_shifted = x as i8 + row_shift;
                        let col_idx_shifted = y as i8 + col_shift;
                        if row_idx_shifted < 0
                            || row_idx_shifted >= self.size as i8
                            || col_idx_shifted < 0
                            || col_idx_shifted >= self.size as i8
                        {
                            continue;
                        }
                        let neighbor =
                            self.cells[row_idx_shifted as usize][col_idx_shifted as usize];
                        if !neighbor.opened && !neighbor.is_mine {
                            self.open_cell(row_idx_shifted as usize, col_idx_shifted as usize);
                        }
                    }
                }
            }
        }
        game_result
    }

    pub fn mark_cell(&mut self, x: usize, y: usize, mark_as: Mark) -> GameResult {
        let cell_status = self.cells[x][y].mark(mark_as);
        self.game_status(cell_status)
    }

    fn game_status(&mut self, cell_status: CellStatus) -> GameResult {
        match cell_status {
            CellStatus::Safe => {
                self.closed_safe_cells_count -= 1;
                match self.closed_safe_cells_count {
                    0 => GameResult::Win,
                    _ => GameResult::Play,
                }
            }
            CellStatus::Marked => GameResult::Play,
            CellStatus::Mine => GameResult::Lose,
        }
    }
    pub fn process_command_args(&mut self, args: Vec<&str>) -> (GameResult, &str) {
        match args[0] {
            "stop" => (GameResult::Stop, ""),
            "open" => match args[1].parse() {
                Ok(x) => match args[2].parse() {
                    Ok(y) => (self.open_cell(x, y), ""),
                    Err(_) => (GameResult::Error, "Integer for coord x expected"),
                },
                Err(_) => (GameResult::Error, "Integer for coord y expected"),
            },
            "mark" => match args[1].parse() {
                Ok(x) => match args[2].parse() {
                    Ok(y) => {
                        let mark_as = match args[3] {
                            "empty" => Mark::Empty,
                            "unknown" => Mark::Unknown,
                            "mine" => Mark::Mine,
                            _ => {
                                println!("Unknown mark type. Marked as empty");
                                Mark::Empty
                            }
                        };
                        (self.mark_cell(x, y, mark_as), "")
                    }
                    Err(_) => (GameResult::Error, "Integer for coord x expected"),
                },
                Err(_) => (GameResult::Error, "Integer for coord y expected"),
            },
            "help" => (GameResult::Info, "here must be help about program"),
            _ => (GameResult::Error, "Unknown command"),
        }
    }

    // pub fn restore_field_from_string(field: String) -> Self {
    //     unimplemented!();
    // }

    // pub fn dump_field_to_string(&self) -> String {
    //     unimplemented!();
    // }
}
