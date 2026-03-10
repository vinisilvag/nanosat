pub mod error;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, i32 as nom_i32, multispace1},
    combinator::map_res,
    multi::separated_list1,
};

use nanosat::core::Cnf;

use error::ParserError;

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>).parse(input)
}

fn parse_header_line(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("p")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("cnf")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, vars) = parse_usize(input)?;
    let (input, _) = multispace1(input)?;
    let (input, clauses) = parse_usize(input)?;
    Ok((input, (vars, clauses)))
}

fn parse_clause_lits(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(multispace1, nom_i32).parse(input)
}

pub fn parse_cnf(input: PathBuf) -> Result<Cnf, ParserError> {
    let file = File::open(input).map_err(|_| ParserError::FailedToOpenFile)?;
    let reader = BufReader::new(file);

    let mut num_vars: Option<usize> = None;
    let mut num_clauses: Option<usize> = None;
    let mut clauses: Vec<Vec<i32>> = Vec::new();
    let mut current_clause: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| ParserError::FailedToRecoverLine)?;
        let line = line.trim();

        // Empty and comment line
        if line.is_empty() || line.starts_with('c') {
            continue;
        }

        // Header
        if line.starts_with('p') {
            let (_, header) = parse_header_line(line).map_err(|_| ParserError::InvalidHeader)?;
            num_vars = Some(header.0);
            num_clauses = Some(header.1);
            continue;
        }

        // Clause
        let (_, ints) = parse_clause_lits(line).map_err(|_| ParserError::InvalidClause)?;
        for lit in ints {
            if lit == 0 {
                clauses.push(std::mem::take(&mut current_clause));
            } else {
                current_clause.push(lit);
            }
        }
    }

    // Unterminated clause (last clause without the 0 terminator)
    if !current_clause.is_empty() {
        return Err(ParserError::UnterminatedClause);
    }

    // // Missing complete header
    // if num_vars.is_none() || num_clauses.is_none() {
    //     return Err("missing complete CNF header".into());
    // }

    Ok(Cnf::new(num_vars.unwrap(), num_clauses.unwrap(), clauses))
}
