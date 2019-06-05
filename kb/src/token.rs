use std::fs::File;
use std::io::{BufReader, Read};
use crate::fact::Fact;
use crate::rule::Rule;
use crate::statement_and_term::Assertion;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;
use crate::symbols::*;
extern crate regex;
use regex::Regex;


///
///  Takes a filename and parses and tokenizes the contents of it.
///  Returns Result<ParsedToken strucutre>
///
fn tokenize_file(filename: &str) -> std::io::Result<ParsedTokens> {
	let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // pattern to to detect facts
    // let re = Regex::new(r"fact:\s\((.+)\)\n").unwrap();
    
    let fact_re = Regex::new(r"fact:\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    let rule_re = Regex::new(r"rule:\s\((.+)\)\s->\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    
    let facts: Vec<Fact> = Vec::new();
    let rules: Vec<Rule> = Vec::new();
    // 
    for content in fact_re.captures_iter(&contents) {
        let pred = intern(&content[1]);	
        let terms: Vec<Term> = content[2].split(" ").map(|i| Term::Constant(Symbol::new(i))).collect();
        let statement = Statement::new(pred, &terms);
        let new_fact = Fact::new(statement, true);
        facts.push(new_fact);
	}
      
    Ok(ParsedTokens {facts, rules})
}



// Structure to hold parsed facts and rules
#[derive(Debug)]
struct ParsedTokens {
	facts: Vec<Fact>,
	rules: Vec<Rule>,
}

impl ParsedTokens {
	pub fn new(facts: Vec<Fact>, rules: Vec<Rule>) -> Self {
		ParsedTokens{facts: facts, rules: rules}
	}
}




#[cfg(test)]
mod tokenize_file_tests {
    use crate::fact::Fact;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn parse_single_fact_correctly() {

    }
    
    fn parse_single_rule_correctly() {

    }

}

