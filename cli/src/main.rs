use clap::Parser;
use std::path::PathBuf;

use nanosat::{core::Assignment, solve};
use parser::parse_cnf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .cnf input file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn print_assignment(model: Vec<Assignment>) {
    for assignment in model {
        if assignment.value {
            print!("-{:?} ", assignment.variable);
        } else {
            print!("{:?} ", assignment.variable);
        }
    }
    println!();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(extension) = args.input.extension() {
        if extension != "cnf" {
            return Err("input file must have .cnf extension".into());
        }
    } else {
        return Err("input file must have some extension".into());
    }

    let cnf = parse_cnf(args.input)?;
    let model = solve(cnf);

    match model {
        None => println!("s UNSATISFIABLE"),
        Some(m) => {
            println!("s SATISFIABLE");
            println!("{:#?}", m);
            // print_assignment(m);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
