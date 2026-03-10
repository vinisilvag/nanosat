pub mod core;
pub mod solver;

use crate::{
    core::Cnf,
    solver::{Solver, SolverOutput},
};

pub fn solve(cnf: Cnf) -> SolverOutput {
    let mut solver = Solver::new(cnf);
    solver.solve()
}
