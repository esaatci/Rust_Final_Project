//use crate::knowledge_base::KnowledgeBase;  this is giving us errors
#![allow(unused_imports, dead_code, unused_variables)]
//mod lib;
extern crate bimap;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::symbols::Symbol;
use std::rc::Rc;
mod fact;
mod kb;
mod rule;
mod statement_and_term;
mod symbols;
mod token;

fn main() {
    println!("Hello, world!");
    //let my_kb = kb::KnowledgeBase::new();
}
