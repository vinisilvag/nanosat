use crate::core::{Assignment, Clause, Cnf, Formula, Literal};

enum ClauseStatus {
    Satisfied,
    Unsatisfied,
    Unit(Literal),
    Unresolved,
}

enum PropagationStatus {
    Unresolved,
    Conflict(Clause),
}

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

    fn assign(&mut self, variable: usize, value: bool, antecedent: Option<Clause>) {
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

    // Unique: appears only once
    // fn propagate_unique_literals(&self) {}
    // Pure: appears only positively or negatively
    // fn propagate_pure_literals(&self) {}
    // fn propagate() {}

    fn is_in_assignment(&self, literal: &Literal) -> Option<bool> {
        for assignment in &self.assignments {
            if literal.value == assignment.variable {
                return Some(assignment.value);
            }
        }
        None
    }

    fn clause_status(&self, clause: &Clause) -> ClauseStatus {
        let literals = &clause.literals;
        let mut values: Vec<Option<bool>> = Vec::new();

        for lit in literals {
            let assigned_value = self.is_in_assignment(lit);
            if assigned_value.is_none() {
                values.push(None);
            } else {
                values.push(Some(!lit.is_negated && assigned_value.unwrap()));
            }
        }

        if values.iter().any(|x| *x == Some(true)) {
            return ClauseStatus::Satisfied;
        }

        if values.iter().filter(|x| **x == Some(false)).count() == values.len() {
            return ClauseStatus::Unsatisfied;
        }

        if values.iter().filter(|x| **x == Some(false)).count() == values.len() - 1 {
            let index = values.iter().position(|x| x.is_none()).unwrap();
            return ClauseStatus::Unit(literals[index]);
        }

        ClauseStatus::Unresolved
    }

    fn propagate(&mut self) -> PropagationStatus {
        let mut propagation_finished = false;
        while !propagation_finished {
            propagation_finished = true;

            for clause in self.formula.clauses.clone() {
                let status = self.clause_status(&clause);
                match status {
                    ClauseStatus::Unit(literal) => {
                        let variable = literal.value;
                        let value = !literal.is_negated;
                        self.assign(variable, value, Some(clause));
                        propagation_finished = false;
                    }
                    ClauseStatus::Unsatisfied => {
                        return PropagationStatus::Conflict(clause);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        PropagationStatus::Unresolved
    }

    fn conflict_analysis(&self, conflict_clause: Clause) -> (usize, Clause) {}

    fn add_learnt_clause(&mut self, learnt_clause: Clause) {
        self.formula.clauses.push(learnt_clause);
    }

    fn backjump(&mut self, new_decision_level: usize) {
        let mut to_remove: Vec<usize> = Vec::new();
        for assignment in self.assignments.clone() {
            if assignment.decision_level > new_decision_level {
                to_remove.push(assignment.variable);
            }
        }
        for variable in to_remove {
            self.unassign(variable);
        }
    }

    pub fn solve(&mut self) -> Option<Vec<Assignment>> {
        if let PropagationStatus::Conflict(_) = self.propagate() {
            return None;
        }

        while !self.all_variables_assigned() {
            let variable = self.decide_variable();
            self.decision_level += 1;
            self.assign(variable, true, None);

            loop {
                let propagation_status = self.propagate();
                match propagation_status {
                    PropagationStatus::Conflict(conflict_clause) => {
                        let (new_decision_level, learnt_clause) =
                            self.conflict_analysis(conflict_clause);
                        self.add_learnt_clause(learnt_clause);
                        self.backjump(new_decision_level);
                        self.decision_level = new_decision_level;
                    }
                    PropagationStatus::Unresolved => {
                        break;
                    }
                }
            }
        }

        Some(self.assignments.clone())
    }
}
