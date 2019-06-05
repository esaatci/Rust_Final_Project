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
///  Takes in a filename and finds and tokenizes facts and rules
///  Return them in a ParsedTokens structure wrapped in Result
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
    let rule_single_re = Regex::new(r"^\(([\d\w]+)\s{1}(.+)\)").unwrap();
    
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

    for content in rule_re.captures_iter(&contents) {
        let rhs_pred = intern(&content[2]);
        let rhs_terms: Vec<Term> = content[3].split(" ").map(|i| Term::Variable(Symbol::new(i))).collect();
        let rhs_statement = Statement::new(rhs_pred, &rhs_terms);
        
        let lhs: Vec<Statement> = Vec::new();
        for sub_statement in content[1].split("  ").into_iter() {
            let pred = intern(&rule_single_re.captures(sub_statement).unwrap()[1]);
            let terms: Vec<Term> = content[2].split(" ").map(|i| Term::Variable(Symbol::new(i))).collect();
            let statement = Statement::new(pred, &terms);
            lhs.push(statement);
        } 

        let rule = Rule::new(lhs, rhs_statement);
        rules.push(rule);
    }

    Ok(ParsedTokens {facts, rules})
}

// retract assert and ask

/// Structure to hold parsed facts and rules
/// vector of facts and vector of rules
#[derive(Debug)]
struct ParsedTokens {
	facts: Vec<Fact>,
	rules: Vec<Rule>,
}

/// takes in a vector of facts and vector rules
/// returns a ParsedTokens Struct
impl ParsedTokens {
	pub fn new(facts: Vec<Fact>, rules: Vec<Rule>) -> Self {
		ParsedTokens{facts: facts, rules: rules}
	}
}

/// function handle  command line functionality

fn handle_command_line() {
    unimplemented!()
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


#[cfg(test)]
mod handle_command_line_tests {
    use crate::fact::Fact;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;
    use super::*;

    
} 

