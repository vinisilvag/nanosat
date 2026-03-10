pub mod error;

use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf};

use nanosat::{
    core::{Assignment, Clause},
    solve,
};
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

fn generate_drat_proof(
    input: &PathBuf,
    learned_clauses: Vec<Clause>,
) -> Result<(), IoError> {
    let proof = input.with_extension("drat");
    let mut proof_file =
        File::create(&proof).map_err(|_| IoError::FailedToCreateProofFile)?;

    for clause in learned_clauses {
        for literal in clause.literals {
            if !literal.is_negated {
                write!(proof_file, "{} ", literal.value)
                    .map_err(|_| IoError::FailedToWriteToProofFile)?;
            } else {
                write!(proof_file, "-{} ", literal.value)
                    .map_err(|_| IoError::FailedToWriteToProofFile)?;
            }
        }
        write!(proof_file, "0\n")
            .map_err(|_| IoError::FailedToWriteToProofFile)?;
    }

    Ok(())
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

    let cnf = parse_cnf(&args.input)?;
    let out = solve(cnf);

    match out {
        Some((m, c)) => {
            println!("s SATISFIABLE");
            print_assignment(m);
            generate_drat_proof(&args.input, c)?;
        }
        None => println!("s UNSATISFIABLE"),
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
