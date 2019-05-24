extern crate bimap;
use bimap::BiMap;

struct kb {
    facts: Vec<Fact>,
    rules: Vec<Rule>,
    objects: BiMap<String, Symbol>,
    factsbypredicate: HashMap<Predicate, Vec<rc<Fact>>>,
    rulesbyRHS: HashMap<Predicate, Vec<rc<Rule>>>,
}

struct fact { // have a fact class? then have asserted/ inferred instances to save space?
    statement: Statement, // i.e. (Larger box circle)
    asserted: bool, // false if fact inferred from rules/facts
    supported_by: HashSet<(rc<Rule>, Vec<rc<Fact>>)>, // list of facts/rule that together instatiate this fact

}

impl fact {
}