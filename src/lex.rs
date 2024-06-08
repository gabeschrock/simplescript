use crate::token_types::*;
use std::cmp::max;
use std::error::Error;
use std::str;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read}
};

pub struct Lexer<T: Sized + Read> {
    reader: BufReader<T>
}

const BUF_SIZE: usize = 1024;

impl<T: Sized + Read> Lexer<T> {
    pub fn new(reader: T) -> Lexer<T> {
        Lexer {
            reader: BufReader::new(reader)
        }
    }

    pub fn read(&mut self) -> Result<(usize, &str), Box<dyn Error>> {
        let data = self.reader.fill_buf()?;
        let len = data.len();
        let string = str::from_utf8(data)?;
        Ok((len, string))
    }

    pub fn lex(&mut self) -> Result<Vec<TokenEnum>, Box<dyn Error>> {
        let mut tokens: Vec<TokenEnum> = vec![];

        let (_, string) = self.read()?;
        let len = string.len();
        let width = len.to_string().len();

        let mut substr_start = 0;
        let mut substr = &string[0..0];

        let mut index = 0;

        println!("code: {string:?}");


        while index < len {
            let char = string[index..index+1].chars().next().unwrap();
            println!("main:   index {index:width$} of {len}: {char:?}");
            if char.is_whitespace() {
                index += 1;

                if substr.len() > 0 {
                    tokens.push(TokenEnum::from(substr));
                }

                substr_start = index;
                substr = &string[substr_start..substr_start];
                continue;
            }

            if TokenEnum::is_ident_char(char) {
                while index < len {
                    println!("ident:  index {index:width$} of {len}: {substr:?}");

                    substr = &string[substr_start..index];

                    let char = string[index..index+1].chars().next().unwrap();
                    if char.is_whitespace() {
                        println!("ident:  end   {index:width$} of {len}: {substr:?}");
                        tokens.push(TokenEnum::from(substr));
                        substr_start = index;
                        substr = &string[substr_start..substr_start];
                        break;
                    }
                    
                    if !TokenEnum::is_ident_char(char) {
                        println!("ident:  end   {index:width$} of {len}: {substr:?}");
                        tokens.push(TokenEnum::from(substr));
                        break;
                    }

                    index += 1;
                }
                continue;
            }

            substr_start = index;
            substr = &string[substr_start..substr_start];

            while index < len {
                println!("symbol: index {index:width$} of {len}: {substr:?}");

                let max_len = max(MAX_PUNCTUATOR_LEN, MAX_OPERATOR_LEN);
                substr = &string[substr_start..index];

                let char = string[index..index+1].chars().next().unwrap();
                if char.is_whitespace() {
                    println!("symbol: endw  {index:width$} of {len}: {substr:?}");
                    tokens.push(TokenEnum::from(substr));
                    substr_start = index;
                    substr = &string[substr_start..substr_start];
                    break;
                }

                /* 
                if substr.len() < max_len {
                    tokens.push(TokenEnum::from(substr));
                    substr_start = index;
                    substr = &string[substr_start..substr_start];
                    break;
                }
                */
                
                if TokenEnum::is_ident_char(char) {
                    println!("symbol: end   {index:width$} of {len}: {substr:?}");
                    tokens.push(TokenEnum::from(substr));
                    substr_start = index;
                    substr = &string[substr_start..substr_start];
                    break;
                }

                index += 1;
            }
        }
        Ok(tokens)
    }
}

impl Lexer<File> {
    pub fn open(path: &str) -> Result<Lexer<File>, io::Error> {
        let file = File::open(path)?;
        Ok(Lexer {
            reader: BufReader::with_capacity(BUF_SIZE, file)
        })
    }
}

impl<T: Sized + Read> From<T> for Lexer<T> {
    fn from(value: T) -> Lexer<T> {
        Lexer {
            reader: BufReader::with_capacity(BUF_SIZE, value)
        }
    }
}
