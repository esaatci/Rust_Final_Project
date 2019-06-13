#![allow(unused_imports, dead_code, unused_variables)]

use crate::fact::Fact;
//use crate::fact::RefFact;
use crate::statement_and_term::Term;
use crate::statement_and_term::{Assertion, Statement};
use std::collections::hash_set::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Eq, Clone, Debug)]
pub struct Rule {
    lhs: Vec<Statement>,
    //does this need to be a vec of statements??
    rhs: Statement,
    supported_by: Vec<Rc<Assertion>>,
    supports_facts: Vec<Rc<RefCell<Fact>>>,
    //all of the other Facts this Rule supports
    supports_rules: Vec<Rc<RefCell<Rule>>>, //all of the other Rules this Rule supports
}

//#[derive(Eq, Clone, Debug, PartialEq)]
//pub struct RefRule(RefCell<Rule>);
//
//
//impl std::hash::Hash for RefRule {
//fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//    self.borrow().lhs.hash(state);
//    self.borrow().rhs.hash(state);
//}
//}


impl Rule {
    /// Creates a new Rule. The lhs of the Rule represents the series of Statments that
    /// need to be satisfied for the Statement on the rhs to be inferred. i.e. if you have
    /// (Motherof Mary Ada) and the rule ((Motherof ?x ?y) => (Childof ?y ?x)), the Rule would
    /// trigger and infer (Childof Ada Mary)
    pub fn new(lhs: Vec<Statement>, rhs: Statement) -> Self {
        Rule {
            lhs,
            rhs,
            supported_by: Vec::new(),
            supports_facts: Vec::new(),
            supports_rules: Vec::new(),
        }
    }

    /// Returns the Statement on the rhs of the Rule
    pub fn get_rhs(&self) -> &Statement {
        &self.rhs
    }

    pub fn get_lhs(&self)->&Vec<Statement>{
        &self.lhs
    }
    pub fn add_supports_fact(&mut self, fact:Rc<RefCell<Fact>>){
        self.supports_facts.push(fact);
    }
    pub fn add_supports_rule(&mut self, rule:Rc<RefCell<Rule>>){
        self.supports_rules.push(rule);
    }
}



impl std::hash::Hash for Rule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lhs.hash(state);
        self.rhs.hash(state);
    }
}

impl std::cmp::PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        for statement in &self.lhs {
            s += &format!("{} ", &statement);
        }
        write!(f, "{}=>{}", s, self.rhs)
    }
}
