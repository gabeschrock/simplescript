#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Keyword(Keyword),
    Punctuator(Punctuator),
    Operator(Operator),
    StringLiteral(String),
    NumLiteral(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    None,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Function,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuator {
    None,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    None,
    Plus,
    DoublePlus,
    Minus,
    DoubleMinus,
    Asterisk,
    Slash,
    Tilde,
    Exclamation,
    Ampersand,
    DoubleAmp,
    Pipe,
    DoublePipe,
    Caret,
    DoubleCaret,
    Equal,
    DoubleEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

pub const MAX_KEYWORD_LEN: usize = 8;
pub const MAX_PUNCTUATOR_LEN: usize = 1;
pub const MAX_OPERATOR_LEN: usize = 2;

impl Token {
    pub fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '$' || c == '_'
    }

    pub fn is_ident(token: &str) -> bool {
        let mut is_first = true;
        for c in token.chars() {
            if is_first && c.is_numeric() {
                return false;
            }
            if !Token::is_ident_char(c) {
                return false;
            }
            is_first = false;
        }
        return true;
    }
}

impl From<String> for Token {
    fn from(value: String) -> Token {
        Token::Ident(value)
    }
}


impl From<&str> for Token {
    fn from(value: &str) -> Token {
        Token::Ident(String::from(value))
    }
}

impl From<Keyword> for Token {
    fn from(value: Keyword) -> Token {
        Token::Keyword(value)
    }
}

impl From<Punctuator> for Token {
    fn from(value: Punctuator) -> Token {
        Token::Punctuator(value)
    }
}

impl From<Operator> for Token {
    fn from(value: Operator) -> Token {
        Token::Operator(value)
    }
}


impl<T: ToString> From<T> for Keyword {
    fn from(value: T) -> Keyword {
        use Keyword::*;
        return match value.to_string().as_str() {
            "if"       => If,
            "else"     => Else,
            "for"      => For,
            "while"    => While,
            "break"    => Break,
            "continue" => Continue,
            "function" => Function,
            "return"   => Return,
            _          => Keyword::None,
        }
    }
}

impl<T: ToString> From<T> for Punctuator {
    fn from(value: T) -> Punctuator {
        use Punctuator::*;
        return match value.to_string().as_str() {
            "(" => LParen,
            ")" => RParen,
            "[" => LBracket,
            "]" => RBracket,
            "{" => LBrace,
            "}" => RBrace,
            ";" => Semicolon,
            "," => Comma,
            _ => Punctuator::None,
        }
    }
}

impl<T: ToString> From<T> for Operator {
    fn from(value: T) -> Operator {
        use Operator::*;
        return match value.to_string().as_str() {
            "+"  => Plus,
            "++" => DoublePlus,
            "-"  => Minus,
            "--" => DoubleMinus,
            "*"  => Asterisk,
            "/"  => Slash,
            "~"  => Tilde,
            "!"  => Exclamation,
            "&"  => Ampersand,
            "&&" => DoubleAmp,
            "|"  => Pipe,
            "||" => DoublePipe,
            "^"  => Caret,
            "^^" => DoubleCaret,
            "="  => Equal,
            "==" => DoubleEqual,
            "+=" => PlusEqual,
            "-=" => MinusEqual,
            "*=" => StarEqual,
            "/=" => SlashEqual,
            _    => None,
        }
    }
}
