use std::{env, process};
use simplescript::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("main.simple");
    let filename = args.get(1).unwrap_or_else(|| &default);
    println!("opening file: {filename}");
    let mut lexer = Lexer::open(&filename)
        .unwrap_or_else(|err| {
            eprintln!("{filename}: {}", err.to_string());
            process::exit(71);
        });

    /*
    let (length, string) = lexer.read().unwrap();
    
    println!("bytes read: {}", length);
    
    println!("{string}");
    */

    println!("------ tokens -----");
    let tokens = lexer.lex().unwrap();
    for token in tokens.clone() {
        println!("{token:?}")
    }

    println!("--- expressions ---");
    let parser = Parser::new(tokens);
    let expressions = parser.parse();
    for expression in expressions {
        println!("{expression:?}");
    }
}
