use ast::Statement;
use grammar::{parse_Program, parse_Stmt};
use lalrpop_util;
use token::{Error, Token, Tokenizer};

pub type ParseError<'input> = lalrpop_util::ParseError<usize, Token<'input>, Error>;

pub fn parse_program(input: &str) -> Result<Vec<Statement>, ParseError> {
    let tokenizer = Tokenizer::new(input, 0);
    parse_Program(input, tokenizer)
}

pub fn parse_stmt(input: &str) -> Result<Statement, ParseError> {
    let tokenizer = Tokenizer::new(input, 0);
    parse_Stmt(input, tokenizer)
}
