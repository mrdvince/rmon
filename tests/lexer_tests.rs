use rmon::lexer::Lexer;
use rmon::token::{Token, Tokens};

#[test]
fn test_next_token() {
    let input = "let five = 5;\
            \nlet ten = 10;\
            \nlet add = fn(x, y) {\
            \n  x + y;\
            \n};\
            \nlet result = add(five, ten);\
            \n!-/*5;
            \n5 < 10 > 5;
            \n";

    let tests = [
        (Tokens::LET.as_str(), "let"),
        (Tokens::IDENT.as_str(), "five"),
        (Tokens::ASSIGN.as_str(), "="),
        (Tokens::INT.as_str(), "5"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::LET.as_str(), "let"),
        (Tokens::IDENT.as_str(), "ten"),
        (Tokens::ASSIGN.as_str(), "="),
        (Tokens::INT.as_str(), "10"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::LET.as_str(), "let"),
        (Tokens::IDENT.as_str(), "add"),
        (Tokens::ASSIGN.as_str(), "="),
        (Tokens::FUNCTION.as_str(), "fn"),
        (Tokens::LPAREN.as_str(), "("),
        (Tokens::IDENT.as_str(), "x"),
        (Tokens::COMMA.as_str(), ","),
        (Tokens::IDENT.as_str(), "y"),
        (Tokens::RPAREN.as_str(), ")"),
        (Tokens::LBRACE.as_str(), "{"),
        (Tokens::IDENT.as_str(), "x"),
        (Tokens::PLUS.as_str(), "+"),
        (Tokens::IDENT.as_str(), "y"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::RBRACE.as_str(), "}"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::LET.as_str(), "let"),
        (Tokens::IDENT.as_str(), "result"),
        (Tokens::ASSIGN.as_str(), "="),
        (Tokens::IDENT.as_str(), "add"),
        (Tokens::LPAREN.as_str(), "("),
        (Tokens::IDENT.as_str(), "five"),
        (Tokens::COMMA.as_str(), ","),
        (Tokens::IDENT.as_str(), "ten"),
        (Tokens::RPAREN.as_str(), ")"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::BANG.as_str(), "!"),
        (Tokens::MINUS.as_str(), "-"),
        (Tokens::SLASH.as_str(), "/"),
        (Tokens::ASTERISK.as_str(), "*"),
        (Tokens::INT.as_str(), "5"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::INT.as_str(), "5"),
        (Tokens::LT.as_str(), "<"),
        (Tokens::INT.as_str(), "10"),
        (Tokens::GT.as_str(), ">"),
        (Tokens::INT.as_str(), "5"),
        (Tokens::SEMICOLON.as_str(), ";"),
        (Tokens::EOF.as_str(), ""),
    ];

    let mut l = Lexer::new(input);
    println!("{l:?}");
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
