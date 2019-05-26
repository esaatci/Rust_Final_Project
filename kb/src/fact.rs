#![allow(unused_imports)]

use std::collections::hash_set::HashSet;
use std::rc::Rc;
use crate::rule::Rule;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;

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
    supports_rules: HashSet<Rc<Rule>>  //all of the other Rules this Fact supports
}
