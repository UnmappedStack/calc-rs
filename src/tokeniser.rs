// Tokenises an equation. This is the first step of the process.

use std::fmt;
use std::process;

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
pub enum TokenType {
    POW,
    ADD,
    MUL,
    DIV,
    SUB,
    NUM
}

#[derive(Copy,Clone,Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub val: f64,
}

// Maps a TokenType to a text string and displays it.
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::POW => write!(f, "POW"),
            TokenType::ADD => write!(f, "ADD"),
            TokenType::MUL => write!(f, "MUL"),
            TokenType::DIV => write!(f, "DIV"),
            TokenType::SUB => write!(f, "SUB"),
            TokenType::NUM => write!(f, "NUM"),
        }
    }
}

// Tokenises a string to a list of tokens.
pub fn tokenise(mut txt: &str)->Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while txt.len() > 0 {
        let c: char = txt.chars().next().unwrap();
        match c {
            ' ' => {txt = &txt[1..]; continue},
            '^' => tokens.push(Token {ttype: TokenType::POW, val: 0.0}),
            '+' => tokens.push(Token {ttype: TokenType::ADD, val: 0.0}),
            '*'|'x'|'X' => tokens.push(Token {ttype: TokenType::MUL, val: 0.0}),
            '/' => tokens.push(Token {ttype: TokenType::DIV, val: 0.0}),
            '-' => tokens.push(Token {ttype: TokenType::SUB, val: 0.0}),
            '0'..='9' => {
                let mut this_char: char = txt.chars().next().unwrap();
                let mut i = 0;
                let whole = txt;
                while ((this_char >= '0' && this_char <= '9') || this_char == '.') && txt.len() > 0 {
                    if txt.len() == 1 { i += 1; break }
                    txt = &txt[1..];
                    this_char = txt.chars().next().unwrap();
                    i += 1;
                }
                println!("i = {}", i);
                let num_str = &whole[0..i];
                println!("num_str: {}", num_str);
                tokens.push(Token {ttype: TokenType::NUM, val: num_str.parse::<f64>().unwrap()});
                if txt.len() == 1 { break }
                continue
            },
            _ => {
                println!("Unknown symbol: `{}`, Exiting.", c);
                process::exit(1);
            }
        }
        txt = &txt[1..];
    }
    tokens
}
