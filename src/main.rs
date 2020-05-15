use clap::{crate_version, App, Arg};

#[macro_use]
extern crate log;
extern crate simple_logger;

mod cell;
mod field;
mod mark;

use field::{Field, GameUI, FIELD_DEFAULT_SIZE};

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
                .help("Set debug for looking at internal messages"),
        )
        .arg(
            // rework with enum in future
            // https://github.com/clap-rs/clap/blob/master/examples/13_enum_values.rs
            Arg::with_name("game mode")
                .short("m")
                .long("mode")
                .help("Select gamemode (tui,"),
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

    let game_mode = match matches.value_of("game mode") {
        Some("tui") => GameUI::TUI,
        Some("gui") => GameUI::GUI,
        Some("wui") => GameUI::WUI,
        _ => GameUI::TUI,
    };
    info!("Game started");
    game_field.play_via(game_mode);
    info!("Game stopped");
}
