use std::env::args;
use std::process;

use calculator::{Config, run};

fn main() {
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    let output = run(&config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    println!("{config} = {output}")
}
