//use crate::knowledge_base::KnowledgeBase;  this is giving us errors
#![allow(unused_imports, dead_code, unused_variables)]
//mod lib;
extern crate bimap;
use crate::fact::Fact;
use crate::rule::Rule;
use crate::symbols::Symbol;
use crate::parse::parse_file;
use std::env;
use std::process;
use std::rc::Rc;
mod fact;
mod kb;
mod rule;
mod statement_and_term;
mod symbols;
mod parse;
use std::io;
use std::io::{BufRead, Write};
mod bindings;

fn main() {
	// get the command line arguments
	// parse the command line arguments
	// try to retrieve the filename
	// pass the filename to the tokenize_file

//	let args: Vec<String> = env::args().collect();
//
//	if args.len() != 2 {
//		eprintln!("not enough arguments");
//		process::exit(1);
//	}
//
//	let mut my_kb = kb::KnowledgeBase::new();
//	let parsed_tokens = parse_file(&args[1]);
//
//	match parsed_tokens {
//		Ok(rules_and_facts) => {
//			for rf in rules_and_facts {
//				my_kb.assert(rf);
//			}
//			let stdin = io::stdin();
//			let mut stdout = io::stdout();
//			let mut in_handle = stdin.lock();
//
//			while let Ok(input) = read_next_query(&mut in_handle, &mut stdout) {
//				my_kb.read_user(&input, &mut stdout).unwrap();
//			}
//		}
//		Err(e) => {
//			eprintln!("{}", e);
//			process::exit(1);
//		}
//	}
	println!("hello world!");
	// loop
	// handle_command_line()
}

fn read_next_query<R, W>(mut input: R, mut output: W) -> Result<String, std::io::Error>
where
	R: BufRead,
	W: Write,
{
	write!(output, ">>> ")?;
	output.flush()?;

	let mut to_input = String::new();
	input.read_line(&mut to_input)?;
	Ok(to_input)
}
