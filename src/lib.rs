pub use rcket_macros::{Lex, Node};

pub trait Node: Sized {
    type Token;
    type Output;
    fn parse_one(tokens: &[Self::Token]) -> Option<(Self::Output, &[Self::Token])>;

    fn parse(tokens: &[Self::Token]) -> Option<Self::Output> {
        if let Some((result, rest)) = Self::parse_infix(tokens, 0) {
            if rest.is_empty() {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_infix(
        tokens: &[Self::Token],
        min_prec: u32,
    ) -> Option<(Self::Output, &[Self::Token])> {
        Self::parse_one(tokens)
    }

    fn operator_precedence(_op: &Self::Token) -> u32 {
        0
    }
}

pub trait Lex: Sized {
    fn lex_one(input: &str) -> Option<(Self, &str)>;

    fn lex(input: &str) -> Vec<Self> {
        let mut tokens = Vec::new();
        let mut remaining = input;
        loop {
            remaining = remaining.trim_start();
            if remaining.is_empty() {
                break;
            }
            match Self::lex_one(remaining) {
                Some((token, rest)) => {
                    tokens.push(token);
                    remaining = rest;
                }
                None => {
                    let skip_to = remaining
                        .char_indices()
                        .nth(1)
                        .map(|(index, _)| index)
                        .unwrap_or(remaining.len());
                    remaining = &remaining[skip_to..];
                }
            }
        }
        tokens
    }
}
