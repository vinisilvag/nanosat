mod parser;
mod types;

use std::{fs::File, io::BufReader};

pub fn solve(problem: BufReader<File>) {
    let formula = parser::parse_instance(problem);
    println!("{}", formula);
}
