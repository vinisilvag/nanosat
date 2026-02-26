#[derive(Debug)]
pub struct Literal {
    pub value: usize,
    pub is_negated: bool,
}

#[derive(Debug)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
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
