#![allow(unused_imports)]
use bimap::BiMap;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
//use crate::statement_and_term::Statement;
//use crate::statement_and_term::Term;
//use crate::statement_and_term::Assertion;
use crate::statement_and_term::*;

pub type Symbol = usize;
pub type Predicate = Symbol;


pub struct KnowledgeBase{
    facts: Vec<Fact>,
    //all of the Facts in the KB
    rules: Vec<Rule>,
    //all of the Rules in the KB
    objects: BiMap<String, Symbol>,
    //maps a string to a Symbol. used for Predicates, variables, constants
    facts_by_predicate: HashMap<Predicate, Vec<Rc<Fact>>>,
    //find Facts by hashing on Predicate
    rules_by_rhs: HashMap<Predicate, Vec<Rc<Rule>>>,  //find Rules by hashing on Predicate of RHS
}

impl KnowledgeBase {
    pub fn new(self) -> Self {
        unimplemented!();
    }

    pub fn assert(self, fact_or_rule:Assertion) -> Self {
        match fact_or_rule {
            Rule => Self::add_rule(self, fact_or_rule),
            Fact => Self::add_fact(self, fact_or_rule)
        }
    }

    pub fn ask(self, query:Fact)-> Option<Vec<Assertion>>{ //returns none if none found, or returns list of its related assertions
        unimplemented!();
    }

    //deletes retraction and any assertions that are dependent on it
    pub fn retract(self, retraction:Fact)->Self{
        unimplemented!()
    }
    //Helpers for Assert
    fn add_rule(self, rule:Rc<Rule>) -> Self {
        unimplemented!();
    }

    fn add_fact(self, rule:Rc<Rule>) -> Self {
        unimplemented!();
    }

}




