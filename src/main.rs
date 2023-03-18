use std::env;
use std::process;

use fitz::configuration::Config;
use fitz::game;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err.message);
        process::exit(err.code);
    });

    game::run(config);
}
