use rmon::lexer::Lexer;
use rmon::token::Tokens;

#[test]
fn test_next_token() {
    let input = "=+(){},;";
    let tests = [
        (Tokens::ASSIGN.as_str(), "="),
        (Tokens::PLUS.as_str(), "+"),
        (Tokens::LPAREN.as_str(), "("),
        (Tokens::RPAREN.as_str(), ")"),
        (Tokens::LBRACE.as_str(), "{"),
        (Tokens::RBRACE.as_str(), "}"),
        (Tokens::COMMA.as_str(), ","),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::EOF.as_str(), ""),
    ];
    let mut l = Lexer::new(input);
    for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(
            tok.r#type, *expected_type,
            "tests[{}] - tokentype wrong. expected={}, got={}",
            i, expected_type, tok.r#type
        );
        assert_eq!(
            tok.literal, *expected_literal,
            "tests[{}] - literal wrong. expected={}, got={}",
            i, expected_literal, tok.literal
        );
    }
}
