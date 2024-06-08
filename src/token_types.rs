pub struct Token {
    pub token_enum: TokenEnum,
    // pub line: f32,
    // pub col: f32,
}

#[derive(Debug)]
pub enum TokenEnum {
    Ident(String),
    Keyword(Keyword),
    Punctuator(Punctuator),
    Operator(Operator),
    StringLiteral(String),
    NumLiteral(f64),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

impl TokenEnum {
    pub fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '$' || c == '_'
    }

    pub fn is_ident(token: &str) -> bool {
        let mut is_first = true;
        for c in token.chars() {
            if is_first && c.is_numeric() {
                return false;
            }
            if !TokenEnum::is_ident_char(c) {
                return false;
            }
            is_first = false;
        }
        return true;
    }
}

impl From<String> for TokenEnum {
    fn from(value: String) -> TokenEnum {
        TokenEnum::Ident(value)
    }
}


impl From<&str> for TokenEnum {
    fn from(value: &str) -> TokenEnum {
        TokenEnum::Ident(String::from(value))
    }
}

impl From<Keyword> for TokenEnum {
    fn from(value: Keyword) -> TokenEnum {
        TokenEnum::Keyword(value)
    }
}

impl From<Punctuator> for TokenEnum {
    fn from(value: Punctuator) -> TokenEnum {
        TokenEnum::Punctuator(value)
    }
}

impl From<Operator> for TokenEnum {
    fn from(value: Operator) -> TokenEnum {
        TokenEnum::Operator(value)
    }
}


impl From<String> for Keyword {
    fn from(value: String) -> Keyword {
        return match value.as_str() {
            "if"       => Keyword::If,
            "else"     => Keyword::Else,
            "for"      => Keyword::For,
            "while"    => Keyword::While,
            "break"    => Keyword::Break,
            "continue" => Keyword::Continue,
            "function" => Keyword::Function,
            "return"   => Keyword::Return,
            _          => Keyword::None,
        }
    }
}

impl<T: ToString> From<T> for Punctuator {
    fn from(value: T) -> Punctuator {
        return match value.to_string().as_str() {
            "(" => Punctuator::LParen,
            ")" => Punctuator::RParen,
            "[" => Punctuator::LBracket,
            "]" => Punctuator::RBracket,
            "{" => Punctuator::LBrace,
            "}" => Punctuator::RBrace,
            ";" => Punctuator::Semicolon,
            "," => Punctuator::Comma,
            _ => Punctuator::None,
        }
    }
}

impl From<String> for Operator {
    fn from(value: String) -> Operator {
        return match value.as_str() {
            "+"  => Operator::Plus,
            "++" => Operator::DoublePlus,
            "-"  => Operator::Minus,
            "--" => Operator::DoubleMinus,
            "*"  => Operator::Asterisk,
            "/"  => Operator::Slash,
            "~"  => Operator::Tilde,
            "!"  => Operator::Exclamation,
            "&"  => Operator::Ampersand,
            "&&" => Operator::DoubleAmp,
            "|"  => Operator::Pipe,
            "||" => Operator::DoublePipe,
            "^"  => Operator::Caret,
            "^^" => Operator::DoubleCaret,
            "="  => Operator::Equal,
            "==" => Operator::DoubleEqual,
            "+=" => Operator::PlusEqual,
            "-=" => Operator::MinusEqual,
            "*=" => Operator::StarEqual,
            "/=" => Operator::SlashEqual,
            _    => Operator::None,
        }
    }
}
