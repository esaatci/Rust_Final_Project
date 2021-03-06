#![allow(unused_imports, dead_code, unused_variables)]
use crate::fact::Fact;
use crate::kb::Predicate;
use crate::rule::Rule;
use crate::symbols::Symbol;
use std::rc::Rc;

#[derive(Eq, Debug)]
pub struct Statement {
    predicate: Predicate,
    terms: Vec<Term>, //varaibles/constants in the fact or rule
}

impl Statement {
    pub fn terms_to_string<'a>(&'a self) -> String {
        let mut s = String::new();
        for term in &self.terms {
            s += &format!("{} ", term);
        }
        s.trim().to_owned()
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({} {})", self.predicate, self.terms_to_string())
    }
}

impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.predicate == other.predicate && self.terms == other.terms
    }
}

impl std::hash::Hash for Statement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.predicate.hash(state);
        for term in &self.terms {
            term.hash(state);
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum Term {
    Variable(Symbol),
    Constant(Symbol),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Term::Constant(s) => write!(f, "{}", s),
            Term::Variable(s) => write!(f, "{}", s),
        }
    }
}

pub enum Assertion {
    Fact(Fact),
    Rule(Rule),
}

// Hash function for rules will have to take into account that the
// order of rhs statements do not matter. Maybe add them or apply a
// commutative function to combine them
