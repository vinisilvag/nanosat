use std::collections::HashSet;

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
    assignments: Vec<Assignment>, // Assignment list
    m: Vec<Option<bool>>,         // Complete array of assignments (fast access)
    decision_level: usize,
}

impl Solver {
    pub fn new(cnf: Cnf) -> Self {
        Solver {
            formula: cnf.formula,
            num_vars: cnf.num_vars,
            assignments: Vec::new(),
            m: (0..cnf.num_vars + 1).map(|_| None).collect(),
            decision_level: 0,
        }
    }

    // TODO: change to `all_clauses_are_satisfied` later
    fn all_variables_assigned(&self) -> bool {
        self.m.iter().skip(1).all(|assignment| assignment.is_some())
    }

    // All calls of this function happens after `all_variables_assigned`
    // Therefore, this `unwrap()` should be fine
    fn decide_variable(&self) -> usize {
        self.m.iter().skip(1).position(|&x| x.is_none()).unwrap() + 1
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
        self.m[literal.value]
    }

    fn clause_status(&self, clause: &Clause) -> ClauseStatus {
        let literals = &clause.literals;
        let mut values: Vec<Option<bool>> = Vec::new();

        for lit in literals {
            let assigned_value = self.is_in_assignment(lit);
            if assigned_value.is_none() {
                values.push(None);
            } else {
                let val = assigned_value.unwrap();
                let lit_value = if lit.is_negated { !val } else { val };
                values.push(Some(lit_value));
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

    fn resolution(&self, a: Clause, b: Clause, x: usize) -> Clause {
        // Group literals from both clauses
        let mut new_literals: HashSet<_> = a
            .literals
            .into_iter()
            .chain(b.literals.into_iter())
            .collect();

        // Remove appearances of x and -x
        new_literals.retain(|lit| {
            *lit != Literal {
                value: x,
                is_negated: false,
            } && *lit
                != Literal {
                    value: x,
                    is_negated: true,
                }
        });

        // Transform back into a clause
        Clause {
            literals: {
                let mut lits: Vec<_> = new_literals.into_iter().collect();
                lits.sort();
                lits
            },
        }
    }

    fn literals_at_current_level(&self, clause: &Clause) -> usize {
        clause
            .literals
            .iter()
            .filter(|lit| {
                self.assignments
                    .iter()
                    .any(|a| a.variable == lit.value && a.decision_level == self.decision_level)
            })
            .count()
    }

    fn conflict_analysis(&self, conflict_clause: Clause) -> (Option<usize>, Option<Clause>) {
        if self.decision_level == 0 {
            return (None, None);
        }

        let mut learnt_clause = conflict_clause.clone();
        let mut implied_literals: Vec<_> = self
            .assignments
            .iter()
            .filter(|assignment| {
                assignment.decision_level == self.decision_level
                    && assignment.antecedent.is_some()
                    && learnt_clause
                        .literals
                        .iter()
                        .any(|lit| lit.value == assignment.variable)
            })
            .collect();

        while self.literals_at_current_level(&learnt_clause) > 1 {
            let literal = implied_literals.first().unwrap();
            let antecedent = literal.antecedent.clone().unwrap();

            // Update learnt clause
            learnt_clause = self.resolution(learnt_clause.clone(), antecedent, literal.variable);

            implied_literals = self
                .assignments
                .iter()
                .filter(|assignment| {
                    assignment.decision_level == self.decision_level
                        && assignment.antecedent.is_some()
                        && learnt_clause
                            .literals
                            .iter()
                            .any(|lit| lit.value == assignment.variable)
                })
                .collect();
        }

        let mut decision_levels: Vec<_> = self
            .assignments
            .iter()
            .filter(|a| {
                learnt_clause
                    .literals
                    .iter()
                    .any(|lit| lit.value == a.variable)
            })
            .map(|a| a.decision_level)
            .collect();

        decision_levels.sort_unstable();
        decision_levels.dedup();
        decision_levels.reverse();

        let backjump_level = if decision_levels.len() <= 1 {
            0
        } else {
            decision_levels[1]
        };

        (Some(backjump_level), Some(learnt_clause))
    }

    fn add_learnt_clause(&mut self, learnt_clause: Clause) {
        self.formula.clauses.push(learnt_clause);
    }

    fn backjump(&mut self, new_decision_level: usize) {
        let vars_to_remove: Vec<_> = self
            .assignments
            .iter()
            .filter(|a| a.decision_level > new_decision_level)
            .map(|a| a.variable)
            .collect();

        for v in vars_to_remove {
            self.unassign(v);
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

                        if new_decision_level.is_none() {
                            return None;
                        }

                        self.add_learnt_clause(learnt_clause.unwrap());
                        self.backjump(new_decision_level.unwrap());
                        self.decision_level = new_decision_level.unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Clause, Cnf, Formula};

    fn dummy_solver(num_vars: usize) -> Solver {
        Solver::new(Cnf {
            num_vars,
            num_clauses: 0,
            formula: Formula { clauses: vec![] },
        })
    }

    // Testing `assign` and `unassign` functions
    #[test]
    fn test_assign_updates_m_and_assignments() {
        let mut solver = dummy_solver(3);

        solver.assign(2, true, None);

        assert_eq!(solver.m[2], Some(true));
        assert_eq!(solver.assignments.len(), 1);

        let a = &solver.assignments[0];
        assert_eq!(a.variable, 2);
        assert_eq!(a.value, true);
        assert_eq!(a.decision_level, 0);
        assert!(a.antecedent.is_none());
    }

    #[test]
    fn test_assign_with_antecedent() {
        let mut solver = dummy_solver(2);

        let clause = Clause { literals: vec![] };

        solver.assign(1, false, Some(clause.clone()));

        let a = &solver.assignments[0];

        assert_eq!(solver.m[1], Some(false));
        assert_eq!(a.variable, 1);
        assert_eq!(a.value, false);
        assert_eq!(a.antecedent, Some(clause));
    }

    #[test]
    fn test_unassign_clears_m() {
        let mut solver = dummy_solver(2);

        solver.assign(1, true, None);
        solver.unassign(1);

        assert_eq!(solver.m[1], None);
    }

    #[test]
    fn test_unassign_removes_assignment() {
        let mut solver = dummy_solver(3);

        solver.assign(1, true, None);
        solver.assign(2, false, None);

        solver.unassign(1);

        assert_eq!(solver.assignments.len(), 1);
        assert_eq!(solver.assignments[0].variable, 2);
    }

    #[test]
    fn test_unassign_does_not_affect_other_variables() {
        let mut solver = dummy_solver(3);

        solver.assign(1, true, None);
        solver.assign(2, false, None);
        solver.assign(3, true, None);

        solver.unassign(2);

        assert_eq!(solver.m[1], Some(true));
        assert_eq!(solver.m[2], None);
        assert_eq!(solver.m[3], Some(true));

        assert_eq!(solver.assignments.len(), 2);
        assert!(solver.assignments.iter().any(|a| a.variable == 1));
        assert!(solver.assignments.iter().any(|a| a.variable == 3));
    }

    // Testing `all_variables_assigned`
    #[test]
    fn test_all_variables_assigned_false_when_some_missing() {
        let mut solver = dummy_solver(3);

        solver.assign(1, true, None);
        solver.assign(2, false, None);

        assert!(!solver.all_variables_assigned());
    }

    #[test]
    fn test_all_variables_assigned_true_when_all_assigned() {
        let mut solver = dummy_solver(3);

        solver.assign(1, true, None);
        solver.assign(2, false, None);
        solver.assign(3, true, None);

        assert!(solver.all_variables_assigned());
    }

    #[test]
    fn test_all_variables_assigned_ignores_position_zero() {
        let mut solver = dummy_solver(2);

        solver.assign(1, true, None);
        solver.assign(2, false, None);

        solver.m[0] = None;
        assert!(solver.all_variables_assigned());

        solver.m[0] = Some(true);
        assert!(solver.all_variables_assigned());

        solver.m[0] = Some(false);
        assert!(solver.all_variables_assigned());
    }

    // Testing `decide_variable`
    #[test]
    fn test_decide_variable_returns_first_unassigned() {
        let mut solver = dummy_solver(4);
        solver.assign(1, true, None);
        solver.assign(2, false, None);
        assert_eq!(solver.decide_variable(), 3);
    }

    #[test]
    fn test_decide_variable_skips_position_zero() {
        let mut solver = dummy_solver(2);
        solver.m[0] = None;
        solver.assign(1, true, None);
        assert_eq!(solver.decide_variable(), 2);
    }

    #[test]
    fn test_decide_variable_finds_gap_in_assignments() {
        let mut solver = dummy_solver(4);
        solver.assign(1, true, None);
        solver.assign(3, false, None);
        assert_eq!(solver.decide_variable(), 2);
    }

    #[test]
    fn test_decide_variable_when_only_last_is_free() {
        let mut solver = dummy_solver(3);
        solver.assign(1, true, None);
        solver.assign(2, false, None);
        assert_eq!(solver.decide_variable(), 3);
    }

    // Testing `is_in_assignment`
    #[test]
    fn test_is_in_assignment_found() {
        let mut solver = dummy_solver(3);

        solver.assign(2, true, None);

        let lit = Literal {
            value: 2,
            is_negated: false,
        };

        assert_eq!(solver.is_in_assignment(&lit), Some(true));
    }

    #[test]
    fn test_is_in_assignment_not_found() {
        let solver = dummy_solver(3);

        let lit = Literal {
            value: 1,
            is_negated: false,
        };

        assert_eq!(solver.is_in_assignment(&lit), None);
    }

    // Testing `clause_status`
    #[test]
    fn test_clause_status_satisfied() {
        let mut solver = dummy_solver(2);

        solver.assign(1, true, None);

        let clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        match solver.clause_status(&clause) {
            ClauseStatus::Satisfied => {}
            _ => panic!("expected clause to be satisfied"),
        }
    }

    #[test]
    fn test_clause_status_unsatisfied() {
        let mut solver = dummy_solver(2);

        solver.assign(1, false, None);
        solver.assign(2, false, None);

        let clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        match solver.clause_status(&clause) {
            ClauseStatus::Unsatisfied => {}
            _ => panic!("expected clause to be unsatisfied"),
        }
    }

    #[test]
    fn test_clause_status_unit() {
        let mut solver = dummy_solver(2);

        solver.assign(1, false, None);

        let clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        match solver.clause_status(&clause) {
            ClauseStatus::Unit(lit) => {
                assert_eq!(lit.value, 2);
            }
            _ => panic!("expected unit clause"),
        }
    }

    #[test]
    fn test_clause_status_unresolved() {
        let solver = dummy_solver(2);

        let clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        match solver.clause_status(&clause) {
            ClauseStatus::Unresolved => {}
            _ => panic!("expected unresolved clause"),
        }
    }

    // Testing `propagate`

    // Testing `resolution`
    #[test]
    fn test_resolution_basic() {
        let solver = dummy_solver(3);

        let a = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        let b = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: true,
                },
                Literal {
                    value: 3,
                    is_negated: false,
                },
            ],
        };

        let result = solver.resolution(a, b, 1);

        assert_eq!(result.literals.len(), 2);
        assert!(result.literals.contains(&Literal {
            value: 2,
            is_negated: false
        }));
        assert!(result.literals.contains(&Literal {
            value: 3,
            is_negated: false
        }));
    }

    #[test]
    fn test_resolution_removes_pivot_literal() {
        let solver = dummy_solver(2);

        let a = Clause {
            literals: vec![Literal {
                value: 1,
                is_negated: false,
            }],
        };

        let b = Clause {
            literals: vec![Literal {
                value: 1,
                is_negated: true,
            }],
        };

        let result = solver.resolution(a, b, 1);
        assert!(result.literals.is_empty());
    }

    // Testing `conflict_analysis`
    #[test]
    fn test_conflict_analysis_level_zero_returns_unsat() {
        let solver = dummy_solver(2);

        let conflict_clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: false,
                },
            ],
        };

        let (level, clause) = solver.conflict_analysis(conflict_clause);

        assert!(level.is_none());
        assert!(clause.is_none());
    }

    #[test]
    fn test_conflict_analysis_single_decision() {
        let mut solver = dummy_solver(1);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        let conflict_clause = Clause {
            literals: vec![Literal {
                value: 1,
                is_negated: true,
            }],
        };

        let (level, learnt) = solver.conflict_analysis(conflict_clause);
        println!("{:?}, {:?}", level, learnt);

        assert!(learnt.is_some());
        assert_eq!(level, Some(0));
    }

    #[test]
    fn test_conflict_analysis_backjump_level() {
        let mut solver = dummy_solver(3);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        solver.decision_level = 2;
        solver.assign(2, true, None);

        solver.assign(
            3,
            false,
            Some(Clause {
                literals: vec![
                    Literal {
                        value: 2,
                        is_negated: true,
                    },
                    Literal {
                        value: 3,
                        is_negated: false,
                    },
                ],
            }),
        );

        let conflict_clause = Clause {
            literals: vec![
                Literal {
                    value: 2,
                    is_negated: true,
                },
                Literal {
                    value: 3,
                    is_negated: true,
                },
            ],
        };

        let (level, learnt) = solver.conflict_analysis(conflict_clause);

        assert!(learnt.is_some());
        assert_eq!(level, Some(1));
    }

    #[test]
    fn test_conflict_analysis_learns_clause() {
        let mut solver = dummy_solver(2);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        solver.assign(
            2,
            false,
            Some(Clause {
                literals: vec![
                    Literal {
                        value: 1,
                        is_negated: true,
                    },
                    Literal {
                        value: 2,
                        is_negated: false,
                    },
                ],
            }),
        );

        let conflict_clause = Clause {
            literals: vec![Literal {
                value: 2,
                is_negated: true,
            }],
        };

        let (_level, learnt) = solver.conflict_analysis(conflict_clause);

        let learnt = learnt.unwrap();

        assert!(!learnt.literals.is_empty());
    }

    // Testing `add_learnt_clause`
    #[test]
    fn test_add_learnt_clause_increases_formula() {
        let mut solver = dummy_solver(2);

        let clause = Clause {
            literals: vec![
                Literal {
                    value: 1,
                    is_negated: false,
                },
                Literal {
                    value: 2,
                    is_negated: true,
                },
            ],
        };

        let initial_len = solver.formula.clauses.len();

        solver.add_learnt_clause(clause.clone());

        assert_eq!(solver.formula.clauses.len(), initial_len + 1);
        assert_eq!(solver.formula.clauses.last(), Some(&clause));
    }

    // Testing `backjump`
    #[test]
    fn test_backjump_removes_higher_levels() {
        let mut solver = dummy_solver(3);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        solver.decision_level = 2;
        solver.assign(2, false, None);

        solver.decision_level = 3;
        solver.assign(3, true, None);

        solver.backjump(1);

        assert_eq!(solver.m[1], Some(true));
        assert_eq!(solver.m[2], None);
        assert_eq!(solver.m[3], None);

        assert_eq!(solver.assignments.len(), 1);
        assert_eq!(solver.assignments[0].variable, 1);
    }

    #[test]
    fn test_backjump_keeps_lower_levels() {
        let mut solver = dummy_solver(3);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        solver.decision_level = 2;
        solver.assign(2, false, None);

        solver.backjump(2);

        assert_eq!(solver.m[1], Some(true));
        assert_eq!(solver.m[2], Some(false));

        assert_eq!(solver.assignments.len(), 2);
    }

    #[test]
    fn test_backjump_to_zero() {
        let mut solver = dummy_solver(3);

        solver.decision_level = 1;
        solver.assign(1, true, None);

        solver.decision_level = 2;
        solver.assign(2, false, None);

        solver.backjump(0);

        assert_eq!(solver.m[1], None);
        assert_eq!(solver.m[2], None);

        assert!(solver.assignments.is_empty());
    }
}
