use crate::fact::Fact;
use crate::rule::Rule;
use crate::statement_and_term::Assertion;
use crate::statement_and_term::Statement;
use crate::statement_and_term::RuleOrFact;
use crate::statement_and_term::Term;
use crate::symbols::*;
use std::fs::File;
use std::io::{BufReader, Read};
extern crate regex;
use regex::Regex;

///
///  Takes in a filename and finds and tokenizes facts and rules
///  Return them in a vector of RuleOrFact structure wrapped in Result
///  

pub fn tokenize_file(filename: &str) -> std::io::Result<Vec<RuleOrFact>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut rules_and_facts: Vec<RuleOrFact> = Vec::new();
    buf_reader.read_to_string(&mut contents)?;

    // pattern to to detect facts
    // let re = Regex::new(r"fact:\s\((.+)\)\n").unwrap();

    let fact_re = Regex::new(r"fact:\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    let rule_re = Regex::new(r"rule:\s\((.+)\)\s->\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    let rule_single_re = Regex::new(r"^\(([\d\w]+)\s{1}(.+)\)").unwrap();
    
    for content in fact_re.captures_iter(&contents) {
        let pred = intern(&content[1]);
        let terms: Vec<Term> = tokenize_helper(&content[2], true);
        let statement = Statement::new(pred, &terms);
        let new_fact = RuleOrFact::Fact(Fact::new(statement, true));
        rules_and_facts.push(new_fact);

        
    }
    for content in rule_re.captures_iter(&contents) {
        let rhs_pred = intern(&content[2]);
        let rhs_terms: Vec<Term> = tokenize_helper(&content[3], true);
        let rhs_statement = Statement::new(rhs_pred, &rhs_terms);
        let mut lhs: Vec<Statement> = Vec::new();
        for sub_statement in content[1].split("  ").into_iter() {
            let parse_sub_statement = rule_single_re.captures(sub_statement).unwrap();  
            let pred = intern(&parse_sub_statement[1]);
            let terms: Vec<Term> = tokenize_helper(&parse_sub_statement[2], false);
            let statement = Statement::new(pred, &terms);
            lhs.push(statement);
        }
        let rule = RuleOrFact::Rule(Rule::new(lhs, rhs_statement));
        rules_and_facts.push(rule);
    }

    Ok(rules_and_facts)
}


// helper to reduce boilerplate code. takes in the string of terms
// creates a vector of terms
// if term_type == true than the terms are variable else
// terms are constant
fn tokenize_helper(terms: &str, term_type: bool) -> Vec<Term> {
    if term_type {
        terms.split(" ").map(|i| Term::Variable(Symbol::new(i)))
        .collect()
    }
    else {
        terms.split(" ").map(|i| Term::Constant(Symbol::new(i)))
        .collect()   
    }
}

// retract assert and ask


/// function handle  command line functionality

fn handle_command_line() {
    unimplemented!()
}

#[cfg(test)]
mod tokenize_file_tests {
    use super::*;
    use crate::fact::Fact;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;

    #[test]
    fn parse_single_fact_correctly() {}

    fn parse_single_rule_correctly() {}

}

#[cfg(test)]
mod handle_command_line_tests {
    use super::*;
    use crate::fact::Fact;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;

}
