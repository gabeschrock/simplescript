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

    pub fn next(&mut self) -> Result<TokenEnum, Box<dyn Error>> {
        let _bytes_read = self.read()?;
        Ok(TokenEnum::from(""))
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
