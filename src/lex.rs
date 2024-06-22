use crate::token_types::*;
use std::vec;
use std::{str, error::Error};

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

    pub fn lex(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        const fn max(a: usize, b: usize) -> usize {
            return match a >= b {
                true => a,
                false => b,
            };
        }
        const fn min(a: usize, b: usize) -> usize {
            return match a <= b {
                true => a,
                false => b,
            };
        }
        fn split_symbols(string: &str) -> Vec<Token> {
            const MAX_LEN: usize = max(MAX_OPERATOR_LEN, MAX_PUNCTUATOR_LEN);

            if string.len() == 0 {
                return vec![];
            }
            let mut tokens = vec![];
            let mut indices = (0, min(string.len(), MAX_LEN));


            while indices.1 != indices.0 {
                println!("indices: ({}, {})", indices.0, indices.1);
                let substr = &string[indices.0..indices.1];
                println!("substr ({}..{}): {substr}", indices.0, indices.1);

                let punct = Punctuator::from(substr);
                if punct != Punctuator::None {
                    println!("punctuator: {punct:?}");
                    tokens.push(Token::from(punct));
                    println!("min({}, {})", string.len(), indices.1 + MAX_LEN);
                    indices = (indices.1, min(string.len(), indices.1 + MAX_LEN));
                    continue;
                }

                let op = Operator::from(substr);

                if op != Operator::None {
                    println!("operator: {op:?}");
                    tokens.push(Token::from(op));
                    println!("min({}, {})", string.len(), indices.1 + MAX_LEN);
                    indices = (indices.1, min(string.len(), indices.1 + MAX_LEN));
                    continue;
                }

                indices.1 -= 1;
            }

            tokens
        }

        let mut tokens: Vec<Token> = vec![];

        let (_, string) = self.read()?;
        let len = string.len();
        let width = len.to_string().len();

        let mut substr_start: usize = 0;
        let mut substr = &string[0..0];

        let mut index = 0;

        println!("code: {string:?}");


        while index < len {
            let char = string[index..index+1].chars().next().unwrap();
            println!("main:   index {index:width$} of {len}: {char:?}");
            if char.is_whitespace() {
                index += 1;

                if substr.len() > 0 {
                    tokens.push(Token::from(substr));
                }

                substr_start = index;
                substr = &string[substr_start..substr_start];
                continue;
            }

            if Token::is_ident_char(char) {
                while index < len {
                    println!("ident:  index {index:width$} of {len}: {substr:?}");

                    substr = &string[substr_start..index];

                    let char = string[index..index+1].chars().next().unwrap();
                    if char.is_whitespace() {
                        println!("ident:  end   {index:width$} of {len}: {substr:?}");
                        let keyword = Keyword::from(substr);
                        if keyword != Keyword::None {
                            tokens.push(Token::Keyword(keyword));
                        } else {
                            tokens.push(Token::from(substr));
                        }
                        substr_start = index;
                        substr = &string[substr_start..substr_start];
                        break;
                    }
                    
                    if !Token::is_ident_char(char) {
                        println!("ident:  end   {index:width$} of {len}: {substr:?}");
                        let keyword = Keyword::from(substr);
                        if keyword != Keyword::None {
                            tokens.push(Token::Keyword(keyword));
                        } else {
                            tokens.push(Token::from(substr));
                        }
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

                substr = &string[substr_start..index];

                let char = string[index..index+1].chars().next().unwrap();
                if char.is_whitespace() {
                    println!("symbol: endw  {index:width$} of {len}: {substr:?}");
                    tokens.append(&mut split_symbols(substr));
                    substr_start = index;
                    substr = &string[substr_start..substr_start];
                    break;
                }
                
                if Token::is_ident_char(char) {
                    println!("symbol: end   {index:width$} of {len}: {substr:?}");
                    tokens.append(&mut split_symbols(substr));
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
