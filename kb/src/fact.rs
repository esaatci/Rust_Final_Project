#![allow(unused_imports, dead_code, unused_variables)]
use crate::rule::Rule;
use crate::statement_and_term::{Assertion, RuleOrFact, Statement, Term};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_set::HashSet;
use std::rc::Rc;

#[derive(Eq, Clone, Debug)]

/// Fact is a data class for KnowledgeBase that stores a Statement, whether the Fact is
/// asserted or not, which Rules and Facts it supports, and what pairs of Rules and Facts
/// support it.
pub struct Fact {
    // have a Fact class? then have asserted/ inferred instances to save space?
    statement: Statement,
    // i.e. (Larger box circle)
    asserted: bool,
    // false if Fact inferred from Rules/Facts
    supported_by: HashSet<Rc<Assertion>>,
    // list of Facts/Rule that together instatiate this Fact
    supports_facts: Supports,
    //all of the other Facts this Fact supports
    supports_rules: HashSet<Rc<Rule>>, //all of the other Rules this Fact supports
}

pub struct Supports(HashSet<Rc<RefCell<Fact>>>);

impl Fact {
    /// Creates a new Fact, with a Statement, and a bool that indicates whether or not
    /// the Fact has been asserted by the user, or inferred when matching rules and facts
    ///
    pub fn new(statement: Statement, asserted: bool) -> Self {
        Fact {
            statement,
            asserted,
            supported_by: HashSet::new(),
            supports_facts: Supports::new(),
            supports_rules: HashSet::new(),
        }
    }

    /// Returns the Statement of the Fact
    pub fn get_statement(&self) -> &Statement {
        &self.statement
    }

    /// Returns a reference to the supports_facts field of the Fact
    pub fn get_supports_facts(&self) -> &HashSet<Rc<RefCell<Fact>>> {
        &self.supports_facts.0
    }

    /// Returns a reference to the supports_rules field of the Fact
    pub fn get_supports_rules(&self) -> &HashSet<Rc<Rule>> {
        &self.supports_rules
    }

    /// Returns whether the rule has been asserted or not
    pub fn get_asserted(&self) -> bool {
        if self.asserted {
            return true;
        }
        return false;
    }

    /// Changes the asserted field
    pub fn set_asserted(&mut self, value: bool) {
        self.asserted = value;
    }

    /// Returns a reference to the pairs of Facts and Rules that the Fact is supported by
    pub fn get_supported_by(&self) -> &HashSet<Rc<Assertion>> {
        &self.supported_by
    }

    /// Replaces the supported_by field of a Rule
    pub fn set_supported_by(&mut self, value: HashSet<Rc<Assertion>>) {
        self.supported_by = value;
    }

    /// Returns a mutable reference to the supports_facts field of the Fact
    pub fn supports_facts_mut(&mut self) -> &mut Supports {
        &mut self.supports_facts
    }

    /// Returns a mutable reference to the supports_rules field of the Fact
    pub fn supports_rules_mut(&mut self) -> &mut HashSet<Rc<Rule>> {
        &mut self.supports_rules
    }

    /// Adds a Fact to the supports_facts field of a Fact
    pub fn add_supports_fact(&mut self, fact: Rc<Fact>) {
        self.supports_facts.insert(fact);
    }

    /// Adds a Rule to the supports_rules field of a Fact
    pub fn add_supports_rule(&mut self, rule: Rc<Rule>) {
        self.supports_rules.insert(rule);
    }

    pub fn remove_support(&mut self, fact: &Fact) {
        self.supported_by.remove(fact);
    }
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
