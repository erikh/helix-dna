use crate::error::ParseError;
use crate::tokenizer::{tokenize, Token};

/// A parsed intent from natural language input.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    pub verb: String,
    pub args: Vec<Arg>,
}

/// An argument in a parsed intent.
#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Number(i64),
    Word(String),
}

/// Parse a natural language command string into an `Intent`.
pub fn parse(input: &str) -> Result<Intent, ParseError> {
    let tokens = tokenize(input)?;

    let mut verb = None;
    let mut args = Vec::new();

    for token in tokens {
        match token {
            Token::Verb(v) => {
                if verb.is_none() {
                    verb = Some(v);
                } else {
                    args.push(Arg::Word(v));
                }
            }
            Token::Number(n) => args.push(Arg::Number(n)),
            Token::Word(w) => args.push(Arg::Word(w)),
        }
    }

    let verb = verb.ok_or(ParseError::MissingVerb)?;

    Ok(Intent { verb, args })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_2_3() {
        let intent = parse("add 2 3").unwrap();
        assert_eq!(
            intent,
            Intent {
                verb: "add".to_string(),
                args: vec![Arg::Number(2), Arg::Number(3)],
            }
        );
    }

    #[test]
    fn parse_multiply_10_20() {
        let intent = parse("multiply 10 20").unwrap();
        assert_eq!(intent.verb, "multiply");
        assert_eq!(intent.args, vec![Arg::Number(10), Arg::Number(20)]);
    }

    #[test]
    fn parse_with_extra_whitespace() {
        let intent = parse("add   5   7").unwrap();
        assert_eq!(intent.verb, "add");
        assert_eq!(intent.args, vec![Arg::Number(5), Arg::Number(7)]);
    }
}
