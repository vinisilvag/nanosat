//! Parser implementation.

mod error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::types::{Clause, Formula, Literal};

pub fn parse_instance(problem: BufReader<File>) -> (usize, usize, Formula) {
    let mut clauses: Vec<Clause> = Vec::new();
    let mut variable_num: usize = 0;
    let mut clause_num: usize = 0;

    let mut current_clause = Clause {
        literals: Vec::new(),
    };

    for line in problem.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if !tokens.is_empty() && tokens[0] != "p" && tokens[0] != "c" {
            for token in tokens {
                let parsed_token = token.parse::<i32>().unwrap();
                if parsed_token == 0 {
                    clauses.push(current_clause);
                    current_clause = Clause {
                        literals: Vec::new(),
                    };
                } else {
                    current_clause.literals.push(Literal {
                        variable: parsed_token.unsigned_abs() as usize,
                        is_negated: parsed_token < 0,
                    });
                }
            }
        } else if tokens[0] == "p" {
            variable_num = tokens[2].parse::<usize>().unwrap();
            clause_num = tokens[3].parse::<usize>().unwrap();
        }
    }

    (variable_num, clause_num, Formula { clauses })
}
