use std::{collections::HashMap, hash::Hash};

use parser::ast::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct UserProc {
    arg_names: HashMap<String, usize>,
    body: Vec<Node>,
    quote_start: Option<usize>,
}

impl Hash for UserProc {
    fn hash<H: ~const std::hash::Hasher>(&self, state: &mut H) {
        for arg in self.arg_names.iter() {
            arg.hash(state)
        }
        self.body.hash(state);
    }
}

impl UserProc {
    pub fn new(args: Vec<String>, body: Vec<Node>) -> UserProc {
        let mut res = UserProc {
            arg_names: HashMap::new(),
            body,
            quote_start: None,
        };

        for (idx, arg) in args.into_iter().enumerate() {
            res.arg_names.insert(arg, idx);
        }
        res
    }

    pub fn quote_starts_at(mut self, idx: usize) -> UserProc {
        self.quote_start = Some(idx);
        self
    }

    pub fn get_arity(&self) -> usize {
        self.arg_names.len()
    }

    pub fn get_body(&self) -> &[Node] {
        &self.body
    }

    pub fn get_args(&self) -> &HashMap<String, usize> {
        &self.arg_names
    }

    pub fn is_quote_list_result(&self) -> Option<usize> {
        self.quote_start
    }
}
