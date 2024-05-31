use std::{env, process};
use simplescript::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("main.ss");
    let filename = args.get(1).unwrap_or_else(|| &default);
    let mut lexer = Lexer::open(&filename)
        .unwrap_or_else(|err| {
            eprintln!("{filename}: {}", err.to_string());
            process::exit(71);
        });

    let (length, string) = lexer.read().unwrap();

    println!("bytes read: {}", length);
    
    println!("{string}");
}
