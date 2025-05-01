#[derive(Debug)]
pub struct Literal {
    pub variable: i32,
    pub is_negated: bool,
}

impl Literal {
    fn negation(&self) -> Literal {
        Literal {
            variable: self.variable,
            is_negated: !self.is_negated,
        }
    }
}

#[derive(Debug)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}
