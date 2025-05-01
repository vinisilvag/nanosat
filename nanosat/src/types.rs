use std::fmt;

#[derive(Debug, Clone)]
pub struct Literal {
    pub variable: usize,
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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negated {
            write!(f, "-{}", self.variable)
        } else {
            write!(f, "{}", self.variable)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.literals
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("∨")
        )
    }
}

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.clauses
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ∧ ")
        )
    }
}

#[derive(Debug)]
pub struct Assignment {
    pub decision_level: usize,
    pub variable: usize,
    pub value: bool,
}
