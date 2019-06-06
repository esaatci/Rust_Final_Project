#![allow(unused_imports, dead_code, unused_variables)]
use crate::fact::Fact;
use crate::kb::Predicate;
use crate::rule::Rule;
use crate::symbols::Symbol;
use std::rc::Rc;
use crate::bindings::Bindings;

#[derive(Eq, Clone, Debug)]
/// Statement contains a predicate i.e. MotherOf and a list of terms
/// such as ["Suzy", "Kate"]
pub struct Statement {
    predicate: Predicate,
    terms: Vec<Term>, //varaibles/constants in the fact or rule
}

impl Statement {
    pub fn new(predicate: Predicate, terms: &[Term]) -> Self {
        Statement {
            predicate,
            terms: terms.to_vec(),
        }
    }

    ///Creates a new statement from a statement and bindings
    pub fn instantiate(statement:&Statement, bindings:&Bindings)->Statement{
        let new_terms:Vec<Term> = statement.terms.iter().map(|t| Statement::instantiate_helper(t.clone(), &bindings)).collect();
        Statement{predicate: statement.predicate.clone(), terms:new_terms}
    }
    fn instantiate_helper(term:Term, bindings:&Bindings)->Term {
        if term.term_is_var() {
            match bindings.get_bindings(&term) {
                None => return term,
                Some(n) => return n
            }
        } else {
            return term;
        }
    }
    pub fn terms_to_string<'a>(&'a self) -> String {
        let mut s = String::new();
        for term in &self.terms {
            s += &format!("{} ", term);
        }
        s.trim().to_owned()
    }
    pub fn get_predicate(&self) -> &Symbol {
        &self.predicate
    }

    pub fn get_terms(&self)-> &Vec<Term>{
        &self.terms
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

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub enum Term {
    Variable(Symbol),
    Constant(Symbol),
}
impl Term {
    pub fn term_is_var(&self) -> bool {
        match self {
            Term::Variable(x) => true,
            Term::Constant(x) => false,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Term::Constant(s) => write!(f, "{}", s),
            Term::Variable(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Assertion {
    fact: Rc<Fact>,
    rule: Rc<Rule>,
}

impl std::hash::Hash for Assertion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fact.hash(state)
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum RuleOrFact {
    Fact(Fact),
    Rule(Rule),
}


#[cfg(test)]
mod term_tests{
    use super::Term;
    use crate::symbols::Symbol;
    #[test]
    fn term_is_var_test(){
        let test = Term::Variable(Symbol::new("hi"));
        let test2 = Term::Constant(Symbol::new("hi"));
        assert!(test.term_is_var());
        assert!(!test2.term_is_var());
    }
}
// Hash function for rules will have to take into account that the
// order of rhs statements do not matter. Maybe add them or apply a
// commutative function to combine them
