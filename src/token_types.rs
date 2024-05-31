pub struct Token {
    pub token_enum: TokenEnum,
    pub line: f32,
    pub col: f32,
}

pub enum TokenEnum {
    Ident(String),
    Keyword(Keyword),
    Punctuator(Punctuator),
    Operator(Operator),
    StringLiteral(String),
    NumLiteral(f64),
}

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

pub enum Punctuator {
    None,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Semicolon,
    EOF,
}

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
    pub fn is_varname_char(c: char) -> bool {
           ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || (c == '$' || c == '_')
    }

    pub fn is_varname(token: &str) -> bool {
        for c in token.chars() {
            if !TokenEnum::is_varname_char(c) {
                return false;
            }
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

impl From<String> for Punctuator {
    fn from(value: String) -> Punctuator {
        return match value.as_str() {
            "(" => Punctuator::LParen,
            ")" => Punctuator::RParen,
            "[" => Punctuator::LBracket,
            "]" => Punctuator::RBracket,
            "{" => Punctuator::LBrace,
            "}" => Punctuator::RBrace,
            ";" => Punctuator::Semicolon,
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
