#![allow(unused_imports, dead_code, unused_variables)]


use std::rc::Rc;
use std::collections::hash_set::HashSet;
use crate::fact::Fact;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;


pub struct Rule {
    lhs: Statement,
    //does this need to be a vec of statements??
    rhs: Statement,
    asserted: bool,
    supported_by: HashSet<(Vec<Rc<Rule>>, Vec<Rc<Fact>>)>,
    supports_facts: HashSet<Rc<Fact>>,
    //all of the other Facts this Rule supports
    supports_rules: HashSet<Rc<Rule>>, //all of the other Rules this Rule supports
}
