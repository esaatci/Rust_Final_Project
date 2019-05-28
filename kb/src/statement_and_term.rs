#![allow(unused_imports, dead_code, unused_variables)]
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::kb::Symbol;
use crate::kb::Predicate;

pub struct Statement{
    predicate: Predicate,
    terms: Vec<Term> //varaibles/constants in the fact or rule
}
pub enum Term{
    Variable(Symbol),
    Constant(Symbol)
}

pub enum Assertion{
    Fact(Fact),
    Rule(Rule)
}