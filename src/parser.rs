use ast::Statement;
use grammar::parse_Stmt;
use lalrpop_util;
use token::{Error, Token, Tokenizer};


pub type ParseError<'input> = lalrpop_util::ParseError<usize, Token<'input>, Error>;

pub fn parse_stmt<'input>(input: &'input str) -> Result<Statement, ParseError<'input>> {
    let tokenizer = Tokenizer::new(input, 0);
    parse_Stmt(input, tokenizer)
}
