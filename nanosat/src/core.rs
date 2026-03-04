#[derive(Debug)]
pub struct Literal {
    pub value: usize,
    pub is_negated: bool,
}

impl Literal {
    pub fn negate(&self) -> Literal {
        Literal {
            value: self.value,
            is_negated: !self.is_negated,
        }
    }
}

#[derive(Debug)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {
    pub fn clause_len(&self) -> usize {
        self.literals.len()
    }
}

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Formula {
    pub fn formula_len(&self) -> usize {
        self.clauses.len()
    }
}

#[derive(Debug)]
pub struct Cnf {
    pub num_vars: usize,
    pub num_clauses: usize,
    pub formula: Formula,
}

impl Cnf {
    pub fn new(num_vars: usize, num_clauses: usize, clauses: Vec<Vec<i32>>) -> Self {
        let parsed_clauses: Vec<Clause> = clauses
            .into_iter()
            .map(|clause| Clause {
                literals: clause
                    .into_iter()
                    .map(|lit| Literal {
                        value: lit.abs() as usize,
                        is_negated: lit < 0,
                    })
                    .collect(),
            })
            .collect();

        Cnf {
            num_vars,
            num_clauses,
            formula: Formula {
                clauses: parsed_clauses,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub variable: usize,
    pub value: bool,
    pub decision_level: usize,
    pub antecedent: Option<usize>,
}

impl Assignment {
    pub fn new(
        variable: usize,
        value: bool,
        decision_level: usize,
        antecedent: Option<usize>,
    ) -> Self {
        Assignment {
            variable,
            value,
            decision_level,
            antecedent,
        }
    }
}
