// This is the parser for the calculator. It creates an abstract syntax tree (AST) from the tokens.

use std::process;
use crate::tokeniser;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Branch {
    pub is_num_node: bool,
    pub val: f64, // only set if is_num_node is true
    pub operation: tokeniser::TokenType,
    pub left: Option<Box<Branch>>,  // <-|
    pub right: Option<Box<Branch>>, // <-|-- only if is_num_node is false
}

/* Prints an AST as a text-based tree diagram, debug function */
fn print_ast_level(branch: &Branch, depth: u32) {
    for _ in 0..depth { print!("  "); }
    if branch.is_num_node {
        println!("-> Num: {}", branch.val);
        return
    }
    println!("-> Left node - {}:", &branch.left.as_ref().unwrap().operation);
    print_ast_level(branch.left.as_ref().unwrap().as_ref(), depth + 1);
    for _ in 0..depth { print!("  "); }
    println!("-> Right node - {}:", &branch.right.as_ref().unwrap().operation);
    print_ast_level(branch.right.as_ref().unwrap().as_ref(), depth + 1);
}

pub fn print_ast(root: &Branch) {
    println!("-> {}", root.operation);
    print_ast_level(root, 1);
}

/* parses a single branch recursively */
fn parse_branch(tokens: &Vec<tokeniser::Token>, priorities: &mut HashMap<tokeniser::TokenType, u8>) -> Box<Branch> {
    // if the whole node is one single token (must be of type NUM), then return a num node
    if tokens.len() == 1 {
        if tokens[0].ttype == tokeniser::TokenType::NUM {
            return Box::new(Branch {is_num_node: true, val: tokens[0].val, operation: tokeniser::TokenType::NUM, left: None, right: None})
        } else {
            println!("Failed to parse and create AST.");
            process::exit(1);
        }
    }
    // find the token of the highest priority (lower priority values are higher priority)
    let mut max_priority_token_idx = 0;
    let mut max_priority = 0;
    let tokens_len = tokens.len();
    for idx in 0..tokens_len {
        if tokens[idx].ttype != tokeniser::TokenType::NUM {
            let priority: u8 = priorities[&tokens[idx].ttype];
            if priority > max_priority {
                max_priority = priority;
                max_priority_token_idx = idx;
            }
        }
    }
    /* create a vector of the nodes on either side of this max priority token, and pass them recursively into parse_branch. 
     * take the output branch and form this branch with either side.
     * You may wonder why I don't just slice the vector. It's because a slice is read-only, and I need this
     * to be mutable.
     */
    // First on the left side
    let mut left_tokens: Vec<tokeniser::Token> = Vec::new();
    for idx in 0..max_priority_token_idx {
        left_tokens.push(tokens[idx]);
    }
    let left_branch = parse_branch(&left_tokens, priorities);
    // now the right side
    let mut right_tokens: Vec<tokeniser::Token> = Vec::new();
    max_priority_token_idx += 1;
    for idx in max_priority_token_idx..tokens_len {
        right_tokens.push(tokens[idx]);
    }
    let right_branch = parse_branch(&right_tokens, priorities);
    // Now it can finally create this branch, and return it in a box
    Box::new(Branch {is_num_node: false, val: 0.0, operation: tokens[max_priority_token_idx - 1].ttype, left: Some(left_branch), right: Some(right_branch)})
}

/* creates the whole AST - sets up the priority hashmap then starts the recursive parsing of
 * branches.
 */
pub fn parse(tokens: &mut Vec<tokeniser::Token>) -> Branch {
    let mut priorities_map: HashMap<tokeniser::TokenType, u8> = HashMap::from([
        (tokeniser::TokenType::POW, 1),
        (tokeniser::TokenType::MUL, 2),
        (tokeniser::TokenType::DIV, 2),
        (tokeniser::TokenType::SUB, 3),
        (tokeniser::TokenType::ADD, 3),
    ]);
    *parse_branch(&tokens, &mut priorities_map)
}
