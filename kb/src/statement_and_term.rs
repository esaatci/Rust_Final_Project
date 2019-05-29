#![allow(unused_imports, dead_code, unused_variables)]
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::kb::Symbol;
use crate::kb::Predicate;

pub struct Statement{
    predicate: Predicate,
    terms: Vec<Term> //varaibles/constants in the fact or rule
}

impl Statement {
    pub fn terms_to_string<'a>(&'a self) -> &'a str {
        self.terms.into_iter().map(|i| format!("{} ", i)).collect<String>();
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
        write!(f, "({} {})", self.predicate, self.terms_to_string())
    }
}

impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.predicate == other.predicate && self.terms == other.terms
    }
}



#[derive(PartialEq)]
pub enum Term{
    Variable(Symbol),
    Constant(Symbol)
}

impl std::fmt::Display for Term {
    fn fmt(&self, &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        
    }
}


pub enum Assertion{
    Fact(Fact),
    Rule(Rule)
}