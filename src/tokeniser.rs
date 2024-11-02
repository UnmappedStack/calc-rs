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
pub fn tokenise(txt: &String)->Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let len = txt.len();
    while i < len {
        let c: char = txt.chars().nth(i).unwrap();
        match c {
            ' ' => {i += 1; continue},
            '^' => tokens.push(Token {ttype: TokenType::POW, val: 0.0}),
            '+' => tokens.push(Token {ttype: TokenType::ADD, val: 0.0}),
            '*'|'x'|'X' => tokens.push(Token {ttype: TokenType::MUL, val: 0.0}),
            '/' => tokens.push(Token {ttype: TokenType::DIV, val: 0.0}),
            '-' => tokens.push(Token {ttype: TokenType::SUB, val: 0.0}),
            '0'..='9' => {
                let mut num_str: String = c.to_string();
                i += 1;
                if i != len {
                    let mut this_char: char = txt.chars().nth(i).unwrap();
                    while (this_char >= '0' && this_char <= '9') || this_char == '.' {
                        num_str.push(this_char);
                        i += 1;
                        if i == len { break }
                        this_char = txt.chars().nth(i).unwrap();
                    }
                }
                tokens.push(Token {ttype: TokenType::NUM, val: num_str.parse::<f64>().unwrap()});
                continue
            },
            _ => {
                println!("Unknown symbol: `{}`, Exiting.", c);
                process::exit(1);
            }
        }
        i += 1;
    }
    tokens
}
