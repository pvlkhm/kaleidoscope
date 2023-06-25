use super::*;

macro_rules! test_lexer_u {
    ($str: expr, $($gold: expr),+) => {
        {
            let mut lx = Lexer::from_str($str);
            $(
                let g = match $gold {
                    Some(el) => Some((0, el, 0)),
                    None => None
                };

                assert_eq!(g, lx.next());
            )+
        }
    };
}

#[test]
fn unit_num() {
    test_lexer_u!("4.3", Some(Token::Num(4.3)));
    test_lexer_u!(".5", Some(Token::Num(0.5)));
}

#[test]
fn unit_fn() {
    test_lexer_u!("fn abc", Some(Token::Fn));
    test_lexer_u!("  fn abc", Some(Token::Fn));
}

#[test]
fn unit_sym() {
    test_lexer_u!(";", Some(Token::Semi));
    test_lexer_u!("  ; ", Some(Token::Semi));
    test_lexer_u!(",", Some(Token::Comma));
    test_lexer_u!("  ,", Some(Token::Comma));
    test_lexer_u!("(", Some(Token::OpenPar));
    test_lexer_u!("  ( ", Some(Token::OpenPar));
    test_lexer_u!(")", Some(Token::ClosePar));
    test_lexer_u!("  )", Some(Token::ClosePar));
}

#[test]
fn unit_ops() {
    test_lexer_u!(" +", Some(Token::Operation(Opcode::Add)));
    test_lexer_u!("- ", Some(Token::Operation(Opcode::Sub)));
    test_lexer_u!(" * ", Some(Token::Operation(Opcode::Mul)));
}

#[test]
fn unit_ide() {
    test_lexer_u!("  hello", Some(Token::Identifier("hello")));
    test_lexer_u!("  my_ident   ", Some(Token::Identifier("my_ident")));
}

#[test]
fn unit_eof() {
    test_lexer_u!("", None);
    test_lexer_u!("# comment", None);
    test_lexer_u!("     ", None);
}

#[test]
fn unit_cmm() {
    test_lexer_u!(r#"
        # this is test of comments
        hello # and we should run
        ;
    "#,
        Some(Token::Identifier("hello")),
        Some(Token::Semi)
    );

    test_lexer_u!(r#"fn
        #commentishere
        ,
    "#,
        Some(Token::Fn),
        Some(Token::Comma)
    );

    test_lexer_u!(r#"my_ident
        #commentishere
    "#,
        Some(Token::Identifier("my_ident")),
        None
    );
}

#[test]
fn system_s1() {
    let code = r#"
        hello();
        2.1 + .3 # here"#;

    test_lexer_u!(code,
        Some(Token::Identifier("hello")),
        Some(Token::OpenPar),
        Some(Token::ClosePar),
        Some(Token::Semi),
        Some(Token::Num(2.1)),
        Some(Token::Operation(Opcode::Add)),
        Some(Token::Num(0.3))
    )
}
