use crate::core::{Assignment, Cnf, Formula};

pub struct Solver {
    formula: Formula,
    num_vars: usize,
    assignments: Vec<Assignment>, // Assignment vec
    m: Vec<Option<bool>>,         // Complete array of assignments (fast access)
    decision_level: usize,
}

impl Solver {
    pub fn new(cnf: Cnf) -> Self {
        Solver {
            formula: cnf.formula,
            num_vars: cnf.num_vars,
            assignments: Vec::new(),
            m: (0..cnf.num_vars).map(|_| None).collect(),
            decision_level: 0,
        }
    }

    fn all_variables_assigned(&self) -> bool {
        self.m.iter().all(|assignment| assignment.is_some())
    }

    // All calls of this function happens after `all_variables_assigned`
    // Therefore, this `unwrap()` should be fine
    fn decide_variable(&self) -> usize {
        self.m.iter().position(|&x| x.is_none()).unwrap()
    }

    fn assign(&mut self, variable: usize, value: bool, antecedent: Option<usize>) {
        self.m[variable] = Some(value);
        self.assignments.push(Assignment::new(
            variable,
            value,
            self.decision_level,
            antecedent,
        ))
    }

    fn unassign(&mut self, variable: usize) {
        self.m[variable] = None;
        self.assignments
            .retain_mut(|assignment| assignment.variable != variable);
    }

    pub fn solve(&mut self) -> Option<Vec<Assignment>> {
        // Try unit propagation before

        while !self.all_variables_assigned() {
            let variable = self.decide_variable();
            self.decision_level += 1;
            self.assign(variable, true, None);

            // Try unit propagation here
        }

        Some(self.assignments.clone())
    }
}
