#![allow(unused_imports, dead_code, unused_variables)]
use std::ops::{Deref, DerefMut};
use crate::rule::Rule;
use crate::statement_and_term::{Assertion, RuleOrFact, Statement, Term};
use std::cell::RefCell;
use std::collections::hash_set::HashSet;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Eq, Clone, Debug)]
pub struct Fact {
    // have a Fact class? then have asserted/ inferred instances to save space?
    statement: Statement,
    // i.e. (Larger box circle)
    asserted: bool,
    // false if Fact inferred from Rules/Facts
    supported_by: Vec<Rc<Assertion>>,
    // list of Facts/Rule that together instatiate this Fact
    supports_facts: Vec<Rc<RefCell<Fact>>>,
    //all of the other Facts this Fact supports
    supports_rules: Vec<Rc<RefCell<Rule>>>, //all of the other Rules this Fact supports
}

//#[derive(Eq, Clone, Debug, PartialEq)]
//pub struct RefFact<Fact>(RefCell<Fact>);
//
//impl<Fact> std::hash::Hash for RefFact<Fact> {
//    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//        &self.statement.hash(state) // might need to hash more fields
//    }
//}
//
//
//impl<Fact> Deref for RefFact<Fact> {
//    type Target = RefCell<Fact>;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//
//impl<Fact> DerefMut for RefFact<Fact> {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.0
//    }
//}

impl Fact {
    /// Creates a new Fact, with a Statement, and a bool that indicates whether or not
    /// the Fact has been asserted by the user, or inferred when matching rules and facts
    pub fn new(statement: Statement, asserted: bool) -> Self {
        Fact {
            statement,
            asserted,
            supported_by: Vec::new(),
            supports_facts: Vec::new(),
            supports_rules: Vec::new(),
        }
    }

    /// Returns the Statement of the Rule
    pub fn get_statement(&self) -> &Statement {
        &self.statement
    }

    pub fn get_supports_facts(&self) -> &Vec<Rc<RefCell<Fact>>> {
        &self.supports_facts
    }

    pub fn get_supports_rules(&self) -> &Vec<Rc<RefCell<Rule>>> {
        &self.supports_rules
    }

    /// Returns whether the rule has been asserted or not
    pub fn get_asserted(&self) -> bool {
        if self.asserted {
            return true;
        }
        return false;
    }

    pub fn set_asserted(&mut self, value: bool) {
        self.asserted = value;
    }

    /// Returns a reference to the pairs of Facts and Rules that the Fact is supported by
    pub fn get_supported_by(&self) -> &Vec<Rc<Assertion>> {
        &self.supported_by
    }

    pub fn set_supported_by(&mut self, value: Vec<Rc<Assertion>>) {
        self.supported_by = value;
    }

    pub fn supports_facts_mut(&mut self) -> &mut Vec<Rc<RefCell<Fact>>> {
        &mut self.supports_facts
    }

    pub fn supports_rules_mut(&mut self) -> &mut Vec<Rc<RefCell<Rule>>> {
        &mut self.supports_rules
    }

    pub fn add_supports_fact(&mut self, fact:Rc<RefCell<Fact>>){
        self.supports_facts.push(fact);
    }
    pub fn add_supports_rule(&mut self, rule:Rc<RefCell<Rule>>){
        self.supports_rules.push(rule);
    }

    //     pub fn remove_supports(&mut self) {
    //         if !self.asserted {
    //             for ft in self.supports_facts_mut().iter() {
    //                 &std::rc::Rc<fact::Fact>::get_mut().remove_supports()
    //             }
    //         }
    //     }
}

//impl std::fmt::Display for Fact {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//        write!(f, "{}", self.statement)
//    }
//}

impl PartialEq for Fact {
    fn eq(&self, other: &Self) -> bool {
        self.statement == other.statement
    }
}

impl std::hash::Hash for Fact {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.statement.hash(state) // might need to hash more fields
    }
}
