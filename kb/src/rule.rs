#![allow(unused_imports, dead_code, unused_variables)]

use crate::fact::Fact;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;
use std::collections::hash_set::HashSet;
use std::rc::Rc;

#[derive(Eq, Clone, Debug)]
pub struct Rule {
    lhs: Vec<Statement>,
    //does this need to be a vec of statements??
    rhs: Statement,
    supported_by: HashSet<(Vec<Rc<Rule>>, Vec<Rc<Fact>>)>,
    supports_facts: HashSet<Rc<Fact>>,
    //all of the other Facts this Rule supports
    supports_rules: HashSet<Rc<Rule>>, //all of the other Rules this Rule supports
}

impl Rule {
    pub fn new(lhs: Vec<Statement>, rhs: Statement) -> Self {
        Rule {
            lhs,
            rhs,
            supported_by: HashSet::new(),
            supports_facts: HashSet::new(),
            supports_rules: HashSet::new(),
        }
    }
    pub fn get_rhs(&self)->&Statement{
        &self.rhs
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
