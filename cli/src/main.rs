pub mod error;

use clap::Parser;
use std::path::PathBuf;

use nanosat::{core::Assignment, solve};
use parser::parse_cnf;

use crate::error::{AppError, IoError};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .cnf input file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn print_assignment(model: Vec<Assignment>) {
    let mut sorted = model.clone();
    sorted.sort_by_key(|k| k.variable);
    for assignment in sorted {
        if assignment.value {
            print!("-{:?} ", assignment.variable);
        } else {
            print!("{:?} ", assignment.variable);
        }
    }
    println!();
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();

    if let Some(extension) = args.input.extension() {
        if extension != "cnf" {
            return Err(AppError::Io(IoError::DifferentExtension));
        }
    } else {
        return Err(AppError::Io(IoError::MissingExtension));
    }

    let cnf = parse_cnf(args.input)?;
    let model = solve(cnf);

    match model {
        None => println!("s UNSATISFIABLE"),
        Some(m) => {
            println!("s SATISFIABLE");
            print_assignment(m);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
