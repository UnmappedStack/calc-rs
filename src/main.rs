// A simple calculator in Rust in the form of an interpeter.

use std::io;
use std::io::Write;

mod tokeniser;
mod parser;
mod interpreter;

fn main() {
    print!("Enter an equation: ");
    io::stdout().flush().unwrap();
    let mut eq = String::new();
    io::stdin()
        .read_line(&mut eq)
        .expect("Couldn't read user input.");
    eq = eq.trim().to_string();
    println!("Tokenising...");
    let mut tokens: Vec<tokeniser::Token> = tokeniser::tokenise(&eq);
    println!("Tokenised, these are the tokens:");
    println!("{:?}", tokens);
    println!("Parsing & creating an AST...");
    let mut ast: parser::Branch = parser::parse(&mut tokens);
    println!("Done, AST generated:");
    parser::print_ast(&ast);
    println!("Interpreting...");
    let result: f64 = interpreter::interpret(&mut ast);
    println!("Done, result is {}", result);
}
