
extern crate regex;
use regex::Regex;


/*
    Takes a filename and parses and tokenizes the contents of it.
    Returns Result<ParsedToken strucutre>
*/

fn tokenize_file(filename: &str) -> std::io::Result<()> {
	let file = File::open("fact.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    // pattern to to detect facts
    let re = Regex::new(r"fact:\s\((.+)\)\n").unwrap();
    
    let fact_re = Regex::new(r"fact:\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    let rule_re = Regex::new(r"rule:\s\((.+)\)\s->\s\(([\d\w]+)\s{1}(.+)\)\n").unwrap();
    
    let facts: Vec<Fact> = Vec::new();
    let rules: Vec<Rule> = Vec::new();
    
    for content in fact_re.captures_iter(&contents) {
        let pred = intern(&content[1]);	
        let terms = &content[2];
        let statement = Statement::new(pred, &terms);
        let new_fact = Fact::new(statement, true);
        facts.push(new_fact);
	}
    
    Ok(())
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


fn tokenize_file_2(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let set = RegexSet::new(&[
    r"fact:\s\((.+)\)\n",
    r"rule:\s"]).unwrap();
}