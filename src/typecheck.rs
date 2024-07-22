use std::collections::HashMap;

use crate::context::Context;
use crate::ast;

pub fn typecheck(_ctx: &mut Context, _parse_tree: ast::Program) -> Result<(), String>  {
    // Record the error and the AST node at which it occurred
    let err_map = HashMap::new();
    run(_ctx, _parse_tree, err_map);
    Ok(())
}

// A recursive function to go through the AST and typecheck each node
fn run(_ctx: &mut Context, _parse_tree: ast::Program, _err_map: HashMap<String, String>) {
    match _parse_tree {
        _ => {
            // All the typechecking logic in this match statement
        }
    }
}