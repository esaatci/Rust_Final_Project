#![allow(unused_imports)]
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::knowledge_base::Symbol;
use crate::knowledge_base::Predicate;

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