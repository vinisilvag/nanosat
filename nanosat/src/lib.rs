mod cdcl;
mod parser;
mod types;
mod utils;

use std::{fs::File, io::BufReader};

use cdcl::CDCL;

pub fn solve(problem: BufReader<File>) {
    let (variable_num, _, formula) = parser::parse_instance(problem);
    println!("{}", formula);
    let mut solver = CDCL::new(variable_num, formula);
    let result = solver.solve();
    println!("{result}");
}
