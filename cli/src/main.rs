mod args;

use std::{fs::File, io::BufReader};

use args::{Cli, Command};
use clap::Parser;

use nanosat::solve;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Solve(options) => {
            let problem = BufReader::new(
                File::open(options.problem_file).expect("Error reading problem file"),
            );
            solve(problem);
        }
    };
}
