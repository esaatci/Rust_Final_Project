//use crate::knowledge_base::KnowledgeBase;  this is giving us errors
#![allow(unused_imports, dead_code, unused_variables)]
//mod lib;
extern crate bimap;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::symbols::Symbol;
use std::rc::Rc;
use crate::token::tokenize_file;
use std::process;
use std::env;
mod fact;
mod kb;
mod rule;
mod statement_and_term;
mod symbols;
mod token;
mod bindings;

fn main() {
    
	// get the command line arguments 
	// parse the command line arguments 
	// try to retrieve the filename
	// pass the filename to the tokenize_file

	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		eprintln!("not enough arguments");
		process::exit(1);
	}

    let mut my_kb = kb::KnowledgeBase::new();    
    let parsed_tokens = tokenize_file(&args[1]);
    

    match parsed_tokens {
    	Ok(rules_and_facts) => {
    		for rf in rules_and_facts {
    			my_kb.assert(rf);
    		}
    	},
    	Err(e) => {
    		eprintln!("{}", e);
    		process::exit(1);
    	},
    }

    // loop 
    // handle_command_line()
}
