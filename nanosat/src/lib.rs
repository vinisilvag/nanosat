pub mod core;
pub mod solver;

use crate::{
    core::{Assignment, Clause, Cnf},
    solver::Solver,
};

pub fn solve(cnf: Cnf) -> Option<(Vec<Assignment>, Vec<Clause>)> {
    let mut solver = Solver::new(cnf);
    solver.solve()
}
