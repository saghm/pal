// Large portions of this are borrowed/copied from [https://github.com/nikomatsakis/lalrpop/blob/d17fdd812731e2794e0196bb21f669534f1e963e/lalrpop/src/tok/mod.rs]

use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::Token::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub message: String,
}

#[inline]
fn error<T>(m: String, l: usize) -> Result<T, Error> {
    Err(Error { location: l, message: m })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'input> {
    // Literals
    True,
    False,
    I64(&'input str),
    StringLiteral(&'input str),

    // Identifiers
    Ident(&'input str),

    // Keywords
    Let,
    Print,
    If,
    Else,
    While,
    Return,
    For,
    In,
    Range,
    Step,

    // Types
    Array,
    Boolean,
    Int,
    Str,
    Void,

    // Operators
    Bang,
    DoubleAmp,
    DoubleBars,
    Equal,
    GreaterThan,
    LessThan,
    Minus,
    Percent,
    Plus,
    Slash,
    Star,

    // Punctuation
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    look_ahead: Option<(usize, char)>,
    line_num: usize,
    shift: usize,
}

macro_rules! eof {
    ($x:expr) => {
        match $x {
            Some(v) => v,
            None => return None
        }
    }
}

pub type Spanned<T> = (usize, T, usize);

impl <'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Self {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            look_ahead: None,
            line_num: 1,
            shift: shift,
        };

        t.bump();
        t
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Token<'input>>, Error>> {
        loop {
            return match self.look_ahead {
                // Operators
                Some((idx0, '!')) => {
                    self.bump();
                    Some(Ok((idx0, Bang, idx0 + 1)))
                }
                Some((idx0, '&')) => {
                    match self.bump() {
                        Some((idx1, '&')) => {
                            self.bump();
                            Some(Ok((idx0, DoubleAmp, idx1 + 1)))
                        }
                        Some((_, c)) => Some(error(format!("Unrecognized token: &{}", c), idx0)),
                        None => Some(error(String::from("Unrecognized token: &"), idx0)),
                    }
                }
                Some((idx0, '|')) => {
                    match self.bump() {
                        Some((idx1, '|')) => {
                            self.bump();
                            Some(Ok((idx0, DoubleBars, idx1 + 1)))
                        }
                        Some((_, c)) => Some(error(format!("Unrecognized token: |{}", c), idx0)),
                        None => Some(error(String::from("Unrecognized token: |"), idx0)),
                    }
                }
                Some((idx0, '=')) => {
                    self.bump();
                    Some(Ok((idx0, Equal, idx0 + 1)))
                }
                Some((idx0, '>')) => {
                    self.bump();
                    Some(Ok((idx0, GreaterThan, idx0 + 1)))
                }
                Some((idx0, '<')) => {
                    self.bump();
                    Some(Ok((idx0, LessThan, idx0 + 1)))
                }
                Some((idx0, '-')) => {
                    match self.bump() {
                        Some((_, c)) if c.is_digit(10) => Some(Ok(self.num(idx0))),
                        _ => Some(Ok((idx0, Minus, idx0 + 1))),
                    }
                }
                Some((idx0, '%')) => {
                    self.bump();
                    Some(Ok((idx0, Percent, idx0 + 1)))
                }
                Some((idx0, '+')) => {
                    self.bump();
                    Some(Ok((idx0, Plus, idx0 + 1)))
                }
                Some((idx0, '/')) => {
                    self.bump();
                    Some(Ok((idx0, Slash, idx0 + 1)))
                }
                Some((idx0, '*')) => {
                    self.bump();
                    Some(Ok((idx0, Star, idx0 + 1)))
                }

                // Punctuation
                Some((idx0, ',')) => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0 + 1)))
                }
                Some((idx0, ';')) => {
                    self.bump();
                    Some(Ok((idx0, Semicolon, idx0 + 1)))
                }
                Some((idx0, '(')) => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0 + 1)))
                }
                Some((idx0, ')')) => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0 + 1)))
                }
                Some((idx0, '{')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBrace, idx0 + 1)))
                }
                Some((idx0, '}')) => {
                    self.bump();
                    Some(Ok((idx0, RightBrace, idx0 + 1)))
                }
                Some((idx0, '"')) => {
                    self.bump();
                    Some(self.string_literal(idx0))
                }
                Some((idx0, '[')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBracket, idx0 + 1)))
                }
                Some((idx0, ']')) => {
                    self.bump();
                    Some(Ok((idx0, RightBracket, idx0 + 1)))
                }

                // Number
                Some((idx0, c)) if c.is_digit(10) => Some(Ok(self.num(idx0))),

                // Words
                Some((idx0, c)) if is_identifier_start(c) => Some(Ok(self.identifierish(idx0))),

                // Whitespace
                Some((_, '\n')) => {
                    self.bump();
                    self.line_num += 1;
                    continue;
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }

                // Others
                Some((idx, c)) => Some(error(format!("Unrecognized token: {}", c), idx)),
                None => None,
            }
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.look_ahead = self.chars.next();
        self.look_ahead
    }

    fn string_literal(&mut self, idx0: usize) -> Result<Spanned<Token<'input>>, Error> {
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else {
                c == '"'
            }
        };
        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the '"'
                let text = &self.text[idx0+1..idx1]; // do not include the "" in the str
                Ok((idx0, StringLiteral(text), idx1+1))
            }
            None => {
                error(String::from("Unterminated string literal"), idx0)
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Spanned<Token<'input>> {
        let (start, word, end) = self.word(idx0);

        match word {
            "let" => (start, Let, end),
            "print" => (start, Print, end),
            "true" => (start, True, end),
            "false" => (start, False, end),
            "if" => (start, If, end),
            "else" => (start, Else, end),
            "while" => (start, While, end),
            "array" => (start, Array, end),
            "boolean" => (start, Boolean, end),
            "int" => (start, Int, end),
            "string" => (start, Str, end),
            "void" => (start, Void, end),
            "return" => (start, Return, end),
            "for" => (start, For, end),
            "in" => (start, In, end),
            "range" => (start, Range, end),
            "step" => (start, Step, end),
            _ => (start, Ident(word), end),
        }
    }

    fn num(&mut self, idx0: usize) -> Spanned<Token<'input>> {
        match self.take_while(|c| c.is_digit(10)) {
            Some(end) => (idx0, I64(&self.text[idx0..end]), end),
            None => (idx0, I64(&self.text[idx0..]), self.text.len()),
        }
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize> where F: FnMut(char) -> bool {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize> where F: FnMut(char) -> bool {
        loop {
            match self.look_ahead {
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
                None => return None,
            }
        }
    }
}

impl <'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Token<'input>>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_unshifted() {
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, message })) =>
                Some(Err(Error { location: location + self.shift, message: message })),
            None => None,
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}
