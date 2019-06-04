fn tokenize_file(filename: &str) -> std::io::Result<()> {

	let file = File::open("fact.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    // pattern to to detect facts
    let re = Regex::new(r"fact:\s\((.+)\)\n").unwrap();
    
    let facts: Vec<Fact> = Vec::new();
    let rules: Vec<Rule> = Vec::new();
    
    for content in re.captures_iter(&contents) {
    	println!("{:?}", &content[1]);
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