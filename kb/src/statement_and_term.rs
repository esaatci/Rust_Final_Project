#![allow(unused_imports, dead_code, unused_variables)]
use crate::fact::Fact;
use crate::kb::Predicate;
use crate::rule::Rule;
use crate::symbols::Symbol;
use std::rc::Rc;

#[derive(Eq, Clone)]
pub struct Statement {
    predicate: Predicate,
    terms: Vec<Term>, //varaibles/constants in the fact or rule
}

impl Statement {
    pub fn new(predicate:Predicate, terms:Vec<Term>)->Self{
        Statement{predicate, terms}
    }
    pub fn terms_to_string<'a>(&'a self) -> String {
        self.terms.clone()
            .into_iter()
            .map(|i| format!("{:?} ", i))
            .collect::<String>()
    }
    pub fn get_predicate(&self)->&Symbol{
        &self.predicate
    }
}

//impl std::fmt::Display for Statement {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//        write!(f, "({} {})", self.predicate, self.terms_to_string())
//    }
//}
//
impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.predicate == other.predicate && self.terms == other.terms
    }
}

impl std::hash::Hash for Statement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.predicate.hash(state);
        for term in self.terms.clone() {
            term.hash(state);
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub enum Term {
    Variable(Symbol),
    Constant(Symbol),
}

// impl std::fmt::Display for Term {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         self.symbol_to_string()
//     }
// }
#[derive(PartialEq, Hash, Eq)]
pub enum Assertion {
    Fact(Fact),
    Rule(Rule),
}

// Hash function for rules will have to take into account that the
// order of rhs statements do not matter. Maybe add them or apply a
// commutative function to combine them
