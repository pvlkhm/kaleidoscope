#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::lexer::*;

use crate::grammar::ASTFileParser;

#[test]
fn parser_init() {
    let lx = Lexer::from_str("fn hello(a) a");

    let got = ASTFileParser::new().parse(lx).unwrap();

    println!("{:?}", got);
}
