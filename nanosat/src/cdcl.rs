use crate::types::{Assignment, Formula};

type Model = Vec<Option<bool>>;

pub struct CDCL {
    variable_num: usize,
    clause_num: usize,
    formula: Formula,
    decision_level: usize,
    assignment: Vec<Option<bool>>,
    M: Vec<Assignment>,
}

impl CDCL {
    pub fn new(variable_num: usize, clause_num: usize, formula: Formula) -> Self {
        let assignment = vec![None; variable_num + 1];
        Self {
            variable_num,
            clause_num,
            formula,
            decision_level: 0,
            assignment,
            M: Vec::new(),
        }
    }

    fn assign(&mut self, variable: usize, value: bool) {
        self.M.push(Assignment {
            decision_level: self.decision_level,
            variable,
            value,
        });
        self.assignment[variable] = Some(value);
    }

    fn unassign(&mut self, variable: usize) {
        self.M.retain(|literal| literal.variable != variable);
        self.assignment[variable] = None;
    }

    fn decide(&self) -> (usize, bool) {
        (1, true)
    }

    fn all_clauses_are_satisfied(&self) -> bool {
        true
    }

    pub fn solve(&self) -> Option<Model> {
        // perform unit propagation
        // if it conflict -> unsat

        while !self.all_clauses_are_satisfied() {}
        Some(self.assignment.clone())
    }
}
