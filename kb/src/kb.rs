#![allow(unused_imports, dead_code, unused_variables)]
use bimap::BiMap;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;
use crate::statement_and_term::Assertion;


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
//#####one Area to work on ##### Efe
//load a text of rules/facts in
//tokenize/parse the file into rule/fact structs
//add those structs to KB
//read from command line/get new statements/add them to the KB


//####Another Area to work on #### this one is small Calypso (and help the other two)
//constructors for facts/rules
    //implement print, partialeq, etc


//####Another AREA to work on #### Rebecca
//Asserting a rule/Fact to kb
    //find all the rules/facts that support it add to supported_by field
    //check all of the rules/facts see if you need to add the New Fact/Rule to their "supports" fields
    //derive new facts

//what does it mean for an asserted fact or rule to be "supported_by" something else
    //is supported_by only for derived facts?

//what does it mean for a fact to support another fact??
//if it's a new fact, will it support any other facts or rules??
//when should we integrate the "symbols" that jesse talked about....
//WE CANT DERIVE RULES // sorry


impl KnowledgeBase {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn assert(&self, fact_or_rule:Assertion){
        match fact_or_rule {
            Assertion::Rule(r) => Self::add_rule(self, Rc::new(r)),
            Assertion::Fact(f) => Self::add_fact(self, Rc::new(f))
        }
    }

    pub fn ask(&self, query:Fact)-> Option<Vec<Assertion>>{ //returns none if none found, or returns list of its related assertions
        unimplemented!();
    }

    //deletes retraction and any assertions that are dependent on it
    pub fn retract(&self, retraction:Fact){
        unimplemented!()
    }
    //Helpers for Assert
    pub fn add_rule(&self, rule:Rc<Rule>){
        unimplemented!();
    }

    pub fn add_fact(&self, fact:Rc<Fact>){
        unimplemented!();
    }



}




