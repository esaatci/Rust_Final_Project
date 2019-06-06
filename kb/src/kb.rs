#![allow(unused_imports, dead_code, unused_variables)]
use crate::fact::Fact;
use crate::rule::Rule;
use crate::statement_and_term::Statement;
use crate::statement_and_term::Term;
use crate::statement_and_term::{Assertion, RuleOrFact};
use crate::symbols::Symbol;
use bimap::BiMap;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::io::Write;
use std::rc::Rc;

pub enum Operation {
    Assert,
    Ask,
    Retract,
    Invalid,
    Exists,
}

// work in progress
pub enum KbError {
    InvalidFact,
    FactAsserted,
}

pub type Predicate = Symbol;
///A knowledge base. Used to store, access, and modify logical facts and rules.
pub struct KnowledgeBase {
    facts: Vec<Fact>,
    //all of the Facts in the KB
    rules: Vec<Rule>,
    //all of the Rules in the KB
    objects: BiMap<String, Symbol>,
    //maps a string to a Symbol. used for Predicates, variables, constants
    facts_by_predicate: HashMap<Predicate, Vec<Rc<Fact>>>,
    //find Facts by hashing on Predicate
    rules_by_rhs: HashMap<Predicate, Vec<Rc<Rule>>>, //find Rules by hashing on Predicate of RHS
}
// #####one Area to work on ##### Efe
// load a text of rules/facts in
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
    ///Creates a new Knowledge Base
    ///let my_kb = KnowledgeBase::new();
    pub fn new() -> Self {
        KnowledgeBase {
            facts: Vec::new(),
            rules: Vec::new(),
            objects: BiMap::new(),
            facts_by_predicate: HashMap::new(),
            rules_by_rhs: HashMap::new(),
        }
    }
    ///Assert adds a fact or rule to the Knowledge Base. It derives new facts based on this information
    /// and updates old rules/facts.
    /// let my_kb = KnowledgeBase::new();
    /// my_kb.assert(my_fact);
    /// my_fact is now added to the Knowledge base and can be accessed, modified, and deleted.

    pub fn assert(&mut self, fact_or_rule: RuleOrFact) {
        match fact_or_rule {
            RuleOrFact::Rule(r) => Self::add_rule(self, r),
            RuleOrFact::Fact(f) => Self::add_fact(self, f),
        }
    }

    /// Takes in a statement (Predicate ?x Alex ?w) that contains any number of constants (Alex), and
    /// variables (?x, ?y) and will return a list of statements from the facts that match the given statement.
    pub fn ask(&self, query: Statement) -> Option<Vec<Statement>> {
        //returns none if none found, or returns list of its related assertions
        unimplemented!();
    }

    /// Returns mutable reference to fact in the kb if it's statement matches the input, otherwise None
    // fn find_fact_mut<'a>(&'a self, statement: &Statement) -> Option<&'a mut Fact> {
    //     let pred = statement.get_predicate();
    //     if let Some(list_of_fact) = self.facts_by_predicate.get_mut(pred) {
    //         for ft in list_of_fact {
    //             if ft.get_statement() == statement {
    //                 return Some(ft);
    //             }
    //         }
    //         None
    //     } else {
    //         None
    //     }
    // }

    /// Returns reference to fact in the kb if it's statement matches the input, otherwise None
    fn find_fact<'a>(&'a self, statement: &Statement) -> Option<&'a Fact> {
        let pred = statement.get_predicate();
        if let Some(list_of_fact) = self.facts_by_predicate.get(pred) {
            for ft in list_of_fact {
                if ft.get_statement() == statement {
                    return Some(ft);
                }
            }
            None
        } else {
            None
        }
    }

    /// If the given statement matches a fact in the kb, and it is an asserted fact, it will be deleted,
    /// and removed from the supported_by lists of any facts it supports, removing them also if they are
    /// not asserted and their supported_by list is now empty
    // pub fn retract(&mut self, retraction: &Statement) -> Result<Option<()>, KbError> {
    //     if let Some(stored_fact) = self.find_fact(retraction) {
    //         if stored_fact.get_supported_by().is_empty() {
    //             for fact in stored_fact.get_supports_facts() {
    //                 let supports = fact.get_supported_by();
    //                 supports.remove(&stored_fact);
    //                 fact.get_supported_by().remove(&stored_fact);
    //                 if fact.asserted == false && fact.supported_by.is_empty() {
    //                     self.retract(fact);
    //                 }
    //             }
    //             Ok(Some(()))
    //         //remove- call funciton and recur on that one for supported facts
    //         } else {
    //             Ok(None)
    //         }
    //     // remove supported facts if they aren't supported
    //     } else {
    //         Ok(None)
    //     }
    // }

    //Helpers for Assert

    fn add_rule(&mut self, rule: Rule) {
        unimplemented!();
        //        if !self.is_rule_in_kb(&rule){ //if fact is not already in kb
        //            let rule_pred:&Symbol = rule.get_rhs().get_predicate();
        //
        //            //adding to facts_by_pred
        //            let option:Option<&mut Vec<Rc<Fact>>> = self.facts_by_predicate.get_mut(fact_pred);
        //            match option {
        //                None => {self.facts_by_predicate.insert(fact_pred.clone(), vec![Rc::new(fact.clone())]);},//no entry for predicate
        //                Some(v) => {v.push(Rc::new(fact.clone()));},   //there is an entry for that predicate. add reference to fact to its vec
        //            };
        //
        //            for rule in &self.rules{ //for every rule in kb, infer more
        //                self.infer(&rule, &fact);
        //            }
        //            self.facts.push(fact);  //adding it to kb facts vec
        //        }
        //        else{
        //            //let supported_by =
        //            let index = self.facts.iter().position(|r| *r == fact).unwrap(); //find fact's index in kb
        //            let mut supported_by = HashSet::new();
        //            if !fact.get_supported_by().is_empty(){//if fact has a supported_by field. ie i it is derived? cant we just check asserted flag?
        //                for f in fact.get_supported_by().iter(){
        //                    //add fact f to the KB's copy supportedby field
        //                    supported_by.insert(f.clone()); //modify/build up its supported by field
        //                }
        //                self.facts[index].set_supported_by(supported_by.clone()); //set the field
        //
        //            }
        //            else{
        //                //mark it as asserted in the KB's copy of that fact
        //                self.facts[index].set_asserted(true);
        //            }
        //
        //        }
    }

    pub fn read_user(
        &mut self,
        input: &str,
        stdout: &mut Write,
    ) -> Result<Option<()>, std::io::Error> {
        let mut split = input.split_whitespace();
        if let Some(first) = split.next() {
            match self.match_input(first) {
                Operation::Assert => match self.build_statement(split) {
                    Some(st) => {
                        self.assert(RuleOrFact::Fact(Fact::new(st, true)));
                        writeln!(stdout, "fact asserted")?;
                        return Ok(Some(()));
                    }
                    None => return Ok(None),
                },
                Operation::Ask => match self.build_statement(split) {
                    Some(st) => match self.ask(st) {
                        Some(matches) => {
                            self.print_matches(&matches, stdout)?;
                            return Ok(Some(()));
                        }
                        None => {
                            writeln!(stdout, "No matches found")?;
                            return Ok(None);
                        }
                    },
                    None => Ok(None),
                },
                Operation::Exists => match self.build_statement(split) {
                    Some(st) => {
                        if self.statement_exists(&st) {
                            writeln!(stdout, "fact in kb");
                        } else {
                            writeln!(stdout, "fact not in kb");
                        }
                        return Ok(None);
                    }
                    None => return Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn statement_exists(&self, statement: &Statement) -> bool {
        if let Some(val) = self.facts_by_predicate.get(statement.get_predicate()) {
            for lt in val {
                if lt.get_statement().get_terms() == statement.get_terms() {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn print_matches(
        &self,
        matches: &[Statement],
        stdout: &mut Write,
    ) -> Result<(), std::io::Error> {
        writeln!(stdout, "matches found:")?;
        for m in matches {
            writeln!(stdout, "{}", m)?;
        }
        Ok(())
    }

    pub fn match_input(&self, input: &str) -> Operation {
        match input {
            "assert" => Operation::Assert,
            "ask" => Operation::Ask,
            "retract" => Operation::Retract,
            "exists?" => Operation::Exists,
            _ => Operation::Invalid,
        }
    }

    pub fn build_fact<'a>(&self, mut input: impl Iterator<Item = &'a str>) -> Option<Statement> {
        if let Some(mut pred) = input.next() {
            pred = &pred[1..];
            let mut terms = Vec::new();
            let mut prev = input.next()?;
            while let Some(term) = input.next() {
                terms.push(Term::Constant(Symbol::new(prev)));
                prev = term;
            }
            prev.to_owned().pop()?;
            terms.push(Term::Constant(Symbol::new(prev)));
            let state = Statement::new(Symbol::new(pred), &terms);
            Some(state)
        } else {
            None
        }
    }

    pub fn build_statement<'a>(
        &self,
        mut input: impl Iterator<Item = &'a str>,
    ) -> Option<Statement> {
        if let Some(next) = input.next() {
            if next == "fact:" {
                self.build_fact(input)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_fact(&mut self, fact: Fact) {
        if !self.is_fact_in_kb(&fact) {
            //if fact is not already in kb
            let fact_pred: &Symbol = fact.get_statement().get_predicate();

            //adding to facts_by_pred
            let option: Option<&mut Vec<Rc<Fact>>> = self.facts_by_predicate.get_mut(fact_pred);
            match option {
                None => {
                    self.facts_by_predicate
                        .insert(fact_pred.clone(), vec![Rc::new(fact.clone())]);
                } //no entry for predicate
                Some(v) => {
                    v.push(Rc::new(fact.clone()));
                } //there is an entry for that predicate. add reference to fact to its vec
            };

            for rule in &self.rules {
                //for every rule in kb, infer more
                self.infer(&rule, &fact);
            }
            self.facts.push(fact); //adding it to kb facts vec
        } else {
            //let supported_by =
            let index = self.facts.iter().position(|r| *r == fact).unwrap(); //find fact's index in kb
            let mut supported_by = HashSet::new();
            if !fact.get_supported_by().is_empty() {
                //if fact has a supported_by field. ie i it is derived? cant we just check asserted flag?
                for f in fact.get_supported_by().iter() {
                    //add fact f to the KB's copy supportedby field
                    supported_by.insert(f.clone()); //modify/build up its supported by field
                }
                self.facts[index].set_supported_by(supported_by.clone()); //set the field
            } else {
                //mark it as asserted in the KB's copy of that fact
                self.facts[index].set_asserted(true);
            }
        }
    }
    fn is_fact_in_kb(&self, fact: &Fact) -> bool {
        let kb_facts = &self.facts_by_predicate;
        let fact_pred = fact.get_statement().get_predicate();
        if kb_facts.contains_key(&fact_pred) {
            //predicate is in kb, now check for specific fact
            let vec_facts = kb_facts.get(&fact_pred).unwrap();
            for f in vec_facts.iter() {
                if *fact == **f {
                    // f is a reference to fact. how to compare fact w/ reference 2 fact
                    return true;
                }
            }
        }
        return false;
    }

    fn is_rule_in_kb(&self, rule: &Rule) -> bool {
        let kb_rules = &self.rules_by_rhs;
        let rule_pred = rule.get_rhs().get_predicate();
        if kb_rules.contains_key(&rule_pred) {
            //predicate is in kb, now check for specific rule
            println!("in if");
            let vec_rules = kb_rules.get(&rule_pred).unwrap(); //vec of pointers to rules w/ that pred
            for r in vec_rules.iter() {
                //r is a pointer to rule
                println!("r is, {:?}", r);
                println!("rule is {:?}", rule);
                if *rule == **r {
                    // r is a reference to rule.
                    return true;
                }
            }
        }
        return false;
    }
    pub fn infer(&self, rule: &Rule, fact: &Fact) {
        unimplemented!();
    }
}

#[cfg(test)]
mod add_fact_tests {}

#[cfg(test)]
mod is_fact_in_kb_tests {
    use super::KnowledgeBase;
    use crate::fact::Fact;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;

    #[test]
    fn empty_kb() {
        let test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");
        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate, &terms);
        let asserted = true;
        let test_fact = Fact::new(statement, asserted);

        assert!(!test_kb.is_fact_in_kb(&test_fact));
    }
    #[test]
    fn is_in_kb() {
        let mut test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");
        let term3 = Symbol::new("term3");
        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate.clone(), &terms);

        let terms2 = vec![Term::Constant(term3)];
        let statement2 = Statement::new(predicate.clone(), &terms2);

        let test_fact = Fact::new(statement, true);
        let another_fact = Fact::new(statement2, true);

        let kb_facts = vec![test_fact.clone(), another_fact.clone()];
        test_kb.facts = kb_facts;
        test_kb.facts_by_predicate.insert(
            predicate,
            vec![Rc::new(test_fact.clone()), Rc::new(another_fact.clone())],
        );

        assert!(test_kb.is_fact_in_kb(&another_fact));
        assert!(test_kb.is_fact_in_kb(&test_fact));
    }

    #[test]
    fn not_in_kb() {
        let mut test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");
        let term3 = Symbol::new("term3");
        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate.clone(), &terms);
        let asserted = true;
        let test_fact = Fact::new(statement, asserted);

        let terms2 = vec![Term::Constant(term3)];
        let statement2 = Statement::new(predicate.clone(), &terms2);
        let another_fact = Fact::new(statement2, true);

        let kb_facts = vec![another_fact.clone()];
        test_kb.facts = kb_facts;
        test_kb
            .facts_by_predicate
            .insert(predicate, vec![Rc::new(another_fact)]);

        assert!(!test_kb.is_fact_in_kb(&test_fact));
    }
}

#[cfg(test)]
mod is_rule_in_kb_tests {
    use super::KnowledgeBase;
    use crate::rule::Rule;
    use crate::statement_and_term::Assertion;
    use crate::statement_and_term::Statement;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;
    use std::rc::Rc;

    #[test]
    fn empty_kb() {
        let test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let predicate2 = Symbol::new("pred2");

        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");

        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate, &terms);
        let statement2 = vec![Statement::new(predicate2, &terms)];

        let test_rule = Rule::new(statement2, statement);

        assert!(!test_kb.is_rule_in_kb(&test_rule));
    }
    #[test]
    fn is_in_kb() {
        let mut test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");
        let term3 = Symbol::new("term3");
        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate.clone(), &terms);

        let terms2 = vec![Term::Constant(term3)];
        let statement2 = Statement::new(predicate.clone(), &terms2);

        let lhs1 = vec![statement.clone(), statement2.clone()];
        let lhs2 = vec![statement2.clone()];

        let rhs1 = statement.clone();
        let rhs2 = statement2.clone();

        let rule_one = Rule::new(lhs1, rhs1);
        let rule_two = Rule::new(lhs2, rhs2);

        let kb_rules = vec![rule_one.clone(), rule_two.clone()];
        test_kb.rules = kb_rules;
        test_kb.rules_by_rhs.insert(
            predicate,
            vec![Rc::new(rule_two.clone()), Rc::new(rule_one.clone())],
        );

        assert!(test_kb.is_rule_in_kb(&rule_one));
        assert!(test_kb.is_rule_in_kb(&rule_two));
    }

    #[test]
    fn not_in_kb() {
        let mut test_kb = KnowledgeBase::new();

        let predicate = Symbol::new("testPred");
        let term1 = Symbol::new("term1");
        let term2 = Symbol::new("term2");
        let term3 = Symbol::new("term3");
        let terms = vec![Term::Constant(term1), Term::Constant(term2)];
        let statement = Statement::new(predicate.clone(), &terms);

        let terms2 = vec![Term::Constant(term3)];
        let statement2 = Statement::new(predicate.clone(), &terms2);

        let test_rule = Rule::new(
            vec![statement.clone(), statement2.clone()],
            statement.clone(),
        );
        let rule_two = Rule::new(vec![statement2.clone()], statement2.clone());

        let kb_rules = vec![test_rule.clone(), rule_two.clone()];
        test_kb.rules = kb_rules;
        test_kb
            .rules_by_rhs
            .insert(predicate, vec![Rc::new(rule_two.clone())]);

        assert!(!test_kb.is_rule_in_kb(&test_rule));
    }
}
