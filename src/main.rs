use clap::{crate_version, App, Arg};

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::io;

mod cell;
mod field;
mod mark;

use field::{Field, GameResult, FIELD_DEFAULT_SIZE};

fn main() {
    let matches = App::new("Minesweeper")
        .about("Just minesweeper, console-only for now")
        .version(crate_version!())
        .arg(
            Arg::with_name("field size")
                .short("s")
                .long("field-size")
                .help("Select field size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Set debug"),
        )
        .get_matches();

    let field_size: usize = matches
        // https://github.com/clap-rs/clap/blob/master/examples/12_typed_values.rs
        // rework with value_of_t in future (needs clap 3.0.0 stable at least)
        .value_of("field size")
        .unwrap_or(&FIELD_DEFAULT_SIZE.to_string())
        .parse()
        .expect("Field size must be integer");

    let log_level: log::Level = if matches.is_present("debug") {
        log::Level::Debug
    } else {
        log::Level::Info
    };

    match simple_logger::init_with_level(log_level) {
        Ok(_) => info!("{}", "Logger successfully initialized"),
        Err(e) => println!("Error with logging: {}", e),
    }

    debug!("{:?}", matches);

    info!("Application started");
    let mut game_field: Field = Field::new(field_size);
    debug!("Field created: {:?} ", game_field);

    info!("Game started");
    loop {
        println!("{}", game_field.draw(false));
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        action = action[..action.len() - 1].to_string(); //strip \n at end of line
        let args = action.split(" ").collect::<Vec<&str>>();

        debug!("{:?}", args);
        let (game_result, message) = game_field.process_command_args(args);
        match game_result {
            GameResult::Stop => {
                // println!("Game played: {:?}", game_field.start_time.elapsed());
                println!("Game stopped. Have a nice day!");
                println!("{}", game_field.draw(true));
                break;
            }
            GameResult::Win => {
                // println!("Game played: {:?}", game_field.start_time.elapsed());
                println!("Congratulations! You won!");
                break;
            }
            GameResult::Lose => {
                // println!("Game played: {:?}", game_field.start_time.elapsed());
                println!("You lose...Try to search mines more carefully next time");
                println!("{}", game_field.draw(true));
                break;
            }
            GameResult::Info => println!("Info: {}", message),
            GameResult::Error => println!("Error: {}", message),
            GameResult::Play => (),
        }
    }
    info!("Game stopped");
}
