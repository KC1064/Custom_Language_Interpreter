use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.kr>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    // Read the file contents
    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    // Initialize the lexer, parser, and interpreter
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let expr = match parser.parse() {
        Some(expression) => expression,
        None => {
            eprintln!("Failed to parse the input");
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    if let Some(result) = interpreter.interpret(expr) {
        println!("Result: {}", result);
    } else {
        eprintln!("Failed to interpret the expression");
    }
}
