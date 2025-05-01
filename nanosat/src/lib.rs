mod cdcl;
mod parser;
mod types;

use std::{fs::File, io::BufReader};

use cdcl::CDCL;

pub fn solve(problem: BufReader<File>) {
    let (variable_num, clause_num, formula) = parser::parse_instance(problem);
    println!("{}", formula);
    let solver = CDCL::new(variable_num, clause_num, formula);
    solver.solve();
}
