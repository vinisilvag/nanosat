pub mod core;
pub mod solver;

use crate::{
    core::{Assignment, Cnf},
    solver::Solver,
};

pub fn solve(cnf: Cnf) -> Option<Vec<Assignment>> {
    let mut solver = Solver::new(cnf);
    solver.solve()
}
