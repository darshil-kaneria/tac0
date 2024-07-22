use std::collections::HashMap;

use crate::args;

pub struct Context {
    pub args: args::CommandlineArgs,
    pub typechecker: HashMap<String, String>
}

impl Context {
    pub fn new() -> Context {
        Context {
            args: args::CommandlineArgs::new(),
            typechecker: HashMap::new()
        }
    }
}