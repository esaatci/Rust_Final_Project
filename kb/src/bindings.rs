use std::collections::hash_map::HashMap;
use crate::statement_and_term::Term;
///Bindings display the connections between constants and variables in given statements
#[derive(Eq, Clone, Debug, PartialEq)]
pub struct Bindings{

    bindings: HashMap<Term, Term> //key is variable. value is the value associated w/ variable
                        //ie if x = 5 key:x, value:5
}

impl Bindings{
    pub fn new()->Self{
        Bindings{bindings:HashMap::new()}
    }
    ///Adds a variable-constant binding
    pub fn add_binding(&mut self, variable:Term, value:Term){
        self.bindings.insert(variable, value);
    }
    ///gets the bindings associated with given variable
    pub fn get_bindings(&self, variable:&Term)->Option<Term>{
        match self.bindings.get(variable){
            None => return None,
            Some(v)=> return Some(v.clone())
        };
    }

    ///check if variable is bound. If not, binds it to value. If it is, return whether value matches the bound value.
    pub fn test_and_bind(&mut self, variable:&Term, value:&Term)->bool{
        let bound = self.get_bindings(variable);
        match bound{
            None=>{
                self.add_binding(variable.clone(), value.clone());
                return true;
            },
            Some(n) => return n == *value,
        };
    }
}
#[cfg(test)]
mod bindings_methods_tests{
    use super::Bindings;
    use crate::statement_and_term::Term;
    use crate::symbols::Symbol;
    use std::collections::hash_map::HashMap;

    #[test]
    fn test_and_bind(){
        let mut test_binding = Bindings::new();
        test_binding.add_binding(Term::Variable(Symbol::new("x")), Term::Constant(Symbol::new("a")));
        test_binding.add_binding(Term::Variable(Symbol::new("x")), Term::Constant(Symbol::new("b")));

        assert!(test_binding.test_and_bind(&Term::Variable(Symbol::new("x")), &Term::Constant(Symbol::new("b"))));
        assert!(!test_binding.test_and_bind(&Term::Variable(Symbol::new("x")), &Term::Constant(Symbol::new("wrong"))));
        assert!(test_binding.test_and_bind(&Term::Variable(Symbol::new("new var")), &Term::Constant(Symbol::new("new val"))));

    }
    #[test]
    fn add_test(){
        let mut test_binding = Bindings::new();
        let mut test_hash = HashMap::new();
        test_hash.insert(Term::Variable(Symbol::new("x")), Term::Constant(Symbol::new("a")));
        test_hash.insert(Term::Variable(Symbol::new("y")), Term::Constant(Symbol::new("b")));

        test_binding.add_binding(Term::Variable(Symbol::new("x")), Term::Constant(Symbol::new("a")));
        test_binding.add_binding(Term::Variable(Symbol::new("y")), Term::Constant(Symbol::new("b")));

        assert_eq!(test_binding.bindings, test_hash)
    }
    #[test]
    fn get_test(){
        let mut test_binding = Bindings::new();

        test_binding.add_binding(Term::Variable(Symbol::new("x")), Term::Constant(Symbol::new("a")));
        test_binding.add_binding(Term::Variable(Symbol::new("y")), Term::Constant(Symbol::new("second vec")));

        //let vec1 = vec![Term::Constant(Symbol::new("a")), Term::Constant(Symbol::new("b")), Term::Constant(Symbol::new("c"))];

        assert_eq!(test_binding.get_bindings(&Term::Variable(Symbol::new("y"))), Some(Term::Constant(Symbol::new("second vec"))));

    }
}