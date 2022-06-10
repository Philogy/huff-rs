use huff_lexer::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn lexes_arg_calls() {
    let source = r#"
    #define macro TRANSFER_TAKE_FROM(error) = takes(3) returns (3) {
        dup2
        [BALANCE_LOCATION] LOAD_ELEMENT_FROM_KEYS(0x00)
        dup1
        dup3
        gt
        <error> jumpi
    }
    "#;

    // Parse tokens
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eat Tokens
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // #define
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // macro keyword
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // macro name
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // error keyword
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // equals
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // takes
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // 3
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // returns
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // 3
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // open brace
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup2
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // bracket
    let _ = lexer.next(); // balance pointer
    let _ = lexer.next(); // bracket
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // func
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Literal
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup1
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup3
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // gt
    let _ = lexer.next(); // Whitespace

    // We should find a left angle
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::LeftAngle, Span::new(184..185)));
    assert_eq!(lexer.span, Span::new(184..185));

    // The we should have an Ident
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Ident("error".to_string()), Span::new(185..190)));
    assert_eq!(lexer.span, Span::new(185..190));

    // Then should find a right angle
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::RightAngle, Span::new(190..191)));
    assert_eq!(lexer.span, Span::new(190..191));

    let _ = lexer.next(); // Whitespace

    // Jumpi Opcode
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Opcode(Opcode::Jumpi), Span::new(192..197)));
    assert_eq!(lexer.span, Span::new(192..197));

    // Eat the rest of the tokens
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // closing brace
    let _ = lexer.next(); // Whitespace

    // Get an EOF token
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(source.len()..source.len())));
    assert_eq!(lexer.span, Span::new(source.len()..source.len()));

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
