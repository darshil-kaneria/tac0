use crate::args;

pub struct Context {
    pub args: args::CommandlineArgs,
}

impl Context {
    pub fn new() -> Context {
        Context {
            args: args::CommandlineArgs::new(),
        }
    }
}