use rcket::{lexer::lex, lexer_types::*};

#[test]
fn lex_float() {
    let input = "5.8";
    let target = vec![Token::Literal(Literal::Float(5.8))];
    let output = lex(input);
    assert_eq!(target, output)
}
