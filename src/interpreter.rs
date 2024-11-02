use crate::parser;
use crate::tokeniser;

fn interpret_branch(branch: &mut parser::Branch) {
    if branch.is_num_node { return }
    // if either side is an equation and not a number, recursively solve for it
    if !branch.left.as_mut().unwrap().is_num_node {
        interpret_branch((*branch.left.as_mut().unwrap()).as_mut());
    }
    if !branch.right.as_mut().unwrap().is_num_node {
        interpret_branch((*branch.right.as_mut().unwrap()).as_mut());
    }
    // now that everything left is a number, just solve the simplified equation :)
    match branch.operation {
        tokeniser::TokenType::ADD => branch.val = branch.left.as_mut().unwrap().val + branch.right.as_mut().unwrap().val,
        tokeniser::TokenType::SUB => branch.val = branch.left.as_mut().unwrap().val - branch.right.as_mut().unwrap().val,
        tokeniser::TokenType::MUL => branch.val = branch.left.as_mut().unwrap().val * branch.right.as_mut().unwrap().val,
        tokeniser::TokenType::DIV => branch.val = branch.left.as_mut().unwrap().val / branch.right.as_mut().unwrap().val,
        tokeniser::TokenType::POW => {
            let left: f64 = branch.left.as_mut().unwrap().val;
            branch.val = left.powf(branch.right.as_mut().unwrap().val);
        },
        _ => panic!("Something went wrong interpreting this."),
    }
    branch.is_num_node = true;
}

/* Interprets the AST to evaluate the final expression. */
pub fn interpret(ast: &mut parser::Branch) -> f64 {
    interpret_branch(ast);
    ast.val
}
