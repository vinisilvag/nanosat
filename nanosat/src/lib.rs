pub mod core;
pub mod solver;

use crate::{
    core::{Assignment, Cnf},
    solver::Solver,
};

pub fn solve(cnf: Cnf) -> Option<Vec<i32>> {
    let mut solver = Solver::new(cnf);

    let model = solver.solve();

    Some(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
}
