//! CDCL framework implementation.

use std::fmt;

use crate::{
    types::{Assignment, Clause, Formula},
    utils::print_model,
};

pub type Model = Vec<Option<bool>>;

enum PropagationStatus {
    Unresolved,
    Conflict(usize),
}

enum ClauseStatus {
    Satisfied,
    Unsatisfied,
    Unit,
    Unresolved,
}

#[derive(Debug)]
pub enum SolverStatus {
    Sat(Model),
    Unsat,
}

impl fmt::Display for SolverStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolverStatus::Sat(model) => {
                write!(f, "s SATISFIABLE\nv {:?}", print_model(model.to_vec()))
            }
            SolverStatus::Unsat => write!(f, "s UNSATISFIABLE"),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct CDCL {
    formula: Formula,
    decision_level: usize,
    assignment: Vec<Option<bool>>,
    m: Vec<Assignment>,
}

impl CDCL {
    pub fn new(variable_num: usize, formula: Formula) -> Self {
        let assignment = vec![None; variable_num + 1];
        Self {
            formula,
            decision_level: 0,
            assignment,
            m: Vec::new(),
        }
    }

    fn assign(&mut self, variable: usize, value: bool) {
        self.m.push(Assignment {
            decision_level: self.decision_level,
            variable,
            value,
        });
        self.assignment[variable] = Some(value);
    }

    fn unassign(&mut self, variable: usize) {
        self.m.retain(|literal| literal.variable != variable);
        self.assignment[variable] = None;
    }

    fn decide(&self) -> (usize, bool) {
        (1, true)
    }

    fn all_clauses_are_satisfied(&self) -> bool {
        for clause in &self.formula.clauses {
            let mut satisfied = false;
            for literal in &clause.literals {
                if let Some(value) = self.assignment[literal.variable] {
                    if value != literal.is_negated {
                        satisfied = true;
                        break;
                    }
                }
            }
            if !satisfied {
                return false;
            }
        }
        true
    }

    fn propagate(&self) -> PropagationStatus {
        fn clause_status(clause: Clause) -> ClauseStatus {
            return ClauseStatus::Unresolved;
        }

        for clause in &self.formula.clauses {
            let status = clause_status(clause.clone());
        }

        PropagationStatus::Unresolved
    }

    fn learn(&mut self, learnt_clause: Clause) {
        self.formula.clauses.push(learnt_clause);
    }

    fn backjump(&mut self, new_decision_level: usize) {
        // let mut to_remove = Vec::new();
        // for assignment in self.m {
        //     if assignment.decision_level > new_decision_level {
        //         to_remove.push(assignment);
        //     }
        // }
        // for assignment in to_remove {
        //     self.unassign(assignment.variable);
        // }
    }

    fn conflict_analysis(&self, conflict_clause: usize) -> (usize, Clause) {
        fn resolution(c1: usize, c2: usize) -> Clause {
            Clause {
                literals: Vec::new(),
            }
        }

        (
            0,
            Clause {
                literals: Vec::new(),
            },
        )
    }

    pub fn solve(&mut self) -> SolverStatus {
        let propagation_status = self.propagate();
        if let PropagationStatus::Conflict(_) = propagation_status {
            return SolverStatus::Unsat;
        }

        while !self.all_clauses_are_satisfied() {
            let (variable, value) = self.decide();
            self.decision_level += 1;
            self.assign(variable, value);

            loop {
                let propagation_status = self.propagate();
                if let PropagationStatus::Conflict(conflict_clause) = propagation_status {
                    if self.decision_level == 0 {
                        return SolverStatus::Unsat;
                    }

                    let (new_decision_level, learnt_clause) =
                        self.conflict_analysis(conflict_clause);
                    self.learn(learnt_clause);
                    self.backjump(new_decision_level);
                } else {
                    break;
                }
            }
        }
        SolverStatus::Sat(self.assignment.clone())
    }
}
