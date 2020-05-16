use crate::cell::{Cell, CellStatus};
use crate::mark::Mark;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
// use std::time;
use std::vec::Vec;
#[derive(Debug)]
pub enum GameResult {
    Win,
    Play,
    Lose,
    Stop,
    Info,
    Error,
}

#[derive(Debug)]
pub enum GameUI {
    TUI,
    GUI,
    WUI,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Field {
    size: usize,
    cells: Vec<Vec<Cell>>,
    mines_count: i16,
    closed_safe_cells_count: i16,
    // start_time: time::Instant,
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
            // start_time: time::Instant::now(),
        }
    }

    fn draw(&self, opened: bool) -> String {
        let mut result: String = "".to_string();
        for row_idx in 0..self.size {
            for col_idx in 0..self.size {
                let cell = {
                    if self.cells[row_idx][col_idx].opened || opened {
                        if self.cells[row_idx][col_idx].is_mine {
                            Mark::Mine.to_string()
                        } else {
                            self.cells[row_idx][col_idx]
                                .mines_neighbors_count
                                .to_string()
                        }
                    } else {
                        self.cells[row_idx][col_idx].marked_as.to_string()
                    }
                };
                result.push_str(&cell);
            }
            result.push('\n');
        }
        result
    }

    fn open_cell(&mut self, x: usize, y: usize) -> GameResult {
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

    fn mark_cell(&mut self, x: usize, y: usize, mark_as: Mark) -> GameResult {
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
    fn process_command_args(&mut self, args: Vec<&str>) -> (GameResult, &str) {
        match args[0] {
            "stop" => (GameResult::Stop, ""), //format!("{:?}", self.start_time.elapsed())
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
                        let mark_as = Mark::from_string(args[3]);
                        (self.mark_cell(x, y, mark_as), "")
                    }
                    Err(_) => (GameResult::Error, "Integer for coord x expected"),
                },
                Err(_) => (GameResult::Error, "Integer for coord y expected"),
            },
            "help" => (GameResult::Info, "here must be help about program"),
            "save" => {
                self.save_to_file(args[1].to_string());
                (GameResult::Info, "Game saved")
            }
            "load" => {
                *self = self.load_from_file(args[1].to_string());
                (GameResult::Info, "Game loaded")
            }
            _ => (GameResult::Error, "Unknown command"),
        }
    }

    fn restore_from_binary(serialized_field: Vec<u8>) -> Self {
        bincode::deserialize(&serialized_field).unwrap()
    }

    fn dump_to_binary(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn save_to_file(&self, filename: String) {
        let serialized_field = self.dump_to_binary();
        fs::write(filename + ".ms", serialized_field).expect("Unable to save");
    }

    fn load_from_file(&self, filename: String) -> Field {
        let serialized_field = fs::read(filename + ".ms").expect("Unable to load");
        Field::restore_from_binary(serialized_field)
    }

    fn play_via_tui(&mut self) {
        loop {
            println!("{}", self.draw(false));
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            action = action.trim().to_string(); //strip \n at end of line
            let args = action.split(" ").collect::<Vec<&str>>();

            debug!("{:?}", args);
            let (game_result, message) = self.process_command_args(args);
            match game_result {
                GameResult::Stop => {
                    println!("Game stopped. Have a nice day!");
                    println!("{}", self.draw(true));
                    break;
                }
                GameResult::Win => {
                    println!("Congratulations! You won!");
                    break;
                }
                GameResult::Lose => {
                    println!("You lose...Try to search mines more carefully next time");
                    println!("{}", self.draw(true));
                    break;
                }
                GameResult::Info => println!("Info: {}", message),
                GameResult::Error => println!("Error: {}", message),
                GameResult::Play => (),
            }
        }
        // println!("Game played: {:?}", self.start_time.elapsed());
    }

    fn play_via_gui(&mut self) {
        unimplemented!();
    }

    fn play_via_wui(&mut self) {
        unimplemented!();
    }

    pub fn play_via(&mut self, game_ui: GameUI) {
        match game_ui {
            GameUI::TUI => self.play_via_tui(),
            GameUI::GUI => self.play_via_gui(),
            GameUI::WUI => self.play_via_wui(),
        }
    }
}

#[test]
fn save_and_restore_field() {
    let field = Field::new(10);
    let serialized_field = field.dump_to_binary();
    let deserialized_field = Field::restore_from_binary(serialized_field);
    assert_eq!(field, deserialized_field);
}

#[test]
fn mark_cell() {
    let mut field = Field::new(3);
    field.mark_cell(0, 0, Mark::Mine);
    assert_eq!(field.cells[0][0].marked_as, Mark::Mine);
}
