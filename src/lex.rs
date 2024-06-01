use crate::token_types::*;
use std::error::Error;
use std::str;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read}
};

pub struct Lexer<T: ?Sized + Read> {
    reader: BufReader<T>
}

const BUF_SIZE: usize = 1024;

impl<T: Sized + Read> Lexer<T> {
    pub fn new(readable: T) -> Lexer<T> {
        Lexer {
            reader: BufReader::with_capacity(BUF_SIZE, readable)
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

        #[derive(Debug, PartialEq)]
        enum TokenType {
            None,
            Ident,
            Other,
        }

        let (_, string) = self.read()?;
        let len = string.len();

        let mut substr_start = 0;
        let mut substr = &string[0..0];
        let mut token_type = TokenType::None;

        let mut index: usize = 0;
        while index < len {
            let mut inc = true;
            let char = string[index..index+1].chars().next().unwrap();
            if char.is_whitespace() {
                
            }
            let is_ident = TokenEnum::is_ident_char(char);
            match token_type {
                TokenType::Ident => {
                    if is_ident {
                        substr = &string[substr_start..index+1];
                    } else {
                        tokens.push(TokenEnum::from(substr));
                        substr_start = index;
                        substr = &string[index..index];
                        token_type = TokenType::None;
                        inc = false;
                    }
                }
                TokenType::Other => {
                    if is_ident || substr.len() >= MAX_PUNCTUATOR_LEN {
                        let punct = Punctuator::from(substr);
                        if punct != Punctuator::None {
                            tokens.push(TokenEnum::from(punct));
                        }
                        substr_start = index;
                        substr = &string[index..index];
                        token_type = TokenType::None;
                        inc = false;
                    } else if char.is_whitespace() {
                        substr_start = index + 1;
                        substr = &string[index+1..index+1];
                        token_type = TokenType::None;
                        inc = false;
                        if char.is_whitespace() {
                            inc = true;
                        }
                    }
                    else {
                        substr = &string[substr_start..index+1];
                    }
                }
                TokenType::None => {
                    token_type = match is_ident {
                        true  => TokenType::Ident,
                        false => TokenType::Other,
                    };
                    inc = false;
                }
            }
            // println!("at index {index:4} {char:?} {is_ident:5} {substr:?} {token_type:?}");

            if inc { index += 1; }
        }
        match token_type {
            TokenType::Ident => { tokens.push(TokenEnum::from(substr)) }
            TokenType::Other => { tokens.push(TokenEnum::from(Punctuator::from(substr))) }
            TokenType::None => ()
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
