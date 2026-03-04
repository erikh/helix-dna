use pest::Parser;
use pest_derive::Parser;

use crate::error::TokenizeError;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CommandParser;

/// A token extracted from the input.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Verb(String),
    Number(i64),
    Word(String),
}

/// Tokenize a natural language command string into a sequence of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let pairs = CommandParser::parse(Rule::command, input)
        .map_err(|e| TokenizeError::PestError(e.to_string()))?;

    let mut tokens = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::verb => {
                    tokens.push(Token::Verb(inner.as_str().to_lowercase()));
                }
                Rule::argument => {
                    let arg = inner.into_inner().next().unwrap();
                    match arg.as_rule() {
                        Rule::number => {
                            let n: i64 = arg
                                .as_str()
                                .parse()
                                .map_err(|e: std::num::ParseIntError| {
                                    TokenizeError::PestError(e.to_string())
                                })?;
                            tokens.push(Token::Number(n));
                        }
                        Rule::word => {
                            tokens.push(Token::Word(arg.as_str().to_lowercase()));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(tokens)
}
