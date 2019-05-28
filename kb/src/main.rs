
//use crate::knowledge_base::KnowledgeBase;  this is giving us errors
#![allow(unused_imports, dead_code, unused_variables)]
//mod lib;
use std::rc::Rc;
use crate::fact::Fact;
use crate::rule::Rule;
mod kb;
mod rule;
mod fact;
mod statement_and_term;

fn main() {
    println!("Hello, world!");
    //let my_kb = kb::KnowledgeBase::new();
}
