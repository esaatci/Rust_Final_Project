#![allow(unused_imports, dead_code, unused_variables)]

use crate::rule::Rule;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;
use std::collections::hash_set::HashSet;
use std::rc::Rc;

#[derive(Eq)]
pub struct Fact {
    // have a Fact class? then have asserted/ inferred instances to save space?
    statement: Statement,
    // i.e. (Larger box circle)
    asserted: bool,
    // false if Fact inferred from Rules/Facts
    supported_by: HashSet<(Vec<Rc<Rule>>, Vec<Rc<Fact>>)>,
    // list of Facts/Rule that together instatiate this Fact
    supports_facts: HashSet<Rc<Fact>>,
    //all of the other Facts this Fact supports
    supports_rules: HashSet<Rc<Rule>>, //all of the other Rules this Fact supports
}

impl Fact {
    pub fn new(statement: Statement, asserted: bool) -> Self {
        Fact {
            statement,
            asserted,
            supported_by: HashSet::new(),
            supports_facts: HashSet::new(),
            supports_rules: HashSet::new(),
        }
    }

    pub fn get_statement(&self)->&Statement{
        &self.statement
    }
    pub fn get_asserted(&self)->bool{
        if self.asserted{
            return true;
        }
        return false;
    }

}

//impl std::fmt::Display for Fact {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//        write!(f, "{}", self.statement)
//    }
//}
//
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
